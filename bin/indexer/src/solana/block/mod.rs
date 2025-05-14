// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::config::Config;
use crate::solana::block::balance::{
    index_token_balance, index_wallet_balance_sol, index_wallet_balance_token,
};
use crate::solana::block::state::{State, StateInner};
use crate::solana::indexer::IndexerRepo;
use crate::solana::watchdog::Watchdog;
use base::model::solana::{Block, Slot, TransactionStatus};
use base::model::{AddressId, Mint, PublicKey, TokenId, WalletId};
use base::repo::{
    AddressRepo, BlockRepo, BlockToInsert, TokenBalanceRepo, TokenBalanceToInsert, TokenPairRepo,
    TokenRepo, TokenToInsert, WalletBalanceSolToInsert, WalletBalanceTokenToInsert, WalletRepo,
};
use common::model::{DecimalAmount, Decimals};
use common::repo::pool::setup_pool;
use common::{ResolveOr, Signal};
use solana::jupiter::parse::JupiterParser;
use solana::parse::InstructionParser;
use solana::pumpfun::PumpFunParser;
use solana::pumpswap::parse::idl_type::{BuyEvent, SellEvent};
use solana::pumpswap::parse::{Instruction, PumpSwapParser};
use solana::pumpswap::repo::RpcPoolInfoLoader;
use solana::pumpswap::service::PumpswapService;
use solana::pumpup::parse::PumpUpParser;
use solana::stream::{BlockStream, RpcBlockStream, RpcBlockStreamConfig, WsSlotStream};
use solana::token_info::rpc::TokenInfoRpcLoader;
use std::collections::{HashMap, HashSet};
use std::process::exit;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::signal::unix::SignalKind;
use tokio::sync::mpsc::channel;
use tokio::task::JoinHandle;
use tokio::{join, select, spawn};
use tracing::{debug, info};

mod balance;
mod jupiter;
mod pumpfun;
mod pumpswap;
mod pumpup;
pub mod state;

pub fn index_blocks(config: Config, signal: Signal) -> JoinHandle<()> {
    spawn(async move {
        log::info!("active");

        let pool = setup_pool(config.postgres.clone()).await;

        let token_info_loader = TokenInfoRpcLoader::new(config.rpc.url.resolve());
        let token_repo = TokenRepo::new(Box::new(token_info_loader));
        let token_pair_repo = TokenPairRepo::new(token_repo.clone());

        let address_repo = AddressRepo::new();

        let pumpfun_swap_repo =
            solana::pumpfun::repo::SwapRepo::new(token_pair_repo.clone(), address_repo.clone());
        let pumpswap_swap_repo = solana::pumpswap::repo::SwapRepo::new(
            solana::pumpswap::repo::PoolRepo::new(
                Box::new(RpcPoolInfoLoader::new(PumpswapService::new(
                    pool.clone(),
                    config.rpc.url.resolve(),
                ))),
                address_repo.clone(),
                token_pair_repo.clone(),
            ),
            address_repo.clone(),
        );
        let pumpup_swap_repo =
            solana::pumpup::repo::SwapRepo::new(token_pair_repo.clone(), address_repo.clone());

        let jupiter_swap_repo =
            solana::jupiter::repo::SwapRepo::new(token_pair_repo.clone(), address_repo.clone());

        let state = State(Arc::new(StateInner {
            pool: pool.clone(),
            block_repo: BlockRepo::new(),
            token_repo: token_repo.clone(),
            address_repo: AddressRepo::new(),
            token_balance_repo: TokenBalanceRepo::new(),
            wallet_repo: WalletRepo::new_no_secret(),
            pumpfun_swap_repo,
            pumpswap_swap_repo,
            pumpup_swap_repo,
            jupiter_swap_repo,
        }));

        let sig = signal.clone();
        tokio::spawn(async move {
            let mut sigterm = tokio::signal::unix::signal(SignalKind::terminate()).unwrap();
            select! {
                _ = sigterm.recv() => {
                    info!("Received SIGTERM. Cleaning up resources...");
                    sig.shutdown();
                }
            }
        });

        let slot_stream = WsSlotStream::new(
            config
                .slotstream
                .url
                .resolve_or("ws://api.mainnet-beta.solana.com".to_string()),
            Some(config.slotstream.delay.resolve_or(Slot::from(0))),
        )
        .await;

        let mut tx = pool.begin().await.unwrap();
        let previous_slot = IndexerRepo::get_solana_indexer_slot(&mut tx)
            .await
            .map(|i| i.slot)
            .ok();
        tx.commit().await.unwrap();

        let (mut blocks, block_stream_handle) = RpcBlockStream::new(
            RpcBlockStreamConfig {
                url: config
                    .blockstream
                    .url
                    .resolve_or("http://api.mainnet-beta.solana.com".to_string())
                    .into(),
                concurrency: config.blockstream.concurrency.resolve_or(1usize),
            },
            slot_stream,
            previous_slot,
        )
        .stream(signal.clone())
        .await;

        let (watchdog_tx, watchdog_rx) = channel::<Slot>(10);
        let watchdog = Watchdog::new(watchdog_rx, Duration::from_secs(30), signal.clone());

        let mut signal = signal.clone();
        let handle = tokio::spawn(async move {
            loop {
                select! {
                     Some(block) = blocks.recv() => {
                        let slot = block.slot.clone();
                        index_block(state.clone(),block).await;
                        let _ = watchdog_tx.send(slot).await;
                     },
                    _ = signal.recv() => {
                        exit(-1);
                    }
                }
            }
        });

        let _ = join!(block_stream_handle, handle, watchdog.spawn());
        ()
    })
}

