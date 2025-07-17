use crate::components::signing::sign_transaction_fn::SignTransactionFn;
use crate::services::common_services::polkadot;
use leptos::prelude::*;

#[component]
pub fn SignTransaction() -> impl IntoView {
    view! { <ExtensionSignIn /> }
}

#[component]
pub fn ExtensionSignIn() -> impl IntoView {
    // let tx = polkadot::tx().positive_externality().get_incentives();
}
