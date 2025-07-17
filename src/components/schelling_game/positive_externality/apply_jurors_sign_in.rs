use crate::components::signing::sign_transaction_fn::SignTransactionFn;
use crate::services::common_services::polkadot;
use leptos::prelude::*;
use std::str::FromStr;
use subxt::utils::AccountId32;

#[component]
pub fn SignTransaction(stake: u128, user_to_calculate: String) -> impl IntoView {
    view! { <ExtensionSignIn stake=stake user_to_calculate=user_to_calculate /> }
}

#[component]
pub fn ExtensionSignIn(stake: u128, user_to_calculate: String) -> impl IntoView {
    let account_id32 = AccountId32::from_str(&user_to_calculate.clone()).unwrap();

    let tx = Box::new(
        polkadot::tx()
            .positive_externality()
            .apply_jurors(account_id32, stake),
    );

    view! {<SignTransactionFn tx=tx />}
}
