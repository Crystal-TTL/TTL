// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::solana::block::state::State;
use base::repo::TokenToInsert;
use common::repo::Tx;
use log::debug;
use tokio::time::Instant;

pub(crate) async fn index_token<'a>(tx: &mut Tx<'a>, state: State, mints: Vec<TokenToInsert>) {
    let start = Instant::now();
    let inserted = state.token_repo.insert_token(tx, mints).await.unwrap();
    for inserted in &inserted {
        debug!("token created: {:?}", inserted.mint);
    }
    debug!("took: {:?} ms", start.elapsed().as_millis());
}
