use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::fa_solid_icons::FaArrowTurnDown};

use crate::{
    Route,
    components::{PrintOverviewComponent, ProfileComponent, ProfileValuesComponent},
};

#[component]
pub fn Report() -> Element {
    rsx! {
        main {
            class: "max-w-3xl mx-auto not-print:mt-16",
            Link {
                class: "print:hidden flex gap-x-2 items-center mb-4 hover:underline",
                to: Route::App {  },
                Icon {
                    icon: FaArrowTurnDown,
                    width: 14,
                    height: 14,
                    class: "rotate-z-90",
                }
                "Back"
            }
            h1 {
                class: "font-semibold text-4xl mb-4 print:hidden",
                "Report"
            }
            p {
                class: "text-xl mb-4 print:hidden",
                "Fill out the form below, then use the browser print functionality to generate the report. Enable 'Print Backgrounds' for better results."
            }
            div {
                class: "rounded-2xl p-4 bg-slate-50 border-slate-300 border-1 print:hidden",
                h3 {
                    class: "text-3xl font-semibold mb-4",
                    "Profile"
                }
                ProfileComponent {  }
            }
            div {
                class: "not-print:mt-10",
                h2 {
                    class: "font-semibold text-3xl mb-4",
                    "Assessment results"
                }
                div {
                    section {
                        class: "mb-4 grid gap-4",
                        ProfileValuesComponent {  }
                    }
                    section {
                        class: "grid grid-cols-2 gap-4 break-before-page",
                        h3 {
                            class: "col-span-2 text-2xl",
                            "SOC assessment scores"
                        }
                        PrintOverviewComponent {  }
                    }
                }
            }
        }
    }
}
