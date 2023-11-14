use std::collections::HashMap;

lazy_static! {
    /// Default mevboost relays to use for queries.
    /// These values can be overridden with CLI arguments and in the library.
    pub static ref DEFAULT_RELAYS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("ultrasound", "https://0xa1559ace749633b997cb3fdacffb890aeebdb0f5a3b6aaa7eeeaf1a38af0a8fe88b9e4b1f61f236d2e64d95733327a62@relay.ultrasound.money");
        m.insert("flashbots", "https://0xac6e77dfe25ecd6110b8e780608cce0dab71fdd5ebea22a16c0205200f2f8e2e3ad3b71d3499c54ad14d6c21b41a37ae@boost-relay.flashbots.net");
        m.insert("aestus", "https://0xa15b52576bcbf1072f4a011c0f99f9fb6c66f3e1ff321f11f461d15e31b1cb359caa092c71bbded0bae5b5ea401aab7e@aestus.live");
        m.insert("agnostic", "https://0xa7ab7a996c8584251c8f925da3170bdfd6ebc75d50f5ddc4050a6fdc77f2a3b5fce2cc750d0865e05d7228af97d69561@agnostic-relay.net");
        m.insert("bloxroute-max-profit", "https://0x8b5d2e73e2a3a55c6c87b8b6eb92e0149a125c852751db1422fa951e42a09b82c142c3ea98d0d9930b056a3bc9896b8f@bloxroute.max-profit.blxrbdn.com");
        m.insert("bloxroute-regulated", "https://0xb0b07cd0abef743db4260b0ed50619cf6ad4d82064cb4fbec9d3ec530f7c5e6793d9f286c4e082c0244ffb9f2658fe88@bloxroute.regulated.blxrbdn.com");
        m
    };

    /// Relay endpoint for getting a list of validator registrations
    /// for validators scheduled to propose in the current and next epoch.
    ///
    /// [Visit the docs](https://flashbots.github.io/relay-specs/#/Builder/getValidators) for more info.
    pub static ref GET_VALIDATORS_ENDPOINT: &'static str = "/relay/v1/builder/validators";

    /// Relay endpoint for checking that a validator is registered with a relay.
    ///
    /// [Visit the docs](https://flashbots.github.io/relay-specs/#/Data/getValidatorRegistration) for more info.
    pub static ref CHECK_VALIDATOR_REGISTRATION: &'static str = "/relay/v1/data/validator_registration";

    /// Relay endpoint for getting the payloads that were delivered to proposers.
    ///
    /// [Visit the docs](https://flashbots.github.io/relay-specs/#/Data/getDeliveredPayloads) for more info.
    pub static ref GET_DELIVERED_PAYLOADS: &'static str = "/relay/v1/data/bidtraces/proposer_payload_delivered";
}
