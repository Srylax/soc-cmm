use cmm_core::{Domain, CMM};
use dioxus::prelude::*;
use strum::VariantArray;


#[component]
fn NavigationLinkComponent(title: String, href: String) -> Element {
    rsx! {
        li {
            class: "mt-2 text-gray-300",
            a {
                class: "hover:text-white transition-colors",
                href: "#{href}",
                alt: "{title}",
                "{title}"
            }
        }
    }
}

#[component]
fn NavigationSectionComponent(title: String, href: String, children: Element) -> Element {
    rsx! {
        div {
            class: "mb-4",
            a {
                href: "#{href}",
                alt: "{title}",
                class: "text-xl",
                span {
                    class: "opacity-50 mr-2",
                    "#"
                }
                "{title}"
            },
            ol {
                {children}
            }
        }
    }
}

#[component]
pub fn SidebarComponent(cmm: Signal<CMM>, children: Element) -> Element {
    rsx! {
        nav {
            class: "fixed z-10 h-full left-0 top-0 bg-gray-950 max-w-[260px] w-full",
            div {
                class: "bg-black p-4",
                {children}
            },
            div {
                class: "p-4 mt-4",
                NavigationSectionComponent {
                    title: "Overview",
                    href: "overview"
                },
                for domain in Domain::VARIANTS {
                    NavigationSectionComponent {
                        title: "{domain}",
                        href: "variant-{domain}",
                        for (i, aspect) in cmm.read().aspect(&domain).unwrap().into_iter().enumerate() {
                            NavigationLinkComponent {
                                title: "{i + 1}. {aspect.title()}",
                                href: "aspect-{domain}-{i + 1}",
                            }
                        }
                    }
                }
            }
        }
    }
}
