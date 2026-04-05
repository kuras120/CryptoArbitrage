use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub maker_commission: i64,
    pub taker_commission: i64,
    pub buyer_commission: i64,
    pub seller_commission: i64,
    pub commission_rates: CommissionRates,
    pub can_trade: bool,
    pub can_withdraw: bool,
    pub can_deposit: bool,
    pub brokered: bool,
    pub require_self_trade_prevention: bool,
    pub prevent_sor: bool,
    pub update_time: i64,
    pub account_type: String,
    pub balances: Vec<Balance>,
    pub permissions: Vec<String>,
    pub uid: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommissionRates {
    pub maker: String,
    pub taker: String,
    pub buyer: String,
    pub seller: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub asset: String,
    pub free: String,
    pub locked: String,
}
