use crate::components::navigation::nav::Nav;
use crate::components::signing::set_phrase_from_pass::SetPhraseFromPass;
use leptos::prelude::*;

#[component]
pub fn EnterPassWithNav() -> impl IntoView {
    view! {
        <>
            <Nav />
            <SetPhraseFromPass />
        </>
    }
}
