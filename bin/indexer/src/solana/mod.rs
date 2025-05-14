// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::config::Config;
use crate::solana::block::index_blocks;
use crate::solana::wallet_swap::refresh_wallet_swap;
use common::Signal;
use futures::future::join_all;
use log::error;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

pub mod block;
pub mod indexer;
mod wallet_swap;
mod watchdog;

pub fn run_solana(runtime: Runtime, config: Config, signal: Signal) {
    runtime.block_on(async {
        let handles: Vec<JoinHandle<()>> = vec![
            index_blocks(config.clone(), signal.clone()),
            refresh_wallet_swap(config.clone(), signal.clone()),
        ];

        for result in join_all(handles).await {
            if let Err(e) = result {
                error!("Task failed: {:?}", e);
            }
        }
    })
}
