use dioxus::prelude::*;

use crate::utils::{use_app_settings, use_soc_data};

#[component]
pub fn ChartComponent() -> Element {
    let data = use_soc_data();
    let settings = use_app_settings();

    use_effect(move || {
        // these lines are required, else the use effect wont update
        data.read();
        settings.read();
        document::eval(
            r#"
            const event = new Event("updateChart");
            document.dispatchEvent(event);
            "#,
        );
    });

    rsx! {
        div {
            class: "w-full max-w-4xl mx-auto print:hidden",
            div {
                class: "w-full h-[500px] bg-slate-200 py-4 rounded",
                canvas {
                    class: "rounded",
                }
            }
        }
    }
}
