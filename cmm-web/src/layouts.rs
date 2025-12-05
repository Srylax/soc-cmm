use cmm_core::{data::SOCData, schema::Schema, score::Stats};
use dioxus::{asset_resolver, prelude::*};

use dioxus_sdk_storage::{LocalStorage, use_synced_storage};

use crate::components::AppSettings;

use crate::Route;

const SCHEME: Asset = asset!("/assets/scheme-2.3.4.json");

#[component]
pub fn SettingsLayout() -> Element {
    let settings = use_synced_storage::<LocalStorage, _>("settings".to_owned(), || AppSettings {
        darkmode: false,
        show_percentage: false,
        show_scores: true,
        show_comparison: false,
    });

    let _ = use_context_provider(|| settings);

    rsx!(Outlet::<Route> {})
}

#[component]
pub fn DataSchemaLayout() -> Element {
    let asset: Resource<Schema> = use_resource( || async {serde_json::from_slice(&asset_resolver::read_asset_bytes(&SCHEME).await.unwrap()).unwrap() });
    let asset = asset.suspend()?;
    let schema = use_context_provider( move || {
        asset.read().clone()
    });

    let data: Signal<SOCData> =
        use_synced_storage::<LocalStorage, _>("cmm".to_owned(), || SOCData::from(&schema));
    let compare_data: Signal<SOCData> =
        use_synced_storage::<LocalStorage, _>("compare-cmm".to_owned(), || SOCData::from(&schema));
    let (data, cmp_data) = use_context_provider(|| (data, compare_data));

    let stats = use_signal(|| Stats::new(data(), schema.clone()));
    let cmp_stats = use_signal(|| Stats::new(cmp_data(), schema.clone()));
    let (mut stats, mut cmp_stats) = use_context_provider(|| (stats, cmp_stats));

    use_effect(move || {
        stats.set(Stats::new(data(), schema.clone()));
        cmp_stats.set(Stats::new(cmp_data(), schema.clone()));
    });

    rsx!(Outlet::<Route> {})
}
