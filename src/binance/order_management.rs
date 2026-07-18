use crate::binance::model::order_book::OpenOrder;
use binance_spot_connector_rust::trade;
use binance_spot_connector_rust::ureq::BinanceHttpClient;
use serde_json::Error as JsonError;
use tracing::info;

pub fn get_open_orders(client: &BinanceHttpClient) -> Result<Vec<OpenOrder>, JsonError> {
    let open_orders_response = client
        .send(trade::open_orders())
        .unwrap()
        .into_body_str()
        .unwrap();

    let orders: Vec<OpenOrder> = serde_json::from_str(&open_orders_response)?;

    info!("Found {} open orders", orders.len());
    for order in &orders {
        info!(
            "Order ID: {}, Symbol: {}, Side: {}, Type: {}, Price: {}, Original Qty: {}, Status: {}",
            order.order_id,
            order.symbol,
            order.side,
            order.r#type,
            order.price,
            order.orig_qty,
            order.status
        );
    }

    Ok(orders)
}

pub fn cancel_order(
    client: &BinanceHttpClient,
    symbol: &str,
    order_id: u64,
) -> Result<OpenOrder, JsonError> {
    let cancel_response = client
        .send(trade::cancel_order(symbol).order_id(order_id))
        .unwrap()
        .into_body_str()
        .unwrap();

    let canceled_order: OpenOrder = serde_json::from_str(&cancel_response)?;

    info!(
        "Successfully canceled order ID: {} for symbol: {}",
        canceled_order.order_id, canceled_order.symbol
    );

    Ok(canceled_order)
}

pub fn cancel_all_orders(
    client: &BinanceHttpClient,
    symbol: &str,
) -> Result<Vec<OpenOrder>, JsonError> {
    let cancel_all_response = client
        .send(trade::cancel_open_orders(symbol))
        .unwrap()
        .into_body_str()
        .unwrap();

    let canceled_orders: Vec<OpenOrder> = serde_json::from_str(&cancel_all_response)?;

    info!(
        "Canceled {} orders for symbol {}",
        canceled_orders.len(),
        symbol
    );

    Ok(canceled_orders)
}
