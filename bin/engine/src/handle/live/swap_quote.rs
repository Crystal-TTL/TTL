// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::handle::live::LiveHandler;
use base::model::requests::{SwapQuoteResult, ToProcess};
use base::model::results::fail;
use base::model::RequestToProcess;
use base::repo::RequestRepo;
use common::sql::AsSqlExecutor;
use solana::PriorityFee;

impl LiveHandler {
    pub async fn swap_quote(&self, executor: impl AsSqlExecutor, request: RequestToProcess) {
        let request_id = request.id;
        let payload: Option<SwapQuoteResult> = request.payload();
        if let Some(swap_quote) = payload {
            self.swap(
                executor,
                request.id,
                request.wallet,
                swap_quote.quote,
                PriorityFee::None,
            )
            .await
        } else {
            let _ =
                RequestRepo::fail(executor, request_id, fail("invalid SwapQuote payload")).await;
        }
    }
}
