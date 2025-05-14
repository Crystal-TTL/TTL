// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::config::SolanaSolConfig;
use crate::log_ms;
use base::repo::SolRepo;
use common::repo::pool::setup_pool;
use common::ResolveOr;
use futures::future::join_all;
use log::{error, info, warn};
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::sleep;

pub fn solana_sol(cfg: SolanaSolConfig) -> JoinHandle<()> {
    tokio::spawn(async move {
        if cfg.active.resolve_or(false) != true {
            info!("not active");
            return;
        }

        info!("active");
        let pool = setup_pool(cfg).await;
        let repo = SolRepo::new();
        let mut handles = Vec::new();

        handles.push(tokio::spawn(async move {
            loop {
                if let Ok(mut tx) = pool.begin().await {
                    log_ms!("calculate_1m", async {
                        repo.calculate_1m(&mut tx).await.unwrap()
                    });

                    log_ms!("calculate_5m", async {
                        repo.calculate_5m(&mut tx).await.unwrap()
                    });

                    log_ms!("calculate_15m", async {
                        repo.calculate_15m(&mut tx).await.unwrap()
                    });

                    log_ms!("calculate_1h", async {
                        repo.calculate_1h(&mut tx).await.unwrap()
                    });

                    log_ms!("calculate_6h", async {
                        repo.calculate_6h(&mut tx).await.unwrap()
                    });

                    log_ms!("calculate_1d", async {
                        repo.calculate_1d(&mut tx).await.unwrap()
                    });

                    let _ = tx.commit().await;
                    sleep(Duration::from_secs(10)).await;
                } else {
                    warn!("Failed to acquire transaction");
                }
            }
        }));

        for result in join_all(handles).await {
            if let Err(e) = result {
                error!("Task failed: {:?}", e);
            }
        }
    })
}
