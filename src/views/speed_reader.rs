use dioxus::prelude::*;

const SPEED_READER_CSS: Asset = asset!("/assets/styling/speed-reader.css");

#[component]
pub fn SpeedReader() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: SPEED_READER_CSS }
        div {
            id: "player",
            class:"player",
            div {
                id: "reader",
                class: "reader"
            }
            div {
                id: "playerControls",
                class: "player-controls",
                
                div {
                    id: "progressBar",
                    class: "progress-bar"
                },
                div {
                    id: "buttonControls",
                    class: "button-controls"
                }
            }
        }
    }
}