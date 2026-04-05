use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapOrder {
    input_mint: String,
    output_mint: String,
    in_amount: String,
    out_amount: String,
    other_amount_threshold: String,
    swap_mode: String,
    slippage_bps: usize,
    platform_fee: Option<usize>,
    price_impact_pct: String,
    route_plan: Vec<RoutePlan>,
    context_slot: usize,
    time_taken: f64,
    swap_usd_value: String,
    simpler_route_used: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    mode: String,
    swap_type: String,
    router: String,
    request_id: String,
    in_amount: String,
    out_amount: String,
    other_amount_threshold: String,
    swap_mode: String,
    slippage_bps: usize,
    price_impact_pct: String,
    route_plan: Vec<RoutePlan>,
    input_mint: String,
    output_mint: String,
    fee_mint: Option<String>,
    fee_bps: usize,
    prioritization_fee_lamports: usize,
    transaction: Option<usize>,
    gasless: bool,
    taker: Option<String>,
    total_time: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RoutePlan {
    swap_info: SwapInfo,
    percent: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SwapInfo {
    amm_key: String,
    label: String,
    input_mint: String,
    output_mint: String,
    in_amount: String,
    out_amount: String,
    fee_amount: String,
    fee_mint: String,
}
