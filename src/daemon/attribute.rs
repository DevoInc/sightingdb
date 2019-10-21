use chrono::{DateTime, Utc};
use std::fmt;
use serde::Serialize;
use chrono::serde::ts_seconds;

#[derive(Serialize)]
pub struct Attribute {
    pub value: String,
    #[serde(with = "ts_seconds")]
    pub first_seen: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub last_seen: DateTime<Utc>,
    pub source: String,
    #[serde(with = "ts_seconds")]
    pub source_timestamp: DateTime<Utc>,
    pub count: u128,
    pub tags: String,
    pub ttl: u128,
}

impl Attribute {
    pub fn new(value: &str) -> Attribute {
        Attribute {
            value: String::from(value), // FIXME: change to Vec<u8>
            first_seen: Utc::now(),
            last_seen: Utc::now(),
            source_timestamp: Utc::now(),
            source: String::from("unknown"),
            count: 0,
            tags: String::from(""),
            ttl: 0,
        }
    }
    pub fn count(&mut self) -> u128 {
        return self.count;
    }
    pub fn incr(&mut self) {
        self.last_seen = Utc::now();
        self.count += 1;
    }
    pub fn set_source(&mut self, src: String) {
        self.source = src;
        self.source_timestamp = Utc::now();
    }
}

impl fmt::Debug for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attribute {{ value: {}, first_seen: {:?}, last_seen: {:?}, count: {}, source: {}, source_timestamp: {:?}, tags: {:?}, ttl: {:?}}}",
               self.value, self.first_seen, self.last_seen, self.count, self.source, self.source_timestamp, self.tags, self.ttl)
    }
}
