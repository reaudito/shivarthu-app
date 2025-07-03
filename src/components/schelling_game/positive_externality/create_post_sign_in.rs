use crate::components::common::spinner::LoadingSpinner;
use crate::components::signing::sign_transaction_fn::SignTransactionFn;
use crate::services::common_services::polkadot;
use leptos::prelude::*;
use polkadot::runtime_types::pallet_support::Content;

#[component]
pub fn SignTransaction(post_cid: String) -> impl IntoView {
    view! { <ExtensionSignIn post_cid=post_cid /> }
}

#[component]
pub fn ExtensionSignIn(post_cid: String) -> impl IntoView {
    let content: Content = Content::IPFS(post_cid.as_bytes().to_vec());

    let tx = Box::new(
        polkadot::tx()
            .positive_externality()
            .create_positive_externality_post(content),
    );

    view! {
        <SignTransactionFn tx=tx />
    }
}
