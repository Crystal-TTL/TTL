// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::rule::pumpfun::setup;
use base::assert_sql;
use base::testing::run_test_with_pool_on_empty_db;
use base::testing::user::get_or_create_test_user;
use engine::rule::pumpfun::run_rules;
use sqlx::Executor;

#[test_log::test(sqlx::test)]
async fn test() {
    // there was a bug in the kitty paws rule which caused it to match
    run_test_with_pool_on_empty_db(|pool| async move {
        let mut tx = pool.begin().await.unwrap();
        get_or_create_test_user(&mut tx).await;
        tx.commit().await.unwrap();

        pool.execute(
            r#"
            insert into solana.rule (id, status, version, name, user_id, sequence, created_at, updated_at, rule) values
                (14, 1, 1, 'Kitty Paws 🐾', 1, '{"action": {"type": "NOTIFY_TELEGRAM", "buttons": [{"value": {"type": "SOL", "value": "0.1"}, "action": "BUY"}, {"value": {"type": "SOL", "value": "0.5"}, "action": "BUY"}, {"value": {"type": "SOL", "value": "1"}, "action": "BUY"}, {"value": {"type": "PERCENT", "value": "25"}, "action": "SELL"}, {"value": {"type": "PERCENT", "value": "50"}, "action": "SELL"}, {"value": {"type": "PERCENT", "value": "100"}, "action": "SELL"}]}, "condition": {"type": "MANAGED", "managed": "PUMP_FUN_MANAGED_KITTY_PAWS"}}', '2025-04-09 01:57:19.325275 +00:00', '2025-04-09 01:57:19.325275 +00:00', 2);

            insert into solana.token (id, version, mint, name, symbol, decimals, supply, block_time) values
                (22675, 0, 'BKb2WhrivhpSYEfgra2SfwW5jssuXunApRoobmpBpump', 'MAD WOLF', 'HOWL', 6, 997489335.785796000000, '2025-03-15 04:10:25');

            insert into solana.token_pair (id, base_id, quote_id) values
                (23073, 22675, 1);

            insert into pumpfun.current (id, slot, virtual_base_reserves, virtual_quote_reserves, progress, complete, price, price_usd, market_cap, market_cap_usd) values
                 (23073, 332291643, 456114760725394.000000000000, 70574344398.000000000000, 77.78152, false, 0.000000154725, 0.000016588711, 154.336537479457, 16547.062191004919);

            insert into pumpfun.summary_6h (token_pair_id, amount_base, amount_base_change, amount_base_percent, amount_base_buy, amount_base_buy_change, amount_base_buy_percent, amount_base_sell, amount_base_sell_change, amount_base_sell_percent, amount_quote, amount_quote_change, amount_quote_percent, amount_quote_buy, amount_quote_buy_change, amount_quote_buy_percent, amount_quote_sell, amount_quote_sell_change, amount_quote_sell_percent, curve_progress_open, curve_progress_open_change, curve_progress_high, curve_progress_high_change, curve_progress_low, curve_progress_low_change, curve_progress_close, curve_progress_close_change, curve_progress_avg, curve_progress_avg_change, market_cap_open, market_cap_open_usd, market_cap_open_change, market_cap_open_usd_change, market_cap_open_percent, market_cap_high, market_cap_high_usd, market_cap_high_change, market_cap_high_usd_change, market_cap_high_percent, market_cap_low, market_cap_low_usd, market_cap_low_change, market_cap_low_usd_change, market_cap_low_percent, market_cap_close, market_cap_close_usd, market_cap_close_change, market_cap_close_usd_change, market_cap_close_percent, market_cap_avg, market_cap_avg_usd, market_cap_avg_change, market_cap_avg_usd_change, market_cap_avg_percent, price_open, price_open_usd, price_open_change, price_open_usd_change, price_open_percent, price_high, price_high_usd, price_high_change, price_high_usd_change, price_high_percent, price_low, price_low_usd, price_low_change, price_low_usd_change, price_low_percent, price_close, price_close_usd, price_close_change, price_close_usd_change, price_close_percent, price_avg, price_avg_usd, price_avg_change, price_avg_usd_change, price_avg_percent, swap, swap_change, swap_percent, swap_buy, swap_buy_change, swap_buy_percent, swap_sell, swap_sell_change, swap_sell_percent, volume, volume_usd, volume_change, volume_usd_change, volume_percent, volume_buy, volume_buy_usd, volume_buy_change, volume_buy_usd_change, volume_buy_percent, volume_sell, volume_sell_usd, volume_sell_change, volume_sell_usd_change, volume_sell_percent) values
                (23073, 12926.161504000000, null, null, 12926.161504000000, null, null, 0.000000000000, null, null, 0.002000000000, null, null, 0.002000000000, null, null, 0.000000000000, null, null, 77.78152, null, 77.78152, null, 77.78152, null, 77.78152, null, 77.78152, null, 154.336537479457, 16505.298769662675, null, null, null, 154.336537479457, 16505.298769662675, null, null, null, 154.336537479457, 16505.298769662675, null, null, null, 154.336537479457, 16505.298769662675, null, null, null, 154.336537479457, 16505.298769662675, null, null, null, 0.000000154725, 0.000016444473, null, null, null, 0.000000154725, 0.000016444473, null, null, null, 0.000000154725, 0.000016444473, null, null, null, 0.000000154725, 0.000016444473, null, null, null, 0.000000154725, 0.000016444473, null, null, null, 1, null, null, 1, null, null, 0, null, null, 0.002000000339, 0.212563915535, null, null, null, 0.002000000339, 0.212563915535, null, null, null, 0.000000000000, 0.000000000000, null, null, null);

            insert into pumpfun.summary_1d (token_pair_id, amount_base, amount_base_change, amount_base_percent, amount_base_buy, amount_base_buy_change, amount_base_buy_percent, amount_base_sell, amount_base_sell_change, amount_base_sell_percent, amount_quote, amount_quote_change, amount_quote_percent, amount_quote_buy, amount_quote_buy_change, amount_quote_buy_percent, amount_quote_sell, amount_quote_sell_change, amount_quote_sell_percent, curve_progress_open, curve_progress_open_change, curve_progress_high, curve_progress_high_change, curve_progress_low, curve_progress_low_change, curve_progress_close, curve_progress_close_change, curve_progress_avg, curve_progress_avg_change, market_cap_open, market_cap_open_usd, market_cap_open_change, market_cap_open_usd_change, market_cap_open_percent, market_cap_high, market_cap_high_usd, market_cap_high_change, market_cap_high_usd_change, market_cap_high_percent, market_cap_low, market_cap_low_usd, market_cap_low_change, market_cap_low_usd_change, market_cap_low_percent, market_cap_close, market_cap_close_usd, market_cap_close_change, market_cap_close_usd_change, market_cap_close_percent, market_cap_avg, market_cap_avg_usd, market_cap_avg_change, market_cap_avg_usd_change, market_cap_avg_percent, price_open, price_open_usd, price_open_change, price_open_usd_change, price_open_percent, price_high, price_high_usd, price_high_change, price_high_usd_change, price_high_percent, price_low, price_low_usd, price_low_change, price_low_usd_change, price_low_percent, price_close, price_close_usd, price_close_change, price_close_usd_change, price_close_percent, price_avg, price_avg_usd, price_avg_change, price_avg_usd_change, price_avg_percent, swap, swap_change, swap_percent, swap_buy, swap_buy_change, swap_buy_percent, swap_sell, swap_sell_change, swap_sell_percent, volume, volume_usd, volume_change, volume_usd_change, volume_percent, volume_buy, volume_buy_usd, volume_buy_change, volume_buy_usd_change, volume_buy_percent, volume_sell, volume_sell_usd, volume_sell_change, volume_sell_usd_change, volume_sell_percent) values
                (23073, 12926.161504000000, null, null, 12926.161504000000, null, null, 0.000000000000, null, null, 0.002000000000, null, null, 0.002000000000, null, null, 0.000000000000, null, null, 77.78152, null, 77.78152, null, 77.78152, null, 77.78152, null, 77.78152, null, 154.336537479457, 16505.298769662675, null, null, null, 154.336537479457, 16505.298769662675, null, null, null, 154.336537479457, 16505.298769662675, null, null, null, 154.336537479457, 16505.298769662675, null, null, null, 154.336537479457, 16505.298769662675, null, null, null, 0.000000154725, 0.000016444473, null, null, null, 0.000000154725, 0.000016444473, null, null, null, 0.000000154725, 0.000016444473, null, null, null, 0.000000154725, 0.000016444473, null, null, null, 0.000000154725, 0.000016444473, null, null, null, 1, null, null, 1, null, null, 0, null, null, 0.002000000339, 0.212563915535, null, null, null, 0.002000000339, 0.212563915535, null, null, null, 0.000000000000, 0.000000000000, null, null, null);

        "#,
        )
        .await
        .unwrap();

        let state = setup(pool.clone());
        run_rules(state).await;

        assert_sql!(&pool,"(select count(*) from solana.invocation) = 0");
    })
    .await
}
