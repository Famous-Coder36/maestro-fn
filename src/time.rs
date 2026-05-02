use std::time::{SystemTime, UNIX_EPOCH};
use std::future::Future;

pub struct Time;

impl Time {

   
    pub async fn sleep(ms: u64) {
        tokio::time::sleep(
            std::time::Duration::from_millis(ms)
        ).await;
    }

    pub async fn sleep_sec(sec: u64) {
        tokio::time::sleep(
            std::time::Duration::from_secs(sec)
        ).await;
    }

    pub fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    pub fn now_ms() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }

    pub async fn timeout<T>(
        ms: u64,
        fut: impl Future<Output = T>
    ) -> Option<T> {
        match tokio::time::timeout(
            std::time::Duration::from_millis(ms),
            fut
        ).await {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

 
    pub fn every(ms: u64) -> TimeInterval {
        TimeInterval { ms }
    }

    pub fn measure<T>(label: &str, f: impl FnOnce() -> T) -> T {
        let start = std::time::Instant::now();
        let result = f();
        println!("{}: {:?}", label, start.elapsed());
        result
    }

    pub fn since(start: std::time::Instant) -> u128 {
        start.elapsed().as_millis()
    }
}

pub struct TimeInterval {
    ms: u64,
}

impl TimeInterval {

    pub async fn run(self, mut f: impl FnMut()) {
    let mut interval =
        tokio::time::interval(
            std::time::Duration::from_millis(self.ms)
        );

    loop {
        interval.tick().await;
        f(); // OK
    }
}
}