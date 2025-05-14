// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::solana::indexer::IndexerRepo;
use base::model::solana::{Indexer, Slot};
use common::model::UpdatedAt;
use common::repo::RepoResult;
use common::sql::AsSqlExecutor;
use sqlx::{query, Row};

impl IndexerRepo {

    pub async fn get_solana_indexer_slot(mut executor: impl AsSqlExecutor) -> RepoResult<Indexer> {
        Ok(query("select * from solana.indexer where id = 1;")
            .fetch_one(executor.as_executor())
            .await
            .map(|r| Indexer {
                slot: r.get::<Slot, _>("slot"),
                updated_at: r.get::<UpdatedAt, _>("updated_at"),
            })?)
    }

    pub async fn get_wallet_swap_slot(
        mut executor: impl AsSqlExecutor,
    ) -> RepoResult<Option<Slot>> {
        Ok(query("select * from solana.indexer where id = 2;")
            .fetch_optional(executor.as_executor())
            .await?
            .map(|r| r.get::<Slot, _>("slot")))
    }
}
