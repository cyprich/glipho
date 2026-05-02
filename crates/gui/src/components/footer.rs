use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        div {
            class: "bg-zinc-950 text-zinc-50 w-full h-24 flex items-center px-8",
            p {
                class: "font-bold text-xl",
                "Rusty fotos"
            }
        }
    }
}
