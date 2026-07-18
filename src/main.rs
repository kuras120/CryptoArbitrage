mod binance;
#[expect(
    dead_code,
    reason = "the Jupiter integration is not wired into the runtime yet"
)]
mod jupiter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // jupiter::run().await;
    binance::run().await;
}
