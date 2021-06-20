use serde::{Deserialize, Deserializer};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PhishingDB {
    #[serde(deserialize_with = "string_to_usize")]
    pub phish_id: usize,
    pub url: String,
    pub phish_detail_url: String,
    pub submission_time: String,
    #[serde(deserialize_with = "string_to_bool")]
    pub verified: bool,
    pub verification_time: String,
    #[serde(deserialize_with = "string_to_bool")]
    pub online: bool,
    pub target: String,
    pub details: Option<Vec<Details>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Details {
    pub ip_address: String,
    pub cidr_block: String,
    pub announcing_network: String,
    pub rir: String,
    pub country: String,
    pub detail_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CheckUrl {
    pub url: String,
    pub in_database: String,
    #[serde(deserialize_with = "string_to_usize")]
    pub phish_id: usize,
    pub phish_detail_page: String,
    #[serde(deserialize_with = "string_to_bool")]
    pub verified: bool,
    pub verified_at: String,
    #[serde(deserialize_with = "string_to_bool")]
    pub valid: bool,
}

fn string_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let s = s.replace('"', "").to_lowercase();
    match s.as_str() {
        "y" | "yes" | "1" => Ok(true),
        _ => Ok(false),
    }
}

fn string_to_usize<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse().map_err(serde::de::Error::custom)?,
        Value::Number(num) => {
            num.as_i64()
                .ok_or_else(|| serde::de::Error::custom("Invalid number"))? as usize
        }
        _ => return Err(serde::de::Error::custom("wrong type")),
    })
}
