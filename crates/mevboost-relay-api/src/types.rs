use chrono::prelude::*;
use serde::{Deserialize, Serialize};
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

/// Filter arguments for the getPayload bidtraces relay query
#[derive(Debug, Default)]
pub struct PayloadDeliveredQueryOptions {
    /// A specific slot number.
    pub slot: Option<u64>,
    /// A starting slot for multiple results.
    pub cursor: Option<u64>,
    /// The number of results.
    pub limit: Option<u64>,
    /// A block hash.
    pub block_hash: Option<String>,
    /// A specific block number.
    pub block_number: Option<u64>,
    /// A specific proposer public key.
    pub proposer_pubkey: Option<String>,
    /// A specific builder public key.
    pub builder_pubkey: Option<String>,
    /// Sort results in order of: `value` or `-value`.
    pub order_by: Option<String>,
}

impl ToString for PayloadDeliveredQueryOptions {
    fn to_string(&self) -> String {
        let mut query = String::new();
        query.push('?');

        if let Some(slot) = self.slot {
            query.push_str(&format!("slot={}&", slot));
        }
        if let Some(cursor) = self.cursor {
            query.push_str(&format!("cursor={}&", cursor));
        }
        if let Some(limit) = self.limit {
            query.push_str(&format!("limit={}&", limit));
        }
        if let Some(block_hash) = &self.block_hash {
            query.push_str(&format!("block_hash={}&", block_hash));
        }
        if let Some(block_number) = self.block_number {
            query.push_str(&format!("block_number={}&", block_number));
        }
        if let Some(proposer_pubkey) = &self.proposer_pubkey {
            query.push_str(&format!("proposer_pubkey={}&", proposer_pubkey));
        }
        if let Some(builder_pubkey) = &self.builder_pubkey {
            query.push_str(&format!("builder_pubkey={}&", builder_pubkey));
        }
        if let Some(order_by) = &self.order_by {
            query.push_str(&format!("order_by={}&", order_by));
        }

        query
    }
}

/// Entry for the validator payload delivered response.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[allow(missing_docs)]
pub struct PayloadBidtrace {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub slot: u64,
    pub parent_hash: String,
    pub block_hash: String,
    pub builder_pubkey: String,
    pub proposer_pubkey: String,
    pub proposer_fee_recipient: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub gas_limit: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub gas_used: u64,
    pub value: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub num_tx: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub block_number: u64,
}

/// Filter arguments for the get builder blocks bidtraces relay query
#[derive(Debug, Default)]
pub struct BuilderBidsReceivedOptions {
    /// A specific slot number.
    pub slot: Option<u64>,
    /// A block hash.
    pub block_hash: Option<String>,
    /// A specific block number.
    pub block_number: Option<u64>,
    /// A specific builder public key.
    pub builder_pubkey: Option<String>,
    /// The number of results.
    pub limit: Option<u64>,
}

impl ToString for BuilderBidsReceivedOptions {
    fn to_string(&self) -> String {
        let mut query = String::new();
        query.push('?');

        if let Some(slot) = self.slot {
            query.push_str(&format!("slot={}&", slot));
        }
        if let Some(block_hash) = &self.block_hash {
            query.push_str(&format!("block_hash={}&", block_hash));
        }
        if let Some(block_number) = self.block_number {
            query.push_str(&format!("block_number={}&", block_number));
        }
        if let Some(builder_pubkey) = &self.builder_pubkey {
            query.push_str(&format!("builder_pubkey={}&", builder_pubkey));
        }
        if let Some(limit) = self.limit {
            query.push_str(&format!("limit={}&", limit));
        }

        query
    }
}

/// Entry for the builder block bidtrace response.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[allow(missing_docs)]
pub struct BuilderBlockBidtrace {
    #[serde(flatten)]
    pub payload: PayloadBidtrace,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub timestamp_ms: u128,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimistic_submission: Option<bool>,
}
