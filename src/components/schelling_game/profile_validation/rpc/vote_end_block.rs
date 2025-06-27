use crate::constants::constant::NODE_URL;
use icondata;
use jsonrpsee_core::{client::ClientT, rpc_params};
use jsonrpsee_wasm_client::WasmClientBuilder;
use leptos::prelude::*;
use leptos_icons::*;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;

async fn load_data(profile_user_account: String, set_end_period: WriteSignal<Option<u32>>) {
    let client = WasmClientBuilder::default().build(NODE_URL).await.unwrap();
    let result: Option<u32> = client
        .request(
            "profilevalidation_voteendblock",
            rpc_params![profile_user_account],
        )
        .await
        .unwrap();
    set_end_period(result);
}

#[component]
pub fn VoteEndBlock(profile_user_account: String) -> impl IntoView {
    let (end_period, set_end_period) = signal::<Option<u32>>(None);

    let action: Action<(String, WriteSignal<Option<u32>>), (), LocalStorage> = Action::new_unsync(
        |(profile_user_account, set_end_period): &(String, WriteSignal<Option<u32>>)| {
            let profile_user_account = profile_user_account.clone();
            let set_end_period = set_end_period.clone();
            async move { load_data(profile_user_account, set_end_period).await }
        },
    );

    let Pausable { .. } = use_interval_fn(
        move || {
            action.dispatch((profile_user_account.clone(), set_end_period));
        },
        5000,
    );

    view! {
        <div>
            {move || {
                if end_period().is_some() {
                    view! {
                        <div>
                            {"Vote Period ends: "}
                            <span id="end-period-time">{move || end_period()}</span>
                        </div>
                    }
                        .into_any()
                } else {
                    view! {
                        <div>
                            {"Vote Period ends: "} <span id="end-period-time">
                                <Icon icon=icondata::ImSpinner6 style="color: green" />
                            </span>
                        </div>
                    }
                        .into_any()
                }
            }}

        </div>
    }
}
