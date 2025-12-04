use dioxus::prelude::*;

use dioxus_markdown::Markdown;

use crate::{
    components::{
        ChartComponent, ControlsListComponent, ImportExportComponent, OverviewComponent,
        SectionTitleComponent, SettingsComponent, SidebarComponent, StarButtonComponent,
    },
    utils::use_soc_data,
};

#[component]
pub fn App() -> Element {
    let data = use_soc_data();

    rsx! {
        document::Script {
            src: asset!("/assets/scripts/highlight-nav.js", JsAssetOptions::new().with_preload(true)),
        }
        document::Script {
            src: "https://cdn.jsdelivr.net/npm/chart.js",
        }
        document::Script {
            src: asset!("/assets/scripts/chart.js" JsAssetOptions::new().with_preload(true)),
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
                        src: data().notes().cloned().unwrap(),
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
                div {
                    class: "pinned-list",
                    ControlsListComponent {
                        pinned: true,
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
