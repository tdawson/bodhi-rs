use std::convert::TryFrom;

use chrono::{DateTime, TimeZone, Utc};

/// human-readable date format internally used by bodhi
pub const BODHI_DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

/// This struct wraps a `chrono::DateTime<chrono::Utc>` instance with implementations for converting
/// to and from the string format that bodhi expects and returns for dates and times.
#[derive(Debug)]
pub struct BodhiDate {
    date: DateTime<Utc>,
}

impl TryFrom<&str> for BodhiDate {
    type Error = chrono::ParseError;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        Ok(BodhiDate {
            date: Utc.datetime_from_str(string, BODHI_DATE_FORMAT)?,
        })
    }
}

impl Into<String> for BodhiDate {
    fn into(self) -> String {
        self.date.format(BODHI_DATE_FORMAT).to_string()
    }
}

// https://serde.rs/custom-date-format.html
#[allow(dead_code)]
pub(crate) mod bodhi_date_format {
    use super::BodhiDate;

    use chrono::{TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &BodhiDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string = date.date.format(super::BODHI_DATE_FORMAT).to_string();
        serializer.serialize_str(&string)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<BodhiDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;

        match Utc.datetime_from_str(&string, super::BODHI_DATE_FORMAT) {
            Ok(result) => Ok(BodhiDate { date: result }),
            Err(error) => Err(error).map_err(serde::de::Error::custom),
        }
    }
}

// https://github.com/serde-rs/serde/issues/1444#issuecomment-447546415
#[allow(dead_code)]
pub(crate) mod option_bodhi_date_format {
    use super::BodhiDate;

    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &Option<BodhiDate>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(ref dt) => super::bodhi_date_format::serialize(dt, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<BodhiDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wrapper(#[serde(with = "super::bodhi_date_format")] BodhiDate);

        let v: Option<Wrapper> = Deserialize::deserialize(deserializer)?;
        Ok(v.map(|Wrapper(a)| a))
    }
}
