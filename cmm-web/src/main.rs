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
                href: asset!("/assets/apple-touch-icon.png"),
            }
            document::Link {
                rel: "icon",
                r#type: "image/png",
                sizes: "16x16",
                href: asset!("/assets/favicon-16x16.png"),
            }
            document::Link {
                rel: "icon",
                r#type: "image/png",
                sizes: "32x32",
                href: asset!("/assets/favicon-32x32.png"),
            }
            document::Link {
                rel: "icon",
                href: asset!("/assets/favicon.ico"),
            }
            document::Link {
                rel: "stylesheet",
                href: asset!("/assets/tailwind.css"),
            }
            document::Link {
                rel: "manifest",
                href: asset!("/assets/site.webmanifest"),
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
