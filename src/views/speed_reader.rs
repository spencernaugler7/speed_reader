use dioxus::prelude::*;

// use crate::components::Hero;

const SPEED_READER_CSS: Asset = asset!("/assets/styling/speed-reader.css");
const PLAY_SVG: Asset = asset!("/assets/player/play.svg");
const PAUSE_SVG: Asset = asset!("/assets/player/pause.svg");
const SKIP_NEXT_SVG: Asset = asset!("/assets/player/skip_next.svg");
const SKIP_PREVIOUS_SVG: Asset = asset!("/assets/player/skip_previous.svg");

#[component]
pub fn SpeedReader() -> Element {
    rsx! {
        div {
            id: "player",
            // class: "player",
            document::Link { rel: "stylesheet", href: SPEED_READER_CSS },
            div {
                id: "reader",
                class: "reader",
                input {
                    type: "file",
                    "select pdf"
                }
            },
            div {
                id: "playerControls",
                class: "flex-col",

                input {
                    class: "player-progress",
                    type: "range",
                    disabled: true,
                    min: 0,
                    max: 100
                }
                div {
                    class: "flex-row",
                    button {
                        id: "previous",
                        class: "player-button",
                        img {
                            src: SKIP_PREVIOUS_SVG
                        }
                    }
                    button {
                        id: "play",
                        class: "player-button",
                        img {
                            src: PAUSE_SVG
                        }
                    }
                    button {
                        id: "next",
                        class: "player-button",
                        img {
                            src: SKIP_NEXT_SVG
                        }
                    }
                }
            }
        }
        // Hero {}
    }
}
