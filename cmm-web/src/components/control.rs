use cmm_core::{answer::Answer, control::Control, Domain, CID, CMM};
use dioxus::prelude::*;
use strum::VariantArray;

#[component]
pub fn ControlListComponent(cmm: ReadOnlySignal<CMM>) -> Element {
    rsx! {
        for domain in Domain::VARIANTS {
            h2 {
                class: "text-3xl mb-2 mt-6 font-semibold",
                id: "variant-{domain}",
                "{domain}"
            },
            for (i, aspect) in cmm.read().aspect(&domain).unwrap().into_iter().enumerate() {
                h3 {
                    class: "text-2xl mb-2 mt-6 font-semibold",
                    id: "aspect-{domain}-{i + 1}",
                    "{i + 1}. {aspect.title()}"
                }
                div {
                    class: "",
                    for (cid,control) in aspect.controls() {
                        ControlItemComponent {
                            key: cid.to_owned() + control.answer().as_value(),
                            domain: *domain,
                            cid: cid.to_owned(),
                            control: control.clone()
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ControlItemComponent(
    domain: Domain,
    cid: ReadOnlySignal<CID>,
    control: ReadOnlySignal<Control>,
) -> Element {
    let mut cmm = use_context::<Signal<CMM>>();
    let indent = cid.read().chars().filter(|c| *c == '.').count();
    if let Answer::Title = control().answer() {
        if indent > 1 {
            return rsx! {
                h5 {
                    class: "mt-4 mb-1 text-xl font-semibold",
                    "{cid} {control().title()}"
                }
            };
        }
        return rsx! {
            h4 {
                class: "mt-4 mb-1 text-xl font-semibold",
                "{cid} {control().title()}"
            }
        };
    }

    let value = control().answer().as_value();

    rsx! {
        div {
            class: "indent-{indent} pt-1 pb-0.5",
            tabindex: "-1",
            details {
                class: "bg-slate-800 border-1 border-slate-700 open:p-3 rounded text-slate-50 not-open:hover:bg-slate-700 duration-100ms ease-in-out group",
                summary {
                    class: "not-in-open:p-3 cursor-pointer flex justify-between w-full",
                    span {
                        span {
                            class: "opacity-70 mr-2",
                            "{cid}"
                        },
                        "{control().title()}"
                    },
                    div {
                        span {
                            class: "bg-slate-600 rounded px-2 py-1 text-sm",
                            "{value}"
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
                            class: "bg-slate-700 rounded px-2 py-1.5 w-full",
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
                    class: "bg-slate-700 rounded px-2 py-1.5 w-full",
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
                        key: cid.clone() + control().answer().as_value() + i,
                        class: "bg-slate-700 py-1 px-2 rounded cursor-pointer hover:bg-slate-600 has-checked:bg-slate-600 has-checked:border-blue-400 border-3 border-transparent w-full",
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
            for (i, variant) in control().answer().variants().into_iter().enumerate() {
                label {
                    key: cid.clone() + control().answer().as_value() + i,
                    class: "bg-slate-700 py-1 px-2 rounded cursor-pointer hover:bg-slate-600 has-checked:bg-slate-600 has-checked:border-blue-400 border-l-4 border-transparent has-focus:outline-2 has-focus:outline-dashed has-focus:outline-blue-400 outline-l-0",
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
