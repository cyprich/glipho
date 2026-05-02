use dioxus::prelude::*;

#[component]
pub fn Main() -> Element {
    // TODO unwrap
    let image = lib::Image::from_file("../../sample.jpg").unwrap();
    let steps = lib::Steps::from_file("../../steps5.toml").unwrap();

    rsx! {
        div {
            class: "bg-zinc-800 text-zinc-50 w-full min-h-[90vh] px-8 py-12 grid grid-cols-2 gap-16",
            StepsComponent { steps },
            ImageComponent { image } ,
        }
    }
}

#[component]
pub fn StepsComponent(steps: lib::Steps) -> Element {
    rsx! {
        div {
            class: "w-full h-full flex flex-col gap-4",
            h2 { "Steps" }
            for step in steps.steps {
                StepComponent { step }
            }
            AddStepComponent { }
        }
    }
}

#[component]
pub fn StepComponent(step: lib::Step) -> Element {
    rsx! {
        div {
            class: "bg-zinc-700 p-6 rounded-xl flex justify-between items-center ",
            {

                match step {
                    lib::Step::Layer(layer) => rsx! { LayerComponent { layer } },
                    lib::Step::Save(path) => rsx! { SaveComponent { path } },
                }
            }

            div {
                class: "flex gap-2 *:bg-zinc-600 *:p-2 *:rounded-lg",
                p { "up" },
                p { "down" }
                p { "delete" }
            }
        }
    }
}

#[component]
pub fn AddStepComponent() -> Element {
    rsx! {
        div {
            class: "w-full h-24 bg-transparent border border-zinc-50/50 rounded-xl flex items-center justify-center hover:bg-zinc-700/20",
            h2 { "+" }
        }
    }
}

#[component]
pub fn LayerComponent(layer: lib::Layer) -> Element {
    rsx! {
        div {
            class: "flex gap-4 items-top",
            OvalComponent {color: "bg-blue-500"}
            div {
                class: "flex flex-col gap-1",
                p { class: "text-blue-300", "Layer" }
                b { "{layer}" }
            }
        }
    }
}

#[component]
pub fn SaveComponent(path: String) -> Element {
    rsx! {
        div {
            class: "flex gap-4 items-top",
            OvalComponent {color: "bg-green-500"}
            div {
                class: "flex flex-col gap-1",
                p { class: "text-green-400", "Save" }
                b { "{path}" }
            }
        }
    }
}

#[component]
pub fn OvalComponent(color: String) -> Element {
    rsx! {
        div {
            class: "w-6 h-12 {color} rounded-full"
        }
    }
}

#[component]
pub fn ImageComponent(image: lib::Image) -> Element {
    rsx! {
        div {
            class: "w-full h-full flex flex-col gap-4",
            h2 { "Image preview" }
            img {
                src: image.base64(),
                class: "w-auto max-h-64"
            }
        }
    }
}
