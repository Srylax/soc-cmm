use cmm_core::{Domain, CMM};
use dioxus::prelude::*;
use strum::VariantArray;

#[component]
pub fn OverviewComponent() -> Element {
    let cmm = use_context::<Signal<CMM>>();

    use_effect(move || {
        // this line is required, else the use effect wont update
        let _ = cmm.read();
        document::eval(r#"
            const event = new Event("updateChart");
            document.dispatchEvent(event);
            "#);
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
            class: "w-full max-w-3xl mx-auto mt-8",
            id: "domain-scores",
            h2 {
                class: "text-3xl mb-4",
                id: "overview",
                "Overview"
            },
            for domain in Domain::VARIANTS {
                DomainOverviewComponent { domain: *domain }
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
                "{(overall_score / 5.0 * 100.0).ceil()}%",
            }
        },
        div {
            class: "h-2 w-full rounded block bg-blue-100 relative mb-4",
            div {
                class: "h-full absolute left-0 top-0 bg-blue-500 rounded",
                width: "{(overall_score / 5.0 * 100.0).ceil()}%"
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
                for (i, aspect) in cmm.read().aspect(&domain).unwrap().into_iter().enumerate() {
                    tr {
                        key: aspect.title() + aspect.maturity_score() + "_" + aspect.capability_score(),
                        td {
                            "{i + 1}. {aspect.title()}"
                        },
                        td {
                            "{aspect.maturity_score()}"
                        }
                    }
                }
            }
        }
    }
}
