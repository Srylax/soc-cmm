use crate::{components::{BadToGoodProgressBarComponent, DomainIconComponent, ScoreComponent, SectionTitleComponent}, utils::{round, use_app_settings, use_schema, use_soc_data}};
use cmm_core::cid::Domain;
use dioxus::prelude::*;
use strum::VariantArray;

#[component]
pub fn OverviewComponent() -> Element {
    let data = use_soc_data();
    let settings = use_app_settings();

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
                        ScoreComponent { 
                            score: data().maturity_score_overall(),
                            precision: 1
                        }
                    },
                    div {
                        class: "text-slate-50 text-right opacity-80",
                        "SOC maturity score",
                        if !settings().show_percentage {
                            " (max {round(data().maturity_score_overall().max(), 1)})"
                        }
                    }
                }
                for domain in Domain::VARIANTS {
                    DomainOverviewComponent { 
                        domain: *domain,
                    }
                }
            }
        }
    }
}

#[component]
fn DomainOverviewComponent(domain: Domain) -> Element {
    let settings = use_app_settings();
    let data = use_soc_data();
    let schema = use_schema();

    let overall_score = data().maturity_score_by_domain(&domain);
    let overall_capability_score = data().capability_score_by_domain(&domain);

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
                    if overall_capability_score.score().is_normal() {
                        div {
                            class: "text-xl text-center grid",
                            title: "{overall_capability_score.score()} / {overall_capability_score.max()}",
                            small {
                                class: "text-xs",
                                "Capability"
                            },
                            ScoreComponent { 
                                score: overall_capability_score, 
                                precision: 1 
                            }
                        }
                    }
                    div {
                        class: "text-xl text-center grid",
                        title: "{overall_score.score()} / {overall_score.max()}",
                        small {
                            class: "text-xs",
                            "Maturity"
                        },
                        ScoreComponent { 
                            score: overall_score, 
                            precision: 1 
                        }
                    }
                },
            },
            div {
                class: "mt-4 bg-slate-50 rounded-2xl p-4 border-1 border-slate-200 dark:border-slate-500 dark:bg-slate-600",
                for (i, aspect) in schema.aspects(&domain).iter().enumerate() {
                    div {
                        key: format!(
                            "{}_{}_{}", 
                            aspect, 
                            data().maturity_score_by_aspect(domain, i).score(), 
                            data().capability_score_by_aspect(domain, i).score()
                        ),
                        span {
                            class: "text-[10px] text-right",
                            "data-aspect-value": "{round(data().maturity_score_by_aspect(&domain, i as u8).score(), 2)}",
                            "{aspect}"
                        },
                        div {
                            div {
                                class: "not-print:hidden",
                                ScoreComponent { 
                                    score: data().maturity_score_by_aspect(&domain, i as u8), 
                                    precision: 2 
                                }
                            },
                            BadToGoodProgressBarComponent {
                                score: data().maturity_score_by_aspect(&domain, i as u8),
                                tooltip_prefix: "{aspect} maturity: "
                            },
                            if data().capability_score_by_aspect(&domain, i as u8).score().is_normal() {
                                div {
                                    class: "mt-1",
                                    BadToGoodProgressBarComponent {
                                        score: data().capability_score_by_aspect(&domain, i as u8),
                                        tooltip_prefix: "{aspect} capability: "
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
