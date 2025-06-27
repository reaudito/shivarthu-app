use crate::services::error::ErrorString;
use serde::{Deserialize, Serialize};
use serde_json::json;
use subxt::ext::codec::{Compact, Encode};
use subxt::utils::Era;
use subxt::{self, OnlineClient, PolkadotConfig};
use wasm_bindgen_futures::JsFuture;

#[subxt::subxt(
    runtime_metadata_path = "./artifacts/metadata.scale",
    derive_for_all_types = "Clone, Debug, Eq, PartialEq"
)]

pub mod polkadot {}

pub(crate) async fn fetch_constant_block_length() -> Result<String, subxt::Error> {
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let constant_query = polkadot::constants().system().block_length();

    let value = api.constants().at(&constant_query)?;
    Ok(format!("{value:?}"))
}

pub(crate) async fn fetch_events_dynamically() -> Result<Vec<String>, subxt::Error> {
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let events = api.events().at_latest().await?;
    let mut event_strings = Vec::<String>::new();
    for event in events.iter() {
        let event = event?;
        let pallet = event.pallet_name();
        let variant = event.variant_name();
        let field_values = event.field_values()?;
        event_strings.push(format!("{pallet}::{variant}: {field_values}"));
    }
    Ok(event_strings)
}
