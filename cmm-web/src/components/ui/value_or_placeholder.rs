use dioxus::prelude::*;

#[component]
pub fn ValueOrPlaceholderComponent(value: String) -> Element {
    if value.is_empty() {
        return rsx! {
            span {
                class: "opacity-70 italic",
                "<empty>"
            }
        };
    }
    rsx! { "{value}" }
}
