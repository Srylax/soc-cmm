use cmm_core::{Domain, CMM};
use dioxus::prelude::*;
use dioxus_storage::{use_synced_storage, LocalStorage};
use strum::VariantArray;


#[component]
fn NavigationLinkComponent(title: String, href: String, score: Option<f64>) -> Element {
    rsx! {
        li {
            class: "text-slate-300 border-l-1 border-gray-700 has-hover:border-white pl-3 py-1 text-md",
            a {
                class: "hover:text-white flex justify-between",
                href: "#{href}",
                alt: "{title}",
                span {
                    "{title}"
                },
                span {
                    class: if score.is_some() { "opacity-80" } else { "opacity-0" },
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
                class: "bg-black p-4",
                {children}
            },
            button {
                class: "px-3 py-1 m-4 bg-blue-400 rounded cursor-pointer",
                onclick: move |_| {
                    show_scores.set(!show_scores());
                },
                if show_scores() {
                    "Hide Scores"
                } else {
                    "Show Scores"
                }
            },
            div {
                class: "p-4 mt-2",
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
