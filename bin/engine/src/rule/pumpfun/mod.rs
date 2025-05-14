// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod fact;
pub mod state;

pub use crate::rule::pumpfun::fact::FactService;
use crate::rule::pumpfun::state::{Service, State, StateInner};
use crate::config::RuleConfig;
use base::model::Action;
use base::model::Venue::PumpFun;
use base::repo::{InvocationCreateCmd, InvocationRepo, NotificationRepo, TokenPairRepo, TokenRepo};
use base::service::{NotificationRuleMatched, NotificationService, RuleService};
use common::repo::pool::setup_pool;
use log::info;
use solana::pumpfun::repo::SummaryRepo;
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Instant};

pub fn start_rules(cfg: RuleConfig) -> JoinHandle<()> {
    tokio::spawn(async move {
        info!("active");

        let pool = setup_pool(cfg).await;

        let token_repo = TokenRepo::new_read_only();

        let state = State(Arc::new(StateInner {
            pool: pool.clone(),
            service: Service {
                fact: FactService::new(
                    pool.clone(),
                    TokenPairRepo::new(token_repo.clone()),
                    SummaryRepo::new(),
                ),
                notification: NotificationService::new(pool.clone(), NotificationRepo::new()),
                rule: RuleService::new(pool.clone()),
            },
        }));

        loop {
            run_rules(state.clone()).await;
            sleep(Duration::from_millis(1000)).await;
        }
    })
}

pub async fn run_rules(state: State) {
    let rules = state.service.rule.list_active().await.unwrap();

    let start = Instant::now();
    let pumpfun_facts = state.service.fact.pumpfun_facts().await;
    println!(
        "{} pumpfun facts - took {}",
        pumpfun_facts.len(),
        (Instant::now().duration_since(start)).as_millis()
    );

    for rule in &rules {
        if !rule.applicable() {
            // FIXME filter them out before hitting this loop
            continue;
        }
        println!("test rule - {}", rule.id.0);
        for (token_pair_id, facts) in &pumpfun_facts {
            if rule.sequence.condition.test(facts) {
                let mut tx = state.pool.begin().await.unwrap();

                match InvocationRepo::new()
                    .create(
                        &mut tx,
                        InvocationCreateCmd {
                            user: rule.user,
                            rule: rule.id,
                            token_pair: *token_pair_id,
                            next: None,
                        },
                    )
                    .await
                {
                    Ok(_) => {
                        println!("met - {token_pair_id}");

                        match &rule.sequence.action {
                            Action::AndThen { .. } => {}
                            Action::Buy => {}
                            Action::NotifyTelegram { buttons } => {
                                let _ = state
                                    .service
                                    .notification
                                    .create_rule_matched_tx(
                                        &mut tx,
                                        NotificationRuleMatched::Telegram {
                                            user: rule.user,
                                            rule: rule.id,
                                            venue: PumpFun,
                                            token_pair: *token_pair_id,
                                            buttons: buttons.clone(),
                                        },
                                    )
                                    .await;
                            }
                            Action::Sell => {}
                        }

                        tx.commit().await.unwrap();
                    }
                    Err(_) => {
                        // FIXME cache already invoked strategies - otherwise this might be heavy on the database
                        tx.rollback().await.unwrap();
                    }
                }
            }
        }
    }
}
