use crate::components::common::global_state::{GlobalState, GlobalStateStoreFields};
use crate::components::navigation::nav::Nav;
use crate::components::signing::set_phrase_from_pass::SetPhraseFromPass;
use crate::constants::constant::NODE_URL;
use leptos::prelude::*;
use leptos::task::spawn_local;
use reactive_stores::Store;
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

    spawn_local(async move {
        if !mnemonic_phrase.get().is_some() {
            set_transaction_state.set(TransactionState::SignIn);
        } else {
            set_transaction_state.set(TransactionState::Transaction);
            if run_bool.get() == true {
                set_run_bool.set(false);
                let mnemonic = Mnemonic::parse(mnemonic_phrase.get().unwrap()).unwrap();
                let keypair = Keypair::from_phrase(&mnemonic, None).unwrap();
                let api = OnlineClient::<PolkadotConfig>::from_url(NODE_URL)
                    .await
                    .unwrap();
                let mut result = api
                    .tx()
                    .sign_and_submit_then_watch_default(&tx, &keypair)
                    .await
                    .unwrap();
                while let Some(status) = result.next().await {
                    match status.unwrap() {
                        // It's finalized in a block!
                        TxStatus::InFinalizedBlock(in_block) => {
                            let message = format!(
                                "Transaction {:?} is finalized in block {:?}",
                                in_block.extrinsic_hash(),
                                in_block.block_hash()
                            );

                            set_transaction_status(message);

                            // grab the events and fail if no ExtrinsicSuccess event seen:
                            let events = in_block.wait_for_success().await.unwrap();
                            // We can look for events (this uses the static interface; we can also iterate
                            // over them and dynamically decode them):
                            let events = format!("Events: {:?}", events);
                            set_transaction_event(events);
                        }
                        // Just log any other status we encounter:
                        other => {
                            let message = format!("Status: {other:?}");
                            set_transaction_event(message);
                        }
                    }
                }
            }
        }
    });

    view! {
        <>
            <main class="p-6 max-w-2xl mx-auto bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 min-h-screen transition-colors duration-300">
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
                            <div
                                role="alert"
                                class="flex items-center gap-3 p-4 border-l-4 border-green-500 bg-green-100 text-green-800 rounded-xl shadow-md"
                            >
                                {move || transaction_status()}
                            </div>

                            <div
                                role="alert"
                                class="flex items-center gap-3 p-4 border-l-4 border-yellow-500 bg-yellow-100 text-yellow-800 rounded-xl shadow-md"
                            >
                                {move || transaction_event()}
                            </div>
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
