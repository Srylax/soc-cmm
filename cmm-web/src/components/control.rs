use cmm_core::{answer::Answer, control::Control, CID};
use dioxus::prelude::*;

#[component]
pub fn ControlComponent(cid: CID, control: Control) -> Element {
    rsx! {
        details {
            summary {
                "{cid} {control.title()}"
            }
            map_control { cid, control }
        }
    }
}

#[component]
fn map_control(cid: String, control: Control) -> Element {
    if let Answer::Any(content) = control.answer() {
        return rsx! {
            input { type: "text", "{content}" }
        };
    }

    rsx! {
        div {
            for (i, variant) in control.answer().variants().into_iter().enumerate() {
                label {
                    key: cid.clone(),
                    class: "grid",
                    "data-description":  control.guidances().get(i).cloned().unwrap_or(String::new()),
                    input {
                        type: "radio",
                        name: cid.clone(),
                        value: variant.to_owned(),
                    }
                    "{variant}"
                }
            }
        }
    }
}
