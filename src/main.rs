use leptos::prelude::*;
use leptos_use::{use_color_mode, ColorMode, UseColorModeReturn};

#[component]
fn Button1() -> impl IntoView {
    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode();

    let toggle = move |_| {
        set_mode.update(|mode| {
            *mode = match *mode {
                ColorMode::Dark => ColorMode::Light,
                _ => ColorMode::Dark,
            };
        });
    };

    view! {
        <button class="px-4 py-2 rounded-2xl font-semibold bg-gray-800 text-gray-100"
            on:click=toggle
        >
            "I don't work (leptos_use)"
        </button>
    }
}

#[component]
fn Button2() -> impl IntoView {
    let (mode, set_mode) = signal("light");

    Effect::new(move |_| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let html = document.document_element().unwrap();

        // Set theme
        if mode.get() == "dark" {
            html.class_list().add_1("dark").unwrap();
            html.class_list().remove_1("light").unwrap();
        } else {
            html.class_list().add_1("light").unwrap();
            html.class_list().remove_1("dark").unwrap();
        }
    });

    let toggle = move |_| {
        set_mode.update(|mode| {
            *mode = match *mode {
                "dark" => "light",
                _ => "dark",
            };
        });
    };

    view! {
        <button class="px-4 py-2 rounded-2xl font-semibold bg-gray-800 text-gray-100"
            on:click=toggle
        >
            "I work (web_sys)"
        </button>
    }
}

#[component]
fn App() -> impl IntoView {
    // let color_mode = use

    view! {
        <Button1/>
        <Button2/>

        <p class="bg-white text-black dark:bg-gray-900 dark:text-white min-h-screen transition-all duration-1000">"Lorem ipsum"</p>
    }
}

fn main() {
    mount_to_body(App);
}
