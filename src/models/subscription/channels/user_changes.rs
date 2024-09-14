use crate::models::subscription::{UserOrdersData, UserTradesData};

use fehler::throw;
use serde::{
    de::{Error, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserChangesData {
    pub trades: Vec<UserTradesData>,
    pub positions: Vec<crate::models::account::GetPositionsResponse>,
    pub orders: Vec<UserOrdersData>,
}

#[derive(Debug, Clone)]
pub enum UserChangesChannel {
    ByInstrument {
        instrument_name: String,
        interval: String,
    },
    ByKind {
        kind: String,
        currency: String,
        interval: String,
    },
}

impl<'de> Deserialize<'de> for UserChangesChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str as Deserialize<'de>>::deserialize(deserializer)?;
        let segments: Vec<_> = s.split(".").collect();
        match segments.as_slice() {
            ["user", "changes", instrument_name, interval] => Ok(UserChangesChannel::ByInstrument {
                instrument_name: instrument_name.to_string(),
                interval: interval.to_string(),
            }),
            ["user", "changes", kind, currency, interval] => Ok(UserChangesChannel::ByKind {
                kind: kind.to_string(),
                currency: currency.to_string(),
                interval: interval.to_string(),
            }),
            _ => throw!(D::Error::invalid_value(
                Unexpected::Str(s),
                &"user.changes.{instrument_name}.{interval} or user.changes.{kind}.{currency}.{interval}"
            )),
        }
    }
}
impl Serialize for UserChangesChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl std::fmt::Display for UserChangesChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserChangesChannel::ByInstrument {
                instrument_name,
                interval,
            } => write!(f, "user.changes.{}.{}", instrument_name, interval),
            UserChangesChannel::ByKind {
                kind,
                currency,
                interval,
            } => write!(f, "user.changes.{}.{}.{}", kind, currency, interval),
        }
    }
}
