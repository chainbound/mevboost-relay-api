use chrono::prelude::*;
use serde::Deserialize;
use serde_aux::prelude::*;

/// Validator info for a given slot.
#[derive(Deserialize, Debug)]
#[allow(missing_docs)]
pub struct RegisteredValidator {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub slot: u64,
    pub validator_index: Option<String>,
    pub entry: ValidatorEntry,
}

/// Validator entry for registered validators in a slot.
#[derive(Deserialize, Debug)]
#[allow(missing_docs)]
pub struct ValidatorEntry {
    pub message: EntryMessage,
    pub signature: String,
}

/// Entry message of registered validators in a slot.
#[derive(Deserialize, Debug)]
#[allow(missing_docs)]
pub struct EntryMessage {
    pub fee_recipient: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub gas_limit: u64,
    #[serde(deserialize_with = "deserialize_datetime_utc_from_seconds")]
    pub timestamp: DateTime<Utc>,
    pub pubkey: String,
}
