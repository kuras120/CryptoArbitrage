use crate::binance::model::account::AccountInfo;
use binance_spot_connector_rust::trade;
use binance_spot_connector_rust::ureq::BinanceHttpClient;
use std::error::Error;
use std::fmt::{Display, Formatter};
use tracing::info;

#[derive(Debug)]
pub enum AccountInfoError {
    Request(String),
    ResponseBody(String),
    Deserialize(serde_json::Error),
}

impl Display for AccountInfoError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Request(error) => write!(formatter, "account request failed: {error}"),
            Self::ResponseBody(error) => {
                write!(
                    formatter,
                    "account response body could not be read: {error}"
                )
            }
            Self::Deserialize(error) => {
                write!(
                    formatter,
                    "account response could not be deserialized: {error}"
                )
            }
        }
    }
}

impl Error for AccountInfoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Deserialize(error) => Some(error),
            Self::Request(_) | Self::ResponseBody(_) => None,
        }
    }
}

pub async fn get_account_info(client: BinanceHttpClient) -> Result<AccountInfo, AccountInfoError> {
    let balance_response = client
        .send(trade::account())
        .map_err(|error| AccountInfoError::Request(format!("{error:?}")))?
        .into_body_str()
        .map_err(|error| AccountInfoError::ResponseBody(format!("{error:?}")))?;

    let account_info = serde_json::from_str::<AccountInfo>(&balance_response)
        .map_err(AccountInfoError::Deserialize)?;

    info!("Account type: {}", account_info.account_type);
    info!("Can trade: {}", account_info.can_trade);

    for balance in account_info
        .balances
        .iter()
        .filter(|b| b.free != "0.00000000" || b.locked != "0.00000000")
    {
        info!(
            "Asset: {}, Free: {}, Locked: {}",
            balance.asset, balance.free, balance.locked
        );
    }

    Ok(account_info)
}
