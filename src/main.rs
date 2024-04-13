#![allow(non_snake_case)]

pub mod face_data;

use std::convert::identity;

use dioxus::prelude::*;
use log::LevelFilter;

use crate::face_data::{get_face_color, is_face_too_long, FaceData};

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    const STYLE_ASSET: &'static str = include_str!("./main.css");
    let mut face_data = use_signal(|| FaceData::load_from_registry());
    let ok_face_data = use_memo(move || face_data().unwrap_or_default());

    rsx! {
        style { dangerous_inner_html: STYLE_ASSET }
        match face_data() {
            Ok(face) => rsx! {FaceEditor {
                face: face,
                on_text_input: move |new_text| {
                    let new_face = FaceData {
                        text: new_text,
                        ..ok_face_data()
                    };
                    face_data.set(Ok(new_face));
                },
                on_color_change: move |new_color_index| {
                    let new_face = FaceData {
                        color_index: new_color_index as u32,
                        ..ok_face_data()
                    };
                    face_data.set(Ok(new_face));
                },
                on_save: move |_| {
                    // TODO: Better error handling
                    ok_face_data().save_to_registry().expect("Failed to save face data");
                },
                on_load: move |_| {
                    face_data.set(FaceData::load_from_registry());
                }
            }},
            Err(e) => rsx! {
                div { "Failed to load the face {e}" }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct FaceEditorProps {
    face: FaceData,
    on_text_input: EventHandler<String>,
    on_color_change: EventHandler<u8>,
    on_save: EventHandler<()>,
    on_load: EventHandler<()>,
}

#[component]
fn FaceEditor(props: FaceEditorProps) -> Element {
    let face_text = use_memo(use_reactive(&props.face.text, identity));
    let face_too_long = use_memo(move || is_face_too_long(&face_text()));
    let face_color = use_memo(use_reactive(&props.face.color_index, get_face_color));

    rsx! {
        div {
            id: "face-editor",
            input {
                autofocus: true,
                value: "{face_text}",
                oninput: move |event| {
                    props.on_text_input.call(event.value());
                },
                background_color: "{face_color}"
            }
            if face_too_long() {
                div { "Why the long face? 🐴 (3 character limit)" }
            }

            label {
                "Color"
                select {
                    value: "{props.face.color_index}",
                    onchange: move |event| {
                        match event.value().parse::<u8>() {
                            Ok(color_index) => props.on_color_change.call(color_index),
                            Err(e) => log::error!("Failed to parse color index: {}", e),
                        }
                    },

                    option {
                        value: "0",
                        "Yellow"
                    }

                    option {
                        value: "1",
                        "Orange"
                    }

                    option {
                        value: "2",
                        "Red"
                    }

                    option {
                        value: "3",
                        "Pink"
                    }

                    option {
                        value: "4",
                        "Blue"
                    }

                    option {
                        value: "5",
                        "Teal"
                    }

                    option {
                        value: "6",
                        "Green"
                    }
                }
            }
            if props.face.color_index > 6 {
                div { "Unknown color index {props.face.color_index}" }
            }
            br { }
            br { }
            br { }

            div {
                id: "face-actions",
                button {
                    onclick: move |_| props.on_load.call(()),
                    "Load"
                }
                button {
                    onclick: move |_| props.on_save.call(()),
                    "Save"
                }
            }
        }
    }
}
