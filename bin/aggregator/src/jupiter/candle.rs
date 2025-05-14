// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::config::JupiterCandleConfig;
use crate::{log_ms, partitioned, send_every, Worker};
use async_trait::async_trait;
use common::model::Partition;
use common::repo::pool::setup_pool;
use common::{ResolveOr, Signal};
use log::{info, warn};
use solana::jupiter::repo::CandleRepo;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;
use tokio::spawn;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

struct S1Worker {
    pool: PgPool,
}

#[async_trait]
impl Worker<Partition> for S1Worker {
    async fn process(&self, partition: Partition) {
        let repo = CandleRepo::new();

        loop {
            if let Ok(mut tx) = self.pool.begin().await {
                log_ms!("1s", partition, async {
                    repo.calculate_1s(&mut tx, partition).await.unwrap();
                });
                let _ = tx.commit().await;
                return;
            } else {
                warn!("failed to acquire transaction - {partition:?}");
            }
        }
    }
}

struct CandleWorker {
    pool: PgPool,
}

#[async_trait]
impl Worker<Partition> for CandleWorker {
    async fn process(&self, partition: Partition) {
        let repo = CandleRepo::new();

        loop {
            if let Ok(mut tx) = self.pool.begin().await {
                log_ms!("1m", partition, async {
                    repo.calculate_1m(&mut tx, partition).await.unwrap();
                });

                log_ms!("5m", partition, async {
                    repo.calculate_5m(&mut tx, partition).await.unwrap();
                });

                log_ms!("15m", partition, async {
                    repo.calculate_15m(&mut tx, partition).await.unwrap();
                });

                log_ms!("1h", partition, async {
                    repo.calculate_1h(&mut tx, partition).await.unwrap();
                });

                log_ms!("6h", partition, async {
                    repo.calculate_6h(&mut tx, partition).await.unwrap();
                });

                log_ms!("1d", partition, async {
                    repo.calculate_1d(&mut tx, partition).await.unwrap();
                });

                let _ = tx.commit().await;
                return;
            } else {
                warn!("failed to acquire transaction - {partition:?}");
            }
        }
    }
}

pub fn jupiter_candle(cfg: JupiterCandleConfig, signal: Signal) -> JoinHandle<()> {
    spawn(async move {
        if cfg.active.resolve_or(false) != true {
            info!("not active");
            return;
        }

        info!("active");
        let pool = setup_pool(cfg).await;

        let s1_pool = pool.clone();
        let s1_signal = signal.clone();
        spawn(async move {
            let mut senders = Vec::new();
            let mut receivers = Vec::new();

            for _ in Partition::enumerate() {
                let (tx, rx) = mpsc::channel::<Partition>(1);
                senders.push(tx);
                receivers.push(rx);
            }

            spawn(partitioned(
                s1_signal,
                receivers,
                Arc::new(S1Worker { pool: s1_pool }),
            ));

            send_every(senders, Duration::from_secs(1)).await;
        });

        let mut senders = Vec::new();
        let mut receivers = Vec::new();

        for _ in Partition::enumerate() {
            let (tx, rx) = mpsc::channel::<Partition>(1);
            senders.push(tx);
            receivers.push(rx);
        }

        spawn(partitioned(
            signal,
            receivers,
            Arc::new(CandleWorker { pool }),
        ));

        send_every(senders, Duration::from_secs(1)).await;
    })
}
