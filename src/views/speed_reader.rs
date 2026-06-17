use dioxus::prelude::*;

// #[component]
// pub fn SpeedReader() -> Element {
//     rsx! {
//         div {
//             id: "speedReaderWorkingContainer",
//             div {
//                 "Hello World"
//             }
//         }
//     }
// }

#[component]
pub fn SpeedReader() -> Element {
    rsx! {    
        div { 
            div {
                style: "display: flex; gap: 0.85rem; align-items: center;",
                img {
                    alt: "Midnight City album art",
                    height: "64",
                    src: "https://avatar.vercel.sh/midnight-city",
                    style: "width: 64px; height: 64px; border-radius: 0.45rem; object-fit: cover; flex-shrink: 0; box-shadow: 0 6px 18px -8px rgba(0,0,0,0.35);",
                    width: "64",
                }
                div { style: "flex: 1; min-width: 0;",
                    p { style: "margin: 0; font-weight: 600; color: var(--secondary-color-3); overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                        "Midnight City"
                    }
                    p { style: "margin: 0.15rem 0 0; color: var(--secondary-color-5); font-size: 0.85rem; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                        "M83 · Hurry Up, We're Dreaming"
                    }
                }
            }
            div {  style: "margin-top: 1.1rem;",
                div {
                    "data-disabled": "false",
                    "data-node-hydration": "236,resize:0,pointerdown:1",
                    "data-orientation": "horizontal",
                    role: "group",
                    div {
                        "data-disabled": "false",
                        "data-orientation": "horizontal",
                        div {
                            "data-disabled": "false",
                            "data-orientation": "horizontal",
                            style: "left: 0%; right: 75.83292452830042%",
                        }
                        button {
                            aria_label: "Track progress",
                            aria_orientation: "horizontal",
                            aria_valuemax: "212",
                            aria_valuemin: "0",
                            aria_valuenow: "51.234200000003085",
                            "data-disabled": "false",
                            "data-dragging": "false",
                            "data-index": "0",
                            "data-node-hydration": "240,mousedown:1,touchstart:1,keydown:1",
                            "data-orientation": "horizontal",
                            r#type: "button",
                            role: "slider",
                            style: "left: 24.16707547169957%",
                            tabindex: "0",
                        }
                    }
                }
                div { style: "display: flex; justify-content: space-between; margin-top: 0.45rem; color: var(--secondary-color-5); font-size: 0.78rem;",
                    span { "0:51" }
                    span { "3:32" }
                }
            }
            div {
                style: "display: flex; align-items: center; justify-content: center; gap: 0.5rem; margin-top: 0.6rem;",
                button {
                    aria_label: "Previous",
                    "data-node-hydration": "245,click:1,mousedown:1,mouseup:1,keydown:1",
                    "data-size": "default",
                    "data-style": "ghost",
                }
                button {
                    aria_label: "Play or pause",
                    "data-node-hydration": "247,click:1,mousedown:1,mouseup:1,keydown:1",
                    "data-size": "default",
                    "data-style": "primary",
                }
                button {
                    aria_label: "Next",
                    "data-node-hydration": "249,click:1,mousedown:1,mouseup:1,keydown:1",
                    "data-size": "default",
                    "data-style": "ghost",
                }
            }
        }
    }
}