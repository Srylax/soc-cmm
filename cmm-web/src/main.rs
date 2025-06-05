use cmm_core::{Domain, CMM};
// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use dioxus::prelude::dioxus_elements::FileEngine;
use dioxus_storage::{use_synced_storage, LocalStorage};
use std::sync::Arc;
use strum::VariantArray;

use crate::components::{ControlComponent, SidebarComponent};

/// Define a components module that contains all shared components for our app.
mod components;

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

        SidebarComponent {
            cmm: cmm,
            div {
                label {
                    class: "text-sm mb-2 block",
                    r#for: "textreader",
                    "Upload CMM values in toml format"
                },
                input {
                    class: "bg-slate-800 p-1 rounded w-full",
                    r#type: "file",
                    accept: ".toml",
                    multiple: false,
                    name: "textreader",
                    directory: false,
                    onchange: upload_cmm,
                }
            },
        },
        main {
            class: "ml-[260px] px-8 py-4",

            div {
                class: "max-w-3xl mx-auto",
                for domain in Domain::VARIANTS {
                    h2 {
                        class: "text-3xl mb-2 mt-6 font-semibold",
                        id: "variant-{domain}",
                        "{domain}"
                    },
                    for (i, aspect) in cmm.read().aspect(&domain).unwrap().into_iter().enumerate() {
                        h3 {
                            class: "text-2xl mb-2 mt-6 font-semibold",
                            id: "aspect-{domain}-{i + 1}",
                            "{i + 1}. {aspect.title()}"
                        }
                        div {
                            class: "",
                            for (cid,control) in aspect.controls() {
                                ControlComponent { key: cid.to_owned() + control.answer().as_value(), domain: *domain, cid: cid.to_owned(), control: control.clone()}
                            }
                        }
                    }
                }
            }
        }
    }
}
