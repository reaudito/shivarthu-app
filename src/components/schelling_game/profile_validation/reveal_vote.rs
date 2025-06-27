use crate::components::schelling_game::profile_validation::change_period::ChangePeriod;
use crate::components::schelling_game::profile_validation::reveal_vote_sign_in::SignTransaction;
use crate::components::schelling_game::profile_validation::rpc::vote_end_block::VoteEndBlock;
use crate::components::schelling_game::profile_validation::storage::get_period::GetPeriod;
use crate::services::common_imp::View;
use crate::services::error::ErrorString;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;

#[component]
pub fn RevealVote(profile_user_account: String) -> impl IntoView {
    // gloo::console::log!(profile_user_account());
    let (current_view, set_current_view) = signal(View::Form);
    let (choice, set_choice) = signal::<Result<Option<u128>, ErrorString>>(Ok(None));
    let (salt, set_salt) = signal(String::from(""));
    let submit_click = move |e: SubmitEvent| {
        e.prevent_default();
        if choice().unwrap().is_some() {
            set_current_view(View::Success);
        } else {
            panic!("Choice not set");
        }
    };

    let choice_changed = move |value: String| {
        let choice_value = value.parse::<u128>().expect("Invalid input");
        gloo::console::log!(choice_value);

        set_choice(Ok(Some(choice_value)));
    };

    let render_view = move || {
        match current_view() {
            View::Form => {
                view! {
                    <div class="max-w-5xl mx-auto max-md:mx-10">
                        <div class="text-gray-900 dark:text-white">
                            <GetPeriod profile_user_account=profile_user_account.clone() />
                            <VoteEndBlock profile_user_account=profile_user_account.clone() />
                            <ChangePeriod profile_user_account=profile_user_account.clone() />
                        </div>
                        <form id="reveal-vote-submit-from" on:submit=submit_click>

                            <div class="mb-5">
                                <label
                                    for="choice"
                                    class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                                >
                                    Choice
                                </label>
                                <input
                                    type="number"
                                    id="choice"
                                    class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                    required
                                    on:input=move |ev| choice_changed(event_target_value(&ev))
                                />
                            </div>
                            <div class="mb-5">
                                <label
                                    for="salt"
                                    class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                                >
                                    Salt
                                </label>
                                <input
                                    type="text"
                                    id="salt"
                                    class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                    required
                                    on:input=move |ev| set_salt(event_target_value(&ev))
                                />
                            </div>
                            <button
                                type="submit"
                                id="reveal-vote-submit"
                                class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                            >

                                Submit
                            </button>
                        </form>
                    </div>
                }.into_any()
            }
            View::Success => {
                view! {
                    <div>
                        <SignTransaction
                            salt=salt()
                            choice=choice().unwrap().unwrap()
                            profile_user_account=profile_user_account.clone()
                        />

                    </div>
                }.into_any()
            }

        }
    };

    view! { <div>{move || render_view()}</div> }
}
