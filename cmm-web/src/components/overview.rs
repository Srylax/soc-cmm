use crate::{components::{BadToGoodProgressBarComponent, DomainIconComponent}, utils::round};
use cmm_core::{CMM, Domain};
use dioxus::prelude::*;
use strum::VariantArray;
use dioxus_free_icons::{icons::fa_solid_icons::FaStar, Icon};

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
            class: "w-full max-w-3xl mx-auto mt-8 mb-16",
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
            class: "w-full bg-slate-100 p-4 rounded-2xl border-slate-300 border-1 dark:border-slate-700 dark:bg-slate-800",
            div {
                class: "bg-slate-200 border-1 border-slate-300 mb-2 p-4 rounded-2xl dark:border-slate-700 dark:bg-slate-900",
                div {
                    class: "flex justify-between items-center w-full",
                    div {
                        class: "flex items-center gap-2",
                        div {
                            class: "bg-blue-500 rounded-xl p-3 aspect-square",
                            DomainIconComponent {
                                width: 20,
                                height: 20,
                                fill: "white",
                                domain
                            }
                        },
                        h2 {
                            class: "text-2xl font-semibold",
                            "{domain}"
                        },
                    }
                    div {
                        class: "text-xl text-center grid",
                        title: "{overall_score} / 5",
                        small {
                            class: "text-xs",
                            "Maturity"
                        }
                        "{round(overall_score / 5.0 * 100.0, 0)}%",
                    }
                },
                // div {
                //     class: "grid items-center gap-2 mb-1 grid-cols-[6rem_1fr]",
                //     span {
                //         class: "text-sm",
                //         "Total maturity"
                //     },
                //     BadToGoodProgressBarComponent {
                //         value: overall_score,
                //         height: 2,
                //         max: 5.0
                //     },
                // },
            },
            div {
                class: "mt-4 bg-slate-50 rounded-2xl p-4 border-1 border-slate-200 dark:border-slate-500 dark:bg-slate-600",
                for (i, aspect) in cmm.read().aspect(&domain).unwrap().iter().enumerate() {
                    div {
                        key: format!("{}{}_{}", aspect.title(), aspect.maturity_score(), aspect.capability_score()),
                        span {
                            class: "text-[10px] text-right",
                            "{aspect.title()}"
                        },
                        div {
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
}
