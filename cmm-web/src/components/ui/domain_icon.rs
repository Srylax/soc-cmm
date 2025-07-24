use cmm_core::cid::Domain;
use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::fa_solid_icons::{
        FaBriefcase,
        FaUsers,
        FaBarsProgress,
        FaWrench,
        FaComputer
    }, Icon};



#[component]
pub fn DomainIconComponent(
    domain: Domain,
    width: u32,
    height: u32,
    fill: String
) -> Element {
    match domain {
        Domain::Business => {
            rsx!{
                Icon {
                    width,
                    height,
                    fill,
                    icon: FaBriefcase
                }
            }
        },
        Domain::People => {
            rsx!{
                Icon {
                    width,
                    height,
                    fill,
                    icon: FaUsers
                }
            }
        },
        Domain::Process => {
            rsx!{
                Icon {
                    width,
                    height,
                    fill,
                    icon: FaBarsProgress
                }
            }
        },
        Domain::Services => {
            rsx!{
                Icon {
                    width,
                    height,
                    fill,
                    icon: FaWrench
                }
            }
        },
        Domain::Technology => {
            rsx!{
                Icon {
                    width,
                    height,
                    fill,
                    icon: FaComputer
                }
            }
        },
    }
}
