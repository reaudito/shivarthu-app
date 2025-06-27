use crate::components::transaction::extension_sign_in::sign_in_with_extension;
use crate::components::transaction::get_accounts_extension::GetAccountsExtension;
use crate::services::common_services::polkadot;
use leptos::prelude::*;
use polkadot::runtime_types::pallet_department_funding::types::TippingName;
use polkadot::runtime_types::pallet_support::Content;

fn match_tipping_name(name: &str) -> Option<TippingName> {
    match name {
        "SmallTipper" => Some(TippingName::SmallTipper),
        "BigTipper" => Some(TippingName::BigTipper),
        "SmallSpender" => Some(TippingName::SmallSpender),
        "MediumSpender" => Some(TippingName::MediumSpender),
        "BigSpender" => Some(TippingName::BigSpender),
        _ => None,
    }
}
#[component]
pub fn SignTransaction(
    post_cid: String,
    department_id: u64,
    tip_name: String,
    funding_needed: u128,
) -> impl IntoView {
    view! {
        <ExtensionSignIn
            post_cid=post_cid
            department_id=department_id
            tip_name=tip_name
            funding_needed=funding_needed
        />
    }
}

#[component]
pub fn ExtensionSignIn(
    post_cid: String,
    department_id: u64,
    tip_name: String,
    funding_needed: u128,
) -> impl IntoView {
    let (account_load, set_account_load) = signal(("".to_owned(), "".to_owned()));

    let render_html = move || {
        if account_load().0.is_empty() || account_load().1.is_empty() {
            view! {
                <div>
                    <GetAccountsExtension set_account_load=set_account_load />
                </div>
            }
            .into_any()
        } else if !account_load().0.is_empty() && !account_load().1.is_empty() {
            view! {
                <div>
                    <ExtensionTransaction
                        post_cid=post_cid.clone()
                        department_id=department_id.clone()
                        tip_name=tip_name.clone()
                        funding_needed=funding_needed.clone()
                        account_address=account_load().0
                        account_source=account_load().1
                    />
                </div>
            }
            .into_any()
        } else {
            view! { <div>{"Some Error Occured"}</div> }.into_any()
        }
    };
    view! { <div>{move || render_html()}</div> }
}

async fn transaction(
    post_cid: String,
    department_id: u64,
    tip_name: String,
    funding_needed: u128,
    account_address: String,
    account_source: String,
    set_error: WriteSignal<String>,
    set_extrinsic_success: WriteSignal<String>,
) {
    let content: Content = Content::IPFS(post_cid.as_bytes().to_vec());

    let tipping_name = match_tipping_name(&tip_name);

    let tx = polkadot::tx()
        .department_funding()
        .create_department_required_fund(
            department_id,
            content,
            tipping_name.unwrap(),
            funding_needed,
        );
    sign_in_with_extension(
        tx,
        account_address,
        account_source,
        set_error,
        set_extrinsic_success,
    )
    .await;
}
#[component]
pub fn ExtensionTransaction(
    post_cid: String,
    department_id: u64,
    tip_name: String,
    funding_needed: u128,
    account_address: String,
    account_source: String,
) -> impl IntoView {
    let (error, set_error) = signal(String::from(""));
    let (extrinsic_success, set_extrinsic_success) = signal(String::from(""));
    let transaction_resource = LocalResource::new(move || {
        transaction(
            post_cid.clone(),
            department_id.clone(),
            tip_name.clone(),
            funding_needed.clone(),
            account_address.clone(),
            account_source.clone(),
            set_error,
            set_extrinsic_success,
        )
    });

    let async_result = move || {
        transaction_resource
            .get()
            .as_deref()
            .map(|_| view! { <div></div> }.into_any())
            // This loading state will only show before the first load
            .unwrap_or_else(|| {
                view! {
                    <div class="alert">
                        <span class="loading loading-spinner"></span>
                        "Loading... Please sign with extension."
                    </div>
                }
                .into_any()
            })
    };

    let error_fn = move || {
        if !error().is_empty() {
            view! {
                <div role="alert" class="alert alert-error">
                    {move || error()}
                </div>
            }
            .into_any()
        } else {
            view! { <div></div> }.into_any()
        }
    };

    let extrinsic_success_fn = move || {
        if !extrinsic_success().is_empty() {
            view! {
                <div role="alert" class="alert alert-success">
                    {move || extrinsic_success()}
                </div>
            }
            .into_any()
        } else {
            view! { <div></div> }.into_any()
        }
    };

    view! {
        <div class="md:container md:mx-auto">
            <div>{async_result}</div>
            <br />
            <br />
            <div>{move || error_fn()}</div>
            <br />
            <div>{move || extrinsic_success_fn()}</div>

        </div>
    }
}
