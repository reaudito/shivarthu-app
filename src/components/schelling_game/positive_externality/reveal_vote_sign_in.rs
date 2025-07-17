use crate::components::signing::sign_transaction_fn::SignTransactionFn;
use crate::services::common_services::polkadot;
use leptos::prelude::*;
use std::str::FromStr;
use subxt::utils::AccountId32;

#[component]
pub fn SignTransaction(salt: String, choice: i64, user_to_calculate: String) -> impl IntoView {
    view! { <ExtensionSignIn salt=salt choice=choice user_to_calculate=user_to_calculate /> }
}

#[component]
pub fn ExtensionSignIn(salt: String, choice: i64, user_to_calculate: String) -> impl IntoView {
    let account_id32 = AccountId32::from_str(&user_to_calculate.clone()).unwrap();
    let salt_vec = salt.as_bytes().to_vec();

    let tx = Box::new(polkadot::tx().positive_externality().reveal_vote(
        account_id32,
        choice,
        salt_vec,
    ));

    view! { <SignTransactionFn tx=tx /> }
}
