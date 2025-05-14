// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

#![cfg_attr(not(debug_assertions), deny(warnings))]

use common::{ResolveOr, Signal};
use indexer::config::Config;
use indexer::solana::run_solana;
use log::info;
use tokio::runtime::Builder;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

pub fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!("{}=debug,solana=info,solana::pumpswap=debug,base=debug", env!("CARGO_CRATE_NAME")).into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::load();

    let rayon_threads = config.rayon.threads.resolve_or(1);
    info!("rayon threads: {}", rayon_threads);

    rayon::ThreadPoolBuilder::new()
        .num_threads(rayon_threads)
        .build()
        .unwrap();

    let tokio_threads = config.tokio.threads.resolve_or(1);
    info!("tokio threads: {}", tokio_threads);

    let runtime = Builder::new_multi_thread()
        .worker_threads(tokio_threads)
        .enable_all()
        .build()
        .unwrap();

    let signal = Signal::new();

    run_solana(runtime, config, signal)
}
