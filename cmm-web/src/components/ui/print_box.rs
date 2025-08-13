use dioxus::prelude::*;

#[component]
pub fn PrintBoxComponent(label: Element, children: Element) -> Element {
    rsx! {
        div {
            class: "rounded-xl border-slate-400 border-1",
            div {
                class: "bg-slate-100 rounded-t-xl border-b-1 border-slate-400 text-slate-700 font-semibold text-[11px] px-3 py-2",
                {label}
            }
            div {
                class: "p-3",
                {children}
            }
        }
    }
}

