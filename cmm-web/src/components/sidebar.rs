use cmm_core::{Domain, CMM};
use dioxus::prelude::*;
use dioxus_storage::{use_synced_storage, LocalStorage};
use strum::VariantArray;

use crate::components::ToggleComponent;


#[component]
fn NavigationLinkComponent(title: String, href: String, score: Option<f64>) -> Element {
    rsx! {
        li {
            class: "dark:text-slate-300 dark:border-slate-600 dark:has-hover:border-slate-50 has-hover:border-slate-800 text-slate-800 border-l-1 border-slate-300  pl-3 py-1 text-md",
            a {
                class: "dark:hover:text-slate-50 hover:text-slate-950 flex justify-between",
                href: "#{href}",
                alt: "{title}",
                span {
                    "{title}"
                },
                span {
                    class: if score.is_some() { "dark:opacity-80 opacity-90" } else { "opacity-0" },
                    "{(score.unwrap_or(10.0) * 10.0).round() / 10.0}"
                }
            }
        }
    }
}

#[component]
fn NavigationSectionComponent(title: String, href: String, score: Option<f64>, children: Element) -> Element {
    let mut score_str = String::new();
    if let Some(s) = score {
        score_str = ((s * 10.0).round() / 10.0).to_string();
    };
    rsx! {
        div {
            class: "mb-4",
            a {
                href: "#{href}",
                alt: "{title}",
                class: "text-lg font-semibold flex justify-between",
                span {
                    "{title}",
                },
                span {
                    class: "opacity-80",
                    "{score_str}"
                }
            },
            ol {
                class: "mt-2",
                {children}
            }
        }
    }
}

#[component]
pub fn SidebarComponent(cmm: Signal<CMM>, children: Element) -> Element {
    let mut show_scores = use_synced_storage::<LocalStorage, _>("show_scores".to_owned(), || false);

    rsx! {
        nav {
            class: "fixed z-10 h-full left-0 top-0 max-w-[280px] w-full overflow-auto",
            div {
                class: "p-4 grid gap-y-3",
                span {
                    class: "text-sm font-semibold",
                    "Settings"
                },
                ToggleComponent {
                    checked: show_scores(),
                    onclick: move |_| {
                        show_scores.set(!show_scores());
                    },
                    label: Some(String::from("Show scores"))
                },
                {children},
            },
            div {
                class: "p-4",
                NavigationSectionComponent {
                    title: "Overview",
                    href: "overview",
                    score: None
                },
                for domain in Domain::VARIANTS {
                    NavigationSectionComponent {
                        title: "{domain}",
                        href: "variant-{domain}",
                        score: if show_scores() { cmm().aspect_maturity_score(domain) } else { None },
                        for (i, aspect) in cmm.read().aspect(&domain).unwrap().into_iter().enumerate() {
                            NavigationLinkComponent {
                                title: "{i + 1}. {aspect.title()}",
                                href: "aspect-{domain}-{i + 1}",
                                score: if show_scores() { Some(aspect.maturity_score()) } else { None },
                            }
                        }
                    }
                }
            }
        }
    }
}
