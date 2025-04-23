use mini_async_runtime::{MiniRuntime, TimerFuture};
use std::time::Duration;

async fn sleep(duration: Duration) {
    TimerFuture::new(duration).await;
}

async fn task_one() {
    println!("task one: start");
    sleep(Duration::from_secs(1)).await;
    println!("task one: done");
}

async fn task_two() {
    println!("task two: start");
    sleep(Duration::from_secs(2)).await;
    println!("task two: done");
}

fn main() {
    let mut rt = MiniRuntime::new();
    println!("Runtime started...");
    rt.spawn(task_one());
    rt.spawn(task_two());
    rt.block_on(async {}); // Kick off polling
}
