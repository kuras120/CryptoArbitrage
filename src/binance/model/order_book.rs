use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub last_update_id: u64,
    pub bids: Vec<(String, String)>,
    pub asks: Vec<(String, String)>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenOrder {
    pub symbol: String,
    pub order_id: u64,
    pub client_order_id: String,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub status: String,
    pub time_in_force: String,
    pub r#type: String,
    pub side: String,
    pub time: u64,
    pub update_time: u64,
    pub working: bool,
}

impl OrderBook {
    #[expect(
        dead_code,
        reason = "best-level accessors are reserved for the arbitrage coordinator"
    )]
    pub fn get_best_bid(&self) -> Option<(Decimal, Decimal)> {
        self.bids.first().map(|(price, qty)| {
            (
                price.parse().unwrap_or(Decimal::ZERO),
                qty.parse().unwrap_or(Decimal::ZERO),
            )
        })
    }

    #[expect(
        dead_code,
        reason = "best-level accessors are reserved for the arbitrage coordinator"
    )]
    pub fn get_best_ask(&self) -> Option<(Decimal, Decimal)> {
        self.asks.first().map(|(price, qty)| {
            (
                price.parse().unwrap_or(Decimal::ZERO),
                qty.parse().unwrap_or(Decimal::ZERO),
            )
        })
    }

    pub fn find_bids_better_than(&self, target_price: Decimal) -> Vec<(Decimal, Decimal)> {
        self.bids
            .iter()
            .filter_map(|(price_str, qty_str)| {
                let price = price_str.parse().ok()?;
                let qty = qty_str.parse().ok()?;
                if price >= target_price {
                    Some((price, qty))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn find_asks_better_than(&self, target_price: Decimal) -> Vec<(Decimal, Decimal)> {
        self.asks
            .iter()
            .filter_map(|(price_str, qty_str)| {
                let price = price_str.parse().ok()?;
                let qty = qty_str.parse().ok()?;
                if price <= target_price {
                    Some((price, qty))
                } else {
                    None
                }
            })
            .collect()
    }
}
