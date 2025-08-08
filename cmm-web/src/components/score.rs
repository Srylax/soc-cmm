use cmm_core::score::Score;
use dioxus::prelude::*;

use crate::utils::{round, use_app_settings};

#[component]
pub fn ScoreComponent(score: Option<Score>, precision: u32) -> Element {
    let settings = use_app_settings();

    let Some(scr) = score else {
        return rsx!();
    };
    
    rsx! {
        if settings().show_percentage {
            "{round(scr.as_percentage(), precision)}%"
        } else {
            "{round(scr.score(), precision)}"
        }
    }
}


#[component]
pub fn SidebarScoreComponent(score: Option<Score>) -> Element {
    let settings = use_app_settings();

    if !settings().show_scores {
        return rsx!();
    }
    rsx!( ScoreComponent { score, precision: 1 })
}
