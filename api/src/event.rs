use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampMilliSeconds};
use strum::{Display, EnumString};

#[derive(Debug, Display, EnumString, Serialize, Deserialize, Eq, PartialEq)]
#[strum(serialize_all = "snake_case")]
enum EventType {
    Tap,
    Upgrade,
    #[strum(disabled)]
    Unknown,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    id: u64,
    event: EventType,
    message: String,
    #[serde_as(as = "TimestampMilliSeconds")]
    timestamp: DateTime<Utc>,
}

impl Default for Event {
    fn default() -> Self {
        Self {
            id: 0,
            event: EventType::Unknown,
            message: String::new(),
            timestamp: DateTime::<Utc>::default(),
        }
    }
}
