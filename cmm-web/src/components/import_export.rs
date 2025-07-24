use cmm_core::data::SOCData;
use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons::FaCopy, Icon};
use wasm_bindgen_futures::JsFuture;

#[component]
pub fn ImportExportComponent(data: Signal<SOCData>) -> Element {
    let mut copied = use_signal(|| false);

    let upload_file_handler = move |evt: FormEvent| async move {
        if let Some(file_engine) = evt.files() {
            let files = file_engine.files();
            for file_name in &files {
                if let Some(contents) = file_engine.read_file_to_string(file_name).await {
                    data.set(toml::from_str(&contents).unwrap());
                }
            }
        }
    };

    let copy_to_clipboard = move |_: MouseEvent| async move {
        let contents = toml::to_string(&data()).unwrap();
        let clipboard = web_sys::window().unwrap().navigator().clipboard();
        if JsFuture::from(clipboard.write_text(&contents)).await.is_ok() {
            copied.set(true);
        }
    };

    // Revert to copy if data has changed
    use_effect(move || {
        data.read();
        copied.set(false);
    });

    rsx! {
        div {
            class: "bg-slate-950 text-slate-50 p-4 max-w-2xl rounded-2xl mx-auto my-10 grid grid-cols-2 gap-2 print:hidden",
            div {
                class: "border-1 p-4 rounded-2xl border-slate-700 bg-slate-900",
                label {
                    class: "text-sm mb-2 block",
                    r#for: "textreader",
                    "Upload CMM values in TOML format"
                },
                input {
                    class: "bg-slate-700 py-1 px-2 rounded cursor-pointer hover:bg-slate-600 w-full border-1 border-slate-500",
                    r#type: "file",
                    accept: ".toml",
                    multiple: false,
                    name: "textreader",
                    directory: false,
                    onchange: upload_file_handler,
                },
            },
            div {
                class: "border-1 p-4 rounded-2xl border-slate-700 bg-slate-900",
                span {
                    class: "text-sm mb-2 block",
                    "Copy the CMM as TOML file"
                },
                button {
                    class: "bg-slate-700 text-left px-2 rounded py-1 cursor-pointer hover:bg-slate-600 border-1 border-slate-500 w-full flex items-center gap-x-2",
                    onclick: copy_to_clipboard,
                    Icon {
                        width: 15,
                        height: 15,
                        fill: "white",
                        icon: FaCopy
                    },
                    if copied() {
                        "Copied ✅"
                    } else {
                        "Copy"
                    }
                }
            }
        }
    }
}
