// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

mod summary;

use crate::rule::pumpup::fact::summary::add_summary_to_facts;
use base::model::Fact::{MarketCapQuote, MarketCapUsd, VenuePumpup};
use base::model::{Fact, Facts, TokenPairId, Value};
use base::repo::TokenPairRepo;
use common::model::{Limit, Timeframe};
use solana::pumpup::repo::{CurrentQuery, CurrentRepo, SummaryQuery, SummaryRepo};
use sqlx::PgPool;
use std::collections::HashMap;
use tokio::time::Instant;

#[derive(Clone)]
pub struct FactService {
    pool: PgPool,
    pair: TokenPairRepo,
    summary: SummaryRepo,
}

impl FactService {
    pub fn new(pool: PgPool, pair: TokenPairRepo, summary: SummaryRepo) -> Self {
        Self {
            pool,
            pair,
            summary,
        }
    }

    pub async fn pumpup_facts(&self) -> HashMap<TokenPairId, Facts> {
        let mut tx = self.pool.begin().await.unwrap();

        let start = Instant::now();
        let mut result: HashMap<TokenPairId, Facts> = self
            .pair
            .list_all(&mut tx)
            .await
            .unwrap()
            .into_iter()
            .map(|tp| {
                let facts = Facts::new();

                // if let Some(age) = tp.base.age() {
                //     facts.set_value(
                //         Fact::AgeBaseDuration,
                //         Value::duration(age.0, TimeUnit::Second),
                //     )
                // }
                //
                // if let Some(age) = tp.quote.age() {
                //     facts.set_value(
                //         Fact::AgeQuoteDuration,
                //         Value::duration(age.0, TimeUnit::Second),
                //     )
                // }

                (tp.id, facts)
            })
            .collect();

        println!(
            "token pairs took: {}",
            Instant::now().duration_since(start).as_millis()
        );

        for current in CurrentRepo::list(
            &mut *tx,
            CurrentQuery {
                limit: Limit::unlimited(),
            },
        )
        .await
        .unwrap()
        {
            if let Some(facts) = result.get_mut(&current.id) {
                // facts.set_value(CurveProgressPercent, Value::percent(BigDecimal::from_f32(current.progress.0).unwrap()));
                // facts.set_value(
                //     CurveProgressAgeDuration,
                //     Value::duration(current.age.0, TimeUnit::Second),
                // );
                facts.set_value(VenuePumpup, Value::boolean(true));

                facts.set_value(Fact::PriceQuote, Value::quote(current.price.0));
                if let Some(usd) = current.price_usd {
                    facts.set_value(Fact::PriceUsd, Value::usd(usd.0))
                }

                if let Some(quote) = current.market_cap {
                    facts.set_value(MarketCapQuote, Value::quote(quote.0));
                }

                if let Some(usd) = current.market_cap_usd {
                    facts.set_value(MarketCapUsd, Value::usd(usd.0))
                }
            }
        }

        for timeframe in [
            Timeframe::M1,
            Timeframe::M5,
            Timeframe::M15,
            Timeframe::H1,
            Timeframe::H6,
            Timeframe::D1,
        ] {
            let start = Instant::now();

            let summary = self
                .summary
                .list(
                    &mut tx,
                    SummaryQuery {
                        limit: Limit::unlimited(),
                        timeframe,
                    },
                )
                .await
                .unwrap();

            for (token_pair_id, summary) in summary {
                let facts = result.entry(token_pair_id).or_insert(Facts::default());
                add_summary_to_facts(facts, summary, timeframe);

                facts.set_value(VenuePumpup, Value::boolean(true));
            }

            println!(
                "Summary {:?} took: {}",
                timeframe,
                Instant::now().duration_since(start).as_millis()
            );
        }
        tx.commit().await.unwrap();

        result
            .into_iter()
            .filter(|(_, facts)| {
                facts
                    .get(&VenuePumpup)
                    .filter(|v| match v {
                        Value::Boolean { value } => *value == true,
                        _ => false,
                    })
                    .is_some()
            })
            .collect()
    }
}
