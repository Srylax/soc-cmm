use crate::utils::round;
use cmm_core::{CMM, Domain};
use dioxus::prelude::*;
use strum::VariantArray;

#[component]
pub fn OverviewComponent() -> Element {
    let cmm = use_context::<Signal<CMM>>();

    use_effect(move || {
        // this line is required, else the use effect wont update
        let _ = cmm.read();
        document::eval(
            r#"
            const event = new Event("updateChart");
            document.dispatchEvent(event);
            "#,
        );
    });

    rsx! {
        div {
            class: "w-full max-w-4xl mx-auto",
            div {
                class: "w-full h-[500px] bg-slate-200 py-4 rounded",
                canvas {
                    class: "rounded",
                },
            },
        },
        div {
            class: "w-full max-w-5xl mx-auto mt-8 mb-16",
            id: "domain-scores",
            h2 {
                class: "text-3xl mb-4",
                id: "overview",
                "Overview"
            },
            div {
                class: "grid grid-cols-2 gap-x-8",
                for domain in Domain::VARIANTS {
                    div {
                        DomainOverviewComponent { domain: *domain }
                    }
                }
            }
        }
    }
}

#[component]
fn DomainOverviewComponent(domain: Domain) -> Element {
    let cmm = use_context::<Signal<CMM>>();
    let overall_score = (cmm().aspect_maturity_score(&domain).unwrap() * 10.0).ceil() / 10.0;

    rsx! {
        div {
            class: "flex mt-6 justify-between items-center mb-2 w-full",
            h2 {
                class: "text-2xl",
                "{domain}"
            },
            div {
                class: "text-xl",
                title: "{overall_score} / 5",
                "{round(overall_score / 5.0 * 100.0, 0)}%",
            }
        },
        div {
            class: "h-2 w-full rounded block dark:bg-blue-100 bg-gray-300 relative mb-4",
            div {
                class: "h-full absolute left-0 top-0 bg-blue-500 rounded",
                width: "{round(overall_score / 5.0 * 100.0, 0)}%"
            }
        },
        table {
            class: "w-full table-fixed",
            thead {
                tr {
                    th {
                        "Aspect"
                    }
                    th {
                        "Maturity Score"
                    }
                }
            },
            tbody {
                for (i, aspect) in cmm.read().aspect(&domain).unwrap().iter().enumerate() {
                    tr {
                        key: format!("{}{}_{}", aspect.title(), aspect.maturity_score(), aspect.capability_score()),
                        td {
                            "{i + 1}. {aspect.title()}"
                        },
                        td {
                            "{round(aspect.maturity_score(), 2)}"
                        }
                    }
                }
            }
        }
    }
}
