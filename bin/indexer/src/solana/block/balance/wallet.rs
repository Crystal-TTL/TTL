// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::repo::{WalletBalanceRepo, WalletBalanceSolToInsert, WalletBalanceTokenToInsert};
use common::repo::Tx;
use log::debug;
use std::time::Instant;

pub(crate) async fn index_wallet_balance_token<'a>(
    tx: &mut Tx<'a>,
    balances: Vec<WalletBalanceTokenToInsert>,
) {
    WalletBalanceRepo::insert_tokens(tx, balances)
        .await
        .unwrap();
}

pub(crate) async fn index_wallet_balance_sol<'a>(
    tx: &mut Tx<'a>,
    balances: Vec<WalletBalanceSolToInsert>,
) {
    let start = Instant::now();
    WalletBalanceRepo::insert_sol(tx, balances).await.unwrap();
    debug!("took: {:?} ms", start.elapsed().as_millis());
}
