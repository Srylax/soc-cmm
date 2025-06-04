use cmm_core::{Domain, CMM};
// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use components::{ControlComponent, Hero};
use strum::VariantArray;
use std::sync::Arc;
use dioxus::{prelude::dioxus_elements::FileEngine};

/// Define a components module that contains all shared components for our app.
mod components;

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundled smaller
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

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
    let mut cmm: Signal<CMM> = use_signal(||serde_json::from_str(include_str!("../../scheme-2.3.4.json")).unwrap());

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
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }


        div {
            label { r#for: "textreader", "Upload CMM values in toml format" }
            input {
                r#type: "file",
                accept: ".toml",
                multiple: false,
                name: "textreader",
                directory: false,
                onchange: upload_cmm,
            }
        }

        for domain in Domain::VARIANTS {
            h2 {
                class: "text-3xl mb-2",
                "{domain}"
            },
            for aspect in cmm.read().aspect(&domain).unwrap() {
                for (cid,control) in aspect.controls() {
                    ControlComponent { key: cid.to_owned(), domain: *domain, cid: cid.to_owned(), control: control.clone()} {}
                }
            }
        }
    }
}
