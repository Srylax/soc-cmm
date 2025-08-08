use cmm_core::data::SOCData;
use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons::FaCopy, Icon};
use wasm_bindgen_futures::JsFuture;

use crate::utils::{use_app_settings, use_soc_compare_data, use_soc_data};

#[component]
pub fn ImportExportComponent() -> Element {
    let mut data = use_soc_data();
    let mut cmp_data = use_soc_compare_data();
    let settings = use_app_settings();

    let mut copied = use_signal(|| false);

    let upload_file_handler = async move |evt: FormEvent| -> Option<SOCData> {
        let file_engine = evt.files()?;
        let files = file_engine.files();
        let content = file_engine.read_file_to_string(files.first()?).await?;
        toml::from_str::<SOCData>(&content).ok()
    };

    let copy_to_clipboard = move |_: MouseEvent| async move {
        let contents = toml::to_string(&data()).unwrap();
        tracing::debug!("{}", contents);
        // does not work in dev mode
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
                }
                input {
                    class: "bg-slate-700 py-1 px-2 rounded cursor-pointer hover:bg-slate-600 w-full border-1 border-slate-500",
                    r#type: "file",
                    accept: ".toml",
                    multiple: false,
                    name: "textreader",
                    directory: false,
                    onchange: move |evt: FormEvent| async move {
                        if let Some(soc) = upload_file_handler(evt).await {
                            data.set(soc)
                        }
                    },
                }
            }
            div {
                class: "border-1 p-4 rounded-2xl border-slate-700 bg-slate-900",
                span {
                    class: "text-sm mb-2 block",
                    "Copy the CMM as TOML file"
                }
                button {
                    class: "bg-slate-700 text-left px-2 rounded py-1 cursor-pointer hover:bg-slate-600 border-1 border-slate-500 w-full flex items-center gap-x-2",
                    onclick: copy_to_clipboard,
                    Icon {
                        width: 15,
                        height: 15,
                        fill: "white",
                        icon: FaCopy,
                    }
                    if copied() {
                        "Copied ✅"
                    } else {
                        "Copy"
                    }
                }
            }
            if settings().show_comparison {
                div {
                    class: "border-1 p-4 rounded-2xl border-slate-700 bg-slate-900 col-span-2",
                    label {
                        class: "text-sm mb-2 block",
                        r#for: "textreader",
                        "Upload CMM values in TOML format for comparison"
                    }
                    input {
                        class: "bg-slate-700 py-1 px-2 rounded cursor-pointer hover:bg-slate-600 w-full border-1 border-slate-500",
                        r#type: "file",
                        accept: ".toml",
                        multiple: false,
                        name: "textreader",
                        directory: false,
                        onchange: move |evt: FormEvent| async move {
                            if let Some(soc) = upload_file_handler(evt).await {
                                cmp_data.set(soc)
                            }
                        },
                    }
                }
            }
        }
    }
}
