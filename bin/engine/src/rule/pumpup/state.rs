// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::rule::pumpup::FactService;
use base::service::{NotificationService, RuleService};
use sqlx::PgPool;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone)]
pub struct State(pub Arc<StateInner>);

impl Deref for State {
    type Target = StateInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Clone)]
pub struct StateInner {
    pub pool: PgPool,
    pub service: Service,
}

#[derive(Clone)]
pub struct Service {
    pub fact: FactService,
    pub notification: NotificationService,
    pub rule: RuleService,
}
