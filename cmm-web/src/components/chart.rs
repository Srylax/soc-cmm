use cmm_core::CMM;
use dioxus::prelude::*;

#[component]
pub fn ChartComponent() -> Element {
    let cmm = use_context::<Signal<CMM>>();

    use_effect(move || {
        // this line is required, else the use effect wont update
        let _ = cmm.read();
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
                },
            },
        },
    }
}
