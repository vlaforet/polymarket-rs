use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// User position information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    #[serde(rename = "proxyWallet")]
    pub proxy_wallet: String,
    pub asset: String,
    #[serde(rename = "conditionId")]
    pub condition_id: String,
    #[serde(deserialize_with = "super::serde_helpers::deserialize_decimal")]
    pub size: Decimal,
    #[serde(
        rename = "avgPrice",
        deserialize_with = "super::serde_helpers::deserialize_decimal"
    )]
    pub avg_price: Decimal,
    #[serde(
        rename = "initialValue",
        deserialize_with = "super::serde_helpers::deserialize_decimal"
    )]
    pub initial_value: Decimal,
    #[serde(
        rename = "currentValue",
        deserialize_with = "super::serde_helpers::deserialize_decimal"
    )]
    pub current_value: Decimal,
    #[serde(
        rename = "cashPnl",
        deserialize_with = "super::serde_helpers::deserialize_decimal"
    )]
    pub cash_pnl: Decimal,
    #[serde(
        rename = "percentPnl",
        deserialize_with = "super::serde_helpers::deserialize_decimal"
    )]
    pub percent_pnl: Decimal,
    #[serde(
        rename = "totalBought",
        deserialize_with = "super::serde_helpers::deserialize_decimal"
    )]
    pub total_bought: Decimal,
    #[serde(
        rename = "realizedPnl",
        deserialize_with = "super::serde_helpers::deserialize_decimal"
    )]
    pub realized_pnl: Decimal,
    #[serde(
        rename = "percentRealizedPnl",
        deserialize_with = "super::serde_helpers::deserialize_decimal"
    )]
    pub percent_realized_pnl: Decimal,
    #[serde(
        rename = "curPrice",
        deserialize_with = "super::serde_helpers::deserialize_decimal"
    )]
    pub cur_price: Decimal,
    pub redeemable: bool,
    pub mergeable: bool,
    pub title: String,
    #[serde(rename = "eventId")]
    pub event_id: String,
    pub outcome: String,
    #[serde(rename = "outcomeIndex")]
    pub outcome_index: u32,
    #[serde(rename = "oppositeOutcome")]
    pub opposite_outcome: String,
    #[serde(rename = "oppositeAsset")]
    pub opposite_asset: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    #[serde(rename = "negativeRisk")]
    pub negative_risk: bool,
}

/// User position value summary
#[derive(Debug, Serialize, Deserialize)]
pub struct PositionValue {
    pub user: String,
    #[serde(deserialize_with = "super::serde_helpers::deserialize_decimal")]
    pub value: Decimal,
}

/// Parameters for querying trades
#[derive(Debug, Clone, Default)]
pub struct TradeParams {
    pub id: Option<String>,
    pub maker_address: Option<String>,
    pub market: Option<String>,
    pub asset_id: Option<String>,
    pub before: Option<u64>,
    pub after: Option<u64>,
}

impl TradeParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn maker_address(mut self, maker_address: impl Into<String>) -> Self {
        self.maker_address = Some(maker_address.into());
        self
    }

    pub fn market(mut self, market: impl Into<String>) -> Self {
        self.market = Some(market.into());
        self
    }

    pub fn asset_id(mut self, asset_id: impl Into<String>) -> Self {
        self.asset_id = Some(asset_id.into());
        self
    }

    pub fn before(mut self, before: u64) -> Self {
        self.before = Some(before);
        self
    }

    pub fn after(mut self, after: u64) -> Self {
        self.after = Some(after);
        self
    }

    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::with_capacity(6);

        if let Some(ref id) = self.id {
            params.push(("id", id.clone()));
        }

        if let Some(ref asset_id) = self.asset_id {
            params.push(("asset_id", asset_id.clone()));
        }

        if let Some(ref market) = self.market {
            params.push(("market", market.clone()));
        }

        if let Some(before) = self.before {
            params.push(("before", before.to_string()));
        }

        if let Some(after) = self.after {
            params.push(("after", after.to_string()));
        }

        if let Some(ref maker_address) = self.maker_address {
            params.push(("maker_address", maker_address.clone()));
        }

        params
    }
}
