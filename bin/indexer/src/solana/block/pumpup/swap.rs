// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::solana::block::state::State;
use common::repo::Tx;
use log::debug;
use solana::pumpup::repo::{CurrentRepo, SwapsToInsert};
use tokio::time::Instant;

pub(crate) async fn index_swap<'a>(tx: &mut Tx<'a>, state: State, swaps: SwapsToInsert) {
    let start = Instant::now();
    let inserted = state
        .pumpup_swap_repo
        .insert_swaps(tx, swaps)
        .await
        .unwrap();
    debug!("swap insert took: {:?} ms", start.elapsed().as_millis());

    let start = Instant::now();
    for swap in inserted {
        CurrentRepo::upsert(&mut *tx, swap).await.unwrap();
    }
    debug!("current upsert took: {:?} ms", start.elapsed().as_millis());
}
