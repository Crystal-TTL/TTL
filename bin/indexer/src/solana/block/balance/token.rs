// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::solana::block::state::State;
use base::repo::TokenBalanceToInsert;
use common::repo::Tx;
use log::debug;
use tokio::time::Instant;

pub(crate) async fn index_token_balance<'a>(
    tx: &mut Tx<'a>,
    state: State,
    balances: Vec<TokenBalanceToInsert>,
) {
    let start = Instant::now();
    state.token_balance_repo.insert(tx, balances).await.unwrap();
    debug!("took: {:?} ms", start.elapsed().as_millis());
}
