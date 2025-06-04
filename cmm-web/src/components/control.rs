use cmm_core::{answer::Answer, control::Control, Domain, CID};
use dioxus::{html::option::selected, prelude::*};

#[component]
pub fn ControlComponent(domain: Domain, cid: CID, control: Control) -> Element {
// pub fn ControlComponent(domain: Domain, cid: ReadOnlySignal<CID>, control: ReadOnlySignal<Control>) -> Element {
    rsx! {
        details {
            open: true,
            class: "bg-gray-900 p-4 m-4 rounded text-gray-50",
            summary {
               "{cid} {control.title()}"
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
// fn map_control(domain: Domain, cid: ReadOnlySignal<String>, control: ReadOnlySignal<Control>) -> Element {
fn map_control(domain: Domain, cid: String, control: Control) -> Element {
    if let Answer::Any(content) = control.answer() {
        return rsx! {
            input {
                class: "bg-gray-800 rounded px-2 py-1.5 mt-2",
                type: "text",
                "{content}"
            }
        };
    }


    rsx! {
        div {
            class: "grid gap-y-2 mt-2",
            for (i, variant) in control.answer().variants().into_iter().enumerate() {
                label {
                    key: cid.clone(),
                    class: "bg-gray-800 py-1 px-2 rounded cursor-pointer hover:bg-gray-700 transition-colors has-checked:bg-gray-700",
                    "data-description":  control.guidances().get(i).cloned().unwrap_or(String::new()),
                    input {
                        class: "mr-2",
                        type: "radio",
                        name:  "{domain}.{cid.clone()}",
                        value: variant.to_owned(),
                        checked: control.answer().variant_eq(variant),
                    }
                    "{variant}"
                }
            }
        }
    }
}
