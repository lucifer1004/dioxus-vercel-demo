use dioxus::prelude::*;
use log;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let mut counter = 0;
    let button_counter = use_state(&cx, || 0);

    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            padding: "10px",
            align_items: "center",
            width: "40%",
            margin: "auto",
            border: "1px dashed lightblue",
            border_radius: "20%",
            onclick: move |_| {
                counter += 1;
                log::debug!("div clicked {counter} times!");
            },
            h1 {
                color: "lightblue",
                "Dioxus demo",
            },
            img {
                src: "https://www.rust-lang.org/logos/rust-logo-blk.svg"
            },
            p {
                "counter: {button_counter}",
            },
            button {
                width: "100px",
                onclick: move |ev| {
                    ev.cancel_bubble();
                    log::debug!("button clicked!");
                    button_counter.set(button_counter.get() + 1);
                },
                "Click me!"
            },
        }
    })
}
