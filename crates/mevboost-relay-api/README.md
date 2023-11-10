# mevboost-relay-api lib

### List of available relays

You can find the updated list defined as a constant [here](../mevboost-relay-api/src/constants.rs).

### Available API methods on the `Client`

#### `get_validators_for_current_and_next_epoch`

[Link to the specs](https://flashbots.github.io/relay-specs/#/Builder/getValidators).

Returns all the validators for the current and next epoch for a given relay name.

#### `get_validator_registration`

[Link to the specs](https://flashbots.github.io/relay-specs/#/Data/getValidatorRegistration).

Returns the validator registration info by the given pubkey and relay name.
Will return an error if the validator is not registered with that relay.

#### `get_validator_registration_on_all_relays`

Convenience method that returns a hashmap of relay name to validator registration info, for the given pubkey.
The returned data means that the validator is registered with the relays in the hashmap and not registered with the rest.

This is a good way to check which relays a validator is registered with.

#### `get_validator_registration_for_all_slots_on_all_relays`

Convenience method that returns a hashmap of slot number to list of relay names that the associated validator
is registered with for that slot.

This is a good way to check, for the entirety of the next epoch, which relays will be likely to
broadcast the block. If no relays are returned for a slot, it means that that slot is likely to be
built without MevBoost (aka, a "Vanilla block").