pub async fn index_block(state: State, block: Block) {
    info!("index {}", block.slot);

    let pumpfun_account =
        PublicKey::from_str("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P").unwrap();

    let pumpswap_account =
        PublicKey::from_str("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA").unwrap();

    let jupiter_account =
        PublicKey::from_str("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4").unwrap();

    let pumpup_account =
        PublicKey::from_str("PdMDrKEMaX8q7CCJb7NvUCxerBCcsFUa4LjBEynTtEd").unwrap();

    // FIXME it would be interesting to see what the time difference is between indexing a block and the actual block time

    let mut jupiter_swaps_to_insert = solana::jupiter::repo::SlotSwaps {
        slot: block.slot,
        timestamp: block.timestamp,
        swaps: vec![],
    };

    let mut pumpfun_swaps_to_insert = solana::pumpfun::repo::SwapsToInsert {
        slot: block.slot,
        timestamp: block.timestamp,
        swaps: vec![],
    };

    let mut pumpswap_swaps_to_insert = solana::pumpswap::repo::SwapsToInsert {
        slot: block.slot,
        timestamp: block.timestamp.0,
        swaps: vec![],
    };

    let mut pumpup_swaps_to_insert = solana::pumpup::repo::SwapsToInsert {
        slot: block.slot,
        timestamp: block.timestamp,
        swaps: vec![],
    };

    let mut pumpfun_mints = vec![];

    let tx_parsing_start = Instant::now();

    for transaction in &block.transactions {
        if transaction.status == TransactionStatus::Success {
            if transaction.keys.contains(&pumpswap_account) {
                if let Ok(instructions) = PumpSwapParser::parse_instructions(&transaction) {
                    for instruction in instructions {
                        match instruction {
                            Instruction::BuyEvent(BuyEvent {
                                pool,
                                user,
                                base_amount_out,
                                quote_amount_in_with_lp_fee,
                                pool_base_token_reserves,
                                pool_quote_token_reserves,
                                ..
                            }) => {
                                pumpswap_swaps_to_insert.swaps.push(
                                    solana::pumpswap::repo::SwapToInsert {
                                        pool,
                                        amount_base: base_amount_out,
                                        amount_quote: quote_amount_in_with_lp_fee,
                                        is_buy: true,
                                        signer: user,
                                        base_reserves: pool_base_token_reserves,
                                        quote_reserves: pool_quote_token_reserves,
                                        signature: transaction.signature.clone(),
                                    },
                                );
                            }
                            Instruction::SellEvent(SellEvent {
                                pool,
                                user,
                                base_amount_in,
                                user_quote_amount_out,
                                pool_base_token_reserves,
                                pool_quote_token_reserves,
                                ..
                            }) => {
                                pumpswap_swaps_to_insert.swaps.push(
                                    solana::pumpswap::repo::SwapToInsert {
                                        pool,
                                        amount_base: base_amount_in,
                                        amount_quote: user_quote_amount_out,
                                        is_buy: false,
                                        signer: user,
                                        base_reserves: pool_base_token_reserves,
                                        quote_reserves: pool_quote_token_reserves,
                                        signature: transaction.signature.clone(),
                                    },
                                );
                            }
                        }
                    }
                }
            }

            if transaction.keys.contains(&pumpfun_account) {
                if let Ok(instructions) = PumpFunParser::parse_instructions(&transaction) {
                    for instruction in instructions {
                        match instruction {
                            solana::pumpfun::model::Instruction::Create {
                                name,
                                symbol,
                                uri,
                                mint,
                                user,
                                ..
                            } => {
                                let mut tx = state.pool.begin().await.unwrap();
                                let creator = state
                                    .address_repo
                                    .get_or_populate_by_key(&mut tx, user)
                                    .await
                                    .unwrap();

                                tx.commit().await.unwrap();

                                pumpfun_mints.push(TokenToInsert {
                                    mint,
                                    name: Some(name),
                                    symbol: Some(symbol),
                                    decimals: Decimals::from(6),
                                    supply: Some(DecimalAmount::from(1_000_000_000i64)),
                                    metadata: Some(uri),
                                    description: None,
                                    image: None,
                                    website: None,
                                    creator: Some(creator.id),
                                    block: Some(block.slot.into()),
                                    block_time: Some(block.timestamp),
                                })
                            }

                            solana::pumpfun::model::Instruction::Swap {
                                mint,
                                sol_amount,
                                token_amount,
                                is_buy,
                                user,
                                virtual_sol_reserves,
                                virtual_token_reserves,
                                ..
                            } => {
                                pumpfun_swaps_to_insert.swaps.push(
                                    solana::pumpfun::repo::SwapToInsert {
                                        base: mint,
                                        amount_base: token_amount,
                                        amount_quote: sol_amount,
                                        is_buy,
                                        wallet: user,
                                        virtual_base_reserves: virtual_token_reserves,
                                        virtual_quote_reserves: virtual_sol_reserves,
                                        signature: transaction.signature.clone(),
                                    },
                                );
                            }
                        }
                    }
                }
            }

            if transaction.keys.contains(&jupiter_account) {
                if let Ok(instructions) = JupiterParser::parse_instructions(&transaction) {
                    for instruction in instructions {
                        match instruction {
                            solana::jupiter::model::Instruction::Swap { swaps, signer } => {
                                for swap in &swaps {
                                    jupiter_swaps_to_insert.swaps.push(
                                        solana::jupiter::repo::SlotSwap {
                                            input_mint: swap.input_mint.clone(),
                                            input_amount: swap.input_amount.clone(),
                                            output_mint: swap.output_mint.clone(),
                                            output_amount: swap.output_amount.clone(),
                                            wallet: signer.clone(),
                                            signature: transaction.signature.clone(),
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
            }

            if transaction.keys.contains(&pumpup_account) {
                if let Ok(instructions) = PumpUpParser::parse_instructions(&transaction) {
                    for instruction in instructions {
                        match instruction {
                            solana::pumpup::parse::Instruction::SwapEvent(
                                solana::pumpup::parse::idl_type::SwapEvent {
                                    mint,
                                    sol_amount,
                                    token_amount,
                                    ai_token_amount,
                                    is_buy,
                                    user,
                                    timestamp: _timestamp,
                                    pumpup_fee: _pumpup_fee,
                                    pool_real_sol_amount,
                                    pool_sol_reserves,
                                    pool_token_reserves,
                                },
                            ) => {
                                pumpup_swaps_to_insert.swaps.push(
                                    solana::pumpup::repo::SwapToInsert {
                                        base: mint,
                                        amount_base: token_amount,
                                        amount_quote: sol_amount,
                                        amount_ai: ai_token_amount,
                                        is_buy,
                                        wallet: user,
                                        base_reserves: pool_token_reserves,
                                        quote_reserves: pool_sol_reserves,
                                        real_quote_reserves: pool_real_sol_amount,
                                        signature: transaction.signature.clone(),
                                    },
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    let tx_parsing_done = Instant::now();
    debug!(
        "transaction parsing took {} ms",
        tx_parsing_done.duration_since(tx_parsing_start).as_millis()
    );

    let mut tx = state.pool.begin().await.unwrap();
    let slot = block.slot;

    let indexing_start = Instant::now();
    pumpfun::index_token(&mut tx, state.clone(), pumpfun_mints).await;
    pumpfun::index_swap(&mut tx, state.clone(), pumpfun_swaps_to_insert).await;
    pumpswap::index_swap(&mut tx, state.clone(), pumpswap_swaps_to_insert).await;
    pumpup::index_swap(&mut tx, state.clone(), pumpup_swaps_to_insert).await;
    jupiter::index_swap(&mut tx, state.clone(), jupiter_swaps_to_insert).await;

    //////////////// track balance

    let mut seen_addresses = HashSet::new();
    let mut addresses = Vec::new();

    let mut seen_mints = HashSet::new();
    let mut mints = Vec::new();

    for transaction in &block.transactions {
        // only track account balances of tokens traded in supported venues
        if transaction.keys.contains(&jupiter_account)
            || transaction.keys.contains(&pumpfun_account)
            || transaction.keys.contains(&pumpswap_account)
            || transaction.keys.contains(&pumpup_account)
        {
            if transaction.status == TransactionStatus::Success {
                for t in &transaction.balance.token {
                    if seen_addresses.insert(t.address.clone()) {
                        addresses.push(t.address.clone());
                    }

                    if seen_mints.insert(t.mint.clone()) {
                        mints.push(t.mint.clone());
                    }
                }
            }
        }
    }

    let addresses = state
        .address_repo
        .list_or_populate(&mut tx, addresses)
        .await
        .unwrap();

    let tokens = state
        .token_repo
        .list_or_populate(&mut tx, mints)
        .await
        .unwrap();

    let wallets = state.wallet_repo.list_all(&mut tx).await.unwrap();

    let addresses: HashMap<PublicKey, AddressId> =
        addresses.into_iter().map(|a| (a.address, a.id)).collect();

    let tokens: HashMap<Mint, TokenId> = tokens.into_iter().map(|m| (m.mint, m.id)).collect();

    let mut token_balances: Vec<TokenBalanceToInsert> = vec![];
    for transaction in &block.transactions {
        if transaction.status == TransactionStatus::Success {
            // only track account balances of tokens traded in supported venues
            if transaction.keys.contains(&jupiter_account)
                || transaction.keys.contains(&pumpfun_account)
                || transaction.keys.contains(&pumpswap_account)
                || transaction.keys.contains(&pumpup_account)
            {
                for token in &transaction.balance.token {
                    token_balances.push(TokenBalanceToInsert {
                        slot: block.slot,
                        timestamp: block.timestamp.0,
                        address: addresses[&token.address],
                        token: tokens[&token.mint].clone(),
                        pre: token.pre.clone(),
                        post: token.post.clone(),
                    })
                }
            }
        }
    }

    let wallets: HashMap<PublicKey, WalletId> =
        wallets.into_iter().map(|w| (w.public_key, w.id)).collect();

    let mut wallet_balance_tokens = Vec::new();
    let mut wallet_sol_balances = Vec::new();

    for transaction in block.transactions {
        if transaction.status == TransactionStatus::Success {
            if transaction.keys.contains(&jupiter_account)
                || transaction.keys.contains(&pumpfun_account)
                || transaction.keys.contains(&pumpswap_account)
                || transaction.keys.contains(&pumpup_account)
            {
                for token in transaction.balance.token {
                    if let Some(wallet) = wallets.get(&token.address) {
                        wallet_balance_tokens.push(WalletBalanceTokenToInsert {
                            slot: block.slot,
                            timestamp: block.timestamp,
                            wallet: wallet.clone(),
                            token: tokens[&token.mint].clone(),
                            pre: token.pre,
                            post: token.post,
                        })
                    }
                }
            }

            for sol in transaction.balance.sol {
                if let Some(wallet) = wallets.get(&sol.address) {
                    wallet_sol_balances.push(WalletBalanceSolToInsert {
                        slot: block.slot,
                        timestamp: block.timestamp,
                        wallet: wallet.clone(),
                        pre: sol.pre,
                        post: sol.post,
                    })
                }
            }
        }
    }

    index_token_balance(&mut tx, state.clone(), token_balances).await;
    index_wallet_balance_token(&mut tx, wallet_balance_tokens).await;
    index_wallet_balance_sol(&mut tx, wallet_sol_balances).await;

    let _ = state
        .block_repo
        .insert(
            &mut tx,
            [BlockToInsert {
                slot,
                hash: block.hash,
                timestamp: block.timestamp,
            }],
        )
        .await
        .unwrap();

    let indexing_done = Instant::now();

    debug!(
        "indexing took {} ms",
        indexing_done.duration_since(indexing_start).as_millis()
    );

    IndexerRepo::set_solana_indexer_slot(&mut tx, slot)
        .await
        .unwrap();
    tx.commit().await.unwrap();
}
