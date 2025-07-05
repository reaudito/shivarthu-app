use crate::components::signing::sign_transaction_fn::SignTransactionFn;
use crate::services::common_services::polkadot;
use leptos::prelude::*;
use std::str::FromStr;
use subxt::utils::AccountId32;

#[component]
pub fn SignTransaction(transfer_address: String, transfer_balance: u128) -> impl IntoView {
    view! { <ExtensionSignIn transfer_address=transfer_address transfer_balance=transfer_balance /> }
}

#[component]
pub fn ExtensionSignIn(transfer_address: String, transfer_balance: u128) -> impl IntoView {
    let account_id32 = AccountId32::from_str(&transfer_address).unwrap();

    let tx = Box::new(polkadot::tx().balances().transfer_allow_death(
        subxt::utils::MultiAddress::Id(account_id32),
        transfer_balance,
    ));

    view! { <SignTransactionFn tx=tx /> }
}
