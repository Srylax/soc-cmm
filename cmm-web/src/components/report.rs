use cmm_core::{cid::Domain, profile::QuestionCategory};
use dioxus::prelude::*;
use crate::{components::{PrintBoxComponent, ScoreComponent, ValueOrPlaceholderComponent}, utils::use_soc_data};
use strum::VariantArray;

use crate::utils::{use_schema, use_stats};

#[component]
pub fn ProfileValuesComponent() -> Element {
    let schema = use_schema();
    let data = use_soc_data();

    rsx! {
        for category in QuestionCategory::VARIANTS {
            PrintBoxComponent {
                label: rsx! {
                    if category == &QuestionCategory::Personal {
                        "Personal information"
                    } else {
                        "SOC & organizational profile"
                    }
                },
                div {
                    class: "grid grid-cols-5 gap-2 items-center text-[10px]",
                    for (id, question) in schema.profile().iter().filter(|(_, question)| question.category() == category) {
                        div {
                            class: "col-span-3 text-[11px] font-medium text-slate-700",
                            "{question.short()}"
                        }
                        div {
                            class: "col-span-2",
                            PrintValueHolderBoxComponent {
                                ValueOrPlaceholderComponent {
                                    value: "{data().profile_answer(id.clone()).cloned().unwrap_or(question.question_type().default_value())}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn PrintValueHolderBoxComponent(children: Element) -> Element {
    rsx! {
        div {
            class: "min-h-[22px] px-2 py-1 rounded-md border border-slate-200 bg-white text-[10px] text-slate-800 whitespace-nowrap overflow-hidden text-ellipsis",
            {children}
        }
    }
}


#[component]
pub fn PrintOverviewComponent() -> Element {
    let schema = use_schema();
    let (stats, _) = use_stats();

    rsx! {
        for domain in Domain::VARIANTS {
            PrintBoxComponent {
                label: rsx! {
                    div {
                        class: "flex justify-between",
                        "{domain} domain",
                        div {
                            span {
                                "Maturity: "
                                ScoreComponent {
                                    score: stats.read().maturity_by_domain(&domain),
                                    precision: 2,
                                    replace_nan: true
                                }
                            }
                            if domain == &Domain::Technology || domain == &Domain::Services {
                                span {
                                    class: "ml-2",
                                    "Capability: "
                                    ScoreComponent {
                                        score: stats.read().capability_by_domain(&domain),
                                        precision: 2,
                                        replace_nan: true
                                    }
                                }
                            }
                        }
                    }
                },
                table {
                    class: "w-full text-left border-collapse",
                    thead {
                        tr {
                            class: "text-[10px] text-slate-500",
                            th {
                                class: "py-1 pr-2 font-medium",
                                "Aspect"
                            }
                            th {
                                class: "py-1 pr-2 font-medium text-center",
                                "Maturity"
                            }
                            if domain == &Domain::Technology || domain == &Domain::Services {
                                th {
                                    class: "py-1 pr-2 font-medium text-center",
                                    "Capability"
                                }
                            }
                        }
                    }
                    tbody {
                        for (i, aspect) in schema.aspects(&domain).iter().enumerate() {
                            tr {
                                key: "{aspect}",
                                class: "border-t border-slate-200",
                                td {
                                    class: "text-[11px] py-[2px]",
                                    "{aspect}"
                                }
                                td {
                                    class: "text-[11px] py-[6px] w-[52px] pr-2 text-right",
                                    PrintValueHolderBoxComponent {
                                        ScoreComponent {
                                            score: stats.read().maturity_by_aspect(&domain, i as u8),
                                            precision: 2,
                                            replace_nan: true
                                        }
                                    }
                                }
                                if domain == &Domain::Technology || domain == &Domain::Services {
                                    td {
                                        class: "text-[11px] w-[52px] pr-2 text-right",
                                        PrintValueHolderBoxComponent {
                                            ScoreComponent {
                                                score: stats.read().capability_by_aspect(&domain, i as u8),
                                                precision: 2,
                                                replace_nan: true
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
    }
}

