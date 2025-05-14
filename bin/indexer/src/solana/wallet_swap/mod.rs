// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::config::Config;
use crate::solana::indexer::IndexerRepo;
use base::repo::WalletSwapRepo;
use common::repo::pool::setup_pool;
use common::Signal;
use log::info;
use sqlx::PgPool;
use std::process::exit;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::interval;
use tokio::{select, spawn};

pub fn refresh_wallet_swap(config: Config, mut signal: Signal) -> JoinHandle<()> {
    spawn(async move {
        info!("active");

        let pool = setup_pool(config.postgres.clone()).await;
        let mut interval = interval(Duration::from_millis(100));
        loop {
            select! {
                _ = interval.tick() => {
                    refresh(pool.clone()).await;
                }
                _ = signal.recv() => {
                    exit(-1);
                }
            }
        }
    })
}

async fn refresh(pool: PgPool) {
    let mut tx = pool.begin().await.unwrap();

    let slot = match IndexerRepo::get_wallet_swap_slot(&mut tx).await.unwrap() {
        Some(slot) => Some(slot),
        None => IndexerRepo::get_solana_indexer_slot(&mut tx)
            .await
            .map(|i| i.slot)
            .ok(),
    };

    if let Some(slot) = slot {
        let next_slot = slot.next();

        let indexer_slot = IndexerRepo::get_solana_indexer_slot(&mut tx)
            .await
            .unwrap()
            .slot;

        // can not refresh for blocks which have not been indexed yet
        if next_slot <= indexer_slot {
            WalletSwapRepo::refresh(&mut tx, next_slot).await.unwrap();

            IndexerRepo::set_wallet_swap_slot(&mut tx, next_slot)
                .await
                .unwrap();
        }
    }
    tx.commit().await.unwrap();
}
