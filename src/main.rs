mod jupiter;
mod binance;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // jupiter::run().await;
    binance::run().await;
}
