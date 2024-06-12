use chrono::{DateTime, Utc};
use derive_more::Into;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Display, EnumString, Serialize, Deserialize, Eq, PartialEq)]
enum EventType {
    Tap,
    Upgrade,
    #[strum(disabled)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Into)]
pub struct Event {
    id: u128,
    event: EventType,
    message: String,
    timestamp: DateTime<Utc>,
}

impl Default for Event {
    fn default() -> Self {
        Self {
            id: 0,
            event: EventType::Unknown,
            message: String::new(),
            timestamp: DateTime::default(),
        }
    }
}
