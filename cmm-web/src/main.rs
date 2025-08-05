use cmm_core::{data::SOCData, schema::Schema};
use dioxus::prelude::*;

use dioxus_storage::{LocalStorage, use_synced_storage};
use dioxus_markdown::Markdown;

use crate::components::{
    AppSettings, ChartComponent, ControlsListComponent, ImportExportComponent, OverviewComponent, SectionTitleComponent, SettingsComponent, SidebarComponent, StarButtonComponent
};

mod components;
mod utils;

fn main() {
    dioxus::launch(App);
}


#[component]
fn App() -> Element {
    let _: Schema = use_context_provider(|| {
        serde_json::from_str(include_str!("../../scheme-2.3.4.json")).unwrap()
    });

    let data: Signal<SOCData> = use_synced_storage::<LocalStorage, _>("cmm".to_owned(), || {
        toml::from_str(include_str!("../../data-2.3.4.toml")).unwrap()
    });
    let compare_data: Signal<(SOCData)> = use_synced_storage::<LocalStorage, _>("compare-cmm".to_owned(), || {
        toml::from_str(include_str!("../../data-2.3.4.toml")).unwrap()
    });
    let (data, cmp_data) = use_context_provider(|| (data, compare_data));

    let settings = use_synced_storage::<LocalStorage, _>("settings".to_owned(), || {
        AppSettings {
            darkmode: false,
            show_percentage: false,
            show_scores: true,
            show_comparison: false
        }
    });
    let settings = use_context_provider(|| settings);

    rsx! {
        document::Link { rel: "apple-touch-icon", sizes: "180x180", href: asset!("/assets/apple-touch-icon.png") }
        document::Link { rel: "icon", type: "image/png", sizes: "16x16", href: asset!("/assets/favicon-16x16.png") }
        document::Link { rel: "icon", type: "image/png", sizes: "32x32", href: asset!("/assets/favicon-32x32.png") }
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        document::Link { rel: "manifest", href: asset!("/assets/site.webmanifest")}
        document::Script { src: asset!("/assets/scripts/highlight-nav.js") }
        document::Script { src: asset!("/assets/scripts/scroll.js") }
        document::Script { src: "https://cdn.jsdelivr.net/npm/chart.js" }
        document::Script { src: asset!("/assets/scripts/chart.js"), defer: true }

        SidebarComponent {
            SettingsComponent { settings }
        },
        main {
            class: "lg:ml-[260px] px-8 py-4",
            div {
                class: "flex mt-10 mb-6",
                h1 {
                    class: "mx-auto text-6xl font-semibold",
                    "SOC CMM"
                },
            },
            ImportExportComponent { data, cmp_data },
            if data().notes().is_some() {
                div {
                    class: "max-w-2xl mx-auto",
                    Markdown {
                        src: data().notes().clone().unwrap()
                    }
                }
            },
            ChartComponent { },
            OverviewComponent { },
            div {
                class: "max-w-3xl mx-auto",
                SectionTitleComponent {
                    id: "pinned",
                    text: "Pinned"
                },
                if data().has_pinned_items() {
                    div {
                        class: "pinned-list",
                        ControlsListComponent {
                            pinned: true
                        },
                    },
                } else {
                    div {
                        class: "opacity-60",
                        key: "no-pinned-{data().has_pinned_items()}",
                        "No pinned items. Click",
                        div {
                            class: "inline-block ml-2 translate-y-[2px] pointer-events-none",
                            StarButtonComponent { active: false },
                        },
                        "on a control to pin it!"
                    }
                },
                div {
                    class: "mt-16",
                    ControlsListComponent {
                        pinned: false
                    }
                }
            }
        }
    }
}
