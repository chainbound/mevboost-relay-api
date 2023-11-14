#![doc = include_str!("../README.md")]
#![warn(
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    rustdoc::all
)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

/// Constants used in the library.
pub mod constants;

/// Types used in the library.
pub mod types;

/// Mevboost relay API client.
///
/// When created with [`Client::default()`], the client will use the default list of relays.
/// These can be overridden in the library by using [`Client::with_relays()`] instead.
#[derive(Debug)]
pub struct Client<'a> {
    /// List of relay names and endpoints to use for queries.
    pub relays: HashMap<&'a str, &'a str>,
    /// HTTP client used for requests.
    inner: reqwest::Client,
}

impl<'a> Default for Client<'a> {
    fn default() -> Self {
        Self {
            relays: constants::DEFAULT_RELAYS.clone(),
            inner: reqwest::Client::new(),
        }
    }
}

impl<'a> Client<'a> {
    /// Create a new MevBoost Relay API client with a custom list of relays.
    ///
    /// Relays are a mapping of relay names to their endpoints.
    /// See [`constants::DEFAULT_RELAYS`] for an example.
    pub fn with_relays(relays: HashMap<&'a str, &'a str>) -> Self {
        let inner = reqwest::Client::new();
        Self { relays, inner }
    }

    /// Perform a relay query for validator registrations for the current and next epochs.
    ///
    /// [Visit the docs](https://flashbots.github.io/relay-specs/#/Builder/getValidators) for more info.
    pub async fn get_validators_for_current_and_next_epoch(
        &self,
        relay_name: &str,
    ) -> anyhow::Result<Vec<types::RegisteredValidator>> {
        let relay_url = self.get_relay_url(relay_name)?;
        let endpoint = format!("{}{}", relay_url, *constants::GET_VALIDATORS_ENDPOINT);
        let response = self.fetch(endpoint).await?;

        serde_json::from_str::<Vec<types::RegisteredValidator>>(&response)
            .map_err(|e| anyhow::anyhow!("Failed to parse JSON response: {}", e))
    }

    /// Perform a relay query to check if a validator with the given pubkey
    /// is registered with the specified relay.
    ///
    /// [Visit the docs](https://flashbots.github.io/relay-specs/#/Data/getValidatorRegistration) for more info.
    pub async fn get_validator_registration(
        &self,
        relay_name: &str,
        pubkey: &str,
    ) -> anyhow::Result<types::ValidatorEntry> {
        let relay_url = self.get_relay_url(relay_name)?;
        let endpoint = format!(
            "{}{}?pubkey={}",
            relay_url,
            *constants::CHECK_VALIDATOR_REGISTRATION,
            pubkey
        );
        let response = self.fetch(endpoint).await?;

        serde_json::from_str::<types::ValidatorEntry>(&response)
            .map_err(|e| anyhow::anyhow!("Failed to parse JSON response: {}", e))
    }

    /// Perform a relay query to get the payloads delivered by the relay to the proposer.
    /// Query options act as filters.
    pub async fn get_payload_delivered_bidtraces(
        &self,
        relay_name: &str,
        opts: types::PayloadDeliveredQueryOptions,
    ) -> anyhow::Result<Vec<types::PayloadBidtrace>> {
        let relay_url = self.get_relay_url(relay_name)?;
        let endpoint = format!(
            "{}{}{}",
            relay_url,
            *constants::GET_DELIVERED_PAYLOADS,
            opts.to_string()
        );
        let response = self.fetch(endpoint).await?;

        serde_json::from_str::<Vec<types::PayloadBidtrace>>(&response)
            .map_err(|e| anyhow::anyhow!("Failed to parse JSON response: {}", e))
    }

    /// Perform a relay query to get the builder bid submissions.
    /// Query options act as filters.
    pub async fn get_builder_blocks_received(
        &self,
        relay_name: &str,
        opts: types::BuilderBidsReceivedOptions,
    ) -> anyhow::Result<Vec<types::BuilderBlockBidtrace>> {
        let relay_url = self.get_relay_url(relay_name)?;
        let endpoint = format!(
            "{}{}{}",
            relay_url,
            *constants::GET_BUILDER_BLOCKS_RECEIVED,
            opts.to_string()
        );
        let response = self.fetch(endpoint).await?;

        serde_json::from_str::<Vec<types::BuilderBlockBidtrace>>(&response)
            .map_err(|e| anyhow::anyhow!("Failed to parse JSON response: {}", e))
    }

