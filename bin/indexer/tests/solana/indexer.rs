// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::testing::run_test_on_empty_db;
use indexer::solana::indexer::IndexerRepo;

#[test_log::test(sqlx::test)]
async fn test_set_solana_indexer_slot_first_time() {
    run_test_on_empty_db(|mut tx| async move {
        IndexerRepo::set_solana_indexer_slot(&mut tx, 42)
            .await
            .unwrap();

        let indexer = IndexerRepo::get_solana_indexer_slot(&mut tx).await.unwrap();
        assert_eq!(indexer.slot, 42);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_set_solana_indexer_slot_multiple_times() {
    run_test_on_empty_db(|mut tx| async move {
        IndexerRepo::set_solana_indexer_slot(&mut tx, 3)
            .await
            .unwrap();

        IndexerRepo::set_solana_indexer_slot(&mut tx, 4)
            .await
            .unwrap();
        IndexerRepo::set_solana_indexer_slot(&mut tx, 10)
            .await
            .unwrap();
        IndexerRepo::set_solana_indexer_slot(&mut tx, 100)
            .await
            .unwrap();

        let indexer = IndexerRepo::get_solana_indexer_slot(&mut tx).await.unwrap();
        assert_eq!(indexer.slot, 100);
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_set_wallet_swap_slot_first_time() {
    run_test_on_empty_db(|mut tx| async move {
        IndexerRepo::set_wallet_swap_slot(&mut tx, 42)
            .await
            .unwrap();

        let slot = IndexerRepo::get_wallet_swap_slot(&mut tx).await.unwrap();
        assert_eq!(slot, Some(42.into()));
    })
    .await
}

#[test_log::test(sqlx::test)]
async fn test_set_wallet_swap_slot_multiple_times() {
    run_test_on_empty_db(|mut tx| async move {
        IndexerRepo::set_wallet_swap_slot(&mut tx, 3).await.unwrap();

        IndexerRepo::set_wallet_swap_slot(&mut tx, 4).await.unwrap();

        IndexerRepo::set_wallet_swap_slot(&mut tx, 10)
            .await
            .unwrap();

        IndexerRepo::set_wallet_swap_slot(&mut tx, 100)
            .await
            .unwrap();

        let slot = IndexerRepo::get_wallet_swap_slot(&mut tx).await.unwrap();
        assert_eq!(slot, Some(100.into()));
    })
    .await
}
