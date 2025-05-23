// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::repo::{AddressRepo, BlockRepo, TokenBalanceRepo, TokenRepo, WalletRepo};
use sqlx::PgPool;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone)]
pub struct State(pub Arc<StateInner>);

impl Deref for State {
    type Target = StateInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

pub struct StateInner {
    pub pool: PgPool,
    pub block_repo: BlockRepo,
    pub token_repo: TokenRepo,
    pub address_repo: AddressRepo,
    pub token_balance_repo: TokenBalanceRepo,
    pub wallet_repo: WalletRepo,
    pub pumpfun_swap_repo: solana::pumpfun::repo::SwapRepo,
    pub pumpswap_swap_repo: solana::pumpswap::repo::SwapRepo,
    pub pumpup_swap_repo: solana::pumpup::repo::SwapRepo,
    pub jupiter_swap_repo: solana::jupiter::repo::SwapRepo,
}
