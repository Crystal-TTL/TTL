// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use async_trait::async_trait;
use base::model::{
    CreateQuote, GetQuotePrice, KeyPair, Mint, QuoteDirection, QuoteMode, QuotePair, QuoteRequest,
    QuoteResult, QuoteToken, QuotedPrice, TokenPairId, Venue,
};
use base::service::QuoteService;
use bigdecimal::BigDecimal;
use common::model::{BasisPoints, DecimalAmount, PriceQuote, TransactionHash};
use engine::handle::result::LiveResult;
use engine::handle::{LiveHandler, SwapService};
use solana::PriorityFee;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

mod swap;
mod swap_quote;
mod swap_token;

pub(crate) struct TestQuoteService {}

#[async_trait]
impl GetQuotePrice for TestQuoteService {
    async fn get_quote_price(&self, _pair: TokenPairId) -> Option<QuotedPrice> {
        Some(QuotedPrice {
            has_graduated: None,
            pool: None,
            quote: 12345i64.into(),
            usd: None,
        })
    }
}

impl CreateQuote for TestQuoteService {
    fn venue(&self) -> Venue {
        Venue::Raydium
    }
}

pub(crate) struct TestFailingQuoteService {}

#[async_trait]
impl GetQuotePrice for TestFailingQuoteService {
    async fn get_quote_price(&self, _pair: TokenPairId) -> Option<QuotedPrice> {
        None
    }
}

impl CreateQuote for TestFailingQuoteService {
    fn venue(&self) -> Venue {
        Venue::Raydium
    }
}

pub(crate) struct TestSwapService {}

#[async_trait]
impl SwapService for TestSwapService {
    async fn swap(
        &self,
        _signer: KeyPair,
        _quote: QuoteResult,
        _priority_fee: PriorityFee,
    ) -> LiveResult<TransactionHash> {
        Ok("SomeTransactionHash".into())
    }
}

fn test_quote_request() -> QuoteRequest {
    QuoteRequest {
        pair: test_pair(),
        direction: QuoteDirection::Buy,
        mode: QuoteMode::ExactIn {
            amount: 12345i64.into(),
        },
        slippage: BasisPoints::from(100),
        venue: Some(Venue::Raydium),
    }
}

fn test_quote() -> QuoteResult {
    QuoteResult {
        pair: test_pair(),
        direction: QuoteDirection::Buy,
        mode: QuoteMode::ExactIn {
            amount: 12345i64.into(),
        },
        slippage: BasisPoints::from(100),
        venue: Venue::Raydium,
        price: QuotedPrice {
            has_graduated: None,
            pool: None,
            quote: PriceQuote(BigDecimal::from_str("42").unwrap()),
            usd: None,
        },
        estimated_base_amount: DecimalAmount::from(1i64),
        estimated_quote_amount: DecimalAmount::from(2i64),
        worst_case_base_amount: DecimalAmount::from(3i64),
        worst_case_quote_amount: DecimalAmount::from(4i64),
    }
}

fn test_pair() -> QuotePair {
    QuotePair {
        id: 123.into(),
        base: QuoteToken {
            id: 1.into(),
            mint: Mint::wsol(),
            decimals: 9.into(),
            symbol: None,
            creator: None,
        },
        quote: QuoteToken {
            id: 2.into(),
            mint: Mint::usdt(),
            decimals: 6.into(),
            symbol: None,
            creator: None,
        },
    }
}

fn test_instance() -> LiveHandler {
    let mut map = HashMap::new();
    map.insert(
        Venue::Raydium,
        Arc::new(TestSwapService {}) as Arc<dyn SwapService>,
    );

    LiveHandler::testing(
        map,
        QuoteService::new([Arc::new(TestQuoteService {}) as Arc<dyn CreateQuote>]),
    )
}
