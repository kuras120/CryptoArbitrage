mod model;

use model::order::SwapOrder;

pub async fn run() {
    const JUP_API_HOST_NAME: &str = "https://lite-api.jup.ag";
    const JUP_API_GET_ORDER_PATH: &str = "/ultra/v1/order";
    const JUP_API_GET_QUOTE_PATH: &str = "/swap/v1/quote";

    const SOL_ADDRESS: &str = "So11111111111111111111111111111111111111112";
    const USDC_ADDRESS: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

    println!(
        "Hello, world! This is jupiter api hostname={}",
        JUP_API_HOST_NAME
    );

    let params = [
        ("inputMint", SOL_ADDRESS),
        ("outputMint", USDC_ADDRESS),
        ("amount", "1000000000"),
    ];
    let url = reqwest::Url::parse_with_params(
        [JUP_API_HOST_NAME, JUP_API_GET_QUOTE_PATH]
            .concat()
            .as_str(),
        params,
    )
        .unwrap();
    let resp_result = reqwest::get(url).await;
    if let Ok(resp_result) = resp_result {
        let resp_status = resp_result.status().clone();
        let resp_headers = resp_result.headers().clone();
        println!("Resp status={:?}, headers={:?}", resp_status, resp_headers);
        let text_payload: String = resp_result.text().await.unwrap();
        let json_payload = serde_json::from_str::<SwapOrder>(text_payload.as_str());
        if let Ok(payload) = json_payload {
            println!("Payload={}", serde_json::to_string_pretty(&payload).unwrap());
        } else if let Err(err) = json_payload {
            println!("Payload error={:?}", text_payload);
            println!("Err={}", err);
        }
    } else {
        println!("Resp error={:?}", resp_result.err());
    }
}
