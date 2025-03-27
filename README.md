# About

This is a mini example of working vs not working css animation example for
darkmode changes. All the logic is contained in the [src/main.rs](./src/main.rs)
file.

To run use `trunk serve --open`.

The code bellow using `leptos_use` doesn't trigger the animation.

```rust
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
```

This one does using `web_sys` does.

```rust
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
```
