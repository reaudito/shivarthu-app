use crate::components::navigation::nav::Nav;
use crate::services::common_imp::View;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

// -----------------------------
// Errors
// -----------------------------

#[derive(Error, Debug, Clone, Deserialize, Serialize)]
pub enum AddressError {
    #[error("District is required and must be ≤ 64 characters")]
    InvalidDistrict,

    #[error("Country is required and must be ≤ 64 characters")]
    InvalidCountry,

    #[error("Latitude must be between -90 and 90")]
    InvalidLatitude,

    #[error("Longitude must be between -180 and 180")]
    InvalidLongitude,

    #[error("Could not parse input as number")]
    ParseFloatError,
}

// -----------------------------
// Data Model
// -----------------------------

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SubmittedAddress {
    pub district: String,
    pub country: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

// -----------------------------
// Success Component
// -----------------------------

#[component]
pub fn ConfirmAddressSubmission(
    district: String,
    country: String,
    latitude: Option<f64>,
    longitude: Option<f64>,
) -> impl IntoView {
    view! {
        <div class="p-6 bg-white dark:bg-gray-800 rounded-lg shadow-md border border-gray-200 dark:border-gray-700">
            <h3 class="text-xl font-semibold mb-4 text-gray-800 dark:text-white">"Confirm Address Submission"</h3>
            <ul class="space-y-2 text-gray-700 dark:text-gray-300">
                <li><strong>"District:"</strong> {district}</li>
                <li><strong>"Country:"</strong> {country}</li>
                <li><strong>"Latitude:"</strong> {move || latitude.map(|v| v.to_string()).unwrap_or("N/A".to_string())}</li>
                <li><strong>"Longitude:"</strong> {move || longitude.map(|v| v.to_string()).unwrap_or("N/A".to_string())}</li>
            </ul>
            <p class="mt-4 text-green-600 dark:text-green-400">"Ready to submit!"</p>
        </div>
    }
}

// -----------------------------
// Main Component
// -----------------------------

#[component]
pub fn AddressSubmission() -> impl IntoView {
    let (current_view, set_current_view) = signal(View::Form);

    // Form fields — now using f64 directly wrapped in Result
    let (district, set_district) = signal(Ok(String::new()));
    let (country, set_country) = signal(Ok(String::new()));
    let (latitude, set_latitude) = signal(Ok(None)); // Result<Option<f64>, _>
    let (longitude, set_longitude) = signal(Ok(None)); // Result<Option<f64>, _>

    // Submit handler
    let submit_click = move |e: SubmitEvent| {
        e.prevent_default();

        // Force reactivity by reading signals
        let has_errors = district().is_err()
            || country().is_err()
            || latitude().is_err()
            || longitude().is_err();

        if has_errors {
            return; // Let ErrorBoundary show messages
        }

        set_current_view(View::Success);
    };

    // Helper: validate bounded string
    let validate_text =
        |value: &str, max_len: usize, err: AddressError| -> Result<String, AddressError> {
            if value.trim().is_empty() || value.len() > max_len {
                Err(err)
            } else {
                Ok(value.to_string())
            }
        };

    // District input
    let on_input_district = move |ev| {
        let val = event_target_value(&ev);
        let result = validate_text(&val, 64, AddressError::InvalidDistrict);
        set_district(result);
    };

    // Country input
    let on_input_country = move |ev| {
        let val = event_target_value(&ev);
        let result = validate_text(&val, 64, AddressError::InvalidCountry);
        set_country(result);
    };

    // Latitude parser
    let parse_latitude = |raw: &str| -> Result<Option<f64>, AddressError> {
        if raw.is_empty() {
            return Ok(None);
        }
        match raw.parse::<f64>() {
            Ok(lat) => {
                if (-90.0..=90.0).contains(&lat) {
                    Ok(Some(lat))
                } else {
                    Err(AddressError::InvalidLatitude)
                }
            }
            Err(_) => Err(AddressError::ParseFloatError),
        }
    };

    // Longitude parser
    let parse_longitude = |raw: &str| -> Result<Option<f64>, AddressError> {
        if raw.is_empty() {
            return Ok(None);
        }
        match raw.parse::<f64>() {
            Ok(lon) => {
                if (-180.0..=180.0).contains(&lon) {
                    Ok(Some(lon))
                } else {
                    Err(AddressError::InvalidLongitude)
                }
            }
            Err(_) => Err(AddressError::ParseFloatError),
        }
    };

    // Latitude input handler
    let on_input_latitude = move |ev| {
        let raw = event_target_value(&ev);
        let result = parse_latitude(&raw);
        set_latitude(result);
    };

    // Longitude input handler
    let on_input_longitude = move |ev| {
        let raw = event_target_value(&ev);
        let result = parse_longitude(&raw);
        set_longitude(result);
    };

    // Render current view
    let render_view = move || {
        match current_view() {
            View::Form => {
                view! {
                    <div class="container mx-auto px-10 py-8">

                        <form on:submit=submit_click class="space-y-6">

                            // District
                            <div class="mb-5">
                                <label
                                    for="district"
                                    class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                                >
                                    "District"
                                </label>
                                <input
                                    type="text"
                                    id="district"
                                    prop:value=move || district().clone().unwrap_or_default()
                                    on:input=on_input_district
                                    class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                    placeholder="E.g. Manhattan"
                                />
                            </div>

                            // Country
                            <div class="mb-5">
                                <label
                                    for="country"
                                    class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                                >
                                    "Country"
                                </label>
                                <input
                                    type="text"
                                    id="country"
                                    prop:value=move || country().clone().unwrap_or_default()
                                    on:input=on_input_country
                                    class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                    placeholder="E.g. USA"
                                />
                            </div>

                            // Latitude
                            <div class="mb-5">
                                <label
                                    for="latitude"
                                    class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                                >
                                    "Latitude (-90 to 90)"
                                </label>
                                <input
                                    type="text"
                                    id="latitude"
                                    on:input=on_input_latitude
                                    class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                    placeholder="40.7128"
                                />
                            </div>

                            // Longitude
                            <div class="mb-5">
                                <label
                                    for="longitude"
                                    class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                                >
                                    "Longitude (-180 to 180)"
                                </label>
                                <input
                                    type="text"
                                    id="longitude"
                                    on:input=on_input_longitude
                                    class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                    placeholder="-74.0060"
                                />
                            </div>

                            // Submit Button
                            <button
                                type="submit"
                                class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                            >
                                "Submit Address"
                            </button>

                        </form>

                        <br />

                        // Error Boundary
                        <ErrorBoundary
                            fallback=|errors| {
                                view! {
                                    <div class="flex items-start gap-3 p-4 border-l-4 border-red-500 bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200 rounded-xl shadow-md">
                                        <ul class="list-disc pl-4 space-y-1">
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
                            }
                        >
                            // Trigger reactive error checking
                            <p class="hidden">
                                {move || district().map(|_| ())}
                                {move || country().map(|_| ())}
                                {move || latitude().map(|_| ())}
                                {move || longitude().map(|_| ())}
                            </p>
                        </ErrorBoundary>

                    </div>
                }.into_any()
            }

            View::Success => {
                let dist = district().ok();
                let ctr = country().ok();
                let lat = latitude().ok().flatten();
                let lon = longitude().ok().flatten();

                view! {
                    <div class="container mx-auto px-10 py-8">
                        <ConfirmAddressSubmission
                            district=dist.unwrap_or_default()
                            country=ctr.unwrap_or_default()
                            latitude=lat
                            longitude=lon
                        />
                    </div>
                }
                .into_any()
            }
        }
    };

    // Final render
    view! {
        <div class="min-h-screen bg-gray-50 dark:bg-gray-900 text-gray-900 dark:text-white transition-colors duration-200">
            <Nav />
            {move || render_view()}
        </div>
    }
}
