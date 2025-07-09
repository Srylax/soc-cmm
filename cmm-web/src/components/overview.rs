use crate::{components::ProgressBarComponent, components::BadToGoodProgressBarComponent, utils::round};
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
                class: "grid grid-cols-2 gap-4",
                for domain in Domain::VARIANTS {
                    DomainOverviewComponent { domain: *domain }
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
            class: "w-full bg-slate-100 p-4 rounded",
            div {
                class: "flex justify-between items-center mb-2 w-full",
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
            BadToGoodProgressBarComponent {
                value: overall_score,
                height: 2,
                max: 5.0
            },
            div {
                class: "mt-2",
                for (i, aspect) in cmm.read().aspect(&domain).unwrap().iter().enumerate() {
                    div {
                        key: format!("{}{}_{}", aspect.title(), aspect.maturity_score(), aspect.capability_score()),
                        span {
                            class: "text-xs",
                            "{aspect.title()}"
                        },
                        BadToGoodProgressBarComponent {
                            max: 5.0,
                            value: aspect.maturity_score()
                        },
                        if aspect.capability_score().is_normal() {
                            div {
                                class: "mt-1",
                                BadToGoodProgressBarComponent {
                                    max: 5.0,
                                    value: aspect.capability_score(),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
