use crate::components::signing::sign_transaction_fn::SignTransactionFn;
use crate::services::common_services::polkadot;
use leptos::prelude::*;
use std::str::FromStr;
use subxt::utils::AccountId32;

#[component]
pub fn SignTransaction(hash: [u8; 32], user_to_calculate: String) -> impl IntoView {
    view! { <ExtensionSignIn hash=hash user_to_calculate=user_to_calculate /> }
}

#[component]
pub fn ExtensionSignIn(hash: [u8; 32], user_to_calculate: String) -> impl IntoView {
    let account_id32 = AccountId32::from_str(&user_to_calculate.clone()).unwrap();

    let tx = Box::new(
        polkadot::tx()
            .positive_externality()
            .commit_vote(account_id32, hash),
    );
    view! { <SignTransactionFn tx=tx /> }
}
