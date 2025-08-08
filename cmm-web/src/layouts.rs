use cmm_core::{data::SOCData, schema::Schema, score::Stats};
use dioxus::prelude::*;

use dioxus_storage::{LocalStorage, use_synced_storage};

use crate::components::AppSettings;

use crate::Route;

#[component]
pub fn SettingsLayout() -> Element {
    let settings = use_synced_storage::<LocalStorage, _>("settings".to_owned(), || AppSettings {
        darkmode: false,
        show_percentage: false,
        show_scores: true,
        show_comparison: false,
    });

    let _ = use_context_provider(|| settings);

    rsx!(
        Outlet::<Route> {

        }
    )
}

#[component]
pub fn DataSchemaLayout() -> Element {
    let schema: Schema = use_context_provider(|| {
        serde_json::from_str(include_str!("../../scheme-2.3.4.json")).unwrap()
    });

    let data: Signal<SOCData> = use_synced_storage::<LocalStorage, _>("cmm".to_owned(), || {
        toml::from_str(include_str!("../../data-2.3.4.toml")).unwrap()
    });
    let compare_data: Signal<SOCData> =
        use_synced_storage::<LocalStorage, _>("compare-cmm".to_owned(), || {
            toml::from_str(include_str!("../../data-2.3.4.toml")).unwrap()
        });
    let (data, cmp_data) = use_context_provider(|| (data, compare_data));

    let stats = use_signal(|| Stats::new(data(), schema.clone()));
    let cmp_stats = use_signal(|| Stats::new(cmp_data(), schema.clone()));
    let (mut stats, mut cmp_stats) = use_context_provider(|| (stats, cmp_stats));

    use_effect(move || {
        stats.set(Stats::new(data(), schema.clone()));
        cmp_stats.set(Stats::new(cmp_data(), schema.clone()));
    });

    rsx!(
        Outlet::<Route> {

        }
    )
}
