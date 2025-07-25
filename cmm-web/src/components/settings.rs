use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::ToggleComponent;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AppSettings {
    pub darkmode: bool,
    pub show_percentage: bool,
    pub show_scores: bool,
}

#[component]
pub fn SettingsComponent(settings: Signal<AppSettings>) -> Element {
    use_effect(move || {
        if settings().darkmode {
            document::eval("document.body.classList.add('dark');");
        } else {
            document::eval("document.body.classList.remove('dark');");
        }
    });

    rsx! {
        ToggleComponent {
            checked: settings().darkmode,
            onclick: move |_| {
                settings.write().darkmode = !settings().darkmode;
            },
            label: "Darkmode"
        },
        ToggleComponent {
            checked: settings().show_scores,
            onclick: move |_| {
                settings.write().show_scores = !settings().show_scores;
            },
            label: "Show Scores"
        },
        ToggleComponent {
            checked: settings().show_percentage,
            onclick: move |_| {
                settings.write().show_percentage = !settings().show_percentage;
            },
            label: "Show Percentage"
        },
    }
}