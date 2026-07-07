use dioxus::prelude::*;

#[component]
pub fn Step(step: lib::Step) -> Element {
    rsx! {
        BaseStep {
            step: step.clone(),

            h3 { "{step.to_type_string()}" }
            {
                match step {
                    lib::Step::Layer(layer) => rsx! { p { "{layer.to_type()}" } },
                    lib::Step::Save(_) => rsx! { p {"//TODO path?"} },
                }

            }
        }

    }
}

#[component]
pub fn AddStep() -> Element {
    rsx! {
        BaseStep {
            step: None,
            p { "Add new Step..." }
        }
    }
}

#[component]
fn BaseStep(children: Element, step: Option<lib::Step>) -> Element {
    let bg_color = match &step {
        Some(lib::Step::Layer(_)) => "bg-blue-500",
        Some(lib::Step::Save(_)) => "bg-green-500",
        None => "bg-neutral-200/50",
    };

    let delete = move || {
        dbg!("delete clicked");
        dbg!("delete clicked");
        dbg!("delete clicked");
        dbg!("delete clicked");
        dbg!("delete clicked")
    };

    rsx! {
        div {
            class: "flex justify-between p-4 rounded-xl border bordercolor bg-neutral-700/40",

            div {
                class: "flex gap-4",

                div { class: "w-8 h-16 rounded-full {bg_color}" },
                div { class: "flex flex-col justify-center", {children} }
            }

            div {
                class: "flex items-center gap-2",

                lucide_dioxus::ChevronUp { class: "size-8 clickable" }
                lucide_dioxus::ChevronDown { class: "size-8 clickable" }
                lucide_dioxus::Trash { class: "size-7 clickable hover:stroke-red-400" }
            }
        }
    }
}
