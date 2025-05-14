// // Copyright (c) nyanbot.com 2025.
// // This file is licensed under the AGPL-3.0-or-later.

use crate::solana::block::state::State;
use common::repo::Tx;
use log::debug;
use solana::jupiter::repo::SlotSwaps;
use tokio::time::Instant;

pub(crate) async fn index_swap<'a>(tx: &mut Tx<'a>, state: State, swaps: SlotSwaps) {
    let start = Instant::now();
    
    let _ = state
        .jupiter_swap_repo
        .insert_swaps(tx, swaps)
        .await
        .unwrap();

    debug!("took: {:?} ms", start.elapsed().as_millis());
}
