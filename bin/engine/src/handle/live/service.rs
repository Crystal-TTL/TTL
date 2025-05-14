// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::handle::live::result::LiveResult;
use async_trait::async_trait;
use base::model::QuoteDirection::Buy;
use base::model::{KeyPair, QuoteResult};
use common::model::TransactionHash;
use solana::pumpfun::service::PumpfunService;
use solana::pumpswap::service::PumpswapService;
use solana::PriorityFee;

#[async_trait]
pub trait SwapService: Send + Sync {
    async fn swap(
        &self,
        signer: KeyPair,
        quote: QuoteResult,
        priority_fee: PriorityFee,
    ) -> LiveResult<TransactionHash>;
}

#[async_trait]
impl SwapService for PumpfunService {
    async fn swap(
        &self,
        signer: KeyPair,
        quote: QuoteResult,
        priority_fee: PriorityFee,
    ) -> LiveResult<TransactionHash> {
        if quote.direction == Buy {
            Ok(self.buy(signer, quote, priority_fee).await?)
        } else {
            Ok(self.sell(signer, quote, priority_fee).await?)
        }
    }
}

#[async_trait]
impl SwapService for PumpswapService {
    async fn swap(
        &self,
        signer: KeyPair,
        quote: QuoteResult,
        priority_fee: PriorityFee,
    ) -> LiveResult<TransactionHash> {
        if quote.direction == Buy {
            Ok(self.buy(signer, quote, priority_fee).await?)
        } else {
            Ok(self.sell(signer, quote, priority_fee).await?)
        }
    }
}