    /// Perform a relay query to check if a validator with the given pubkey
    /// is registered with any of the relays in the client. Returns a hashmap
    /// of relay names to validator entries. If an entry is not found for a
    /// given relay, it will not be included in the hashmap.
    pub async fn get_validator_registration_on_all_relays(
        &self,
        pubkey: &str,
    ) -> anyhow::Result<HashMap<&'a str, types::ValidatorEntry>> {
        let mut validator_registrations = HashMap::new();
        for relay_name in self.relays.keys() {
            match self.get_validator_registration(relay_name, pubkey).await {
                Ok(relay_res) => {
                    validator_registrations.insert(*relay_name, relay_res);
                }
                Err(e) => {
                    tracing::warn!(
                        "Failed to get validator registration for pubkey {} on relay {}: {}",
                        pubkey,
                        relay_name,
                        e
                    );
                    continue;
                }
            }
        }

        Ok(validator_registrations)
    }

    /// Performs the following steps:
    /// 1. Get validator registrations for the current and next epochs for all relays
    /// 2. Build a map of slot number to relay names that have a validator registered for that slot
    pub async fn get_validator_registration_for_all_slots_on_all_relays(
        &self,
    ) -> anyhow::Result<HashMap<u64, Vec<&'a str>>> {
        let mut validator_registrations = HashMap::new();

        for relay_name in self.relays.keys() {
            let relay_res = self
                .get_validators_for_current_and_next_epoch(relay_name)
                .await?;

            for validator in relay_res {
                let relay_names = validator_registrations
                    .entry(validator.slot)
                    .or_insert_with(Vec::new);

                relay_names.push(*relay_name);
            }
        }

        // Fill all slots with no registrations with an empty vector.
        // The total number of slots for current + next epoch is 32 + 32 = 64.
        if let Some(initial_slot) = validator_registrations.keys().min() {
            for slot in *initial_slot..(*initial_slot + 63) {
                validator_registrations.entry(slot).or_insert_with(Vec::new);
            }
        }

        Ok(validator_registrations)
    }

    /// Returns a list of slot numbers for which no relays are registered for the current and next epochs.
    pub async fn get_vanilla_slots_for_current_and_next_epoch(&self) -> anyhow::Result<Vec<u64>> {
        let all = self
            .get_validator_registration_for_all_slots_on_all_relays()
            .await?;

        Ok(all
            .into_iter()
            .filter(|(_, v)| v.is_empty())
            .map(|(k, _)| k)
            .collect())
    }

    /// Helper function to perform an HTTP get request with standard headers.
    async fn fetch(&self, endpoint: String) -> anyhow::Result<String> {
        let response = self
            .inner
            .request(reqwest::Method::GET, endpoint)
            .header("content-type", "application/json")
            .header("accept", "application/json")
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
    }

    /// Helper function to get the URL for a given relay name.
    fn get_relay_url(&self, relay_name: &str) -> anyhow::Result<&str> {
        self.relays
            .get(relay_name)
            .map(|x| x.to_owned())
            .ok_or(anyhow::anyhow!(
                "Relay `{}` not found in list of relays",
                relay_name
            ))
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_get_validator_registrations_for_current_and_next_epoch() -> anyhow::Result<()> {
        let client = super::Client::default();
        let response = client
            .get_validators_for_current_and_next_epoch("flashbots")
            .await?;

        assert!(!response.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_validator_registration() -> anyhow::Result<()> {
        let client = super::Client::default();
        let pubkey = "0xacb2e8af472337d76290b8da9345d4edf6a5f7ce573a319340ce53112551f465878d996ad6745b80b64db1104e20c5d3";
        let response = client
            .get_validator_registration("flashbots", pubkey)
            .await?;

        assert_eq!(response.message.pubkey, pubkey);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_validator_registration_on_all_relays() -> anyhow::Result<()> {
        let client = super::Client::default();
        let pubkey = "0xacb2e8af472337d76290b8da9345d4edf6a5f7ce573a319340ce53112551f465878d996ad6745b80b64db1104e20c5d3";
        let response = client
            .get_validator_registration_on_all_relays(pubkey)
            .await?;

        assert!(!response.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_validator_registration_for_all_slots_on_all_relays() -> anyhow::Result<()> {
        let client = super::Client::default();
        let response = client
            .get_validator_registration_for_all_slots_on_all_relays()
            .await?;

        assert!(!response.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_vanilla_slots_for_current_and_next_epoch() -> anyhow::Result<()> {
        let client = super::Client::default();
        let _response = client
            .get_vanilla_slots_for_current_and_next_epoch()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_get_payload_delivered_bidtraces() -> anyhow::Result<()> {
        let client = super::Client::default();
        let opts = super::types::PayloadDeliveredQueryOptions {
            slot: Some(7761220),
            ..Default::default()
        };

        let response = client
            .get_payload_delivered_bidtraces("ultrasound", opts)
            .await?;

        assert!(!response.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_builder_blocks_received() -> anyhow::Result<()> {
        let client = super::Client::default();
        let opts = super::types::BuilderBidsReceivedOptions {
            slot: Some(7761220),
            ..Default::default()
        };

        let response = client
            .get_builder_blocks_received("ultrasound", opts)
            .await?;

        dbg!(&response);
        assert!(!response.is_empty());
        Ok(())
    }
}
