use crate::components::schelling_game::project_tips::apply_staking_period::ApplyStakingPeriod;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn ApplyStakingPeriodParams() -> impl IntoView {
    let params = use_params_map();

    let project_id = move || {
        params.with(|params| {
            params
                .get("project_id")
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap_or_default()
        })
    };

    let params_value = untrack(move || project_id());

    view! {
        <div>
            <ApplyStakingPeriod project_id=params_value />
        </div>
    }
}
