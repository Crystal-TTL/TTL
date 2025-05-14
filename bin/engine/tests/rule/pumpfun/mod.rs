// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::repo::{NotificationRepo, TokenPairRepo, TokenRepo};
use base::service::{NotificationService, RuleService};
use engine::rule::pumpfun::FactService;
use engine::rule::pumpfun::state::{Service, State, StateInner};
use solana::pumpfun::repo::SummaryRepo;
use sqlx::PgPool;
use std::sync::Arc;

mod pf_001;

pub(crate) fn setup(pool: PgPool) -> State {
    State(Arc::new(StateInner {
        pool: pool.clone(),
        service: Service {
            fact: FactService::new(
                pool.clone(),
                TokenPairRepo::new(TokenRepo::new_read_only()),
                SummaryRepo::new(),
            ),
            notification: NotificationService::new(pool.clone(), NotificationRepo::new()),
            rule: RuleService::new(pool.clone()),
        },
    }))
}
