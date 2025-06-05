use cmm_core::{answer::Answer, control::Control, Domain, CID, CMM};
use dioxus::prelude::*;

#[component]

pub fn ControlComponent(
    domain: Domain,
    cid: ReadOnlySignal<CID>,
    control: ReadOnlySignal<Control>,
) -> Element {
    if let Answer::None = control().answer() {
        return rsx! {
            h4 {
                class: "mt-4 mb-1 text-xl font-semibold",
                "{cid} {control().title()}"
            }
        };
    }

    let value = control().answer().as_value();

    rsx! {
        details {
            class: "bg-slate-800 mt-2 open:p-4 rounded text-slate-50 not-open:hover:bg-slate-700 transition-colors duration-100ms ease-in-out group",
            summary {
                class: "not-in-open:p-4 cursor-pointer flex justify-between w-full",
                span {
                    "{cid} {control().title()}"
                },
                span {
                    class: "bg-slate-600 rounded px-2 py-1",
                    "{value}"
                }
            },
            map_control {
                domain,
                cid,
                control
            }
        }
    }
}

#[component]
fn map_control(
    domain: Domain,
    cid: ReadOnlySignal<String>,
    control: ReadOnlySignal<Control>,
) -> Element {
    if let Answer::Any(content) = control().answer() {
        return rsx! {
            input {
                class: "bg-slate-700 rounded px-2 py-1.5 mt-2",
                type: "text",
                "{content}"
            }
        };
    }

    let mut cmm = use_context::<Signal<CMM>>();

    rsx! {
        div {
            class: "grid gap-y-2 mt-2",
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
}
