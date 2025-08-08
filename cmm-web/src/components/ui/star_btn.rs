use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons::FaStar as FasStar, icons::fa_regular_icons::FaStar, Icon};

use crate::components::SmallButtonComponent;

#[component]
pub fn StarButtonComponent(
    active: bool,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    rsx! {
        SmallButtonComponent {
            onclick,
            if active {
                Icon {
                    width: 15,
                    height: 15,
                    fill: "white",
                    icon: FasStar,
                }
            } else {
                Icon {
                    width: 15,
                    height: 15,
                    fill: "white",
                    icon: FaStar,
                }
            }
        }
    }
}
