use dioxus::prelude::*;
use crate::utils::round;

#[component]
pub fn ProgressBarComponent(
    max: f64,
    value: f64,
    height: Option<u32>,
    class: Option<String>,
    tooltip: Option<String>
) -> Element {
    let classes = class.unwrap_or(String::from("bg-blue-400"));
    rsx! {
        div {
            // Tailwind: h-1 h-2 h-3 h-4
            class: "h-{height.unwrap_or(1)} w-full rounded block relative dark:bg-blue-100 bg-gray-300 group",
            class: if tooltip.is_some() { "hover:outline-2 outline-slate-200 cursor-pointer" },
            div {
                class: "h-full absolute left-0 top-0 rounded-l {classes}",
                class: if value >= max { "rounded-r" },
                width: "{round(value / max * 100.0, 0)}%",
            },
            if tooltip.is_some() {
                div {
                    class: "absolute not-group-hover:hidden left-1/2 -top-[8px] -translate-y-full bg-slate-100 border-1 border-slate-400 py-1.5 px-2 rounded -translate-x-1/2 shadow mb-2 text-xs w-auto text-slate-900",
                    "{tooltip.clone().unwrap()}"
                },
                div {
                    // CSS triangle
                    class: "absolute not-group-hover:hidden left-1/2 top-0 -translate-y-full -translate-x-1/2 shadow border-solid border-t-slate-300 border-t-8 border-x-transparent border-x-8 border-b-0 opacity-50",
    
                }
            }
        }
    }
}

#[component]
pub fn BadToGoodProgressBarComponent(
    max: f64,
    value: f64,
    height: Option<u32>,
    tooltip_prefix: Option<String>
) -> Element {
    let percentage = value / max * 100.0;
    let class = if percentage < 12.5 {
        "bg-red-600"
    } else if percentage < 25.0 {
        "bg-red-500"
    } else if percentage < 37.5 {
        "bg-orange-500"
    } else if percentage < 50.0 {
        "bg-orange-400"
    } else if percentage < 62.5 {
        "bg-yellow-400"
    } else if percentage < 75.0 {
        "bg-yellow-300"
    } else if percentage < 87.5 {
        "bg-green-400"
    } else {
        "bg-green-500"
    };
    rsx! {
        ProgressBarComponent {
            max,
            value,
            height,
            class,
            tooltip: if tooltip_prefix.is_some() { 
                "{tooltip_prefix.unwrap()} {round(percentage, 2)}%" 
            } else { 
                "{round(percentage, 2)}%" 
            },
        }
    }
}
