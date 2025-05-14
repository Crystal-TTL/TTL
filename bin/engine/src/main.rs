// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

#![cfg_attr(not(debug_assertions), deny(warnings))]

use common::ResolveOr;
use engine::rule::start_automate;
use engine::config::Config;
use engine::handle::start_handle;
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
                format!("{}=debug,solana=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::load();
    info!("Start");
    let tokio_threads = config.tokio.threads.resolve_or(1);
    info!("tokio threads: {}", tokio_threads);

    let runtime = Builder::new_current_thread()
        .worker_threads(tokio_threads)
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let handles: Vec<JoinHandle<()>> = vec![
            start_automate(
                config.rule.unwrap_or_default(),
                config.rule_pumpfun.unwrap_or_default(),
                config.rule_pumpup.unwrap_or_default(),
            ),
            start_handle(config.handle.unwrap_or_default()),
        ];

        for result in join_all(handles).await {
            if let Err(e) = result {
                error!("Task failed: {:?}", e);
            }
        }
    });

    info!("All done")
}
