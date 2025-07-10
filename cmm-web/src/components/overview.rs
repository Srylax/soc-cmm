use crate::{components::{BadToGoodProgressBarComponent, DomainIconComponent, SectionTitleComponent}, utils::round};
use cmm_core::{CMM, Domain};
use dioxus::prelude::*;
use strum::VariantArray;

#[component]
pub fn OverviewComponent(
    show_percentage: bool
) -> Element {
    let cmm = use_context::<Signal<CMM>>();

    rsx! {
        div {
            class: "w-full max-w-3xl mx-auto mt-8 mb-16",
            id: "domain-scores",
            SectionTitleComponent { 
                id: "overview",
                text: "Overview"
            },
            div {
                class: "grid grid-cols-2 gap-4",
                div {
                    class: "bg-blue-500 border-1 border-blue-600 rounded-2xl w-full grid place-content-center",
                    div {
                        class: "text-slate-50 text-8xl font-extrabold text-shadow",
                        if show_percentage {
                            "{round(cmm().cmm_maturity_score() / cmm().cmm_max_maturity_score() * 100.0, 1)}%"
                        } else {
                            "{round(cmm().cmm_maturity_score(), 1)}"
                        }
                    },
                    div {
                        class: "text-slate-50 text-right opacity-80",
                        "SOC maturity score",
                        if !show_percentage {
                            " (max {round(cmm().cmm_max_maturity_score(), 1)})"
                        }
                    }
                }
                for domain in Domain::VARIANTS {
                    DomainOverviewComponent { 
                        show_percentage: show_percentage, 
                        domain: *domain 
                    }
                }
            }
        }
    }
}

#[component]
fn DomainOverviewComponent(domain: Domain, show_percentage: bool) -> Element {
    let cmm = use_context::<Signal<CMM>>();
    let overall_score = (cmm().aspect_maturity_score(&domain).unwrap() * 10.0).ceil() / 10.0;
    let overall_capability_score = (cmm().aspect_capability_score(&domain).unwrap() * 10.0).ceil() / 10.0;

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
                            class: "bg-blue-500 print:hidden rounded-xl p-3 aspect-square",
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
                    if overall_capability_score.is_normal() {
                        div {
                            class: "text-xl text-center grid",
                            title: "{overall_capability_score} / 5",
                            small {
                                class: "text-xs",
                                "Capability"
                            }
                            if show_percentage {
                                "{round(overall_capability_score / 5.0 * 100.0, 0)}%",
                            } else {
                                "{round(overall_capability_score, 1)}",
                            }
                        }
                    }
                    div {
                        class: "text-xl text-center grid",
                        title: "{overall_score} / 5",
                        small {
                            class: "text-xs",
                            "Maturity"
                        }
                        if show_percentage {
                            "{round(overall_score / 5.0 * 100.0, 0)}%",
                        } else {
                            "{round(overall_score, 1)}",
                        }
                    }
                },
            },
            div {
                class: "mt-4 bg-slate-50 rounded-2xl p-4 border-1 border-slate-200 dark:border-slate-500 dark:bg-slate-600",
                for (_i, aspect) in cmm.read().aspect(&domain).unwrap().iter().enumerate() {
                    div {
                        key: format!("{}{}_{}", aspect.title(), aspect.maturity_score(), aspect.capability_score()),
                        span {
                            class: "text-[10px] text-right",
                            "data-aspect-value": "{round(aspect.maturity_score(), 2)}",
                            "{aspect.title()}"
                        },
                        div {
                            div {
                                class: "not-print:hidden",
                                if show_percentage {
                                    "{round(aspect.maturity_score() / 5.0 * 100.0, 2)}%"
                                } else {
                                    "{round(aspect.maturity_score(), 2)}"
                                }
                            },
                            BadToGoodProgressBarComponent {
                                max: 5.0,
                                value: aspect.maturity_score(),
                                tooltip_prefix: "{aspect.title()} maturity: "
                            },
                            if aspect.capability_score().is_normal() {
                                div {
                                    class: "mt-1",
                                    BadToGoodProgressBarComponent {
                                        max: 5.0,
                                        value: aspect.capability_score(),
                                        tooltip_prefix: "{aspect.title()} capability: "
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
