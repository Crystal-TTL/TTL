// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

pub mod result;
mod service;
mod swap;
mod swap_quote;
mod swap_token;

pub use crate::handle::live::service::SwapService;
pub use crate::handle::Handler;

use async_trait::async_trait;
use base::model::Venue::PumpSwap;
use base::model::{CreateQuote, RequestToProcess, RequestType, Venue};
use base::repo::WalletRepo;
use base::service::QuoteService;
use common::crypt::SecretKey;
use common::model::RpcUrl;
use common::repo::Tx;
use solana::pumpfun::service::PumpfunService;
use solana::pumpswap::service::PumpswapService;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use Venue::PumpFun;

pub struct LiveHandler {
    pub quote_service: QuoteService,
    pub swap_services: HashMap<Venue, Arc<dyn SwapService>>,
    pub wallet_repo: WalletRepo,
}

impl LiveHandler {
    pub fn new(pool: PgPool, secret: SecretKey, rpc_url: impl Into<RpcUrl>) -> Self {
        let rpc_url = rpc_url.into();

        let pumpfun = Arc::new(PumpfunService::new(pool.clone(), rpc_url.clone()));
        let pumpswap = Arc::new(PumpswapService::new(pool, rpc_url));

        let mut swap_services = HashMap::new();
        swap_services.insert(PumpFun, pumpfun.clone() as Arc<dyn SwapService>);
        swap_services.insert(PumpSwap, pumpswap.clone() as Arc<dyn SwapService>);

        Self {
            quote_service: QuoteService::new([
                pumpfun.clone() as Arc<dyn CreateQuote>,
                pumpswap.clone() as Arc<dyn CreateQuote>,
            ]),
            swap_services,
            wallet_repo: WalletRepo { secret },
        }
    }

    pub fn testing(
        swap_services: HashMap<Venue, Arc<dyn SwapService>>,
        quote_service: QuoteService,
    ) -> Self {
        Self {
            quote_service,
            swap_services,
            wallet_repo: WalletRepo {
                secret: "3d7948d31771b3924dbeec3de83d905580d988c84964a6afd4c9cedd06776e91".into(),
            },
        }
    }
}

#[async_trait]
impl Handler for LiveHandler {
    async fn handle<'a>(&self, tx: &mut Tx<'a>, request: RequestToProcess) {
        match request.request {
            RequestType::SendNative => unimplemented!(),
            RequestType::SendToken => unimplemented!(),
            RequestType::SwapQuoteRequest => self.swap_token(tx, request).await,
            RequestType::SwapQuoteResult => self.swap_quote(tx, request).await,
        }
    }
}
