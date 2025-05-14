// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::solana::indexer::IndexerRepo;
use base::model::solana::Slot;
use common::repo::RepoResult;
use common::sql::AsSqlExecutor;
use sqlx::query;

impl IndexerRepo {
    pub async fn set_solana_indexer_slot(
        mut executor: impl AsSqlExecutor,
        slot: impl Into<Slot> + Send,
    ) -> RepoResult<()> {
        query(
            r#"
insert into solana.indexer (id, slot, updated_at) values (1, $1, now())
on conflict (id) do update set slot = $1, updated_at = now();
"#,
        )
        .bind(slot.into())
        .execute(executor.as_executor())
        .await?;
        Ok(())
    }

    pub async fn set_wallet_swap_slot(
        mut executor: impl AsSqlExecutor,
        slot: impl Into<Slot> + Send,
    ) -> RepoResult<()> {
        query(
            r#"
insert into solana.indexer (id, slot, updated_at) values (2, $1, now())
on conflict (id) do update set slot = $1, updated_at = now();
"#,
        )
        .bind(slot.into())
        .execute(executor.as_executor())
        .await?;
        Ok(())
    }
}
