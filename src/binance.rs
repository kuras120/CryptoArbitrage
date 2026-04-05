mod model;
mod account_info;
mod order_management;

use binance_spot_connector_rust::http::Credentials;
use binance_spot_connector_rust::{market, trade};
use binance_spot_connector_rust::trade::order::{Side, TimeInForce};
use binance_spot_connector_rust::ureq::BinanceHttpClient;

use rust_decimal_macros::dec;
use tracing::info;

use model::order_book::OrderBook;

pub async fn run() {
    let credentials = Credentials::from_hmac(
        "{api_key}",
        "{secret_key}"
    );
    let client = BinanceHttpClient::with_url("https://testnet.binance.vision")
        .credentials(credentials);

    let account_info = account_info::get_account_info(client.clone()).await;

    if account_info.unwrap().can_trade {
        let symbol = "BTCUSDT";
        let target_price_max = dec!(110000); // Maximum price willing to pay when buying
        let target_price_min = dec!(45000); // Minimum price willing to accept when selling
        let desired_quantity = dec!(0.001); // How much you want to trade
        let trade_side = "BUY"; // BUY or SELL

        // Fetch the current order book with good depth (100 levels)
        let depth_result = client.send(
            market::depth(symbol).limit(100) // Using higher limit to see more orders
        ).unwrap().into_body_str().unwrap();

        match serde_json::from_str::<OrderBook>(&depth_result) {
            Ok(order_book) => {
                info!("Fetched order book with {} bids and {} asks", order_book.bids.len(), order_book.asks.len());

                if trade_side == "BUY" {
                    // We're buying, so we're interested in asks (sell orders)
                    info!("Looking for asks (sell orders) with price <= {}", target_price_max);

                    // Filter asks that are within our target price
                    let viable_asks = order_book.find_asks_better_than(target_price_max);

                    if viable_asks.is_empty() {
                        info!("No suitable orders found in the order book within your price range");
                    } else {
                        // Sort by price (ascending for asks - we want the cheapest first)
                        let mut sorted_asks = viable_asks.clone();
                        sorted_asks.sort_by(|a, b| a.0.cmp(&b.0));

                        info!("Found {} viable orders in the order book:", sorted_asks.len());
                        for (i, (price, qty)) in sorted_asks.iter().enumerate() {
                            info!("Order #{}: Price: {}, Quantity: {}", i + 1, price, qty);
                        }

                        // Select the best price
                        let best_order = &sorted_asks[0];
                        info!("Selected best order: Price: {}, Quantity: {}", best_order.0, best_order.1);

                        // Execute a trade against the selected order by placing a limit order at the exact price
                        info!("Executing trade against selected order at exact price {}", best_order.0);

                        // Use the exact quantity of the target order or our desired quantity (whichever is smaller)
                        let trade_quantity = std::cmp::min(
                            desired_quantity,
                            best_order.1
                        );

                        // To match a specific order, we place a LIMIT order at exactly the same price
                        // with time-in-force = FOK (Full or Cancel), which means it will match immediately
                        // or be canceled if not filled
                        let order_response = client.send(
                            trade::new_order(symbol, Side::Buy, "LIMIT")
                                .quantity(trade_quantity)
                                .price(best_order.0)          // Use the exact price from the order book
                                .time_in_force(TimeInForce::Fok) // Immediate or Cancel - match immediately or cancel
                        ).unwrap().into_body_str().unwrap();

                        info!("Limit FOK order executed to match with specific ask order. Response: {}", order_response);
                        info!("Note: If the target order was already filled by someone else, this order will be canceled automatically.");
                    }
                } else if trade_side == "SELL" {
                    // We're selling, so we're interested in bids (buy orders)
                    info!("Looking for bids (buy orders) with price >= {}", target_price_min);

                    // Filter bids that are within our target price
                    let viable_bids = order_book.find_bids_better_than(target_price_min);

                    if viable_bids.is_empty() {
                        info!("No suitable orders found in the order book within your price range");
                    } else {
                        // Sort by price (descending for bids - we want the highest price first)
                        let mut sorted_bids = viable_bids.clone();
                        sorted_bids.sort_by(|a, b| b.0.cmp(&a.0));

                        info!("Found {} viable orders in the order book:", sorted_bids.len());
                        for (i, (price, qty)) in sorted_bids.iter().enumerate() {
                            info!("Order #{}: Price: {}, Quantity: {}", i + 1, price, qty);
                        }

                        // Select the best price
                        let best_order = &sorted_bids[0];
                        info!("Selected best order: Price: {}, Quantity: {}", best_order.0, best_order.1);

                        // Execute a trade against the selected order by placing a limit order at the exact price
                        info!("Executing trade against selected order at exact price {}", best_order.0);

                        // Use the exact quantity of the target order or our desired quantity (whichever is smaller)
                        let trade_quantity = std::cmp::min(
                            desired_quantity,
                            best_order.1
                        );

                        // To match a specific order, we place a LIMIT order at exactly the same price
                        // with time-in-force = FOK (Full or Cancel) which means it will match immediately
                        // or be canceled if not filled
                        let order_response = client.send(
                            trade::new_order(symbol, Side::Sell, "LIMIT")
                                .quantity(trade_quantity)
                                .price(best_order.0)          // Use the exact price from the order book
                                .time_in_force(TimeInForce::Fok) // Immediate or Cancel - match immediately or cancel
                        ).unwrap().into_body_str().unwrap();

                        info!("Limit FOK order executed to match with specific bid order. Response: {}", order_response);
                        info!("Note: If the target order was already filled by someone else, this order will be canceled automatically.");
                    }
                }
            }
            Err(e) => {
                info!("Failed to parse order book: {:?}", e);
                info!("Raw response: {}", depth_result);
            }
        }
    } else {
        info!("Cannot trade: trading not enabled for this account");
    }
}
