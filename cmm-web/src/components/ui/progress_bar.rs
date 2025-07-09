use dioxus::prelude::*;
use crate::utils::round;

#[component]
pub fn ProgressBarComponent(
    max: f64,
    value: f64,
    height: Option<u32>,
    class: Option<String>,
) -> Element {
    let classes = class.unwrap_or(String::from("bg-blue-400"));
    rsx! {
        div {
            // Tailwind: h-1 h-2 h-3 h-4
            class: "h-{height.unwrap_or(1)} w-full rounded block relative dark:bg-blue-100 bg-gray-300",
            div {
                class: "h-full absolute left-0 top-0 rounded-l {classes}",
                class: if value >= max { "rounded-r" },
                width: "{round(value / max * 100.0, 0)}%",
            }
        }
    }
}

#[component]
pub fn BadToGoodProgressBarComponent(
    max: f64,
    value: f64,
    height: Option<u32>,
) -> Element {
    let percentage = value / max * 100.0;
    let class = if percentage < 25.0 {
        "bg-red-500"
    } else if percentage < 50.0 {
        "bg-orange-400"
    } else if percentage < 90.0 {
        "bg-yellow-400"
    } else {
        "bg-green-500"
    };
    rsx! {
        ProgressBarComponent {
            max,
            value,
            height,
            class,
        }
    }
}
