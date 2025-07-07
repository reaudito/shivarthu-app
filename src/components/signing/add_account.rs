use crate::components::navigation::nav::Nav;
use crate::components::signing::accounts_store::{Account, AccountStore};
use codee::string::JsonSerdeCodec;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos_use::storage::use_local_storage;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use subxt_core::utils::AccountId32;
use subxt_signer::{bip39::Mnemonic, sr25519::Keypair};

#[component]
pub fn AddAccount() -> impl IntoView {
    let (account_store, set_account_store, _reset_account_store) =
        use_local_storage::<AccountStore, JsonSerdeCodec>("account-store-state");
    let (seed, set_seed) = signal("".to_string());
    let (password, set_password) = signal("".to_string());
    let (name, set_name) = signal("".to_string());
    let (confirm_password, set_confirm_password) = signal(String::from(""));
    let (form_submission, set_form_submission) = signal(true);
    let (error_message, set_error_message) = signal(String::from(""));
    let is_valid = move || password().as_str() == confirm_password().as_str();

    let submit_click = move |e: SubmitEvent| {
        e.prevent_default();
        let mnemonic = Mnemonic::parse(seed()).unwrap();
        let keypair = Keypair::from_phrase(&mnemonic, None).unwrap();
        let account_address = keypair.public_key().to_account_id();
        let account_string = format!("{}", account_address);
        gloo::console::log!(account_string.clone());

        let mc = new_magic_crypt!(password(), 256);
        let base64 = mc.encrypt_str_to_base64(seed());
        if !is_valid() {
            set_error_message("Passwords do not match.".to_string());
        } else {
            let account_exists = account_store.with(|store| {
                store
                    .accounts
                    .iter()
                    .any(|a| a.account_address == account_string.clone())
            });

            if account_exists {
                set_error_message("This account already exists.".to_string());
            } else {
                let new_account = Account {
                    hash: base64,
                    account_address: account_string.clone(),
                    name: name().clone(),
                };

                set_account_store.update(move |store| {
                    store.accounts.push(new_account);
                });
                set_form_submission(false);
            }
        }
    };

    view! {
        <>
            <Nav />
            {move || {
                if form_submission() {
                    view! {
                        <>
                            <div>
                                <form
                                    class="max-w-sm mx-auto"
                                    id="seed-submit-form"
                                    on:submit=submit_click
                                >
                                    <div class="mb-5">
                                        <label
                                            for="seed"
                                            class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                                        >
                                            Seed
                                        </label>
                                        <input
                                            type="seed"
                                            id="seed"
                                            class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                            placeholder="Enter the seed"
                                            required
                                            prop:value=move || seed()
                                            on:input=move |e| {
                                                set_seed(event_target_value(&e));
                                                set_error_message(String::new());
                                            }
                                        />
                                    </div>

                                    <div class="mb-5">
                                        <label
                                            for="name"
                                            class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                                        >
                                            Account Name
                                        </label>
                                        <input
                                            type="name"
                                            id="name"
                                            class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                            placeholder="Enter a Name"
                                            required
                                            prop:value=move || name()
                                            on:input=move |e| {
                                                set_name(event_target_value(&e));
                                            }
                                        />
                                    </div>
                                    <div class="mb-5">
                                        <label
                                            for="password"
                                            class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                                        >
                                            Your password
                                        </label>
                                        <input
                                            type="password"
                                            id="password"
                                            class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                            required
                                            prop:value=move || password()
                                            on:input=move |e| set_password(event_target_value(&e))
                                        />
                                    </div>
                                    <div class="mb-5">
                                        <label
                                            for="confirm-password"
                                            class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                                        >
                                            Confirm password
                                        </label>
                                        <input
                                            type="password"
                                            id="confirm-password"
                                            class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                            required
                                            prop:value=move || confirm_password()
                                            on:input=move |e| set_confirm_password(
                                                event_target_value(&e),
                                            )
                                        />

                                    </div>
                                    {move || {
                                        if !error_message().is_empty() {
                                            view! {
                                                <>
                                                    <div
                                                        role="alert"
                                                        class="alert alert-error text-gray-900 dark:text-white"
                                                    >
                                                        <svg
                                                            xmlns="http://www.w3.org/2000/svg"
                                                            class="stroke-current shrink-0 h-6 w-6"
                                                            fill="none"
                                                            viewBox="0 0 24 24"
                                                        >
                                                            <path
                                                                stroke-linecap="round"
                                                                stroke-linejoin="round"
                                                                stroke-width="2"
                                                                d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
                                                            ></path>
                                                        </svg>
                                                        <span class="text-gray-900 dark:text-white">
                                                            {error_message()}
                                                        </span>
                                                    </div>
                                                    <br />
                                                </>
                                            }
                                                .into_any()
                                        } else {
                                            view! {
                                                <>
                                                    <div></div>
                                                </>
                                            }
                                                .into_any()
                                        }
                                    }}

                                    <button
                                        type="submit"
                                        class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                                    >

                                        Submit
                                    </button>
                                </form>
                            </div>
                        </>
                    }
                        .into_any()
                } else {
                    view! {
                        <>
                            <div class="max-w-sm mx-auto text-gray-900 dark:text-white">
                                <div role="alert" class="alert alert-info">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        class="stroke-current shrink-0 w-6 h-6"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                                        ></path>
                                    </svg>
                                    <span class="text-gray-900 dark:text-white">
                                        "Adding Account successful.".
                                    </span>
                                </div>
                                <ul>
                                    {move || {
                                        account_store
                                            .with(|s| {
                                                s.accounts
                                                    .iter()
                                                    .map(|a| {
                                                        view! {
                                                            <li class="text-gray-900 dark:text-white">
                                                                {a.account_address.clone()}
                                                            </li>
                                                        }
                                                    })
                                                    .collect::<Vec<_>>()
                                            })
                                    }}
                                </ul>

                            </div>
                        </>
                    }
                        .into_any()
                }
            }}
        </>
    }
}
