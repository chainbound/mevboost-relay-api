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
    relays: HashMap<&'a str, &'a str>,
    /// HTTP client used for requests.
    client: reqwest::Client,
}

impl<'a> Default for Client<'a> {
    fn default() -> Self {
        Self {
            relays: constants::DEFAULT_RELAYS.clone(),
            client: reqwest::Client::new(),
        }
    }
}

impl<'a> Client<'a> {
    /// Create a new MevBoost Relay API client with a custom list of relays.
    ///
    /// Relays are a mapping of relay names to their endpoints.
    /// See [`constants::DEFAULT_RELAYS`] for an example.
    pub fn with_relays(relays: HashMap<&'a str, &'a str>) -> Self {
        let client = reqwest::Client::new();
        Self { relays, client }
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

    /// Helper function to perform an HTTP get request with standard headers.
    async fn fetch(&self, endpoint: String) -> anyhow::Result<String> {
        let response = self
            .client
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
}
