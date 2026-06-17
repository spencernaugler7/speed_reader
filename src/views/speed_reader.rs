use dioxus::prelude::*;

#[component]
pub fn SpeedReader() -> Element {
    rsx! {
        div {
            id: "speedReaderWorkingContainer",
            div {
                "Hello World"
            }
        }
    }
}
