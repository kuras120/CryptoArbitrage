use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteResponse {
    input_mint: String,
    output_mint: String,
    in_amount: String,
    out_amount: String,
    other_amount_threshold: String,
    swap_mode: String,
    slippage_bps: usize,
    platform_fee: Option<PlatformFee>,
    price_impact_pct: String,
    route_plan: Vec<RoutePlan>,
    context_slot: usize,
    time_taken: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlatformFee {
    amount: String,
    fee_bps: usize,
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

#[cfg(test)]
mod tests {
    use super::QuoteResponse;

    #[test]
    fn deserializes_documented_quote_response() {
        let quote: QuoteResponse = serde_json::from_str(include_str!(
            "../../../tests/fixtures/jupiter_quote_response.json"
        ))
        .expect("documented Jupiter quote fixture should deserialize");

        let platform_fee = quote
            .platform_fee
            .expect("fixture should include a platform fee");

        assert_eq!(platform_fee.amount, "1000");
        assert_eq!(platform_fee.fee_bps, 10);
        assert_eq!(quote.context_slot, 299_283_763);
        assert_eq!(quote.time_taken, 0.015_093_836);
    }
}
