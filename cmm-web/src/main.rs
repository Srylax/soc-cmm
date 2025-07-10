use cmm_core::CMM;
// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use dioxus::prelude::dioxus_elements::FileEngine;
use dioxus_free_icons::{icons::fa_solid_icons::FaCopy, Icon};
use dioxus_storage::{LocalStorage, use_synced_storage};
use std::sync::Arc;
use dioxus_markdown::Markdown;

use crate::components::{
    ChartComponent, ControlsListComponent, OverviewComponent, SectionTitleComponent, SidebarComponent, StarButtonComponent, ToggleComponent
};

/// Define a components module that contains all shared components for our app.
mod components;
mod utils;

fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::launch(App);
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    let cmm: Signal<CMM> = use_synced_storage::<LocalStorage, _>("cmm".to_owned(), || {
        serde_json::from_str(include_str!("../../scheme-2.3.4.json")).unwrap()
    });

    let mut darkmode = use_synced_storage::<LocalStorage, _>("darkmode".to_owned(), || false);
    let mut show_percentage = use_synced_storage::<LocalStorage, _>("show_percentage".to_owned(), || true);

    let mut cmm = use_context_provider(|| cmm);

    let read_cmm_from_file = move |file_engine: Arc<dyn FileEngine>| async move {
        let files = file_engine.files();
        for file_name in &files {
            if let Some(contents) = file_engine.read_file_to_string(file_name).await {
                let simple_cmm = toml::from_str(&contents).unwrap();
                cmm.write().extend_with_simple(simple_cmm).unwrap();
            }
        }
    };
    let upload_cmm = move |evt: FormEvent| async move {
        if let Some(file_engine) = evt.files() {
            read_cmm_from_file(file_engine).await;
        }
    };

    let mut download_text = use_signal(|| "Copy");

    let copy_cmm = move |_: MouseEvent| async move {
        let file_content = toml::to_string(&cmm.read().as_simple()).unwrap();
        let cb = web_sys::window().unwrap().navigator().clipboard();
        if wasm_bindgen_futures::JsFuture::from(cb.write_text(&file_content))
            .await
            .is_ok()
        {
            *download_text.write() = "Copied ✅";
        }
    };

    // Revert to copy if cmm has changed
    use_effect(move || {
        cmm.read();
        *download_text.write() = "Copy";
    });

    use_effect(move || {
        if darkmode() {
            document::eval("document.body.classList.add('dark');");
        } else {
            document::eval("document.body.classList.remove('dark');");
        }
    });

    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    rsx! {
        // In addition to element and text (which we will see later), rsx can contain other components. In this case,
        // we are using the `document::Link` component to add a link to our favicon and main CSS file into the head of our app.
        document::Link { rel: "apple-touch-icon", sizes: "180x180", href: asset!("/assets/apple-touch-icon.png") }
        document::Link { rel: "icon", type: "image/png", sizes: "16x16", href: asset!("/assets/favicon-16x16.png") }
        document::Link { rel: "icon", type: "image/png", sizes: "32x32", href: asset!("/assets/favicon-32x32.png") }
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        document::Link { rel: "manifest", href: asset!("/assets/site.webmanifest")}
        document::Script { src: asset!("/assets/scripts/highlight-nav.js") }
        document::Script { src: "https://cdn.jsdelivr.net/npm/chart.js" }
        document::Script { src: asset!("/assets/scripts/chart.js"), defer: true }

        SidebarComponent {
            cmm: cmm,
            show_percentage: show_percentage(),
            ToggleComponent {
                checked: darkmode(),
                onclick: move |_| {
                    darkmode.set(!darkmode());
                },
                label: "Darkmode"
            },
            ToggleComponent {
                checked: show_percentage(),
                onclick: move |_| {
                    show_percentage.set(!show_percentage());
                },
                label: "Show Percentage"
            },
        },
        main {
            class: "lg:ml-[260px] px-8 py-4",
            div {
                class: "flex mt-10 mb-6",
                h1 {
                    class: "mx-auto text-6xl font-semibold",
                    "SOC CMM"
                },
            },
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
                        onchange: upload_cmm,
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
                        onclick: copy_cmm,
                        Icon {
                            width: 15,
                            height: 15,
                            fill: "white",
                            icon: FaCopy
                        },
                        "{download_text()}"
                    }
                }
            },
            if cmm().custom_description().is_some() {
                div {
                    class: "max-w-2xl mx-auto",
                    Markdown { 
                        src: cmm().custom_description().clone().unwrap() 
                    }
                }
            },
            ChartComponent {},
            OverviewComponent {
                show_percentage: show_percentage()
            },
            div {
                class: "max-w-3xl mx-auto",
                SectionTitleComponent {
                    id: "pinned",
                    text: "Pinned"
                },
                if cmm().has_pinned_items() {
                    div {
                        class: "pinned-list",
                        ControlsListComponent {
                            cmm: cmm,
                            pinned: true
                        },
                    },
                } else {
                    div {
                        class: "opacity-60",
                        key: "no-pinned-{cmm().has_pinned_items()}",
                        "No pinned items. Click",
                        div {
                            class: "inline-block ml-2 translate-y-[2px] pointer-events-none",
                            StarButtonComponent { active: false },
                        },
                        "on a control to pin it!"
                    }
                },
                div {
                    class: "mt-16",
                    ControlsListComponent {
                        cmm: cmm,
                        pinned: false
                    }
                }
            }
        }
    }
}
