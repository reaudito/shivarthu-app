use crate::components::signing::create_account::CreateAccount;
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
                <Route path=path!("/sign-in") view=SignInForm />
                <Route path=path!("/create-account") view=CreateAccount />
            </Routes>
        </Router>
    }
}
