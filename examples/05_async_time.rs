use maestro_fn::*;

#[tokio::main]
async fn main() {
    println!("start");

    Time::sleep_sec(1).await;

    let now = Time::now();
    println!("timestamp: {}", now);
}
