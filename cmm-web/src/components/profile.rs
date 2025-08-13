use cmm_core::profile::{ProfileQuestion, QuestionType};
use dioxus::prelude::*;

use crate::{components::{PrintBoxComponent, ValueOrPlaceholderComponent}, utils::{use_schema, use_soc_data}};

#[component]
pub fn ProfileComponent() -> Element {
    let schema = use_schema();

    rsx! {
        
        for (id, question) in schema.profile() {
            div {
                key: "{id}",
                label {
                    class: "text-sm/6 block mb-5",
                    span {
                        class: "font-semibold",
                        "{question.question()}"
                    },
                    if question.description().is_some() {
                        span {
                            class: "block text-xs/snug opacity-70",
                            dangerous_inner_html: question.description().unwrap().replace("\n", "<br>")
                        }
                    }
                    QuestionTypeHandlerComponent { id, question: question.clone() }
                }
            }
        }
    }
}




#[component]
fn QuestionTypeHandlerComponent(id: String, question: ProfileQuestion) -> Element {
    let mut data = use_soc_data();

    let mut value = use_signal(|| match data().profile_answer(id.clone()) {
        Some(value) => value.clone(),
        None => question.question_type().default_value(),
    });

    use_effect(move || {
        data.write().set_profile_answer(id.clone(), value());
    });

    let class = "outline-1 outline-slate-400 focus:outline-blue-400 focus:outline-2 rounded block py-1.5 px-3 text-base -outline-offset-1 min-h-[36px] mt-2 bg-white";

    match question.question_type() {
        QuestionType::Select { items } => {
            rsx! {
                select {
                    class: "{class} cursor-pointer",
                    onchange: move |event| {
                        value.set(event.value());
                    },
                    for item in items {
                        option {
                            selected: if item == &value() {
                                "true"
                            } else {
                                "false"
                            },
                            "{item}"
                        }
                    }
                }
            }
        }
        QuestionType::Date => {
            rsx! {
                input {
                    class: "{class} cursor-text",
                    value: value(),
                    r#type: "date",
                    onchange: move |event| {
                        value.set(event.value());
                    },
                }
            }
        }
        QuestionType::Text | QuestionType::Number => {
            rsx! {
                input {
                    class: "{class} cursor-text",
                    value: value(),
                    "data-1p-ignore": "true", // for some reason, 1p lags the whole website with this one field
                    onchange: move |event| {
                        value.set(event.value());
                    },
                    r#type: match question.question_type() {
                        QuestionType::Number => "number",
                        _ => "text",
                    }
                }
            }
        }
        QuestionType::YesNo => {
            rsx! {
                select {
                    class: "{class} cursor-pointer",
                    onchange: move |event| {
                        value.set(event.value());
                    },
                    for item in vec!["No", "Yes"] {
                        option {
                            selected: if item == &value() {
                                "true"
                            } else {
                                "false"
                            },
                            "{item}"
                        }
                    }
                }
            }
        }
    }
}
