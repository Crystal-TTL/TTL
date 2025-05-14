// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::handle::live::{test_instance, test_quote_request, TestFailingQuoteService};
use base::assert_sql;
use base::model::requests::SwapQuoteRequest;
use base::model::{CreateQuote, RequestPayload, RequestToProcess, RequestType};
use base::repo::RequestRepo;
use base::service::QuoteService;
use base::testing::{run_test_with_pool, serializable_tx};
use engine::handle::LiveHandler;
use serde_json::json;
use sqlx::Executor;
use std::collections::HashMap;
use std::sync::Arc;

#[test_log::test(sqlx::test)]
async fn test_ok() {
    run_test_with_pool(|pool| async move {
		let test_instance = test_instance();

		let mut tx = serializable_tx(&pool).await;
		RequestRepo::submit(&mut tx, SwapQuoteRequest {
			wallet: 1.into(),
			user: 1.into(),
			request: test_quote_request(),
		}).await.unwrap();
		tx.commit().await.unwrap();

		let mut tx = pool.begin().await.unwrap();
		let (_attempt, request) = RequestRepo::attempt(&pool).await.unwrap().unwrap();

		test_instance.swap_token(&mut tx, request).await;
		tx.commit().await.unwrap();

		assert_sql!(&pool, r#"(select status from solana.request where id = 1 ) = 2"#);
		assert_sql!(&pool, r#"(select payload->>'hash' from solana.request_attempt where id = 1 and attempt = 1) = 'SomeTransactionHash'"#);

		assert_sql!(&pool, r#"(select md5(quote::text) from solana.result_swap where id = 1) = 'c009c33305ac3478fe1480259848ab85'"#);
		assert_sql!(&pool, r#"(select swap_hash from solana.result_swap where id = 1) = 'SomeTransactionHash'"#);
		assert_sql!(&pool, r#"(select user_id from solana.result_swap where id = 1) = 1"#);
		assert_sql!(&pool, r#"(select wallet_id from solana.result_swap where id = 1) = 1"#);
	}).await
}

#[test_log::test(sqlx::test)]
async fn test_fails_to_retrieve_quote() {
    run_test_with_pool(|pool| async move {
		let test_instance = test_instance_unable_to_retrieve_quote();

		let mut tx = serializable_tx(&pool).await;
		RequestRepo::submit(&mut tx, SwapQuoteRequest {
			wallet: 1.into(),
			user: 1.into(),
			request: test_quote_request(),
		}).await.unwrap();
		tx.commit().await.unwrap();

		let mut tx = pool.begin().await.unwrap();
		let (_attempt, request) = RequestRepo::attempt(&pool).await.unwrap().unwrap();

		test_instance.swap_token(&mut tx, request).await;
		tx.commit().await.unwrap();

		assert_sql!(&pool, r#"(select status from solana.request where id = 1 ) = 4"#);
		assert_sql!(&pool, r#"(select payload->>'message' from solana.request_attempt where id = 1 and attempt = 1) = 'failed to retrieve quote'"#);

		assert_sql!(&pool, r#"(select count(*) from solana.result_swap) = 0"#);
	}).await
}

#[test_log::test(sqlx::test)]
async fn test_unable_to_read_payload() {
    // there was a bug in the kitty paws rule which caused it to match
    run_test_with_pool(|pool| async move {
		pool.execute(
			r#"
        insert into solana.request (id, user_id, wallet_id, status, request, payload) values
		    (2, 1, 1, 1, 3, '{}');
        "#,
		)
			.await
			.unwrap();

		let test_instance = test_instance();

		RequestRepo::attempt(&pool).await.unwrap().unwrap();
		let mut tx = pool.begin().await.unwrap();
		test_instance
			.swap_token(
				&mut tx,
				RequestToProcess {
					id: 2.into(),
					user: 1.into(),
					wallet: 1.into(),
					request: RequestType::SendToken,
					payload: RequestPayload(json!({})),
				},
			)
			.await;
		tx.commit().await.unwrap();

		assert_sql!(&pool, r#"(select status from solana.request where id = 2 ) = 4"#);
		assert_sql!(&pool, r#"(select payload->>'message' from solana.request_attempt where id = 2 and attempt = 1) = 'invalid SwapToken payload'"#);
		assert_sql!(&pool, r#"(select status from solana.request_attempt where id = 2 and attempt = 1) = 4"#);

		assert_sql!(&pool, r#"(select count(*) from solana.result_swap) = 0"#);
	})
		.await
}

fn test_instance_unable_to_retrieve_quote() -> LiveHandler {
    LiveHandler::testing(
        HashMap::new(),
        QuoteService::new([Arc::new(TestFailingQuoteService {}) as Arc<dyn CreateQuote>]),
    )
}
