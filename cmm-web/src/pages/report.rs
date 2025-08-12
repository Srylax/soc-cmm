use dioxus::prelude::*;

use crate::components::{PrintOverviewComponent, ProfileComponent, ProfileValuesComponent};

#[component]
pub fn Report() -> Element {
    rsx! { 
        main {
            class: "max-w-3xl mx-auto",
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
