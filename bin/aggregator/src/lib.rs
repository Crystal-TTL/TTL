// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use async_trait::async_trait;
use common::model::Partition;
use common::Signal;
use std::sync::Arc;
use std::time::Duration;
use tokio::spawn;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::sleep;

pub mod config;
pub mod jupiter;
pub mod leaderboard;
pub mod pumpfun;
pub mod pumpswap;
pub mod pumpup;
pub mod solana;
pub mod time;

#[async_trait]
pub(crate) trait Worker<T>: Send + Sync {
    async fn process(&self, task: T);
}

pub(crate) async fn partitioned<T, W>(signal: Signal, receivers: Vec<Receiver<T>>, worker: Arc<W>)
where
    T: Send + 'static,
    W: Worker<T> + 'static,
{
    for (i, mut rx) in receivers.into_iter().enumerate() {
        let worker = Arc::clone(&worker);
        let signal = signal.clone();
        spawn(async move {
            while let Some(task) = rx.recv().await {
                worker.process(task).await;
                sleep(Duration::from_millis(100)).await;
            }

            signal.terminate(format!(
                "worker for partition {i} has exited (channel closed)."
            ));
        });
    }
}

pub(crate) async fn send_every(senders: Vec<Sender<Partition>>, every: Duration) {
    let mut i = 0;
    loop {
        let tx = &senders[i];
        tx.send(Partition(i + 1)).await.unwrap();
        i = (i + 1) % 8;

        if i == 0 {
            sleep(every).await;
        }
    }
}
