// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use crate::solana::block::setup;
use base::assert_sql;
use base::model::solana::Slot;
use base::testing::run_test_with_pool_on_empty_db;
use indexer::solana::block::index_block;
use solana::convert::convert_block;
use sqlx::{Executor, PgPool};

#[test_log::test(sqlx::test)]
async fn test_ok() {
    run_test_with_pool_on_empty_db(|pool| async move {
		let block = serde_json::from_str(include_str!("block_328790579.json")).unwrap();
		let block = convert_block(Slot::from(328790579), block).await.unwrap().unwrap();

		pool.acquire().await.unwrap().execute(r#"
insert into solana.address (id, address, created_at, updated_at)
values  (1, 'EkoSFDKDoRmsDgHRyTAW2Lzy2zsqFptyDo9RGUaBBnc9', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (2, 'GmzgKbFs4nQ7d8uVP6wHW1EjtsEK5udFfxqNubwyToud', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (3, 'CKZ5jyHk7hPiADKjwVYieyKG1wenT6HrAgZuYbGBz8wJ', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (4, '6qYUmDYZmxUC162oKJTtuAYnmWjjS8BbsGs9VryLTvpJ', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (5, 'A94db2UH1V6GHNo1HX9LafRrBEZQPCv2GZycvHa6c4Zf', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (6, 'Fbq9LGgzGMjPfdEEX8bXvoCApWUCwKiyeE2z1n2hawbb', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (7, '9NkRpwF9knJLKbdBbdr4jPr9bAMVCCzjHssHEZEgaWht', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (8, '3XVKrAYiMBwx84REecsvgQm9MqSPU1Hsvgd6oTRKb8UN', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (9, '7aJdQDLqHE8BH6za9jhKTeLNbgHMsNunJ5eDtnChyXcY', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (10, '4N7MFjZ3d1J7b1HM8Zb5qLGEFRwuc87ikJk17wCd7fif', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (11, 'HzzpZbFRmxJmoXVrwxbD4imtVKe5HiA6Aavtq9e4KzQx', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (12, '8PMW1TKipZEJC8SNcPwMda39jhMQ7PGQC94LKq9fjzYg', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (13, '5Bh6NufMmpFChHYHVqwGeeNKXxY4Mz7Y6a21PsVs6jTz', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (14, 'CXDJxtuwfJkBxSG9kp1tun58mecbNFimAfECtjCuuRHR', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (15, '2cLrpU9vubv4kX5mTquYkk791uhEZj8Wnwo1g3U998D6', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (16, '8Yty76PSoEiP11t5E155Y17qSxECdycTDnKFY65C8R1G', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (17, '4Jukcb2QkQBzxyLRVvpWv7B28aN9LEvBQg5vwfyj3p1U', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (18, '6yynbdd1LUgRQbHzW6SXc5rH8JfDhzRDTLinLBQtBqv3', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (19, 'CapuXNQoDviLvU1PxFiizLgPNQCxrsag1uMeyk6zLVps', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (20, '4Kzz4qADy2ymdKgeedTZwoB6QyuA6ZKab6rtp263cp4F', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (21, 'DqQ3XYGEivaiTH4ZWZpeKfrB65wqq1g6uFnfMo1yUgeH', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (22, 'JDPaiG4hCWtyy8peGzmPpsohMJEPtNGF55s4kQTKyCDC', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (23, '9biwty1G1z47Pa5qutTNZDtuSmATf1JfPJG7FYKCX5uo', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (24, '5N27U1i8Roeruq2HT3e8gfx7c3EnBVLxxJBXx1GxdyaD', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (25, '4gmWCfri8c8zDLUW8YevsvtY7Ahkd7npjHojX97sPRmZ', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (26, '58Gn8H2VFk7gbKzLKsaMBY8rAdApYXCADUnJJF27jFXg', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (27, 'BE44Bh6Mv1owoNyV9DupupS7oVwG9yxPR5Vqx5v8Tabn', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (28, '6TRP3NwBnJoSmC7MRVbLKFPTWN1VjhM5pgKrzJL5MjHU', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (29, 'De7LNcUzYyVaviBqWLCW1NH1HwQ5koLFNBCue3girqNg', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (30, 'Bw9QfVSBu9XbaoKii2B71nqS7XjenwnmGF39ZKemQrzU', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (31, 'YfAb7SgowdwuV49T3spoCtaCA6gkqThUJcDWncAz4Kg', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (32, '6CqLBSMGseZxwwydAQ3bKRmTeS1MaTwCqz5tgi6ctM5j', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (33, 'FJdgD6ZNbuKZbdq4egLT8wtXYZU8Y8Tx1qK8vqFL6JgF', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (34, 'BKsD4GjPKGbwxcKb4FKY2K4yoMdpMjQ8WbhW5TX23wGc', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (35, 'B78WfsMcjvw2bbQmpBvewVRFpjFzM7FZQmbsLtMdcEnq', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (36, '5LZNgcgZKxbqtd4Nqi4aQERoYLj6DfZxYqbBbYG1qJdg', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (37, '5SDtcA5dqUuRkFcS9rYuHZUBAg5KYrtGuRDqn9DWc4sg', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (38, '9BmEbAEux5pTZMjYPt22PvkX1HNsCyXuWa5yhMMMGoKM', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (39, '2haGTPvRkCroZ5QyteaPAxJpx5hSWJg1VrBT2ynE2miK', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (40, '6skVHiDszw2W2ww9sv6nt9FRBGKrgWqjwhL8u9vGAGoz', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (41, 'G36TQ8u3r18WRUqbfkiPU9TVufYnT5pBVWbejHZ4NTpL', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (42, '5nQLsRXxx4mdHZZrgFa9WbWc38ihWUcLqk88dzZ11ggX', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (43, 'JEFTv9UTwyaeCRrQ23D8pBjzDJErnTZkf4V4M2nsZ3CT', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (44, 'DMvoW8ZeEfPaNipuy1vhrRHsB2ZctCXssCb6oqdj4Spj', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (45, '9hWTinWkWiyjRSDRre9kh4Nw44PmJkNFV5uAwHP5ta2c', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (46, 'B3fz1i5wgXSh6oHMXNbUf4m442AFN5BHGsYwwEwhyxbx', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (47, '3Z36kWSYX9YcBTR3BHcnrhuppWbTjPUEYXvsKV6dg1ny', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (48, 'BxQRjVu1fXZJ32ozMM4jnajD8HuMRMAPbmFQonJdZhSQ', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (49, '5fiZzY6Y9rXW14HfmDY4y1HtYFHNCZeSpRLUTR7x8Xux', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (50, '3uSX1HiCNGWeuaFoQg8tiUW9uemohcQTsramjkv1dLAA', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (51, 'HAsiywogH8UNDnBN8SGK9TNhwkwQQ2ZpqPUHc1dCNbg1', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (52, 'K89C5ELTX98KiKET8PQCqe15NzogVgo9QqixYtuH4ok', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (53, '9AbEi5bGGBvMT9eCciiWwPxaGkqzWvCQRyvGyT5Qw4ze', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (54, '7HQBN2ANVJwLh9rWtUQU7UcvG95N4pAMzbPAbngPQKtz', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (55, 'BQ72nSv9f3PRyRKCBnHLVrerrv37CYTHm5h3s9VSGQDV', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (56, '9yj3zvLS3fDMqi1F8zhkaWfq8TZpZWHe6cz1Sgt7djXf', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (57, 'tqCiSbcbr1QKCv7fNA3gaEgCzTgy8acoghebPhN9XzU', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (58, '7imnGYfCovXjMWKdbQvETFVMe72MQDX4S5zW4GFxMJME', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (59, '8UgoPZAR8ZLoEmV6pJ8SZ6JKESP2X8nbnrZSdSgNtg1y', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (60, '8cjeuVV3KQ9k8RqW1JUyCfey2TDAhuo7f4hPDMeGfxv', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (61, '9MkixYmjT2UbMgnNnPBTYkRjzdmi4zP1jkMdCkR89L67', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (62, '62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (63, '7VtfL8fvgNfhz17qKRMjzQEXgbdpnHHHQRh54R9jP2RJ', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (64, '5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (65, 'ESXSXUx7hFi78p2RikRdS5Cdxi2H94UUkFCLBVWDNSrg', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (66, 'AtfwFfmzS5BQr5fe2cfSSm44XG1Ruqh9BvtXoXThGbd6', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (67, 'HVoJWyPbQn4XikG9BY2A8wP27HJQzHAoDnAs1SfsATes', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (68, 'CbYf9QNrkVgNRCMTDiVdvzMqSzXh8AAgnrKAoTfEACdh', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (69, 'Hz8tThx5x7WHWScKGEcQQTfyAp7YkMghhKEt2qCjaJDa', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (70, 'AgeSxtVWWMojFWYNrXKnVp9cFuC5CQ7M4rzmrseLxfUj', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (71, '5L7CNPZULyomdYZo9Wa5QNYSTtd4heDazJMn1DC72wWn', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (72, 'AVmoTthdrX6tKt4nDjco2D775W2YK3sDhxPcMmzUAmTY', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (73, 'GpMZbSM2GgvTKHJirzeGfMFoaZ8UR2X7F4v8vHTvxFbL', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (74, 'FWsW1xNtWscwNmKv6wVsU1iTzRN6wmmk3MjxRP5tT7hz', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (75, 'CTXZhvzSKjR1ojk8z1oLjzYPb83mQcDEADNLhGtjuYs7', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (76, '8WJFRitZoUpaybVkut8gc6NowcJzp4r1x7edNMqcbD5r', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (77, 'HgjVpoEBB7guQokeCyqDXdEmtupBjjEmb29Lyk2DRY48', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (78, '8q7qFpNimqLf75MCEFaUQtSy5YWHPGzHN4iDKd6MTqcm', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (79, '7VWyTyf2rQEgVdBnN8t94kNNyEkAAkHe1ZMLbnZWdLFb', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (80, 'JCRGumoE9Qi5BBgULTgdgTLjSgkCMSbF62ZZfGs84JeU', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (81, '9rPYyANsfQZw3DnDmKE3YCQF5E8oD89UXoHn9JFEhJUz', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (82, '5YuTJTN14Asd1eK2xef3d2boYu1pWYtRBxvpE3CjwEJY', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (83, 'DBFARLLDx33TwrimmUtD6hwCmrYbTwRptz3gpFWtEpEq', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (84, '83v8iPyZihDEjDdY8RdZddyZNyUtXngz69Lgo9Kt5d6d', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (85, 'DTi7woaMv6dXHGy14Ysbg88vwZLnX1bTYuECuwNrB6y2', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (86, 'EwRHB2tu6GL78HBKVxXq7p2Cmyf5bVfZD8ggoPUbkk7r', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (87, 'FERjPVNEa7Udq8CEv68h6tPL46Tq7ieE49HrE2wea3XT', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (88, '3ESUFCnRNgZ7Mn2mPPUMmXYaKU8jpnV9VtA17M7t2mHQ', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (89, 'EdhfvmEoYWqQZPoU27sHRgKUXsEDVBBVfVMcnC4FpL4D', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (90, '7hTckgnGnLQR6sdH7YkqFTAA7VwTfYFaZ6EhEsU3saCX', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (91, '9B9BUPcs1WJW6K7oYHCTywkJYADdLhMBSDVcCt62T1Fw', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (92, '9nnLbotNTcUhvbrsA6Mdkx45Sm82G35zo28AqUvjExn8', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (93, '8psNvWTrdNTiVRNzAgsou9kETXNJm2SXZyaKuJraVRtf', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (94, 'B4TKzPXoiAnC6gwre649XViDS1fmXeukACmaZixHuFmy', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (95, '3nMFwZXwY1s1M5s8vYAHqd4wGs4iSxXE4LRoUMMYqEgF', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (96, '45ruCyfdRkWpRNGEqWzjCiXRHkZs8WXCLQ67Pnpye7Hp', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (97, '81MPQqJY58rgT83sy99MkRHs2g3dyy6uWKHD24twV62F', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (98, '2MFoS3MPtvyQ4Wh4M9pdfPjz6UhVoNbFbGJAskCPCj3h', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (99, 'fEe1SXYGDYGY7c7ttEY2Jyffzotx12heiw8xdrctvi1', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (100, 'CvKXXfxq2YzgQ9V7PBfNCzFmRSrj1VX49tjAJqJy68AU', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (101, '2w4A1eGyjRutakyFdmVyBiLPf98qKxNTC2LpuwhaCruZ', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (102, '5BEYpX8vFT9FZx7doc8cdhzUXZh7Tbr3GR27tHYcZQjB', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (103, '2sf5NYcY4zUPXUSmG6f66mskb24t5F8S11pC1Nz5nQT3', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (104, 'BWBHrYqfcjAh5dSiRwzPnY4656cApXVXmkeDmAfwBKQG', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (105, 'EXHyQxMSttcvLPwjENnXCPZ8GmLjJYHtNBnAkcFeFKMn', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (106, '6LXutJvKUw8Q5ue2gCgKHQdAN4suWW8awzFVC6XCguFx', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (107, '5guD4Uz462GT4Y4gEuqyGsHZ59JGxFN4a3rF6KWguMcJ', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (108, 'G5UZAVbAf46s7cKWoyKu8kYTip9DGTpbLZ2qa9Aq69dP', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00'),
        (109, '7HkzG4LYyCJSrD3gopPQv3VVzQQKbHBZcm9fbjj5fuaH', '2025-04-21 07:55:34.338515 +00:00', '2025-04-21 07:55:34.338515 +00:00');

insert into solana.token (id, version, mint, name, symbol, decimals, supply, metadata, description, image, website, creator_id, block_id, block_time, created_at, updated_at)
values  (1000, 0, 'pb21pnV8UPWBSF1DxgcUq8FBUA5CXZPoPVCCsgGkCYP', 'franklin', 'FRANKLIN', 6, 999826204.632303000000, 'https://ipfs.io/ipfs/QmPdCpXgxgxhCiACKVdLEhCj3x6bUiyvqfUJBxWmL9wAMp', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1001, 0, '4VkG69WawzgkGxTtEvB51X5dNngsqqEcfgTzm9Mxpump', 'Kaito AI', 'KaitoAI', 6, 1000000000.000000000000, 'https://ipfs.io/ipfs/QmYbNwHuJeHUJsPnCS8Mh3KozajEz7sdURgPwXZHXTj9NZ', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1002, 0, '5oychkdWHyk59cQNa1pmtFUicDNMFGi97Tat2b5Ypump', 'Cancel that fggt RED', 'REDwFAGGT', 6, 999999999.989057000000, 'https://ipfs.io/ipfs/QmaJLmMopYyKGCuyjszot1PMKJJJQgSdFgUBqw1A4ppqxE', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1003, 0, 'GKuH7SzV6mYc3RmAsYF7sit7QMfK6oj1c1BP59hQpump', 'BBL Sheep', 'BBL', 6, 998519926.343851000000, 'https://ipfs.io/ipfs/QmXJdRjqTpQM7Fy3N1SceWk6yCEFpj1dH5W6NHFMtdMaKF', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1004, 0, '6xswx32qhaUiBjANwYRnrkCHXpxtBiWRbip6NxNHpump', 'IGF', 'IGF', 6, 999294719.656026000000, 'https://ipfs.io/ipfs/QmSS3z7FCfRzut3ZKk17QUmAQy493NY3wyvsVcmrkGTbTQ', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1005, 0, '3bcJXepf6TxAJe4TE7v4M9s9tJdgqDNzfMjM2DKwpump', 'Justice for Lily Stewart', 'LILY', 6, 999874701.775718000000, 'https://ipfs.io/ipfs/QmZMVVZ3svNmx3hqFhdRkfidDoTQcabbpPC2CnJxrw3Kbg', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1006, 0, '4k44Enmo58ZwxfqtXffK2EzhCrbhLxJmTUfsEKWepump', 'OPTIMARKY', 'OPTIMARKY', 6, 999695385.447051000000, 'https://ipfs.io/ipfs/QmTQj96iSMeoJ3pZ1RYDuxAcbgqQx6r6CpmS5TcBwfyRAv', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1007, 0, 'GfbQUTZYB3SdAzSQJBJxndd8bmtu6ktgjWd7bPbPpump', 'VIRGY', 'VIRGY', 6, 998774136.308469000000, 'https://ipfs.io/ipfs/QmWxAJrFoLAsJHcpiFVzo8D4kRwpMZ1UsUAQiiiXgkAiq2', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1008, 0, '3wth71poCxAckMcXKR6QLY7xKFoiSLwLgUShHRzXpump', 'Saratoga Spring Water', 'Saratoga', 6, 999428021.027604000000, 'https://ipfs.io/ipfs/QmbH5WNUmqrbAMzuZk17RfXTgVv77qNfYMg8QBTRpS6ZLj', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1009, 0, '3YBHLH1p1EVxix3VeKper323R3NJigNWx5HUy82epump', 'Downlon Musk', 'DOWNLON', 6, 999136816.652802000000, 'https://ipfs.io/ipfs/QmZnKg1B6de1h6QkxguNojehdCw6pd9M9KMghjACCXGpHH', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1010, 0, 'DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263', 'Bonk', 'Bonk', 5, 88835984794399.041710000000, 'https://arweave.net/QPC6FYdUn-3V8ytFNuoCS85S2tHAuiDblh6u3CIZLsw', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1011, 0, 'USDSwr9ApdHk5bvJKMjzff41FfuX8bSxdKcR81vTwcA', 'USDS', 'USDS', 6, 120880410.736824000000, 'https://ipfs.io/ipfs/QmbpUGHbPVYGGfiayCP3mMWv11Tnnz8o5E5A1pBe9Vdw1A', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1012, 0, 'DEJiPKx5GActUtB6qUssreUxkhXtL4hTQAAJZ7Ccw8se', 'Rawr', 'XD', 6, 981062998.742517000000, 'https://ipfs.io/ipfs/QmRqD1p6iEXa2JuzTa1TvCUq5P7VYgVNeU94xvCE1DTpyx', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1013, 0, 'J3NKxxXZcnNiMjKw9hYb2K4LUxgwB6t1FtPtQVsv3KFr', 'SPX6900 (Wormhole)', 'SPX', 8, 116722302.634397760000, '', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1014, 0, '6zb14yL62nMSjcANXK39tvLghMjxMRXMtvyrxP5UZMa3', 'mubarak', 'mubarak', 6, 99999999996.057867000000, 'https://ipfs.io/ipfs/QmaSxVZDH2UQzR8iY7ANaaXy57tjUYkHLuxMNQV5wjSYPs', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1015, 0, 'AEsHWNsasurhsjuErPoFxHwzJYBYMUj2zgNzTggcwTXB', '$Benji', 'BJ', 9, 999993337.143209783000, 'https://ipfs.io/ipfs/QmNRD9V3tc6nVxKpqVoyHnryZE2kKTGH95ipHvL9MePhBC', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1016, 0, 'HV3wS3R5PW3aVxWwnKgirjWVo2CfggwCFPWfpv9CmQLE', 'Nillion', 'NIL', 6, 94854053816.953456000000, 'https://ipfs.io/ipfs/QmXTFQdv1xtevUrxJk7hPgyJY3heCRizk35kFykCCB2pbu', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1017, 0, '3fP61JWqLxxrvBv5NSxJdAz4GtCB7UpfU3UFxZTgj1yG', 'Warrior Coin', '$WAR', 6, 139999511113.176430000000, 'https://ipfs.io/ipfs/QmPi33DnaVzqauLXXspgy4Nub3XiDMRMjJ8Xf2zv8n3kVV', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1018, 0, '3X5knBqSB7x7qNgif4VU6DmDa37mrXL5AdCD7BP2RaNv', 'Donald Trump', 'TRUMP', 9, 4999.998909528000, 'https://ipfs.io/ipfs/QmSct97y5yKWBFBd4sg8Bxvttb1FkRbhUPjFvq3YUiwgZg', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1019, 0, 'A8C3xuqscfmyLrte3VmTqrAq8kgMASius9AFNANwpump', 'FWOG', 'FWOG', 6, 975620467.904113000000, 'https://ipfs.io/ipfs/QmVAvr3r1q2NrFHsY5fvrkJCdBniGM326U2pAvfpvgkwDR', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1020, 0, '5mbK36SZ7J19An8jFochhQS4of8g6BwUjbeCSxBSoWdp', 'michi', '$michi', 6, 555765197.296285000000, 'https://ipfs.io/ipfs/QmaFSKo4FX43NsVETn4nPfnFrXMqcGyK4mvKuCz2Pg65ji', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1021, 0, 'B5WTLaRwaUQpKk7ir1wniNB6m5o8GgMrimhKMYan2R6B', 'Pepe', 'Pepe', 6, 999473404.013471000000, 'https://ipfs.io/ipfs/QmSfYnQSoUfuvd2SUGfHdfvqUSq8tyYmWBA7k7dt1o5MWV', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1022, 0, 'Grass7B4RdKfBCjTKgSqnXkqjwiGvQyFbuSCUJr3XXjs', 'Grass', 'GRASS', 9, 999995275.731816853000, 'https://static.getgrass.io/grass/metadata.json', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1023, 0, 'FeR8VBqNRSUD5NtXAj2n3j1dAHkZHfyDktKuLXD4pump', 'jelly-my-jelly', 'jellyjelly', 6, 999973966.660491000000, 'https://ipfs.io/ipfs/QmRaah2aa24T3F2hGQCf8XefSuaNFZM2wGx2W2UnsxfLxM', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1024, 0, 'FmFnRZWRLnZMcRDQGDCoHSdgLUFMZ7bAtD2qk75Vpump', 'Pyramids', 'Giza', 6, 999626398.612413000000, 'https://ipfs.io/ipfs/Qma9RUEYSuJQHXzfvJh45AmGByeifb2LQzA6XdQA1dggN1', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1025, 0, 'FbQYjLEq1vNCszmxmxZDoFiy9fgyfdPxzt9Fu5zk5jJ4', null, null, 9, 4178.841065009000, null, null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1026, 0, '5NAdTPgqGV5w2VjhRojSWHkukyZRz3NExMHHZQ1xLMcV', null, null, 6, 115561745.674238000000, null, null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1027, 0, 'HhvU1ZDn2m42gLyWQMkNUkW7N6WJcJ8Krtr2KmRhR4pH', null, null, 8, 163385.054308040000, null, null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1028, 0, 'Cqc5qH8kRqU4ASorJF2fiH9t1BSU4a7StagJiyexpump', 'Downald Trump', 'Downald', 6, 999578496.787052000000, 'https://ipfs.io/ipfs/QmQaygd78nY28htENGqUU5KXSYnyjTAHSzvVWfW55UBBnf', null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1029, 0, '9uZvszw4PxgGdD3CxCGtoFLGUqQpff5XwY23joeMt1Eb', null, null, 6, 17447952004.697174000000, null, null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1030, 0, '3RpEekjLE5cdcG15YcXJUpxSepemvq2FpmMcgo342BwC', null, null, 6, 15945112.248057000000, null, null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1031, 0, 'FZN7QZ8ZUUAxMPfxYEYkH3cXUASzH8EqA6B4tyCL8f1j', null, null, 9, 218620.897713706000, null, null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00'),
        (1032, 0, 'CPSDnt3NQHXeP74jb1Xx1RQKBsB3M2LyEsbBXdQzbDY4', null, null, 9, 90.262319483000, null, null, null, null, null, null, null, '2025-04-21 05:49:22.677882 +00:00', '2025-04-21 05:49:22.677882 +00:00');

insert into solana.token_pair (id, base_id, quote_id) values
        (1000, 1000, 1),
        (1001, 1001, 1),
        (1002, 1002, 1),
        (1003, 1003, 1),
        (1004, 1004, 1),
        (1005, 1005, 1),
        (1006, 1006, 1),
        (1007, 1007, 1),
        (1008, 1008, 1),
        (1009, 1009, 1),
        (1010, 1010, 3),
        (1011, 1011, 3),
        (1012, 1011, 1),
        (1013, 1012, 1),
        (1014, 1013, 3),
        (1015, 1014, 1),
        (1016, 1015, 1),
        (1017, 1016, 1),
        (1018, 1017, 3),
        (1019, 1018, 1),
        (1020, 1019, 1),
        (1021, 1020, 1),
        (1022, 1021, 1),
        (1023, 2, 3),
        (1024, 1022, 1),
        (1025, 1023, 1),
        (1026, 1024, 1);

insert into pumpswap.pool (id, token_pair_id, inverted, creator_id, slot)
values  (24, 1003, false, 25, null),
        (26, 1004, false, 27, null),
        (28, 1005, false, 29, null),
        (30, 1006, false, 31, null),
        (32, 1007, false, 33, null),
        (34, 1008, false, 35, null),
        (36, 1009, false, 37, null);

        "#).await.unwrap();

		let state = setup(pool.clone());
		index_block(state, block).await;

		assert_all_pumpswap_swaps(pool.clone()).await;
		assert_pumpswap_swap(pool.clone()).await;
		assert_pumpswap_micro_swap(pool.clone()).await;

	})
		.await
}

async fn assert_all_pumpswap_swaps(pool: PgPool) {
    assert_sql!(&pool, "(select count(*) from pumpswap.swap) = 16");
    assert_sql!(&pool, "(select count(*) from pumpswap.micro_swap) = 5");

    // makes sure that all signatures found in solscan are in our tables as well

    assert_sql!(&pool, "(select count(*) from pumpswap.swap where signature = '21dzCXhx1xj5Yvjk7nviCZ1eKEA1dksp6ns3CQeBb5YFxTPY9mnyvDQyxeuybFadDqbpQVj8bbXsA5GFe5cFjeJj') = 2");
    assert_sql!(&pool, "(select count(*) from pumpswap.swap where signature = 'Cdsd7XSEDCsd5NZQ6JNTiLwGXNTyT3S8PYh5cCY4y57cDxt5VyPYSvtNkSAsBV25Tbf9JB9hfL3s9yqqL1Hxfni') = 2");
    assert_sql!(&pool, "(select count(*) from pumpswap.swap where signature = '4RF7wYt4seMhTnNLMreuerM6AW79mtfuSXrax7BA2bryqRfsyixvzBUfQo1zHgbxZYJERyx9cbDsFRMrsZ9AWx6D') = 2");

    for signature in [
        "3BmWyJZB8zYiPZ6nXqTvzPZDsdjh5uxu5rRFUtbaEQz2g5yaWfL4xkpK7jmpePUhepydQ63gBe55bfPBm6xMWcjr",
        "3fkiJoFGJ9foYa8ji5SNS1rc1gv8AT2sc5rqwdWTRxUfobnXXMAbCD3p5R8b2mm1uKbENJXcg9fic1WKTt7zzTiY",
        "XMTdBUkK2HQWGCrm6vS5GdH3aPg31W9KGyH31SJM63t3XUW6wr6QvNnQUGXzGymJcK3D7GgZzuZV7km1KmvWeP5",
        "4oM32qWRxTEo2oe1Hgr8A4Y6KZ1c9f6a3RvVSf1f3PLMmRPJig8UAvtG6hQrRSbSYvUjRZTRVJXrfhTYkTZwp6WG",
        "4jm1mY8qD9xehmQcrKSNBKByCj7gZMJ5yDtUyR3ZFZbX43WAQ8k34mp5yyPg8YdB83nEyazPK6R54JPN3gAQ7QPC",
        // "21dzCXhx1xj5Yvjk7nviCZ1eKEA1dksp6ns3CQeBb5YFxTPY9mnyvDQyxeuybFadDqbpQVj8bbXsA5GFe5cFjeJj",
        // "Cdsd7XSEDCsd5NZQ6JNTiLwGXNTyT3S8PYh5cCY4y57cDxt5VyPYSvtNkSAsBV25Tbf9JB9hfL3s9yqqL1Hxfni",
        "2u37jvKmxe1TcpJfLURWvDVnoJg9NsqurfYi2No4GR4idFKCrPzYA4RGPrqzcKcFJgGvwcw2Y4TsdLJuz2EsbNkB",
        "3edGEeCPViMTrDoPjgfZ7nCvcEQ8CnPa2mgjcKUGc6pzUGyNRMErd2VZH6KJin8zVJxCApaeKHxmfm7EG9ZNunNA",
        "396mYuEggVhczMiB3GXZdnpqpPFMFWN4iUvggZN9LusdKtPZmZRRNMgKL3FyZMemsqiszAu4bzNfSjKuEq5mtnzN",
        "5nfTXNKwPzgMp51wmPXn6sE2rStKRNkrMxpNeKxG2URsUbWr4B5FVoo13WtqKzXv3y7bb3fzjdTnrxyGtzYN5tHg",
        "4kJGrYQTujJPzHrgg7iYAeBFKs6rCnXeG4rh65tPUGWWV9YFSmD4C1SvTAywqm1NBBtcQ4s7zwV8UsgJ8MpnPKBY",
        "2T5vxZs8vnfGX2R9dxmi5EkCGhpRJ3jHvQfpq8Bj3usFHy9BBBfM7YW7QQMhtsZ2ow8JUeqarwRHKfu2ThWfhzi2",
        "3mX3SLcuTZ5aqZnDPLpxBsU9rk2r46zk3vPT8TwuBB5gn2AiutaKpcPtxgkCabUKv22rkDKwP7Q4vAxqEunfGf74",
        "5tWbP73HwLHXLqbeZAxBpsnL3d8Ho1aP13qxUC3MNV5M1HoKpfemnWWqDmxyN8fHGMctmCSJJVyAWbFA1ezNW86n",
        "5DzcMrUBt6cV3Zvcvqs3aF7unvfhsu8AkKMpSpQhdTBxkyUidtD5yaUA6tRGB1WcTJzixz26hAYXqxqU29xPLXRc",
        "CVKcLQmSVy5xMgMdb6d9xqGxmGcrCsxo6VSXn4f1t1ZvYWPSzWzp7fJQguXQjitSkQF3xTGN5ScwjnuBVKZE4ZJ",
        // "4RF7wYt4seMhTnNLMreuerM6AW79mtfuSXrax7BA2bryqRfsyixvzBUfQo1zHgbxZYJERyx9cbDsFRMrsZ9AWx6D",
    ] {
        assert_sql!(
            &pool,
            format!(
                r#"
(
	select 1 from pumpswap.swap where signature = '{signature}'
	union all
	select 1 from pumpswap.micro_swap where signature = '{signature}'
) = 1
    	"#
            )
        );
    }
}

async fn assert_pumpswap_swap(pool: PgPool) {
    assert_sql!(&pool, "(select count(*) from pumpswap.swap where signature = '21dzCXhx1xj5Yvjk7nviCZ1eKEA1dksp6ns3CQeBb5YFxTPY9mnyvDQyxeuybFadDqbpQVj8bbXsA5GFe5cFjeJj') = 2");
    // buy
    assert_sql!(&pool, "(select count(*) from pumpswap.swap where signature = '21dzCXhx1xj5Yvjk7nviCZ1eKEA1dksp6ns3CQeBb5YFxTPY9mnyvDQyxeuybFadDqbpQVj8bbXsA5GFe5cFjeJj' and is_buy = true) = 1");
    assert_sql!(&pool, "(select token_pair_id from pumpswap.swap where signature = '21dzCXhx1xj5Yvjk7nviCZ1eKEA1dksp6ns3CQeBb5YFxTPY9mnyvDQyxeuybFadDqbpQVj8bbXsA5GFe5cFjeJj' and is_buy = true) = 1006");
    assert_sql!(&pool, "(select pool_id from pumpswap.swap where signature = '21dzCXhx1xj5Yvjk7nviCZ1eKEA1dksp6ns3CQeBb5YFxTPY9mnyvDQyxeuybFadDqbpQVj8bbXsA5GFe5cFjeJj' and is_buy = true) = 30");
    assert_sql!(&pool, "(select address_id from pumpswap.swap where signature = '21dzCXhx1xj5Yvjk7nviCZ1eKEA1dksp6ns3CQeBb5YFxTPY9mnyvDQyxeuybFadDqbpQVj8bbXsA5GFe5cFjeJj' and is_buy = true) = 11");
    assert_sql!(&pool, "(select slot from pumpswap.swap where signature = '21dzCXhx1xj5Yvjk7nviCZ1eKEA1dksp6ns3CQeBb5YFxTPY9mnyvDQyxeuybFadDqbpQVj8bbXsA5GFe5cFjeJj' and is_buy = true) = 328790579");
    assert_sql!(&pool, "(select amount_quote from pumpswap.swap where signature = '21dzCXhx1xj5Yvjk7nviCZ1eKEA1dksp6ns3CQeBb5YFxTPY9mnyvDQyxeuybFadDqbpQVj8bbXsA5GFe5cFjeJj' and is_buy = true) = '1.023694759'");
    assert_sql!(&pool, "(select amount_base from pumpswap.swap where signature = '21dzCXhx1xj5Yvjk7nviCZ1eKEA1dksp6ns3CQeBb5YFxTPY9mnyvDQyxeuybFadDqbpQVj8bbXsA5GFe5cFjeJj' and is_buy = true) = '1443114.296867'");

    // sell
    assert_sql!(&pool, "(select count(*) from pumpswap.swap where signature = '21dzCXhx1xj5Yvjk7nviCZ1eKEA1dksp6ns3CQeBb5YFxTPY9mnyvDQyxeuybFadDqbpQVj8bbXsA5GFe5cFjeJj' and is_buy = false) = 1");
    assert_sql!(&pool, "(select token_pair_id from pumpswap.swap where signature = '21dzCXhx1xj5Yvjk7nviCZ1eKEA1dksp6ns3CQeBb5YFxTPY9mnyvDQyxeuybFadDqbpQVj8bbXsA5GFe5cFjeJj' and is_buy = false) = 1006");
    assert_sql!(&pool, "(select pool_id from pumpswap.swap where signature = '21dzCXhx1xj5Yvjk7nviCZ1eKEA1dksp6ns3CQeBb5YFxTPY9mnyvDQyxeuybFadDqbpQVj8bbXsA5GFe5cFjeJj' and is_buy = false) = 30");
    assert_sql!(&pool, "(select address_id from pumpswap.swap where signature = '21dzCXhx1xj5Yvjk7nviCZ1eKEA1dksp6ns3CQeBb5YFxTPY9mnyvDQyxeuybFadDqbpQVj8bbXsA5GFe5cFjeJj' and is_buy = false) = 11");
    assert_sql!(&pool, "(select slot from pumpswap.swap where signature = '21dzCXhx1xj5Yvjk7nviCZ1eKEA1dksp6ns3CQeBb5YFxTPY9mnyvDQyxeuybFadDqbpQVj8bbXsA5GFe5cFjeJj' and is_buy = false) = 328790579");
    assert_sql!(&pool, "(select amount_quote from pumpswap.swap where signature = '21dzCXhx1xj5Yvjk7nviCZ1eKEA1dksp6ns3CQeBb5YFxTPY9mnyvDQyxeuybFadDqbpQVj8bbXsA5GFe5cFjeJj' and is_buy = false) = '1.019115871'");
    assert_sql!(&pool, "(select amount_base from pumpswap.swap where signature = '21dzCXhx1xj5Yvjk7nviCZ1eKEA1dksp6ns3CQeBb5YFxTPY9mnyvDQyxeuybFadDqbpQVj8bbXsA5GFe5cFjeJj' and is_buy = false) = '1443114.296867'");
}

async fn assert_pumpswap_micro_swap(pool: PgPool) {
    // 5nfTXNKwPzgMp51wmPXn6sE2rStKRNkrMxpNeKxG2URsUbWr4B5FVoo13WtqKzXv3y7bb3fzjdTnrxyGtzYN5tHg
    assert_sql!(&pool, "(select is_buy from pumpswap.micro_swap where signature = '5nfTXNKwPzgMp51wmPXn6sE2rStKRNkrMxpNeKxG2URsUbWr4B5FVoo13WtqKzXv3y7bb3fzjdTnrxyGtzYN5tHg') = true");
    assert_sql!(&pool, "(select slot from pumpswap.micro_swap where signature = '5nfTXNKwPzgMp51wmPXn6sE2rStKRNkrMxpNeKxG2URsUbWr4B5FVoo13WtqKzXv3y7bb3fzjdTnrxyGtzYN5tHg') = 328790579");
    assert_sql!(&pool, "(select amount_quote from pumpswap.micro_swap where signature = '5nfTXNKwPzgMp51wmPXn6sE2rStKRNkrMxpNeKxG2URsUbWr4B5FVoo13WtqKzXv3y7bb3fzjdTnrxyGtzYN5tHg') = '0.000000448000'");
    assert_sql!(&pool, "(select amount_base from pumpswap.micro_swap where signature = '5nfTXNKwPzgMp51wmPXn6sE2rStKRNkrMxpNeKxG2URsUbWr4B5FVoo13WtqKzXv3y7bb3fzjdTnrxyGtzYN5tHg') = '0.519627000000'");

    // 3edGEeCPViMTrDoPjgfZ7nCvcEQ8CnPa2mgjcKUGc6pzUGyNRMErd2VZH6KJin8zVJxCApaeKHxmfm7EG9ZNunNA
    assert_sql!(&pool, "(select is_buy from pumpswap.micro_swap where signature = '3edGEeCPViMTrDoPjgfZ7nCvcEQ8CnPa2mgjcKUGc6pzUGyNRMErd2VZH6KJin8zVJxCApaeKHxmfm7EG9ZNunNA') = false");
    assert_sql!(&pool, "(select slot from pumpswap.micro_swap where signature = '3edGEeCPViMTrDoPjgfZ7nCvcEQ8CnPa2mgjcKUGc6pzUGyNRMErd2VZH6KJin8zVJxCApaeKHxmfm7EG9ZNunNA') = 328790579");
    assert_sql!(&pool, "(select token_pair_id from pumpswap.micro_swap where signature = '3edGEeCPViMTrDoPjgfZ7nCvcEQ8CnPa2mgjcKUGc6pzUGyNRMErd2VZH6KJin8zVJxCApaeKHxmfm7EG9ZNunNA') = 1007");
    assert_sql!(&pool, "(select pool_id from pumpswap.micro_swap where signature = '3edGEeCPViMTrDoPjgfZ7nCvcEQ8CnPa2mgjcKUGc6pzUGyNRMErd2VZH6KJin8zVJxCApaeKHxmfm7EG9ZNunNA') = 32");
    assert_sql!(&pool, "(select address_id from pumpswap.micro_swap where signature = '3edGEeCPViMTrDoPjgfZ7nCvcEQ8CnPa2mgjcKUGc6pzUGyNRMErd2VZH6KJin8zVJxCApaeKHxmfm7EG9ZNunNA') = 14");
    assert_sql!(&pool, "(select amount_quote from pumpswap.micro_swap where signature = '3edGEeCPViMTrDoPjgfZ7nCvcEQ8CnPa2mgjcKUGc6pzUGyNRMErd2VZH6KJin8zVJxCApaeKHxmfm7EG9ZNunNA') = '0.000000479000'");
    assert_sql!(&pool, "(select amount_base from pumpswap.micro_swap where signature = '3edGEeCPViMTrDoPjgfZ7nCvcEQ8CnPa2mgjcKUGc6pzUGyNRMErd2VZH6KJin8zVJxCApaeKHxmfm7EG9ZNunNA') = '0.559996000000'");
}
