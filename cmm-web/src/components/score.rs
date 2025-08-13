use cmm_core::score::Score;
use dioxus::prelude::*;

use crate::utils::{round, use_app_settings};

#[component]
pub fn CompletenessScoreComponent(score: Score) -> Element {
    let label = if score.score().is_nan() || score.score() == 0.0 {
        "Incomplete"
    } else if score.as_percentage() < 35.0 {
        "Partially complete"
    } else if score.as_percentage() < 70.0 {
        "Averagely complete"
    } else if score.score() == score.max() {
        "Fully complete"
    } else {
        "Mostly complete"
    };

    rsx!{
        "{label}"
    }
}

#[component]
pub fn ScoreComponent(score: Option<Score>, precision: u32, replace_nan: Option<bool>) -> Element {
    let settings = use_app_settings();

    let Some(scr) = score else {
        return rsx!();
    };

    if replace_nan.is_some_and(|x| x) && round(scr.score(), precision).is_nan() {
        return rsx! {
            "0",
            if settings().show_percentage { "%" }
        };
    }
    
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
    rsx!(
        ScoreComponent {
            score,
            precision: 1,
        }
    )
}
