use dioxus::prelude::*;

#[component]
pub fn ToggleComponent(
    checked: bool,
    onclick: EventHandler<MouseEvent>,
    label: Option<String>,
) -> Element {
    rsx! {
        div {
            class: "flex items-center",
            button {
                aria_checked: checked,
                class: "relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent bg-gray-200 transition-colors duration-200 ease-in-out focus:ring-2 focus:ring-blue-400 focus:ring-offset-2 focus:outline-hidden group aria-checked:bg-blue-400",
                r#type: "button",
                role: "switch",
                onclick: move |evt| onclick.call(evt),
                span {
                    aria_hidden: "true",
                    class: "pointer-events-none inline-block size-5 translate-x-0 transform rounded-full bg-white shadow-sm ring-0 transition duration-200 ease-in-out group-aria-checked:translate-x-5",
                }
            },
            if label.clone().is_some() {
                span {
                    class: "ml-3 text-sm",
                    span {
                        class: "font-medium dark:text-slate-50 text-slate-950",
                        "{label.clone().unwrap()}"
                    }
                }
            } else {
                ""
            }
        }
    }
}
