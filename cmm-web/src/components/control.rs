use cmm_core::{answer::Answer, cid::{Domain, CID}, control::Control, schema::ControlSchema};
use dioxus::prelude::*;
use strum::VariantArray;
use dioxus_free_icons::{icons::fa_solid_icons::{FaArrowRightLong, FaCircleInfo}, Icon};

use crate::{components::{DomainIconComponent, SmallButtonComponent, StarButtonComponent}, utils::{use_app_settings, use_schema, use_soc_compare_data, use_soc_data}};

#[component]
pub fn ControlsListComponent(pinned: bool) -> Element {
    let schema = use_schema();
    let data = use_soc_data();

    let indent_list = |controls: Vec<(&CID, &ControlSchema)>| -> Vec<Vec<(CID, ControlSchema)>> {
        let mut output: Vec<Vec<(CID, ControlSchema)>> = vec![];
        let mut current_list: Vec<(CID, ControlSchema)> = vec![];
        let mut current_indent: usize = 0;
        for (cid, control) in controls {
            let indent = cid.indent();
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
                    class: "text-3xl mb-2 mt-6 font-semibold flex items-center gap-2",
                    id: "variant-{domain}",
                    DomainIconComponent {
                        domain: domain.clone(),
                        width: 24,
                        height: 24,
                        fill: "currentColor",
                    }
                    "{domain}"
                }
            }
            div {

                for (i , aspect) in schema.aspects(domain).iter().enumerate() {
                    if !pinned {
                        h4 {
                            class: "text-2xl mb-2 mt-6 font-semibold",
                            id: "aspect-{domain}-{i + 1}",
                            "{i + 1}. {aspect}"
                        }
                    }
                    div {

                        for indent_items in indent_list(schema.controls_by_aspect(&domain, i as u8 + 1).collect()) {
                            if indent_items.len() > 0 {
                                div {

                                    for (cid , _) in indent_items {
                                        ControlItemComponent {
                                            key: "{cid}_{pinned}",
                                            cid: cid.to_owned(),
                                            control_option: data().control(&cid).cloned(),
                                            pinned,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ControlItemComponent(
    cid: CID,
    pinned: bool,
    // we need to pass this for reactivity to work as intented
    control_option: Option<Control>
) -> Element {
    let mut data = use_soc_data();
    let schema = use_schema();
    let compare_data = use_soc_compare_data();
    let settings = use_app_settings();

    let ctrl_schema = schema.control_schema(&cid).unwrap();

    let indent = cid.indent() - 1;

    let Some(control) = control_option else {
        if pinned {
            return rsx!();
        }
        if indent > 1 {
            return rsx! {
                h6 {
                    class: "mt-4 mb-1 text-lg font-semibold",
                    id: "{cid}",
                    "{cid.as_short_string()} {ctrl_schema.title()}"
                }
            };
        }
        return rsx! {
            h5 {
                class: "mt-4 mb-1 text-xl font-semibold",
                id: "{cid}",
                "{cid.as_short_string()} {ctrl_schema.title()}"
            }
        };
    };

    if pinned && !control.bookmark() {
        return rsx!();
    }

    if !control.answer().control_type_eq(ctrl_schema.control_type()) {
        return rsx!{
            div {
                class: "bg-red-500 text-white",
                "Field type mismatch!"
            }
        }
    }

    let show_comparison = |cid: &CID| -> bool {
        let Some(cmp_ctrl) = compare_data().control(&cid).cloned() else {
            return false;
        };
        cmp_ctrl.answer().ne(control.answer())
    };

    rsx! {
        div {
            class: "indent-{indent} pt-1 pb-0.5 relative",
            tabindex: "-1",
            details {
                class: "dark:bg-slate-800 bg-slate-100 border-1 dark:border-slate-700 border-slate-300 open:p-3 rounded dark:text-slate-50 text-slate-950 dark:not-open:hover:bg-slate-700 dark:not-open:hover:border-slate-600 not-open:hover:bg-slate-200 duration-100ms ease-in-out group group/details",
                id: "{cid}",
                summary {
                    class: "not-in-open:p-3 cursor-pointer flex justify-between w-full",
                    span {

                        if pinned {
                            "{cid.domain()} > "
                        }
                        span {
                            class: "opacity-70 mr-2",
                            "{cid.as_short_string()}"
                        }
                        "{ctrl_schema.title()}"
                    }
                    div {
                        class: if !control.bookmark() { "bookmark-button" },
                        div {
                            key: "{cid}_{control.bookmark()}_{control.answer()}",
                            class: "flex gap-2 items-center",
                            StarButtonComponent {
                                onclick: move |_| {
                                    data.write().toggle_bookmark(&cid);
                                },
                                active: control.bookmark(),
                            }
                            if show_comparison(&cid) && settings().show_comparison {
                                span {
                                    class: "opacity-60",
                                    SmallButtonComponent {

                                        ValueOrPlaceholderComponent {
                                            value: "{compare_data().control(&cid).unwrap().answer()}",
                                        }
                                    }
                                }
                                Icon {
                                    class: "opacity-60",
                                    icon: FaArrowRightLong,
                                    width: 18,
                                    height: 18,
                                }
                            }
                            ControlItemValuePreviewComponent {
                                cid,
                                control: control.clone(),
                            }
                        }
                    }
                }
                if ctrl_schema.remarks().is_some() {
                    div {
                        class: "mt-4 opacity-90 rounded py-2 px-3 dark:bg-slate-900 bg-slate-200 flex gap-x-2",
                        div {
                            class: "mt-0.5 block",
                            Icon {
                                height: 18,
                                width: 18,
                                icon: FaCircleInfo,
                            }
                        }
                        "{ctrl_schema.remarks().clone().unwrap()}"
                    }
                }
                div {
                    class: "grid gap-2 mt-4 grid-cols-[3fr_2fr]",
                    span {

                    }
                    span {
                        class: "text-sm",
                        "Comment"
                    }
                }
                div {
                    class: "grid gap-2 mt-1 grid-cols-[3fr_2fr]",
                    ControlInputComponent {
                        key: "{cid}{control.answer()}",
                        cid,
                        control: control.clone(),
                        control_schema: ctrl_schema.clone(),
                        pinned,
                    }
                    label {
                        class: "min-h-2xl flex flex-wrap",
                        textarea {
                            class: "dark:bg-slate-700 bg-slate-200 not-dark:border-1 not-dark:border-slate-300 rounded px-2 py-1.5 w-full",
                            value: control.comment().clone().unwrap_or(String::new()),
                            onchange: move |evt| {
                                data.write().set_comment(&cid, Some(evt.value()));
                            },
                        }
                    }
                }
            }
            if show_comparison(&cid) && settings().show_comparison {
                div {
                    class: "absolute h-full pt-1 pb-0.5 w-1 flex items-center -right-2 top-0 translate-x-full",
                    span {
                        class: "w-2 h-full bg-blue-500 rounded-xs",
                    }
                }
            }
        }
    }
}


#[component]
fn ValueOrPlaceholderComponent(
    value: String
) -> Element {
    if value.is_empty() {
        return rsx!{
            span {
                class: "opacity-70 italic",
                "<empty>"
            }
        }
    }
    rsx!{ "{value}" }
}

#[component]
fn ControlInputComponent(
    cid: CID,
    control: ReadOnlySignal<Control>,
    control_schema: ControlSchema,
    pinned: bool
) -> Element {
    let mut data = use_soc_data();

    if let Answer::Any(content) = control().answer() {
        return rsx! {
            div {

                input {
                    class: "dark:bg-slate-700 bg-slate-200 not-dark:border-1 not-dark:border-slate-300 rounded px-2 py-1.5 w-full block",
                    r#type: "text",
                    value: "{content}",
                    oninput: move |evt| {
                        data.write().set_answer(&cid, Answer::Any(evt.value()));
                    },
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
                        key: "{cid}{control().answer()}",
                        class: "dark:bg-slate-700 bg-slate-200 py-1 px-2 rounded cursor-pointer dark:hover:bg-slate-600 hover:bg-slate-300 has-checked:bg-slate-200 dark:has-checked:bg-slate-600 has-checked:border-blue-300 border-3 border-transparent w-full",
                        input {
                            class: "appearance-none opacity-0",
                            tabindex: "0",
                            r#type: "radio",
                            name: "{cid}.{pinned}",
                            checked: content == &(value == "True"),
                            onclick: move |_| {
                                data.write().set_answer(&cid, Answer::Bool(value == "True"));
                            },
                        }
                        "{value}"
                    }
                }
            }
        };
    }

    rsx! {
        div {
            class: "grid gap-2",
            for (i , variant) in control().answer().variants().iter().enumerate() {
                label {
                    key: "{cid}_{control().answer()}_{i}",
                    class: "dark:bg-slate-700 bg-slate-200 py-1 px-2 rounded cursor-pointer dark:hover:bg-slate-600 hover:bg-slate-300 has-checked:bg-slate-200 dark:has-checked:bg-slate-600 has-checked:border-blue-400 border-l-4 border-transparent has-focus:outline-2  has-focus:outline-blue-400 not-dark:not-has-focus:outline-1 not-dark:not-has-focus:outline-slate-300",
                    "data-description": control_schema.guidances().get(i).cloned().unwrap_or(String::new()),
                    input {
                        class: "appearance-none opacity-0",
                        tabindex: "0",
                        r#type: "radio",
                        name: "{cid}.{pinned}",
                        value: variant.to_owned(),
                        checked: control().answer().variant_eq(variant),
                        onclick: move |_evt| {
                            data.write()
                                .set_answer(&cid, control().answer().extend_from_variant(variant).unwrap());
                        },
                    }
                    "{variant}"
                }
            }
        }
    }
}

#[component]
fn ControlItemValuePreviewComponent(
    cid: CID,
    control: ReadOnlySignal<Control>,
) -> Element {
    let mut data = use_soc_data();

    let Answer::Bool(_) = control().answer() else {
        return rsx! {
            SmallButtonComponent {

                ValueOrPlaceholderComponent {
                    value: "{control().answer()}",
                }
            }
        };
    };

    rsx! {
        SmallButtonComponent {
            onclick: move |evt: MouseEvent| {
                evt.prevent_default();
                data.write()
                    .set_answer(&cid, Answer::Bool(control().answer().eq(&Answer::Bool(false))));
            },
            "{control().answer()}"
        }
    }
}
