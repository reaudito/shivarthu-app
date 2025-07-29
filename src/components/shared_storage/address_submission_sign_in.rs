use crate::components::signing::sign_transaction_fn::SignTransactionFn;
use crate::services::common_services::polkadot;
use leptos::prelude::*;

#[component]
pub fn SignTransaction(
    district: String,
    country: String,
    city: String,
    latitude: Option<f64>,
    longitude: Option<f64>,
) -> impl IntoView {
    view! { <ExtensionSignIn district=district country=country city=city latitude=latitude longitude=longitude /> }
}

#[component]
pub fn ExtensionSignIn(
    district: String,
    country: String,
    city: String,
    latitude: Option<f64>,
    longitude: Option<f64>,
) -> impl IntoView {
    let district = district.as_bytes().to_vec();
    let country = country.as_bytes().to_vec();
    let city = city.as_bytes().to_vec();
    // Convert latitude and longitude to Option<i32>
    let latitude = latitude.map(|lat| lat as i32 * 1_000_000);
    let longitude = longitude.map(|lon| lon as i32 * 1_000_000);

    let tx = Box::new(
        polkadot::tx()
            .shared_storage()
            .save_address(district, country, city, latitude, longitude),
    );
    view! { <SignTransactionFn tx=tx /> }
}
