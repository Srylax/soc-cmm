use dioxus::prelude::*;

#[component]
pub fn ButtonComponent(
    onclick: Option<EventHandler<MouseEvent>>,
    additional_class: Option<String>,
    children: Element
) -> Element {
    rsx! {
        button {
            class: "border-slate-200 dark:border-slate-700 bg-white dark:bg-slate-800 rounded-lg px-3 py-2 border-[1px] flex items-center font-semibold gap-2 cursor-pointer hover:bg-slate-100 dark:hover:bg-slate-700 dark:hover:border-slate-600 shadow",
            class: "{additional_class.clone().unwrap_or_default()}",
            onclick: move |evt| {
                if onclick.is_some() {
                    onclick.unwrap().call(evt)
                }
            },
            {children}
        },
    }
}
