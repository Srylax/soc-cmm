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
    let indent = cid.read().chars().filter(|c| *c == '.').count();
    if let Answer::None = control().answer() {
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
            details {
                class: "bg-slate-800 border-1 border-slate-700 open:p-3 rounded text-slate-50 not-open:hover:bg-slate-700 transition-colors duration-100ms ease-in-out group",
                summary {
                    class: "not-in-open:p-3 cursor-pointer flex justify-between w-full",
                    span {
                        "{cid} {control().title()}"
                    },
                    div {
                        span {
                            class: "bg-slate-600 rounded px-2 py-1 text-sm",
                            "{value}"
                        }
                    }
                },
                div {
                    class: "grid gap-2 mt-4 grid-cols-[60%_40%]",
                    span { },
                    span {
                        class: "text-sm",
                        "Comment",
                    },
                },
                div {
                    class: "grid gap-2 mt-1 grid-cols-[60%_40%]",
                    div {
                        class: "grid gap-2",
                        ControlInputComponent {
                            domain,
                            cid,
                            control
                        },
                    },
                    label {
                        class: "min-h-2xl flex flex-wrap",
                        textarea {
                            class: "bg-slate-700 rounded px-2 py-1.5 w-full",
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
    if let Answer::Any(content) = control().answer() {
        return rsx! {
            div {
                input {
                    class: "bg-slate-700 rounded px-2 py-1.5 w-full",
                    type: "text",
                    value: "{content}"
                }
            }
        };
    }

    let mut cmm = use_context::<Signal<CMM>>();

    rsx! {
        for (i, variant) in control().answer().variants().into_iter().enumerate() {
            label {
                key: cid.clone() + control().answer().as_value() + i,
                class: "bg-slate-700 py-1 px-2 rounded cursor-pointer hover:bg-slate-600 transition-colors has-checked:bg-slate-600",
                "data-description":  control().guidances().get(i).cloned().unwrap_or(String::new()),
                input {
                    class: "mr-2",
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
