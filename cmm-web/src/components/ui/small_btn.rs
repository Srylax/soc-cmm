use dioxus::prelude::*;

#[component]
pub fn SmallButtonComponent(
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element
) -> Element {
    rsx! {
        button {
            class: "flex items-center bg-slate-300 dark:bg-slate-600 text-white py-1 px-2 mr-2 rounded text-xs cursor-pointer",
            class: if onclick.is_some() { "hover:bg-blue-300" },
            onclick: move |evt| {
                if onclick.is_some() {
                    onclick.unwrap().call(evt)
                }
            },
            {children}
        },
    }
}
