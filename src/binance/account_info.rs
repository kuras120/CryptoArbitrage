use binance_spot_connector_rust::trade;
use binance_spot_connector_rust::ureq::BinanceHttpClient;
use tracing::info;
use crate::binance::model::account::AccountInfo;

pub async fn get_account_info(client: BinanceHttpClient) -> Option<AccountInfo> {
    let balance_response = client.send(
        trade::account()
    ).unwrap()
        .into_body_str()
        .unwrap();

    match serde_json::from_str::<AccountInfo>(&balance_response) {
        Ok(account_info) => {
            info!("Account type: {}", account_info.account_type);
            info!("Can trade: {}", account_info.can_trade);

            for balance in account_info.balances.iter().filter(
                |b| b.free != "0.00000000" || b.locked != "0.00000000"
            ) {
                info!("Asset: {}, Free: {}, Locked: {}", balance.asset, balance.free, balance.locked);
            }
            Some(account_info)
        },
        Err(e) => {
            info!("Failed to parse account information: {:?}", e);
            info!("Raw response: {}", balance_response);
            None
        }
    }
}
