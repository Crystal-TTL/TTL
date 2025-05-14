// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::handle::live::result::LiveResult;
use crate::handle::live::LiveHandler;
use base::model::results::{fail, ConfirmedTransaction};
use base::model::{KeyPair, QuoteResult, RequestId, WalletId};
use base::repo::{RequestRepo, ResultSwapToInsert, ResulttRepo};
use common::model::TransactionHash;
use common::sql::AsSqlExecutor;
use log::error;
use solana::PriorityFee;

impl LiveHandler {
    pub(crate) async fn swap(
        &self,
        mut executor: impl AsSqlExecutor,
        request: impl Into<RequestId>,
        wallet: impl Into<WalletId>,
        quote: QuoteResult,
        priority_fee: PriorityFee,
    ) {
        let request = request.into();
        let wallet = wallet.into();

        let pk = match self
            .wallet_repo
            .get_private_key(executor.as_executor(), wallet)
            .await
        {
            Ok(signer) => signer,
            Err(_err) => {
                error!("failed to get wallet private key");
                let _ =
                    RequestRepo::fail(executor, request, fail("failed to get private key")).await;
                return;
            }
        };
        let signer = KeyPair::from_base58(pk.0);

        let result = self.execute_swap(signer, quote.clone(), priority_fee).await;
        Self::handle_result(executor, wallet, request, result, quote).await;
    }

    async fn execute_swap<'a>(
        &self,
        signer: KeyPair,
        quote: QuoteResult,
        priority_fee: PriorityFee,
    ) -> LiveResult<TransactionHash> {
        let service = self
            .swap_services
            .get(&quote.venue)
            .expect("unable to get swap service");

        service.swap(signer, quote, priority_fee).await
    }

    async fn handle_result<'a>(
        mut executor: impl AsSqlExecutor,
        wallet: impl Into<WalletId>,
        request: impl Into<RequestId>,
        result: LiveResult<TransactionHash>,
        quote: QuoteResult,
    ) {
        let wallet = wallet.into();
        let request = request.into();

        match result {
            Ok(hash) => {
                if let Some(err) = RequestRepo::complete(
                    executor.as_executor(),
                    request,
                    ConfirmedTransaction { hash: hash.clone() },
                )
                .await
                .err()
                {
                    error!("failed to complete request: {:?}", err);
                    return;
                }

                if let Some(err) = ResulttRepo::insert_swap(
                    executor,
                    ResultSwapToInsert {
                        id: request,
                        wallet,
                        quote,
                        swap_hash: hash.clone(),
                    },
                )
                .await
                .err()
                {
                    error!("failed to insert swap: {}", err);
                    return;
                }
            }
            Err(err) => {
                error!("swap failed: {:?}", err);
                let _ = RequestRepo::fail(executor, request, fail(err.to_string())).await;
            }
        }
    }
}
