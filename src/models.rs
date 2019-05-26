pub mod account;
pub mod authentication;
pub mod internal;
pub mod market_data;
pub mod session_management;
pub mod subscription;
pub mod support;
pub mod trading;

use crate::errors::DeribitError;
use failure::{Error, Fallible};
use serde_derive::{Deserialize, Serialize};
use serde_json::from_str;
use std::fmt::{Display, Error as FmtError, Formatter};
use std::result::Result as StdResult;

pub use account::{
    GetAccountSummaryRequest, GetAccountSummaryResponse, GetPositionsRequest, GetPositionsResponse,
    GetSubaccountsRequest, GetSubaccountsResponse,
};
pub use authentication::{AuthRequest, AuthResponse, GrantType};
pub use internal::{
    HeartbeatType, JSONRPCRequest, JSONRPCResponse, JSONRPCVersion, SubscriptionData,
    SubscriptionMessage, SubscriptionParams,
};
pub use market_data::{
    GetBookSummaryByCurrencyRequest, GetBookSummaryByCurrencyResponse, GetIndexRequest,
    GetIndexResponse, GetInstrumentsRequest, GetInstrumentsResponse,
};
pub use session_management::{SetHeartbeatRequest, SetHeartbeatResponse};
pub use subscription::{
    PrivateSubscribeRequest, PrivateUnsubscribeRequest, PublicSubscribeRequest,
    PublicUnsubscribeRequest, SubscribeResponse,
};
pub use support::{
    GetTimeRequest, GetTimeResponse, HelloRequest, HelloResponse, TestRequest, TestResponse,
};
pub use trading::{
    BuyRequest, BuyResponse, CancelAllByCurrencyRequest, CancelAllByInstrumentRequest,
    CancelAllRequest, CancelAllResponse, CancelOrderType, CancelRequest, CancelResponse,
    EditRequest, EditResponse, GetOpenOrderType, GetOpenOrdersByCurrencyRequest,
    GetOpenOrdersByCurrencyResponse, GetOpenOrdersByInstrumentRequest,
    GetOpenOrdersByInstrumentResponse, GetOrderStateRequest, GetOrderStateResponse, Order,
    SellRequest, SellResponse, Trade, TradeRequest, TradeResponse,
};

pub trait Request {
    const METHOD: &'static str;
    type Response;
}

trait VoidRequest {
    fn empty(&self) -> bool;
}

