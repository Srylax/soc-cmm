use cmm_core::{answer::Answer, cid::{Domain, CID}, control::Control, schema::ControlSchema};
use dioxus::prelude::*;
use indexmap::IndexMap;
use strum::VariantArray;

use crate::{components::{SmallButtonComponent, StarButtonComponent}, utils::{use_schema, use_soc_data}};

#[component]
pub fn ControlsListComponent(pinned: bool) -> Element {
    let data = use_soc_data();
    let schema = use_schema();

    let indent_list = |controls: &IndexMap<CID, Control>| -> Vec<Vec<(CID, Control)>> {
        let mut output: Vec<Vec<(CID, Control)>> = vec![];
        let mut current_list: Vec<(CID, Control)> = vec![];
        let mut current_indent: usize = 0;
        for (cid, control) in controls {
            // TODO: use the array?
            let indent = cid.to_string().chars().filter(|c| *c == '.').count();
            if indent != current_indent {
                current_indent = indent;
                output.push(current_list.clone());
                current_list.clear();
            }
            current_list.push((*cid, control.clone()));
        }
        output.push(current_list.clone());
        output
    };

    rsx! {
        for domain in Domain::VARIANTS {
            if !pinned {
                h3 {
                    class: "text-3xl mb-2 mt-6 font-semibold",
                    id: "variant-{domain}",
                    "{domain}"
                },
            }
            div {
                for (i, aspect) in schema.aspects(domain).iter().enumerate() {
                    if !pinned {
                        h4 {
                            class: "text-2xl mb-2 mt-6 font-semibold",
                            id: "aspect-{domain}-{i + 1}",
                            "{i + 1}. {aspect}"
                        }
                    }
                    div {
                        class: "",
                        // for indent_items in indent_list(data().controls_by_aspect(domain, i)) {
                        //     if indent_items.len() > 0 {
                        //         div {
                        //             for (cid, control) in indent_items {
                        //                 if (pinned && control.bookmark()) || !pinned {
                        //                     ControlItemComponent {
                        //                         key: format!("{cid}{domain}"),
                        //                         domain: *domain,
                        //                         cid: cid.to_owned(),
                        //                         control: control.clone(),
                        //                         pinned
                        //                     }
                        //                 }
                        //             }
                        //         }
                        //     }
                        // }
                    }
                }
            }
        }
    }
}

