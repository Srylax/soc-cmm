use cmm_core::cid::Domain;
use dioxus::prelude::*;
use crate::{components::ScoreComponent, utils::round};
use strum::VariantArray;

use crate::utils::{use_schema, use_stats};

#[component]
pub fn PrintOverviewComponent() -> Element {
    let schema = use_schema();
    let (stats, _) = use_stats();

    rsx! {
        table {
            class: "text-xs print-table",
            tr {
                th {
                    "SOC assessment scores"
                }
                th {
                    "Maturity score"
                }
                th {
                    "Maturity target"
                }
                th {
                    "Capability score"
                }
                th {
                    "Capability target"
                }
            }
            for domain in Domain::VARIANTS {
                tr {
                    td {
                        class: "border-none !p-1"
                    }
                }
                tr {
                    td {
                        class: "font-bold border-none text-right",
                        "{domain}"
                    }
                    td {
                        ScoreComponent {
                            score: stats.read().maturity_by_domain(&domain),
                            precision: 2,
                            replace_nan: true
                        }
                    }
                    td {
                        "5"
                    }
                    if domain == &Domain::Technology || domain == &Domain::Services {
                        td {
                            ScoreComponent {
                                score: stats.read().capability_by_domain(&domain),
                                precision: 2,
                                replace_nan: true
                            }
                        }
                        td {
                            "5"
                        }
                    }
                }
                for (i, aspect) in schema.aspects(&domain).iter().enumerate() {
                    tr {
                        td {
                            class: "text-right",
                            "{aspect}"
                        }
                        td {
                            ScoreComponent {
                                score: stats.read().maturity_by_aspect(&domain, i as u8),
                                precision: 2,
                                replace_nan: true
                            }
                        }
                        td {
                            class: "opacity-0 border-none",
                        }
                        if domain == &Domain::Technology || domain == &Domain::Services {
                            td {
                                ScoreComponent {
                                    score: stats.read().capability_by_aspect(&domain, i as u8),
                                    precision: 2,
                                    replace_nan: true
                                }
                            }
                            td {
                                class: "opacity-0 border-none",
                            }
                        }
                    }
                } 
            }
        }
    }
}