impl<R: Request> VoidRequest for R {
    #[inline]
    default fn empty(&self) -> bool {
        false
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Currency {
    #[serde(alias = "btc")]
    BTC,
    #[serde(alias = "eth")]
    ETH,
    #[serde(alias = "usd")]
    USD,
}

impl Default for Currency {
    fn default() -> Currency {
        Currency::BTC
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl std::str::FromStr for Currency {
    type Err = Error;
    fn from_str(s: &str) -> Fallible<Currency> {
        from_str(&format!(r#""{}""#, s))
            .map_err(|_| DeribitError::UnknownCurrency(s.to_string()).into())
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AssetKind {
    #[serde(alias = "future")]
    Future,
    #[serde(alias = "option")]
    Option,
}


impl std::str::FromStr for AssetKind {
    type Err = Error;
    fn from_str(s: &str) -> Fallible<AssetKind> {
        from_str(&format!(r#""{}""#, s))
            .map_err(|_| DeribitError::UnknownAssetKind(s.to_string()).into())
    }
}


#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Buy,
    Sell,
    Zero, // Admin says it is a leftover from bug hunting.
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> StdResult<(), FmtError> {
        write!(f, "{:?}", self)
    }
}

impl Direction {
    pub fn sign(self) -> i64 {
        match self {
            Direction::Buy => 1,
            Direction::Sell => -1,
            Direction::Zero => 0,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
pub enum Liquidity {
    #[serde(rename = "M")]
    Maker,
    #[serde(rename = "T")]
    Taker,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    Limit,
    Market,
    StopLimit,
    StopMarket,
    Liquidation,
}

impl Default for OrderType {
    fn default() -> Self {
        OrderType::Limit
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderState {
    Open,
    Closed,
    Filled,
    Rejected,
    Cancelled,
    Untriggered,
    Archive,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TimeInForce {
    GoodTilCancelled,
    FillOrKill,
    ImmediateOrCancel,
}

impl Default for TimeInForce {
    fn default() -> Self {
        TimeInForce::GoodTilCancelled
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Trigger {
    IndexPrice,
    MarkPrice,
    LastPrice,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
pub enum AdvanceOption {
    #[serde(rename = "usd")]
    USD,
    #[serde(rename = "implv")]
    ImplV,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub fn map_left<F, U>(self, f: F) -> Either<U, R>
    where
        F: FnOnce(L) -> U,
    {
        match self {
            Either::Left(l) => Either::Left(f(l)),
            Either::Right(r) => Either::Right(r),
        }
    }

    pub fn map_right<F, U>(self, f: F) -> Either<L, U>
    where
        F: FnOnce(R) -> U,
    {
        match self {
            Either::Right(r) => Either::Right(f(r)),
            Either::Left(l) => Either::Left(l),
        }
    }
    pub fn left_result(self) -> Result<L, R> {
        match self {
            Either::Left(l) => Ok(l),
            Either::Right(r) => Err(r),
        }
    }
    pub fn right_result(self) -> Result<R, L> {
        match self {
            Either::Left(l) => Err(l),
            Either::Right(r) => Ok(r),
        }
    }
}

impl<T> Either<T, T> {
    pub fn unwrap(self) -> T {
        match self {
            Either::Left(l) => l,
            Either::Right(r) => r,
        }
    }
}

impl<L, R> Either<L, R> {
    pub fn unwrap_left(self) -> L {
        match self {
            Either::Left(l) => l,
            Either::Right(_) => panic!("Either is right"),
        }
    }

    pub fn left(self) -> Option<L> {
        match self {
            Either::Left(l) => Some(l),
            Either::Right(_) => None,
        }
    }

    pub fn unwrap_right(self) -> R {
        match self {
            Either::Left(_) => panic!("Either is left"),
            Either::Right(r) => r,
        }
    }

    pub fn right(self) -> Option<R> {
        match self {
            Either::Left(_) => None,
            Either::Right(r) => Some(r),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Any3<O1, O2, O3> {
    First(O1),
    Second(O2),
    Third(O3),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Any4<O1, O2, O3, O4> {
    First(O1),
    Second(O2),
    Third(O3),
    Fourth(O4),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Any5<O1, O2, O3, O4, O5> {
    First(O1),
    Second(O2),
    Third(O3),
    Fourth(O4),
    Fifth(O5),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Any6<O1, O2, O3, O4, O5, O6> {
    First(O1),
    Second(O2),
    Third(O3),
    Fourth(O4),
    Fifth(O5),
    Sixth(O6),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Any7<O1, O2, O3, O4, O5, O6, O7> {
    First(O1),
    Second(O2),
    Third(O3),
    Fourth(O4),
    Fifth(O5),
    Sixth(O6),
    Seventh(O7),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Any8<O1, O2, O3, O4, O5, O6, O7, O8> {
    First(O1),
    Second(O2),
    Third(O3),
    Fourth(O4),
    Fifth(O5),
    Sixth(O6),
    Seventh(O7),
    Eighth(O8),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Any9<O1, O2, O3, O4, O5, O6, O7, O8, O9> {
    First(O1),
    Second(O2),
    Third(O3),
    Fourth(O4),
    Fifth(O5),
    Sixth(O6),
    Seventh(O7),
    Eighth(O8),
    Ninth(O9),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Any10<O1, O2, O3, O4, O5, O6, O7, O8, O9, O10> {
    First(O1),
    Second(O2),
    Third(O3),
    Fourth(O4),
    Fifth(O5),
    Sixth(O6),
    Seventh(O7),
    Eighth(O8),
    Ninth(O9),
    Tenth(O10),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Any11<O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11> {
    First(O1),
    Second(O2),
    Third(O3),
    Fourth(O4),
    Fifth(O5),
    Sixth(O6),
    Seventh(O7),
    Eighth(O8),
    Ninth(O9),
    Tenth(O10),
    Eleventh(O11),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Any12<O1, O2, O3, O4, O5, O6, O7, O8, O9, O10, O11, O12> {
    First(O1),
    Second(O2),
    Third(O3),
    Fourth(O4),
    Fifth(O5),
    Sixth(O6),
    Seventh(O7),
    Eighth(O8),
    Ninth(O9),
    Tenth(O10),
    Eleventh(O11),
    Twelfth(O12),
}