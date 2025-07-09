use dioxus::prelude::*;

#[component]
pub fn SectionTitleComponent(
    text: String,
    id: String
) -> Element {
    rsx! {
        h2 {
            class: "text-3xl mb-4",
            id,
            "{text}"
        },
    }
}
