use crate::components::common_transaction::balance_tranfer_sign_in::SignTransaction;
use crate::components::navigation::nav::Nav;
use crate::services::common_imp::View;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use subxt::utils::AccountId32;
use thiserror::Error;

#[derive(Error, Debug, Clone, Deserialize, Serialize)]
pub enum NumberError {
    #[error("Not valid address: `{0}`")]
    NotValidAddress(String),
    #[error("Error parsing to number: `{0}`")]
    ParseIntError(String),
}

#[component]
pub fn BalanceTransfer() -> impl IntoView {
    let (current_view, set_current_view) = signal(View::Form);
    let (transfer_address, set_transfer_address) = signal(Ok(String::from("")));
    let (transfer_balance, set_transfer_balance) = signal(Ok(0));

    let submit_click = move |e: SubmitEvent| {
        e.prevent_default();
        set_current_view(View::Success);
    };

    let on_transfer_address = move |ev| {
        let result_value = event_target_value(&ev).parse::<String>();
        let result = match result_value {
            Ok(parsed_value) => {
                let account_id32 = AccountId32::from_str(&parsed_value);
                match account_id32 {
                    Ok(_) => Ok(parsed_value),
                    Err(_) => Err(NumberError::NotValidAddress(parsed_value)),
                }
            }
            Err(value) => Err(NumberError::ParseIntError(value.to_string())),
        };
        set_transfer_address(result)
    };

    let on_input_balance = move |ev| {
        let result_value = event_target_value(&ev).parse::<u128>();
        let result = match result_value {
            Ok(parsed_value) => Ok(parsed_value),

            Err(value) => Err(NumberError::ParseIntError(value.to_string())),
        };
        set_transfer_balance(result)
    };

    let render_view = move || {
        match current_view() {
            View::Form => {
                view! {
                    <div class="container mx-auto px-10">

                        <form id="transfer-balance-submit-from" on:submit=submit_click>

                            <div class="mb-5">
                                <label
                                    for="Transfer Address"
                                    class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                                >
                                    Transfer Address
                                </label>
                                <input
                                    type="text"
                                    id="transfer-address"
                                    class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                    required
                                    on:input=move |ev| on_transfer_address(ev)
                                />
                            </div>

                            <label
                                for="Transfer Balance"
                                class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                            >
                                Transfer Balance
                            </label>
                            <input
                                type="number"
                                id="transfer-balance"
                                class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                required
                                on:input=move |ev| on_input_balance(ev)
                            />

                            <button
                                type="submit"
                                id="transfer-balance-submit"
                                class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                            >

                                Submit
                            </button>

                        </form>
                        <br />
                        <ErrorBoundary fallback=|errors| {
                            view! {
                                <div class="flex items-center gap-3 p-4 border-l-4 border-red-500 bg-red-100 text-red-800 rounded-xl shadow-md dark:bg-red-900 dark:text-red-200">
                                    <ul>
                                        {move || {
                                            errors
                                                .get()
                                                .into_iter()
                                                .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                                .collect::<Vec<_>>()
                                        }}
                                    </ul>
                                </div>
                            }
                        }>
                            <p>
                                {move || transfer_balance.get().map(|_| ())}
                                {move || transfer_address.get().map(|_| ())}
                            </p>
                        </ErrorBoundary>
                    </div>
                }.into_any()
            }

            View::Success => view! {
                <div>

                    <SignTransaction
                        transfer_address=transfer_address().unwrap()
                        transfer_balance=transfer_balance().unwrap()
                    />
                </div>
            }
            .into_any(),
        }
    };

    view! {
        <div>
            <Nav />
            {move || render_view()}
        </div>
    }
}
