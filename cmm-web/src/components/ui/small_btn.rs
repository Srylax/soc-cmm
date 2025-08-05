use dioxus::prelude::*;

#[component]
pub fn SmallButtonComponent(
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element
) -> Element {
    rsx! {
        button {
            class: "flex items-center bg-slate-400 dark:bg-slate-600 text-white py-1 px-2 rounded text-xs cursor-pointer border-slate-500 border-1",
            class: if onclick.is_some() { "hover:bg-blue-500 hover:border-blue-400" },
            onclick: move |evt| {
                if onclick.is_some() {
                    onclick.unwrap().call(evt)
                }
            },
            {children}
        },
    }
}
