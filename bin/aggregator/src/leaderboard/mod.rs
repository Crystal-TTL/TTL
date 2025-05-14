// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::config::LeaderboardConfig;
use common::repo::pool::setup_pool;
use common::{ResolveOr, Signal};
use log::{error, info};
use sqlx::PgPool;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::interval;
use tokio::{select, spawn};

pub fn leaderboard_refresh(cfg: LeaderboardConfig, mut signal: Signal) -> JoinHandle<()> {
    spawn(async move {
        if cfg.active.resolve_or(false) != true {
            info!("not active");
            return;
        }

        info!("active");
        let pool = setup_pool(cfg).await;

        let mut interval = interval(Duration::from_secs(10));
        loop {
            select! {
                _ = interval.tick() => {
                    if let Err(err) = refresh_views(&pool).await {
                        error!("error refreshing materialized views: {:?}", err);
                    }
                }
                _ = signal.recv() => {
                    return;
                }
            }
        }
    })
}

async fn refresh_views(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("refresh materialized view concurrently nyanbot.point_referral")
        .execute(pool)
        .await?;
    sqlx::query("refresh materialized view concurrently nyanbot.point_check_in")
        .execute(pool)
        .await?;
    sqlx::query("refresh materialized view concurrently nyanbot.point")
        .execute(pool)
        .await?;
    Ok(())
}
