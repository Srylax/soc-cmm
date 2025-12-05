use crate::{
    layouts::{DataSchemaLayout, SettingsLayout},
    pages::{App, Report},
};
use dioxus::prelude::*;

mod components;
mod layouts;
mod pages;
mod utils;

fn main() {
    dioxus::launch(|| {
        rsx! {
            document::Link {
                rel: "apple-touch-icon",
                sizes: "180x180",
                href: asset!("/assets/apple-touch-icon.png", ImageAssetOptions::new().with_avif()),
            }
            document::Link {
                rel: "icon",
                r#type: "image/png",
                sizes: "16x16",
                href: asset!("/assets/favicon-16x16.png", ImageAssetOptions::new().with_avif()),
            }
            document::Link {
                rel: "icon",
                r#type: "image/png",
                sizes: "32x32",
                href: asset!("/assets/favicon-32x32.png", ImageAssetOptions::new().with_avif()),
            }
            document::Link {
                rel: "icon",
                href: asset!("/assets/favicon.ico", ImageAssetOptions::new().with_avif()),
            }
            document::Link {
                rel: "stylesheet",
                href: asset!("/assets/tailwind.css", CssAssetOptions::new().with_preload(true)),
            }
            document::Link {
                rel: "manifest",
                href: asset!("/assets/site.webmanifest"),
            }
            document::Script {
                src: asset!("/assets/scripts/scroll.js", JsAssetOptions::new().with_preload(true)),
            }

            Router::<Route> {

            }
        }
    });
}

#[derive(Routable, Debug, PartialEq, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(SettingsLayout)]
        #[layout(DataSchemaLayout)]
            #[route("/")]
            App {},

            #[route("/report")]
            Report {},
}
