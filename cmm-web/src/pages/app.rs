use cmm_core::{data::SOCData, schema::Schema, score::Stats};
use dioxus::prelude::*;

use dioxus_storage::{LocalStorage, use_synced_storage};
use dioxus_markdown::Markdown;

use crate::{components::{
    AppSettings, ChartComponent, ControlsListComponent, ImportExportComponent, OverviewComponent, ProfileComponent, SectionTitleComponent, SettingsComponent, SidebarComponent, StarButtonComponent
}, utils::use_soc_data};

#[component]
pub fn App() -> Element {
    let data = use_soc_data();

    rsx! {
        document::Script {
            src: asset!("/assets/scripts/highlight-nav.js"),
        }
        document::Script {
            src: "https://cdn.jsdelivr.net/npm/chart.js",
        }
        document::Script {
            src: asset!("/assets/scripts/chart.js"),
            defer: true,
        }

        SidebarComponent {

            SettingsComponent {

            }
        }
        main {
            class: "lg:ml-[290px] px-8 py-4",
            div {
                class: "flex mt-10 mb-6",
                h1 {
                    class: "mx-auto text-6xl font-semibold",
                    "SOC CMM"
                }
            }
            ImportExportComponent {

            }
            if data().notes().is_some() {
                div {
                    class: "max-w-2xl mx-auto mb-6 md-content",
                    Markdown {
                        src: data().notes().clone().unwrap(),
                    }
                }
            }
            ChartComponent {

            }
            OverviewComponent {

            }
            div {
                class: "max-w-3xl mx-auto",
                SectionTitleComponent {
                    id: "pinned",
                    text: "Pinned",
                }
                if data().has_pinned_items() {
                    div {
                        class: "pinned-list",
                        ControlsListComponent {
                            pinned: true,
                        }
                    }
                } else {
                    div {
                        class: "opacity-60",
                        key: "no-pinned-{data().has_pinned_items()}",
                        "No pinned items. Click"
                        div {
                            class: "inline-block mx-2 translate-y-[2px] pointer-events-none",
                            StarButtonComponent {
                                active: false,
                            }
                        }
                        "on a control to pin it!"
                    }
                }
                div {
                    class: "mt-16",
                    ControlsListComponent {
                        pinned: false,
                    }
                }
            }
        }
    }
}
