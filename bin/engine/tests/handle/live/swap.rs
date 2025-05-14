// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::handle::live::test_quote;
use base::assert_sql;
use base::model::requests::SwapQuoteResult;
use base::repo::{RequestRepo, WalletRepo};
use base::service::QuoteService;
use base::testing::{run_test_with_pool, serializable_tx};
use engine::handle::LiveHandler;
use std::collections::HashMap;

#[test_log::test(sqlx::test)]
async fn test_fails_to_get_private_key() {
    run_test_with_pool(|pool| async move {
		let test_instance = test_instance_invalid_secret();

		let mut tx = serializable_tx(&pool).await;
		RequestRepo::submit(&mut tx, SwapQuoteResult {
			wallet: 1.into(),
			user: 1.into(),
			quote: test_quote(),
		}).await.unwrap();
		tx.commit().await.unwrap();

		let (_attempt, request) = RequestRepo::attempt(&pool).await.unwrap().unwrap();

		test_instance.swap_quote(&pool, request).await;

		assert_sql!(&pool, r#"(select status from solana.request where id = 1 ) = 4"#);
		assert_sql!(&pool, r#"(select payload->>'message' from solana.request_attempt where id = 1 and attempt = 1) = 'failed to get private key'"#);
		assert_sql!(&pool, r#"(select status from solana.request_attempt where id = 1 and attempt = 1) = 4"#);
		assert_sql!(&pool, r#"(select count(*) from solana.result_swap) = 0"#);
	}).await
}

pub fn test_instance_invalid_secret() -> LiveHandler {
    LiveHandler {
        quote_service: QuoteService::new([]),
        swap_services: HashMap::new(),
        wallet_repo: WalletRepo {
            secret: "3333333333333333333333333333333333333333333333333333333333333333".into(),
        },
    }
}
