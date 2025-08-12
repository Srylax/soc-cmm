use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons::FaArrowTurnDown, Icon};

use crate::{components::{ButtonComponent, PrintOverviewComponent, ProfileComponent, ProfileValuesComponent}, Route};

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
                "Fill out the form below, then use the browser print functionality to generate the report."
            }
            div {
                class: "rounded-2xl p-4 bg-slate-50 border-slate-300 border-1 print:hidden",
                h3 {
                    class: "text-3xl font-semibold mb-4",
                    "Profile"
                }
                ProfileComponent {  }
            }
            h2 {
                class: "font-semibold text-3xl mb-4",
                "Assessment results"
            }
            h3 {
                class: "text-sm mb-0.5 font-semibold",
                "Profile"
            }
            ProfileValuesComponent {  }
            div {
                class: "mt-5 block",
            }
            PrintOverviewComponent {  }
        }
    }
}
