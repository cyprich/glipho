use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            class: "fixed top-0 left-0 flex items-center gap-6 p-6 w-full h-20 z-50 bg-neutral-800 border-b bordercolor",

            lucide_dioxus::PanelLeft { class: "text-neutral-200 clickable" }
            div { class: "h-full bg-red border-l bordercolor" }
            p { class: "text-lg", "glipho" }
        }
    }
}
