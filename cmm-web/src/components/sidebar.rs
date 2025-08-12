use crate::{
    Route,
    components::{ButtonComponent, DomainIconComponent, SidebarScoreComponent},
    utils::{use_app_settings, use_schema, use_stats},
};
use cmm_core::{cid::Domain, score::Score};
use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::fa_solid_icons::{FaBars, FaGear, FaMoon, FaPlus, FaSun},
};
use strum::VariantArray;

#[component]
pub fn SidebarComponent(children: Element) -> Element {
    let schema = use_schema();
    let mut settings = use_app_settings();
    let (stats, _) = use_stats();

    let mut sidebar_open = use_signal(|| false);
    let mut settings_open = use_signal(|| false);

    rsx! {
        button {
            class: "fixed z-30 left-3 top-2.5 cursor-pointer bg-white rounded p-2 lg:hidden lg:invisible print:hidden border-1 border-slate-200",
            class: if sidebar_open() { "left-[280px] translate-x-1/2" } else { "shadow" },
            onclick: move |_1| {
                sidebar_open.set(!sidebar_open());
            },
            if sidebar_open() {
                Icon {
                    height: 20,
                    width: 20,
                    fill: "black",
                    class: "rotate-45",
                    icon: FaPlus,
                }
            } else {
                Icon {
                    height: 20,
                    width: 20,
                    fill: "black",
                    icon: FaBars,
                }
            }
        }
        span {
            class: "fixed z-10 h-full w-full bg-black opacity-30 print:hidden",
            class: if !settings_open() { "lg:hidden lg:invisible" },
            class: if !sidebar_open() && !settings_open() { "invisible hidden" },
            onclick: move |_| {
                if settings_open() {
                    settings_open.set(false);
                } else {
                    sidebar_open.set(!sidebar_open());
                }
            },
        }
        nav {
            class: "fixed z-20 h-[calc(100%-20px)] left-2.5 top-2.5 rounded-2xl border-[1px] border-slate-200 dark:border-slate-800 shadow-xs max-w-[280px] w-full overflow-auto bg-white dark:bg-slate-900 no-scrollbar print:hidden",
            class: if !sidebar_open() { "not-lg:invisible not-lg:hidden" },
            div {
                class: "fixed z-30 max-w-[280px] bottom-2.5 left-2.5 bg-gradient-to-t from-white dark:from-slate-900 to-transparent from-50% w-full p-4 pt-12 rounded-b-2xl border-x-[1px] border-b-[1px] border-slate-200 dark:border-slate-800 gap-2 flex",
                ButtonComponent {
                    additional_class: "grow",
                    onclick: move |_| settings_open.set(!settings_open()),
                    Icon {
                        icon: FaGear,
                        width: 18,
                        height: 18,
                    }
                    "Settings"
                }
                ButtonComponent {
                    onclick: move |_| settings.write().darkmode = !settings().darkmode,
                    if settings().darkmode {
                        Icon {
                            width: 18,
                            height: 18,
                            icon: FaSun,
                        }
                    } else {
                        Icon {
                            width: 18,
                            height: 18,
                            icon: FaMoon,
                        }
                    }
                }
            }
            dialog {
                class: "p-6 fixed top-1/2 left-1/2 max-w-3xs w-full bg-white dark:bg-slate-900 rounded-2xl dark:border-2 dark:border-slate-700 -translate-1/2 shadow-2xl",
                open: settings_open(),
                div {
                    class: "grid gap-y-2",
                    h2 {
                        class: "text-2xl dark:text-slate-50 mb-2 font-semibold",
                        "Settings"
                    }
                    {children}
                }
            }
            div {
                class: "p-4 mb-14",
                div {
                    class: "mb-4",
                    Link {
                        class: "text-lg font-semibold flex justify-between",
                        to: Route::Report {},
                        "Report"
                    }
                }
                NavigationSectionComponent {
                    title: "Overview",
                    href: "overview",
                }
                NavigationSectionComponent {
                    title: "Pinned",
                    href: "pinned",
                }
                for domain in Domain::VARIANTS {
                    NavigationSectionComponent {
                        title: "{domain}",
                        domain: domain.clone(),
                        href: "variant-{domain}",
                        score: stats.read().maturity_by_domain(&domain),
                        for (i , aspect) in schema.aspects(&domain).iter().enumerate() {
                            NavigationLinkComponent {
                                title: "{aspect}",
                                href: "aspect-{domain}-{i + 1}",
                                score: stats.read().maturity_by_aspect(&domain, i as u8 + 1),
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn NavigationLinkComponent(title: String, href: String, score: Score) -> Element {
    rsx! {
        li {
            class: "dark:text-slate-300 dark:border-slate-600 dark:has-hover:border-slate-50 has-hover:border-slate-800 text-slate-800 border-l-2 border-slate-300 pl-3.5 ml-1.5 py-1 text-[15px] relative group",
            span {
                class: "absolute w-2 h-1/2 -bottom-[1px] not-group-last:hidden dark:bg-slate-900 bg-white -left-[1px] -translate-x-1/2",
            }
            span {
                class: "absolute w-2 h-1/2 top-0 pt-2 not-group-first:hidden dark:bg-slate-900 bg-white -left-[1px] -translate-x-1/2",
            }
            span {
                class: "dot absolute w-1.5 h-1.5 dark:bg-slate-500 bg-slate-400 group-has-hover:bg-slate-900 dark:group-has-hover:bg-white rounded-full -left-[1px] -translate-1/2 top-1/2",
            }
            a {
                class: "dark:hover:text-slate-50 hover:text-slate-950 flex justify-between gap-x-1",
                href: "#{href}",
                alt: "Move to {title}",
                span {

                    "{title}"
                }
                span {
                    // element should always exist to prevent layout shift
                    class: "dark:opacity-80 opacity-90",
                    SidebarScoreComponent {
                        score,
                    }
                }
            }
        }
    }
}

#[component]
fn NavigationSectionComponent(
    title: String,
    href: String,
    domain: Option<Domain>,
    score: Option<Score>,
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
                    class: "flex items-center gap-2",
                    if domain.is_some() {
                        DomainIconComponent {
                            domain: domain.unwrap().clone(),
                            width: 14,
                            height: 14,
                            fill: "currentColor",
                        }
                    }
                    "{title}"
                }
                span {
                    class: "opacity-80",
                    SidebarScoreComponent {
                        score,
                    }
                }
            }
            ol {
                class: "mt-2",
                {children}
            }
        }
    }
}
