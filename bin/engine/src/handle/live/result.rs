// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use solana::pumpfun::service::PumpfunServiceError;
use solana::pumpswap::service::PumpswapServiceError;
use solana::rpc::RpcClientUnhandledError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum LiveError {
    AccountNotFound,
    DecodingFailed,
    ExceedsSlippage,
    NotEnoughBalance,
    NotEnoughLiquidity,
    PoolNotFound,
    RecentHashNotFound,
    RecentHashOutOfDate,
    TooManyRequests,
    TokenPairNotFound,
    TokenCreatorUnknown,
    TransactionNotConfirmed,
    TransactionNotFound,
    TransactionSimulationFailed(String),
    UnableToQuote,
    Unhandled(RpcClientUnhandledError),
}

impl Display for LiveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LiveError::AccountNotFound => f.write_str("account not found"),
            LiveError::DecodingFailed => f.write_str("failed to decode"),
            LiveError::ExceedsSlippage => f.write_str("exceeds slippage"),
            LiveError::NotEnoughBalance => f.write_str("account has not enough balance"),
            LiveError::NotEnoughLiquidity => f.write_str("pool has not enough liquidity"),
            LiveError::PoolNotFound => f.write_str("pool not found"),
            LiveError::RecentHashNotFound => f.write_str("unable to retrieve recent slot and hash"),
            LiveError::RecentHashOutOfDate => {
                f.write_str("unable to retrieve recent slot and hash")
            }
            LiveError::TooManyRequests => f.write_str("too many requests"),
            LiveError::TokenPairNotFound => f.write_str("token pair not found"),
            LiveError::TokenCreatorUnknown => f.write_str("token creator unknown"),
            LiveError::TransactionNotConfirmed => f.write_str("transaction not confirmed"),
            LiveError::TransactionNotFound => f.write_str("transaction not found"),
            LiveError::TransactionSimulationFailed(msg) => {
                f.write_fmt(format_args!("transaction simulation failed: {msg}"))
            }
            LiveError::UnableToQuote => f.write_str("unable to quote"),
            LiveError::Unhandled(err) => f.write_fmt(format_args!("rpc error: {:?}", err)),
        }
    }
}
impl std::error::Error for LiveError {}

pub type LiveResult<T> = Result<T, LiveError>;

impl From<PumpfunServiceError> for LiveError {
    fn from(value: PumpfunServiceError) -> Self {
        match value {
            PumpfunServiceError::AccountNotFound => Self::AccountNotFound,
            PumpfunServiceError::NotEnoughBalance => Self::NotEnoughBalance,
            PumpfunServiceError::RecentHashNotFound => Self::RecentHashNotFound,
            PumpfunServiceError::RecentHashOutOfDate => Self::RecentHashOutOfDate,
            PumpfunServiceError::TokenPairNotFound => Self::TokenPairNotFound,
            PumpfunServiceError::TokenCreatorUnknown => Self::TokenCreatorUnknown,
            PumpfunServiceError::TooManyRequests => Self::TooManyRequests,
            PumpfunServiceError::TransactionNotConfirmed => Self::TransactionNotConfirmed,
            PumpfunServiceError::TransactionNotFound => Self::TransactionNotFound,
            PumpfunServiceError::TransactionSimulationFailed(msg) => {
                Self::TransactionSimulationFailed(msg)
            }
            PumpfunServiceError::UnableToQuote => Self::UnableToQuote,
            PumpfunServiceError::Unhandled(err) => Self::Unhandled(err),
        }
    }
}

impl From<PumpswapServiceError> for LiveError {
    fn from(value: PumpswapServiceError) -> Self {
        match value {
            PumpswapServiceError::AccountNotFound => Self::AccountNotFound,
            PumpswapServiceError::ExceedsSlippage => Self::ExceedsSlippage,
            PumpswapServiceError::DecodingFailed => Self::DecodingFailed,
            PumpswapServiceError::NotEnoughBalance => Self::NotEnoughBalance,
            PumpswapServiceError::NotEnoughLiquidity => Self::NotEnoughLiquidity,
            PumpswapServiceError::PoolNotFound => Self::PoolNotFound,
            PumpswapServiceError::RecentHashNotFound => Self::RecentHashNotFound,
            PumpswapServiceError::RecentHashOutOfDate => Self::RecentHashOutOfDate,
            PumpswapServiceError::PairNotFound => Self::TokenPairNotFound,
            PumpswapServiceError::TooManyRequests => Self::TooManyRequests,
            PumpswapServiceError::TransactionNotConfirmed => Self::TransactionNotConfirmed,
            PumpswapServiceError::TransactionNotFound => Self::TransactionNotFound,
            PumpswapServiceError::TransactionSimulationFailed(msg) => {
                Self::TransactionSimulationFailed(msg)
            }
            PumpswapServiceError::UnableToQuote => Self::UnableToQuote,
            PumpswapServiceError::Unhandled(err) => Self::Unhandled(err),
        }
    }
}
