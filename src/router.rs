use crate::components::signing::add_account::AddAccount;
use crate::components::signing::create_account::CreateAccount;
use crate::components::signing::set_phrase_from_pass::SetPhraseFromPass;
use crate::components::signing::sign_in_form::SignInForm;
use crate::pages::home::Home;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;
#[component]
pub fn RouterApp() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Not Found.">
                <Route path=path!("/") view=Home />
                // <Route path=path!("/sign-in") view=SignInForm />
                <Route path=path!("/create-account") view=CreateAccount />
                <Route path=path!("/sign-in") view=SetPhraseFromPass />
                <Route path=path!("/add-account") view=AddAccount />
            </Routes>
        </Router>
    }
}
