// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::solana::block::state::State;
use common::repo::Tx;
use log::debug;
use solana::pumpswap::repo::CurrentRepo;
use solana::pumpswap::repo::SwapsToInsert;

pub(crate) async fn index_swap<'a>(tx: &mut Tx<'a>, state: State, to_insert: SwapsToInsert) {
    let start = std::time::Instant::now();
    let inserted = state
        .pumpswap_swap_repo
        .insert_swaps(tx, to_insert)
        .await
        .unwrap();
    debug!("swap insert took: {:?} ms", start.elapsed().as_millis());

    let start = std::time::Instant::now();
    let len = inserted.len();
    CurrentRepo::upsert(&mut *tx, &inserted).await.unwrap();
    debug!(
        "current upsert {} took: {:?} ms",
        len,
        start.elapsed().as_millis()
    );
}
