#![allow(non_snake_case)]

pub mod face_data;

use std::convert::identity;

use dioxus::prelude::*;
use log::LevelFilter;
use thiserror::Error;

use crate::face_data::FaceData;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut face_data = use_signal(|| FaceData::load_from_registry());
    let ok_face_data = use_memo(move || face_data().unwrap_or_default());

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        match face_data() {
            Ok(face) => rsx! {FaceEditor {
                face: face,
                on_text_input: move |new_text| {
                    let new_face = FaceData {
                        face_text: new_text,
                        ..ok_face_data()
                    };
                    face_data.set(Ok(new_face));
                },
                on_save: move |_| {
                    // TODO: Better error handling
                    ok_face_data().save_to_registry().expect("Failed to save face data");
                },
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
    on_save: EventHandler<()>,
}

#[component]
fn FaceEditor(props: FaceEditorProps) -> Element {
    let face_text = use_memo(use_reactive(&props.face.face_text, identity));
    let is_face_too_long = use_memo(move || face_text().chars().count() > 3);

    rsx! {
        div {
            id: "face-editor",
            input {
                placeholder: "Type a face :3",
                value: "{face_text}",
                oninput: move |event| {
                    props.on_text_input.call(event.value());
                }
            }
            div {
                "Current color: {props.face.face_color_index}"
            }

            if is_face_too_long() {
                div { "Why the long face? 🐴 (3 character limit)" }
            }
            button {
                onclick: move |_| {
                    props.on_save.call(());
                },
                "Save"
            }
        }
    }
}
