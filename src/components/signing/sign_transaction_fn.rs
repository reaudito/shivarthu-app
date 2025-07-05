use crate::components::common::global_state::{GlobalState, GlobalStateStoreFields};
use crate::components::common::spinner::LoadingSpinner;
use crate::components::navigation::nav::Nav;
use crate::components::signing::set_phrase_from_pass::SetPhraseFromPass;
use crate::constants::constant::NODE_URL;
use crate::services::common_services::polkadot;
use leptos::prelude::*;
use leptos::task::spawn_local;
use reactive_stores::Store;
use std::rc::Rc;
use subxt::tx::TxStatus;
use subxt::{OnlineClient, PolkadotConfig};
use subxt_core::utils::AccountId32;
use subxt_signer::{bip39::Mnemonic, sr25519::Keypair};

#[derive(Clone, Copy, PartialEq)]
enum TransactionState {
    Idle,
    SignIn,
    Transaction,
}

#[component]
pub fn SignTransactionFn(tx: Box<dyn subxt::tx::Payload>) -> impl IntoView {
    let (transaction_state, set_transaction_state) = signal(TransactionState::Idle);
    let state = expect_context::<Store<GlobalState>>();
    let (run_bool, set_run_bool) = signal(true);

    let (transaction_status, set_transaction_status) = signal(String::from(""));

    let (transaction_event, set_transaction_event) = signal(String::from(""));

    let account = state.account_state();
    let mnemonic_phrase = state.mnemonic_phrase();
    let phase_exists_in_state = state.phase_exists_in_state();
    let tx = Rc::new(tx);

    Effect::new(move |_| {
        if let Some(phrase) = mnemonic_phrase.get() {
            // Only run if run_bool is true
            if run_bool.get() {
                set_transaction_state.set(TransactionState::Transaction);
                set_run_bool.set(false);
                let tx_clone = Rc::clone(&tx);
                spawn_local({
                    let set_transaction_status = set_transaction_status.clone();
                    let set_transaction_event = set_transaction_event.clone();

                    async move {
                        let mnemonic = Mnemonic::parse(phrase).unwrap();
                        let keypair = Keypair::from_phrase(&mnemonic, None).unwrap();
                        let api = OnlineClient::<PolkadotConfig>::from_url(NODE_URL).await;

                        let api = match api {
                            Ok(r) => r,
                            Err(e) => {
                                set_transaction_status(format!(
                                    "Failed to submit transaction: {e}"
                                ));
                                return;
                            }
                        };

                        let mut result = api
                            .tx()
                            .sign_and_submit_then_watch_default(&tx_clone, &keypair)
                            .await
                            .unwrap();

                        // let mut result = match result {
                        //     Ok(r) => r,
                        //     Err(e) => {
                        //         set_transaction_status(format!(
                        //             "Failed to submit transaction: {e}"
                        //         ));
                        //         return;
                        //     }
                        // };

                        while let Some(status) = result.next().await {
                            match status.unwrap() {
                                TxStatus::InFinalizedBlock(in_block) => {
                                    let message = format!(
                                        "Transaction {:?} is finalized in block {:?}",
                                        in_block.extrinsic_hash(),
                                        in_block.block_hash()
                                    );
                                    set_transaction_status(message);

                                    let events = in_block.wait_for_success().await;

                                    let events = match events {
                                        Ok(r) => r,
                                        Err(e) => {
                                            set_transaction_status(format!("{e}"));
                                            return;
                                        }
                                    };

                                    let events_str = format!("{:?}", &events);
                                    web_sys::console::log_1(&events_str.into());
                                    for event in
                                        events.find::<polkadot::system::events::ExtrinsicSuccess>()
                                    {
                                        web_sys::console::log_1(&format!("{:?}", event).into());
                                    }

                                    let success = events
                                        .find_first::<polkadot::system::events::ExtrinsicSuccess>()
                                        .unwrap();
                                    match success {
                                        Some(remark_event) => {
                                            set_transaction_event(format!("{:?}", remark_event));
                                        }
                                        None => {
                                            set_transaction_event("Transaction failed".to_string());
                                        }
                                    }
                                }
                                other => {
                                    let message = format!("Status: {other:?}");
                                    web_sys::console::log_1(&format!("{:?}", message).into());
                                }
                            }
                        }
                    }
                });
            }
        } else {
            set_transaction_state.set(TransactionState::SignIn);
        }
    });

    view! {
        <>
            <main class="p-6 max-w-4xl mx-auto bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 min-h-screen transition-colors duration-300">
                <h1 class="text-2xl font-bold mb-4 text-center">Sign Transaction</h1>

                {move || match transaction_state.get() {
                    TransactionState::Idle => view! { <></> }.into_view().into_any(),
                    TransactionState::SignIn => {

                        view! {
                            <>
                                <SetPhraseFromPass />
                            </>
                        }
                            .into_view()
                            .into_any()
                    }
                    TransactionState::Transaction => {

                        view! {
                            <>
                                {move || {
                                    if !transaction_status().is_empty() {
                                        view! {
                                            <div
                                                role="alert"
                                                class="flex  p-4 items-center gap-3  mb-4 border-l-4 border-green-500 bg-green-100 text-green-800 rounded-xl shadow-md dark:bg-green-900 dark:text-green-200"
                                            >
                                                {transaction_status()}
                                            </div>
                                        }
                                            .into_view()
                                            .into_any()
                                    } else {
                                        view! {
                                            <>
                                                <LoadingSpinner />
                                            </>
                                        }
                                            .into_view()
                                            .into_any()
                                    }
                                }}
                                {move || {
                                    if !transaction_event().is_empty() {
                                        view! {
                                            <div
                                                role="alert"
                                                class="flex  p-4 items-center gap-3 border-l-4 border-yellow-500 bg-yellow-100 text-yellow-800 rounded-xl shadow-md dark:bg-yellow-900 dark:text-yellow-200"
                                            >
                                                {transaction_event()}
                                            </div>
                                        }
                                            .into_view()
                                            .into_any()
                                    } else {
                                        view! { <></> }.into_view().into_any()
                                    }
                                }}
                            </>
                        }
                            .into_view()
                            .into_any()
                    }
                }}

            </main>
        </>
    }
}