#[component]
fn ControlItemComponent(
    domain: Domain,
    cid: ReadOnlySignal<CID>,
    control: ReadOnlySignal<Control>,
    pinned: bool,
) -> Element {
    let data = use_soc_data();
    let schema = use_schema();

    let ctrl_schema = schema.control_schema(&cid()).unwrap();
    
    let indent = cid().to_string().chars().filter(|c| *c == '.').count();
    if let Answer::Title = control().answer() {
        if indent > 1 {
            return rsx! {
                h6 {
                    class: "mt-4 mb-1 text-xl font-semibold",
                    id: "{domain}.{cid}",
                    "{cid} {ctrl_schema.title()}"
                }
            };
        }
        return rsx! {
            h5 {
                class: "mt-4 mb-1 text-xl font-semibold",
                id: "{domain}.{cid}",
                "{cid} {ctrl_schema.title()}"
            }
        };
    }

    rsx! {
        div {
            class: "indent-{indent} pt-1 pb-0.5",
            tabindex: "-1",
            details {
                class: "dark:bg-slate-800 bg-slate-100 border-1 dark:border-slate-700 border-slate-300 open:p-3 rounded dark:text-slate-50 text-slate-950 dark:not-open:hover:bg-slate-700 not-open:hover:bg-slate-200 duration-100ms ease-in-out group group/details",
                id: "{domain}.{cid}",
                summary {
                    class: "not-in-open:p-3 cursor-pointer flex justify-between w-full",
                    span {
                        if pinned {
                            "{domain} > "
                        },
                        span {
                            class: "opacity-70 mr-2",
                            "{cid}"
                        },
                        "{ctrl_schema.title()}"
                    },
                    div {
                        class: if !control().bookmark() { "bookmark-button" },
                        div {
                            class: "flex",
                            StarButtonComponent {
                                onclick: move |_| {
                                    data().toggle_bookmark(&cid());
                                },
                                active: control().bookmark()
                            },
                            ControlItemValuePreviewComponent {
                                domain,
                                cid,
                                control
                            }
                        }
                    }
                },
                div {
                    class: "grid gap-2 mt-4 grid-cols-[3fr_2fr]",
                    span { },
                    span {
                        class: "text-sm",
                        "Comment",
                    },
                },
                div {
                    class: "grid gap-2 mt-1 grid-cols-[3fr_2fr]",
                    ControlInputComponent {
                        domain,
                        cid,
                        control,
                        control_schema: ctrl_schema.clone(),
                        pinned
                    },
                    label {
                        class: "min-h-2xl flex flex-wrap",
                        textarea {
                            class: "dark:bg-slate-700 bg-slate-200 not-dark:border-1 not-dark:border-slate-300 rounded px-2 py-1.5 w-full",
                            value: control().comment().clone().unwrap_or(String::new()),
                            onchange: move |evt| {
                                data().set_comment(&cid(), Some(evt.value()));
                            }
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn ControlInputComponent(
    domain: Domain,
    cid: ReadOnlySignal<CID>,
    control: ReadOnlySignal<Control>,
    control_schema: ControlSchema,
    pinned: bool
) -> Element {
    let data = use_soc_data();

    if let Answer::Any(content) = control().answer() {
        return rsx! {
            div {
                input {
                    class: "dark:bg-slate-700 bg-slate-200 not-dark:border-1 not-dark:border-slate-300 rounded px-2 py-1.5 w-full block",
                    type: "text",
                    value: "{content}",
                    oninput: move |evt| {
                        data().set_answer(
                            &cid(), 
                            Answer::Any(evt.value())
                        );
                    }
                }
            }
        };
    }

    if let Answer::Bool(content) = control().answer() {
        return rsx! {
            div {
                class: "w-full flex items-baseline gap-x-2",
                for value in vec!["True", "False"] {
                    label {
                        key: format!(
                            "{}{}", 
                            cid, 
                            control().answer().as_value()
                        ),
                        class: "dark:bg-slate-700 bg-slate-200 py-1 px-2 rounded cursor-pointer dark:hover:bg-slate-600 hover:bg-slate-300 has-checked:bg-slate-200 dark:has-checked:bg-slate-600 has-checked:border-blue-300 border-3 border-transparent w-full",
                        input {
                            class: "appearance-none opacity-0",
                            tabindex: "0",
                            type: "radio",
                            name:  "{domain}.{&cid}.{pinned}",
                            checked: content == &(value == "True"),
                            onclick: move |_| {
                                data().set_answer(
                                    &cid(), 
                                    Answer::Bool(value == "True")
                                );
                            }
                        },
                        "{value}"
                    }
                }
            }
        };
    }

    rsx! {
        div {
            class: "grid gap-2",
            for (i, variant) in control().answer().variants().iter().enumerate() {
                label {
                    key: format!(
                        "{}{}{}",
                        cid, 
                        control().answer().as_value(), 
                        i
                    ),
                    class: "dark:bg-slate-700 bg-slate-200 py-1 px-2 rounded cursor-pointer dark:hover:bg-slate-600 hover:bg-slate-300 has-checked:bg-slate-200 dark:has-checked:bg-slate-600 has-checked:border-blue-400 border-l-4 border-transparent has-focus:outline-2  has-focus:outline-blue-400 not-dark:not-has-focus:outline-1 not-dark:not-has-focus:outline-slate-300",
                    "data-description": control_schema
                                            .guidances()
                                            .get(i)
                                            .cloned()
                                            .unwrap_or(String::new()),
                    input {
                        class: "appearance-none opacity-0",
                        tabindex: "0",
                        type: "radio",
                        name:  "{domain}.{cid.clone()}.{pinned}",
                        value: variant.to_owned(),
                        checked: control().answer().variant_eq(variant),
                        onclick: move |_evt| {
                            data().set_answer(
                                &cid(), 
                                control()
                                    .answer()
                                    .extend_from_variant(variant)
                                    .unwrap()
                                );
                        }
                    }
                    "{variant}"
                }
            }
        }
    }
}

#[component]
fn ControlItemValuePreviewComponent(
    domain: Domain,
    cid: ReadOnlySignal<CID>,
    control: ReadOnlySignal<Control>,
) -> Element {
    let data = use_soc_data();

    let Answer::Bool(_) = control().answer() else {
        return rsx! {
            SmallButtonComponent {
                "{control().answer()}"
            }
        };
    };

    rsx! {
        SmallButtonComponent {
            onclick: move |evt: MouseEvent| {
                evt.prevent_default();
                data()
                    .set_answer(
                        &cid(), 
                        Answer::Bool(
                            control()
                            .answer()
                            .eq(&Answer::Bool(false)))
                    );
            },
            "{control().answer()}"
        }
    }
}
