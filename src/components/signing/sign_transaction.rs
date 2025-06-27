use crate::components::common::global_state::{GlobalState, GlobalStateStoreFields};
use crate::components::navigation::nav::Nav;
use crate::components::signing::set_phrase_from_pass::SetPhraseFromPass;
use leptos::prelude::*;
use leptos::task::spawn_local;
use reactive_stores::Store;
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
pub fn SignTransaction(tx: Box<dyn subxt::tx::Payload>) -> impl IntoView {
    let (transaction_state, set_transaction_state) = signal(TransactionState::Idle);
    let state = expect_context::<Store<GlobalState>>();

    let account = state.account_state();
    let mnemonic_phrase = state.mnemonic_phrase();
    let phase_exists_in_state = state.phase_exists_in_state();

    spawn_local(async move {
        if !mnemonic_phrase.get().is_some() {
            set_transaction_state.set(TransactionState::SignIn);
        } else {
            let mnemonic = Mnemonic::parse(mnemonic_phrase.get().unwrap()).unwrap();
            let keypair = Keypair::from_phrase(&mnemonic, None).unwrap();
            let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();
            api.tx()
                .sign_and_submit_then_watch_default(&tx, &keypair)
                .await
                .unwrap()
                .wait_for_finalized_success()
                .await
                .unwrap();
            set_transaction_state.set(TransactionState::Transaction);
        }
    });

    view! {
        <>
            <Nav />
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
                                <SetPhraseFromPass />
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
