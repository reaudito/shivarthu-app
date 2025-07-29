use crate::components::common::global_state::{GlobalState, GlobalStateStoreFields};
use crate::constants::constant::NODE_URL;
use crate::services::common_services::polkadot;
use leptos::prelude::*;
use leptos::task::spawn_local;
use reactive_stores::Store;
use std::str::FromStr;
use subxt::utils::AccountId32;
use subxt::{OnlineClient, PolkadotConfig};

#[component]
pub fn Balance() -> impl IntoView {
    let (balance, set_balance) = signal(None);
    let state = expect_context::<Store<GlobalState>>();

    let account = state.account_address();

    Effect::new(move |_| {
        // account is not empyt string
        if account.get().is_empty() {
            return;
        }

        spawn_local(async move {
            let account_id32 = AccountId32::from_str(&account.get()).unwrap();

            let client = subxt::client::OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
                .await
                .unwrap();
            let balance_storage = polkadot::storage().system().account(account_id32);
            let balance_details = client
                .storage()
                .at_latest()
                .await
                .unwrap()
                .fetch(&balance_storage)
                .await
                .unwrap();

            if let Some(balance_details) = balance_details {
                // log!(format!("{:?}", balance_details.data.free));
                set_balance(Some(balance_details.data.free))
            }
        });
    });

    view! {
        <div>
            {move || {
                match balance.get() {
                    Some(free) => view!{<span>{format!("{} SHIV", free as f64 / 10f64.powi(10))}</span>}.into_any(), // adjust decimals based on your chain
                    None =>  view!{<span>{format!{"Balance not set"}}</span>}.into_any(),
                }
            }}
        </div>
    }
}
