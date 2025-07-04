use cmm_core::{CID, CMM, Domain, answer::Answer, control::Control};
use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons::FaStar as FasStar, icons::fa_regular_icons::FaStar, Icon};
use strum::VariantArray;

#[component]
pub fn ControlListComponent(cmm: ReadOnlySignal<CMM>, pinned: bool) -> Element {
    rsx! {
        for domain in Domain::VARIANTS {
            if !pinned {
                h2 {
                    class: "text-3xl mb-2 mt-6 font-semibold",
                    id: "variant-{domain}",
                    "{domain}"
                },
            }
            for (i, aspect) in cmm.read().aspect(domain).unwrap().iter().enumerate() {
                if !pinned {
                    h3 {
                        class: "text-2xl mb-2 mt-6 font-semibold",
                        id: "aspect-{domain}-{i + 1}",
                        "{i + 1}. {aspect.title()}"
                    }
                }
                div {
                    class: "",
                    for (cid, control) in aspect.controls() {
                        if (pinned && control.bookmark()) || !pinned {
                            ControlItemComponent {
                                key: format!("{cid}{domain}"),
                                domain: *domain,
                                cid: cid.to_owned(),
                                control: control.clone(),
                                pinned
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ControlItemValuePreviewComponent(
    domain: Domain,
    cid: ReadOnlySignal<CID>,
    control: ReadOnlySignal<Control>,
) -> Element {
    let mut cmm = use_context::<Signal<CMM>>();

    let value = control().clone().answer().as_value();
    let Answer::Bool(_) = control().answer() else {
        return rsx! {
            span {
                class: "dark:bg-slate-600 bg-slate-300 rounded px-2 py-1 text-sm",
                "{value}"
            }
        };
    };

    rsx! {
        span {
            class: "dark:bg-slate-600 bg-slate-300 cursor-pointer hover:bg-blue-300 rounded px-2 py-1 text-sm",
            role: "button",
            onclick: move |e| {
                e.prevent_default();
                cmm.write().set_answer(&domain, cid(), Answer::Bool(value == "false"));
            },
            "{value}"
        }
    }
}

#[component]
fn ControlItemComponent(
    domain: Domain,
    cid: ReadOnlySignal<CID>,
    control: ReadOnlySignal<Control>,
    pinned: bool,
) -> Element {
    let mut cmm = use_context::<Signal<CMM>>();
    let indent = cid.read().chars().filter(|c| *c == '.').count();
    if let Answer::Title = control().answer() {
        if indent > 1 {
            return rsx! {
                h5 {
                    class: "mt-4 mb-1 text-xl font-semibold",
                    id: "{domain}.{cid}",
                    "{cid} {control().title()}"
                }
            };
        }
        return rsx! {
            h4 {
                class: "mt-4 mb-1 text-xl font-semibold",
                id: "{domain}.{cid}",
                "{cid} {control().title()}"
            }
        };
    }

    rsx! {
        div {
            class: "indent-{indent} pt-1 pb-0.5",
            tabindex: "-1",
            details {
                class: "dark:bg-slate-800 bg-slate-100 border-1 dark:border-slate-700 border-slate-300 open:p-3 rounded dark:text-slate-50 text-slate-950 dark:not-open:hover:bg-slate-700 not-open:hover:bg-slate-200 duration-100ms ease-in-out group group/details",
                id: "{domain}.{cid}",
                summary {
                    class: "not-in-open:p-3 cursor-pointer flex justify-between w-full",
                    span {
                        if pinned {
                            "{domain} > "
                        },
                        span {
                            class: "opacity-70 mr-2",
                            "{cid}"
                        },
                        "{control().title()}"
                    },
                    div {
                        class: if !control().bookmark() { "bookmark-button" },
                        div {
                            class: "flex",
                            span {
                                role: "button",
                                class: "flex items-center cursor-pointer bg-slate-300 dark:bg-slate-600 hover:bg-blue-300 text-white py-1 px-2 mr-2 rounded text-xs",
                                onclick: move |_| {
                                    cmm.write().toggle_bookmark(&domain, cid());
                                },
                                if control().bookmark() {
                                    Icon {
                                        width: 15,
                                        height: 15,
                                        fill: "white",
                                        icon: FasStar
                                    }
                                } else {
                                    Icon {
                                        width: 15,
                                        height: 15,
                                        fill: "white",
                                        icon: FaStar
                                    }
                                }
                            },
                            ControlItemValuePreviewComponent {
                                domain,
                                cid,
                                control
                            }
                        }
                    }
                },
                div {
                    class: "grid gap-2 mt-4 grid-cols-[3fr_2fr]",
                    span { },
                    span {
                        class: "text-sm",
                        "Comment",
                    },
                },
                div {
                    class: "grid gap-2 mt-1 grid-cols-[3fr_2fr]",
                    ControlInputComponent {
                        domain,
                        cid,
                        control
                    },
                    label {
                        class: "min-h-2xl flex flex-wrap",
                        textarea {
                            class: "dark:bg-slate-700 bg-slate-200 not-dark:border-1 not-dark:border-slate-300 rounded px-2 py-1.5 w-full",
                            value: control().comment().clone().unwrap_or(String::new()),
                            onchange: move |evt| {
                                cmm.write().set_comment(&domain, cid(), evt.value());
                            }
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn ControlInputComponent(
    domain: Domain,
    cid: ReadOnlySignal<String>,
    control: ReadOnlySignal<Control>,
) -> Element {
    let mut cmm = use_context::<Signal<CMM>>();

    if let Answer::Any(content) = control().answer() {
        return rsx! {
            div {
                input {
                    class: "dark:bg-slate-700 bg-slate-200 not-dark:border-1 not-dark:border-slate-300 rounded px-2 py-1.5 w-full block",
                    type: "text",
                    value: "{content}",
                    oninput: move |evt| {
                        cmm.write().set_answer(&domain, cid(), Answer::Any(evt.value()));
                    }
                }
            }
        };
    }

    if let Answer::Bool(content) = control().answer() {
        return rsx! {
            div {
                class: "w-full flex items-baseline gap-x-2",
                for value in vec!["True", "False"] {
                    label {
                        key: format!("{}{}",cid, control().answer().as_value()),
                        class: "dark:bg-slate-700 bg-slate-200 py-1 px-2 rounded cursor-pointer dark:hover:bg-slate-600 hover:bg-slate-300 has-checked:bg-slate-200 dark:has-checked:bg-slate-600 has-checked:border-blue-300 border-3 border-transparent w-full",
                        input {
                            class: "appearance-none opacity-0",
                            tabindex: "0",
                            type: "radio",
                            name:  "{domain}.{cid.clone()}",
                            checked: content == &(value == "True"),
                            onclick: move |_| {
                                cmm.write().set_answer(&domain, cid(), Answer::Bool(value == "True"));
                            }
                        },
                        "{value}"
                    }
                }
            }
        };
    }

    rsx! {
        div {
            class: "grid gap-2",
            for (i, variant) in control().answer().variants().iter().enumerate() {
                label {
                    key: format!("{}{}{}",cid, control().answer().as_value(), i),
                    class: "dark:bg-slate-700 bg-slate-200 py-1 px-2 rounded cursor-pointer dark:hover:bg-slate-600 hover:bg-slate-300 has-checked:bg-slate-200 dark:has-checked:bg-slate-600 has-checked:border-blue-400 border-l-4 border-transparent has-focus:outline-2  has-focus:outline-blue-400 not-dark:not-has-focus:outline-1 not-dark:not-has-focus:outline-slate-300",
                    "data-description":  control().guidances().get(i).cloned().unwrap_or(String::new()),
                    input {
                        class: "appearance-none opacity-0",
                        tabindex: "0",
                        type: "radio",
                        name:  "{domain}.{cid.clone()}",
                        value: variant.to_owned(),
                        checked: control().answer().variant_eq(variant),
                        onclick: move |_evt| {
                            cmm.write().set_answer(&domain, cid(), control().answer().extend_from_variant(variant).unwrap());
                        }
                    }
                    "{variant}"
                }
            }
        }
    }
}
