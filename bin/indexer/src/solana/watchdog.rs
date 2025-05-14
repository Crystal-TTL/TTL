// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use base::model::solana::Slot;
use common::Signal;
use log::{error, trace};
use mpsc::Receiver;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::time;

pub struct Watchdog {
    receiver: Receiver<Slot>,
    timeout: Duration,
    signal: Signal,
}

impl Watchdog {
    pub fn new(receiver: Receiver<Slot>, timeout: Duration, signal: Signal) -> Self {
        Self {
            receiver,
            timeout,
            signal,
        }
    }

    pub fn spawn(mut self) -> JoinHandle<()> {
        tokio::spawn(async move {
            let mut last_seen = Instant::now();
            loop {
                tokio::select! {
                    Some(slot) = self.receiver.recv() => {
                        trace!("received: {}", slot);
                        last_seen = Instant::now();
                    }
                    _ = time::sleep(Duration::from_secs(1)) => {
                        if last_seen.elapsed() > self.timeout {
                            let message = format!("no blocks indexed in the last {} seconds", self.timeout.as_secs());
                            error!("{}", message);
                            self.signal.terminate(message);
                            return;
                        }
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::solana::watchdog::Watchdog;
    use base::model::solana::Slot;
    use common::{Signal, SignalType};
    use std::time::Duration;
    use tokio::join;
    use tokio::sync::mpsc::channel;
    use tokio::time::sleep;

    #[test_log::test(tokio::test)]
    async fn test_does_not_trigger() {
        let mut signal = Signal::default();
        let (watchdog_tx, watchdog_rx) = channel::<Slot>(1);
        let test_instance = Watchdog::new(watchdog_rx, Duration::from_millis(10), signal.clone());

        let handle = tokio::spawn(async move {
            for idx in 0..100 {
                watchdog_tx.send(idx.into()).await.unwrap();
                sleep(Duration::from_millis(1)).await;
            }
        });

        test_instance.spawn();
        let _ = join!(handle);

        assert_eq!(signal.recv_maybe().await, None);
    }

    #[test_log::test(tokio::test)]
    async fn test_timeout_triggers_terminal() {
        let mut signal = Signal::default();
        let (_watchdog_tx, watchdog_rx) = channel::<Slot>(1);
        let test_instance = Watchdog::new(watchdog_rx, Duration::from_millis(10), signal.clone());

        let handle = test_instance.spawn();
        let _ = join!(handle);

        let signal = signal.recv().await;
        assert_eq!(
            signal,
            SignalType::terminate("no blocks indexed in the last 0 seconds")
        );
    }
}
