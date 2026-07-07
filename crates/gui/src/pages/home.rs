use dioxus::prelude::*;

use crate::components::{AddStep, Step};

#[component]
pub fn Home() -> Element {
    let mut steps = use_signal::<Vec<lib::Step>>(|| vec![]);

    use_effect(move || {
        let values = vec![
            lib::Step::Layer(lib::Layer::Invert),
            lib::Step::Layer(lib::Layer::Brightness(10)),
            lib::Step::Layer(lib::Layer::Brightness(11)),
            lib::Step::Layer(lib::Layer::Brightness(12)),
            lib::Step::Save("pokus.png".into()),
        ];

        values.into_iter().for_each(|v| steps.push(v));
    });

    rsx! {
        main {
            class: "h-64 grid h-full w-full",
            style: "grid-template-columns: 3fr auto 2fr",

            Steps { steps: steps() },
            div { class: "mx-8 w-0 border-r bordercolor" }
            div {
                // class: "bg-orange-500",

                h3 { "Preview" }
                p { "//TODO" }
            }
        }
    }
}

#[component]
fn Steps(steps: Vec<lib::Step>) -> Element {
    rsx! {
        div {
            // class: "bg-red-500"

            h2 { "Welcome to glipho!" },

            div {
                class: "flex flex-col gap-4 mt-4",

                for s in steps {
                    Step { step: s }
                }

                AddStep {  }
            }
        }
    }
}
