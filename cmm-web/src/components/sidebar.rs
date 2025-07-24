use cmm_core::{cid::Domain, schema::Schema, score::Score};
use dioxus::prelude::*;
use strum::VariantArray;
use crate::{components::ScoreComponent, utils::{use_app_settings, use_schema, use_soc_data}};
use dioxus_free_icons::{icons::fa_solid_icons::FaBars, icons::fa_solid_icons::FaPlus, Icon};


#[component]
pub fn SidebarComponent(
    children: Element
) -> Element {
    let settings = use_app_settings();
    let schema = use_schema();
    let data = use_soc_data();

    let mut sidebar_open = use_signal(|| false);

    rsx! {
        button {
            class: "fixed z-30 left-0 top-5 cursor-pointer bg-white rounded-r p-2 lg:hidden lg:invisible print:hidden",
            class: if sidebar_open() { 
                "left-[260px] translate-x-1/2" 
            } else { 
                "shadow"
            },
            onclick: move |_1| {
                sidebar_open.set(!sidebar_open());
            },
            if sidebar_open() {
                Icon {
                    height: 20,
                    width: 20,
                    fill: "black",
                    class: "rotate-45",
                    icon: FaPlus
                }
            } else {
                Icon {
                    height: 20,
                    width: 20,
                    fill: "black",
                    icon: FaBars
                }
            }
        },
        span {
            class: "fixed z-10 h-full w-full bg-black opacity-30 lg:hidden lg:invisible print:hidden",
            class: if !sidebar_open() { "invisible hidden" },
            onclick: move |_1| {
                sidebar_open.set(!sidebar_open());
            }
        },
        nav {
            class: "fixed z-20 h-full left-0 top-0 max-w-[280px] w-full overflow-auto bg-white dark:bg-slate-900 no-scrollbar print:hidden",
            class: if !sidebar_open() { "not-lg:invisible not-lg:hidden" },
            div {
                class: "p-4 grid gap-y-3",
                span {
                    class: "text-sm font-semibold",
                    "Settings"
                },
                {children},
            },
            div {
                class: "p-4",
                NavigationSectionComponent {
                    title: "Overview",
                    href: "overview",
                },
                NavigationSectionComponent {
                    title: "Pinned",
                    href: "pinned",
                },
                for domain in Domain::VARIANTS {
                    NavigationSectionComponent {
                        title: "{domain}",
                        href: "variant-{domain}",
                        score: data().maturity_score_by_domain(domain),
                        for (i, aspect) in schema.aspects(domain).iter().enumerate() {
                            NavigationLinkComponent {
                                title: "{i + 1}. {aspect}",
                                href: "aspect-{domain}-{i + 1}",
                                score: data().maturity_score_by_aspect(domain, i as u8),
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn NavigationLinkComponent(
    title: String, 
    href: String, 
    score: Score,
) -> Element {

    rsx! {
        li {
            class: "dark:text-slate-300 dark:border-slate-600 dark:has-hover:border-slate-50 has-hover:border-slate-800 text-slate-800 border-l-1 border-slate-300  pl-3 py-1 text-md",
            a {
                class: "dark:hover:text-slate-50 hover:text-slate-950 flex justify-between gap-x-1",
                href: "#{href}",
                alt: "Move to {title}",
                span {
                    "{title}"
                },
                span {
                    // element should always exist to prevent layout shift
                    class: "dark:opacity-80 opacity-90",
                    SidebarScoreComponent { score }
                }
            }
        }
    }
}

#[component]
fn NavigationSectionComponent(
    title: String,
    href: String,
    score: Score,
    children: Element,
) -> Element {
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
                    SidebarScoreComponent { score }
                }
            },
            ol {
                class: "mt-2",
                {children}
            }
        }
    }
}
