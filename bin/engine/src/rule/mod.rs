// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::config::{RuleConfig, RulePumpfunConfig, RulePumpupConfig};
use common::ResolveOr;
use futures_util::future::join_all;
use log::error;
use tokio::task::JoinHandle;

pub mod pumpfun;
pub mod pumpup;

pub fn start_automate(
    cfg: RuleConfig,
    pf_config: RulePumpfunConfig,
    pu_config: RulePumpupConfig,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut handles = Vec::new();

        if pf_config.active.resolve_or(false) {
            handles.push(pumpfun::start_rules(cfg.clone()));
        }

        if pu_config.active.resolve_or(false) {
            handles.push(pumpup::start_rules(cfg.clone()));
        }

        for result in join_all(handles).await {
            if let Err(err) = result {
                error!("task failed {:?}", err);
            }
        }
    })
}
