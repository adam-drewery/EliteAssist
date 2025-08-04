use chrono::{DateTime, TimeZone, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};
use crate::event::Materials;

pub mod date {
    use super::*;

    // Custom serializer for DateTime<Utc>
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.to_rfc3339();
        serializer.serialize_str(&s)
    }

    // Custom deserializer for DateTime<Utc>
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_rfc3339(&s)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(serde::de::Error::custom)
    }
}

pub mod optional_date {
    use super::*;

    // Custom serializer for Option<DateTime<Utc>>
    pub fn serialize<S>(date_opt: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date_opt {
            Some(date) => {
                let s = date.to_rfc3339();
                serializer.serialize_str(&s)
            }
            None => serializer.serialize_none(),
        }
    }

    // Custom deserializer for Option<DateTime<Utc>>
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        match s {
            Some(s) => DateTime::parse_from_rfc3339(&s)
                .map(|dt| Some(dt.with_timezone(&Utc)))
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}