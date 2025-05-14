// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::repo::{AddressRepo, BlockRepo, TokenBalanceRepo, TokenRepo, WalletRepo};
use base::test::NeverCalledTokenInfoLoader;
use indexer::solana::block::state::{State, StateInner};
use solana::pumpswap::repo::NeverCalledPoolInfoLoader;
use sqlx::PgPool;
use std::sync::Arc;

mod block_317897944;
mod block_317897944_store_block_info;
mod block_318124628;
mod block_323481688;
mod block_326027759;
mod block_326027759_wallet;
mod block_328790579;
mod block_329064506;
mod block_334886841;
mod block_334958000;
mod block_336313076;

pub(crate) fn setup(pool: PgPool) -> State {
    let token_repo = TokenRepo::testing_read_only();

    let pumpfun_swap_repo =
        solana::pumpfun::repo::SwapRepo::testing(Box::new(NeverCalledTokenInfoLoader {}));

    let pumpswap_swap_repo = solana::pumpswap::repo::SwapRepo::testing(
        Box::new(NeverCalledPoolInfoLoader {}),
        Box::new(NeverCalledTokenInfoLoader {}),
    );

    let pumpup_swap_repo =
        solana::pumpup::repo::SwapRepo::testing(Box::new(NeverCalledTokenInfoLoader {}));

    let jupiter_swap_repo =
        solana::jupiter::repo::SwapRepo::testing(Box::new(NeverCalledTokenInfoLoader {}));

    State(Arc::new(StateInner {
        block_repo: BlockRepo::new(),
        token_repo: token_repo.clone(),
        address_repo: AddressRepo::new(),
        token_balance_repo: TokenBalanceRepo::new(),
        wallet_repo: WalletRepo::new_no_secret(),
        pool: pool.clone(),
        pumpfun_swap_repo,
        pumpswap_swap_repo,
        pumpup_swap_repo,
        jupiter_swap_repo,
    }))
}

// use base::model::solana::Slot;
// use base::testing::run_test_with_pool_on_empty_db;
// use dotenv::dotenv;
// use solana::pumpswap::repo::RpcPoolInfoLoader;
// use indexer::solana::block::index_block;
// use solana::convert::convert_block;
// use solana::pumpswap::service::PumpswapService;
// use solana::rpc::Rpc;
// use solana::token_info::rpc::TokenInfoRpcLoader;
// use std::{env, fs};
//
// #[test_log::test(sqlx::test)]
// async fn prepare_test() {
//     run_test_with_pool_on_empty_db(|pool| async move {
//         dotenv().ok();
//
//         let slot = Slot::from(336313076);
//
//         let rpc_url = env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set");
//         let rpc_client = Rpc::new(rpc_url.clone());
//
//         let block = rpc_client.get_ui_block_for_testing(slot).await.unwrap();
//
//         fs::write(
//             format!("/tmp/block_{slot}.json"),
//             serde_json::to_string(&block).unwrap(),
//         )
//         .unwrap();
//
//         let block = convert_block(Slot::from(0), block).await.unwrap().unwrap();
//
//         let rpc_loader = TokenInfoRpcLoader::new(
//             env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set"),
//         );
//
//         let pumpfun_swap_repo =
//             solana::pumpfun::repo::SwapRepo::testing(Box::new(rpc_loader.clone()));
//         let pumpswap_swap_repo = solana::pumpswap::repo::SwapRepo::testing(
//             Box::new(RpcPoolInfoLoader::new(PumpswapService::new(
//                 pool.clone(),
//                 rpc_url,
//             ))),
//             Box::new(rpc_loader.clone()),
//         );
//         let pumpup_swap_repo =
//             solana::pumpup::repo::SwapRepo::testing(Box::new(rpc_loader.clone()));
//
//         let jupiter_swap_repo =
//             solana::jupiter::repo::SwapRepo::testing(Box::new(rpc_loader.clone()));
//
//         let state = State(Arc::new(StateInner {
//             token_repo: TokenRepo::new(Box::new(rpc_loader.clone())),
//             pool: pool.clone(),
//             pumpfun_swap_repo,
//             wallet_repo: WalletRepo::new_no_secret(),
//             jupiter_swap_repo,
//             address_repo: AddressRepo::new(),
//             token_balance_repo: Default::default(),
//             block_repo: BlockRepo::new(),
//             pumpswap_swap_repo,
//             pumpup_swap_repo
//         }));
//
//         index_block(state, block).await;
//     })
//     .await;
// }
