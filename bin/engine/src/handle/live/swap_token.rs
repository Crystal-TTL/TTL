// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::handle::live::LiveHandler;
use base::model::requests::{SwapQuoteRequest, ToProcess};
use base::model::results::fail;
use base::model::RequestToProcess;
use base::repo::RequestRepo;
use common::sql::AsSqlExecutor;
use solana::PriorityFee;

impl LiveHandler {
    pub async fn swap_token(&self, executor: impl AsSqlExecutor, request: RequestToProcess) {
        let request_id = request.id;
        let payload: Option<SwapQuoteRequest> = request.payload();
        if let Some(swap_token) = payload {
            let Some(quote) = self.quote_service.quote(swap_token.request).await else {
                let _ =
                    RequestRepo::fail(executor, request_id, fail("failed to retrieve quote")).await;
                return;
            };
            self.swap(
                executor,
                request.id,
                request.wallet,
                quote,
                PriorityFee::None,
            )
            .await
        } else {
            let _ =
                RequestRepo::fail(executor, request_id, fail("invalid SwapToken payload")).await;
        }
    }
}
