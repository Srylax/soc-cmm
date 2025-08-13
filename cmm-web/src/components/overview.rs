use crate::{components::{BadToGoodProgressBarComponent, DomainIconComponent, ScoreComponent, SectionTitleComponent}, utils::{round, use_app_settings, use_schema, use_stats}};
use cmm_core::{cid::Domain, score::Stats};
use dioxus::prelude::*;
use strum::VariantArray;

#[component]
pub fn OverviewComponent() -> Element {
    let (stats, cmp_stats) = use_stats();
    let settings = use_app_settings();

    rsx! {
        div {
            class: "w-full max-w-3xl mx-auto mt-8 mb-16",
            id: "domain-scores",
            SectionTitleComponent {
                id: "overview",
                text: "Overview",
            }
            div {
                class: "grid md:grid-cols-2 gap-4",
                if settings().show_comparison {
                    div {
                        class: "text-center",
                        "comparison"
                    }
                    div {
                        class: "text-center",
                        "current"
                    }
                    OverallScoreComponent {
                        stats: cmp_stats,
                    }
                }
                OverallScoreComponent {
                    stats,
                }
                for domain in Domain::VARIANTS {
                    if settings().show_comparison {
                        DomainOverviewComponent {
                            domain: *domain,
                            stats: cmp_stats,
                        }
                    }
                    DomainOverviewComponent {
                        domain: *domain,
                        stats,
                    }
                }
            }
        }
    }
}

#[component]
fn OverallScoreComponent(
    stats: ReadOnlySignal<Stats>
) -> Element {
    let settings = use_app_settings();

    rsx! {
        div {
            class: "bg-blue-500 border-1 border-blue-600 rounded-2xl w-full grid place-content-center min-h-64",
            div {
                class: "text-slate-50 text-8xl font-extrabold text-shadow",
                ScoreComponent {
                    score: stats.read().score_overall(),
                    precision: 1,
                }
            }
            div {
                class: "text-slate-50 text-right opacity-80",
                "SOC maturity score"
                if !settings().show_percentage {
                    " (max {round(stats.read().score_overall().max(), 1)})"
                }
            }
        }
    }
}

#[component]
fn DomainOverviewComponent(
    domain: Domain,
    stats: ReadOnlySignal<Stats>
) -> Element {
    let schema = use_schema();

    let overall_score = stats.read().maturity_by_domain(&domain);
    let overall_capability_score = stats.read().capability_by_domain(&domain);

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
                                domain,
                            }
                        }
                        h2 {
                            class: "text-2xl font-semibold",
                            "{domain}"
                        }
                    }
                    if domain == Domain::Technology || domain == Domain::Services {
                        div {
                            class: "text-xl text-center grid",
                            title: "{overall_capability_score}",
                            small {
                                class: "text-xs",
                                "Capability"
                            }
                            ScoreComponent {
                                score: overall_capability_score,
                                precision: 1,
                                replace_nan: true
                            }
                        }
                    }
                    div {
                        class: "text-xl text-center grid",
                        title: "{overall_score}",
                        small {
                            class: "text-xs",
                            "Maturity"
                        }
                        ScoreComponent {
                            score: overall_score,
                            precision: 1,
                        }
                    }
                }
            }
            div {
                class: "mt-4 bg-slate-50 rounded-2xl p-4 border-1 border-slate-200 dark:border-slate-500 dark:bg-slate-600",
                for (i , aspect) in schema.aspects(&domain).iter().enumerate() {
                    div {
                        key: format!(
                            "{}_{}_{}",
                            aspect,
                            stats.read().maturity_by_aspect(&domain, i as u8 + 1).score(),
                            stats.read().capability_by_aspect(&domain, i as u8 + 1).score(),
                        ),
                        span {
                            class: "text-[10px] text-right",
                            "data-aspect-value": "{round(stats.read().maturity_by_aspect(&domain, i as u8 + 1).score(), 2)}",
                            "{aspect}"
                        }
                        div {

                            div {
                                class: "not-print:hidden",
                                ScoreComponent {
                                    score: stats.read().maturity_by_aspect(&domain, i as u8 + 1),
                                    precision: 2,
                                }
                            }
                            BadToGoodProgressBarComponent {
                                score: stats.read().maturity_by_aspect(&domain, i as u8 + 1),
                                tooltip_prefix: "{aspect} maturity: ",
                            }
                            if domain == Domain::Technology || domain == Domain::Services {
                                div {
                                    class: "mt-1",
                                    BadToGoodProgressBarComponent {
                                        score: stats.read().capability_by_aspect(&domain, i as u8 + 1),
                                        tooltip_prefix: "{aspect} capability: ",
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
