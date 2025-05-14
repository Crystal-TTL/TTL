// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

pub use live::*;

mod live;
mod mock;

use crate::config::HandleConfig;
use crate::handle::mock::MockHandler;
use async_trait::async_trait;
use base::model::RequestToProcess;
use base::repo::RequestRepo;
use common::crypt::SecretKey;
use common::repo::pool::setup_pool;
use common::repo::Tx;
use common::{ConfigValue, ResolveOr};
use log::info;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::sleep;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Mode {
    Mock,
    Live,
}

impl ResolveOr<Mode> for ConfigValue {
    fn resolve_or(&self, default: Mode) -> Mode {
        self.try_resolve()
            .ok()
            .and_then(|s| match s.as_str() {
                "LIVE" => Some(Mode::Live),
                _ => None,
            })
            .unwrap_or(default)
    }
}

#[async_trait]
pub trait Handler: Send {
    async fn handle<'a>(&self, tx: &mut Tx<'a>, request: RequestToProcess);
}

pub fn start_handle(cfg: HandleConfig) -> JoinHandle<()> {
    tokio::spawn(async move {
        if cfg.active.resolve_or(false) != true {
            info!("not active");
            return;
        }

        info!("active");
        let mode = cfg.mode.resolve_or(Mode::Mock);
        info!("Mode: {:?}", mode);

        let pool = setup_pool(cfg.clone()).await;

        let handler: Box<dyn Handler> = match mode {
            Mode::Mock => Box::new(MockHandler {}),
            Mode::Live => Box::new(LiveHandler::new(
                pool.clone(),
                SecretKey::from(cfg.secret.resolve()),
                cfg.rpc_url
                    .resolve_or("https://api.mainnet-beta.solana.com".to_string()),
            )),
        };

        loop {
            let mut tx = pool.begin().await.unwrap();
            if let Some((_attempt, request)) = RequestRepo::attempt(&mut *tx).await.unwrap() {
                handler.handle(&mut tx, request).await;
                tx.commit().await.unwrap();
            }
            sleep(Duration::from_millis(1000)).await;
        }
    })
}
