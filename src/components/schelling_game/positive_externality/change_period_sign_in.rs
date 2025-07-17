use crate::components::navigation::nav::Nav;
use crate::components::signing::sign_transaction_fn::SignTransactionFn;
use crate::services::common_services::polkadot;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use std::str::FromStr;
use subxt::utils::AccountId32;

#[component]
pub fn SignTransaction() -> impl IntoView {
    let params = use_params_map();

    let user_to_calculate =
        move || params.with(|params| params.get("user_to_calculate").unwrap_or_default());

    view! {
        <div>
            <Nav />
            <ExtensionSignIn user_to_calculate=user_to_calculate() />
        </div>
    }
}

#[component]
pub fn ExtensionSignIn(user_to_calculate: String) -> impl IntoView {
    let account_id32 = AccountId32::from_str(&user_to_calculate.clone()).unwrap();

    let tx = Box::new(
        polkadot::tx()
            .positive_externality()
            .pass_period(account_id32),
    );
    view! { <SignTransactionFn tx=tx /> }
}
