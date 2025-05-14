// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

#![cfg_attr(not(debug_assertions), deny(warnings))]

use aggregator::config::Config;
use aggregator::jupiter::{jupiter_candle, jupiter_mcap, jupiter_twap, jupiter_usd};
use aggregator::leaderboard::leaderboard_refresh;
use aggregator::pumpfun::{
    pumpfun_candle, pumpfun_mcap, pumpfun_progress, pumpfun_summary, pumpfun_twap, pumpfun_usd,
};
use aggregator::pumpswap::{
    pumpswap_candle, pumpswap_mcap, pumpswap_summary, pumpswap_twap, pumpswap_usd,
};
use aggregator::pumpup::{
    pumpup_candle, pumpup_mcap, pumpup_progress, pumpup_summary, pumpup_twap, pumpup_usd,
};
use aggregator::solana::solana_sol;
use common::{ResolveOr, Signal};
use futures::future::join_all;
use log::{error, info};
use tokio::runtime::Builder;
use tokio::task::JoinHandle;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

fn main() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,common=info", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = Config::load();
    info!("Start");
    let tokio_threads = cfg.tokio.threads.resolve_or(1);
    info!("tokio threads: {}", tokio_threads);

    let runtime = Builder::new_multi_thread()
        .worker_threads(tokio_threads)
        .enable_all()
        .build()
        .unwrap();

    let signal = Signal::default();

    runtime.block_on(async {
        let handles: Vec<JoinHandle<()>> = vec![
            jupiter_candle(cfg.jupiter_candle.unwrap_or_default(), signal.clone()),
            jupiter_mcap(cfg.jupiter_mcap.unwrap_or_default(), signal.clone()),
            jupiter_twap(cfg.jupiter_twap.unwrap_or_default(), signal.clone()),
            jupiter_usd(cfg.jupiter_usd.unwrap_or_default(), signal.clone()),
            pumpfun_candle(cfg.pumpfun_candle.unwrap_or_default(), signal.clone()),
            pumpfun_mcap(cfg.pumpfun_mcap.unwrap_or_default(), signal.clone()),
            pumpfun_progress(cfg.pumpfun_progress.unwrap_or_default(), signal.clone()),
            pumpfun_summary(cfg.pumpfun_summary.unwrap_or_default(), signal.clone()),
            pumpfun_twap(cfg.pumpfun_twap.unwrap_or_default(), signal.clone()),
            pumpfun_usd(cfg.pumpfun_usd.unwrap_or_default(), signal.clone()),
            pumpswap_candle(cfg.pumpswap_candle.unwrap_or_default(), signal.clone()),
            pumpswap_mcap(cfg.pumpswap_mcap.unwrap_or_default(), signal.clone()),
            pumpswap_summary(cfg.pumpswap_summary.unwrap_or_default(), signal.clone()),
            pumpswap_twap(cfg.pumpswap_twap.unwrap_or_default(), signal.clone()),
            pumpswap_usd(cfg.pumpswap_usd.unwrap_or_default(), signal.clone()),
            pumpup_candle(cfg.pumpup_candle.unwrap_or_default(), signal.clone()),
            pumpup_mcap(cfg.pumpup_mcap.unwrap_or_default(), signal.clone()),
            pumpup_progress(cfg.pumpup_progress.unwrap_or_default(), signal.clone()),
            pumpup_summary(cfg.pumpup_summary.unwrap_or_default(), signal.clone()),
            pumpup_twap(cfg.pumpup_twap.unwrap_or_default(), signal.clone()),
            pumpup_usd(cfg.pumpup_usd.unwrap_or_default(), signal.clone()),
            solana_sol(cfg.solana_sol.unwrap_or_default()),
            leaderboard_refresh(cfg.leaderboard.unwrap_or_default(), signal.clone()),
        ];

        for result in join_all(handles).await {
            if let Err(e) = result {
                error!("Task failed: {:?}", e);
            }
        }
    });

    info!("all done")
}
