use mini_async_runtime::{MiniRuntime, TimerFuture};
use std::time::{Duration, Instant};

async fn sleep(duration: Duration) {
    TimerFuture::new(duration).await;
}

async fn task_one() {
    println!("task one: start");
    let start = Instant::now();
    sleep(Duration::from_secs(1)).await;
    let elapsed = start.elapsed().as_secs();
    println!("task one: done   [~{}s]", elapsed);
}

async fn task_two() {
    println!("task two: start");
    let start = Instant::now();
    sleep(Duration::from_secs(2)).await;
    let elapsed = start.elapsed().as_secs();
    println!("task two: done   [~{}s]", elapsed);
}

fn main() {
    let mut rt = MiniRuntime::new();
    println!("Runtime started...");

    rt.spawn(task_one());
    rt.spawn(task_two());

    // Just run an empty future to start the runtime loop
    rt.block_on(async {});
}
