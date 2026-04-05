use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvgPrice {
    mins: usize,
    price: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    close_time: DateTime<Utc>,
}
