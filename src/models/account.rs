use crate::models::{AssetKind, Currency, Direction};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

crate::define_request! {
    Name => GetPositions;
    Method => "private/get_positions";
    Request => {
        pub currency: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub kind: Option<AssetKind>,
    };
    Response => Vec<GetPositionsResponse>;
}

impl GetPositionsRequest {
    pub fn all(currency: String) -> Self {
        Self {
            currency,
            kind: None,
        }
    }

    pub fn futures(currency: String) -> Self {
        Self {
            currency,
            kind: Some(AssetKind::Future),
        }
    }

    pub fn options(currency: String) -> Self {
        Self {
            currency,
            kind: Some(AssetKind::Option),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum GetPositionsResponse {
    Future {
        average_price: f64,
        delta: f64,
        direction: Direction,
        estimated_liquidation_price: Option<f64>,
        floating_profit_loss: f64,
        index_price: f64,
        initial_margin: f64,
        instrument_name: String,
        interest_value: Option<f64>,
        leverage: i64,
        maintenance_margin: f64,
        mark_price: f64,
        open_orders_margin: f64,
        realized_funding: Option<f64>,
        realized_profit_loss: f64,
        settlement_price: f64,
        size: f64,
        size_currency: f64,
        total_profit_loss: f64,
    },
    Option {
        average_price: f64,
        average_price_usd: f64,
        delta: f64,
        direction: Direction,
        floating_profit_loss: f64,
        floating_profit_loss_usd: f64,
        gamma: f64,
        index_price: f64,
        initial_margin: f64,
        instrument_name: String,
        maintenance_margin: f64,
        mark_price: f64,
        realized_profit_loss: f64,
        settlement_price: f64,
        size: f64,
        theta: f64,
        total_profit_loss: f64,
        vega: f64,
    },
}

crate::define_request! {
    Name => GetAccountSummary;
    Method => "private/get_account_summary";
    Request => {
        pub currency: Currency,
        pub extended: bool,
    };
    Response => {
        pub options_gamma: f64,
        pub projected_maintenance_margin: Option<f64>,
        pub system_name: Option<String>,
        pub margin_balance: f64,
        pub tfa_enabled: Option<bool>,
        pub username: Option<String>,
        pub equity: f64,
        pub futures_pl: f64,
        pub options_session_upl: f64,
        pub id: Option<u64>,
        pub options_vega: f64,
        pub session_funding: Option<f64>,
        pub currency: Currency,
        pub r#type: Option<String>,
        pub futures_session_rpl: f64,
        pub options_theta: f64,
        pub portfolio_margin_enabled: Option<bool>,
        pub session_rpl: f64,
        pub delta_total: f64,
        pub options_pl: f64,
        pub available_withdrawal_funds: f64,
        pub maintenance_margin: f64,
        pub initial_margin: f64,
        pub futures_session_upl: f64,
        pub options_session_rpl: f64,
        pub available_funds: f64,
        pub email: Option<String>,
        pub session_upl: f64,
        pub total_pl: f64,
        pub options_delta: f64,
        pub balance: f64,
        pub projected_initial_margin: Option<f64>,
        pub deposit_address: Option<String>,
        pub referrer_id: Option<String>,
    };
}

impl GetAccountSummaryRequest {
    pub fn abridged(currency: Currency) -> Self {
        Self {
            currency,
            ..Default::default()
        }
    }
    pub fn extended(currency: Currency) -> Self {
        Self {
            currency,
            extended: true,
        }
    }
}

crate::define_request! {
    Name => GetSubaccounts;
    Method => "private/get_subaccounts";
    Request => {
        pub with_portfolio: bool,
    };
    Response => Vec<GetSubaccountsResponse>;
}

impl GetSubaccountsRequest {
    pub fn new() -> Self {
        Self {
            with_portfolio: false,
        }
    }

    pub fn with_portfolio() -> Self {
        Self {
            with_portfolio: true,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Portfolio {
    pub available_funds: f64,
    pub available_withdrawal_funds: f64,
    pub balance: f64,
    pub currency: Currency,
    pub equity: f64,
    pub initial_margin: f64,
    pub maintenance_margin: f64,
    pub margin_balance: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetSubaccountsResponse {
    pub email: String,
    pub id: u64,
    pub is_password: bool,
    pub login_enabled: bool,
    pub not_confirmed_email: Option<String>,
    pub portfolio: HashMap<Currency, Portfolio>,
    pub receive_notifications: bool,
    pub system_name: String,
    pub tfa_enabled: Option<bool>,
    pub r#type: String,
    pub username: String,
    pub referrals_count: u64,
    pub security_keys_enabled: bool,
}
