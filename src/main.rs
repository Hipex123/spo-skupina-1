#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/POST")]
    POST {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx!(
        h1 {
            "Napake na maticni plosci"
        }
        article {
            h2 {
                "Uvod"
            }
            p {
                "Napake na maticni polsci so lahko kar tezko diagnizicirane.
                Ampak ko napako dosezemo je lahko nadlezna.
                Placeholder text to fill in the blanks."
            }
        }
        div {
            h2 {
                "vec:"
            }
            Link{
                to: "/POST",
                "Go to POST"
            }
        }
    )
}

#[component]
fn POST() -> Element {
    rsx!(
        div {
            "Hello from POST"
        }
    )
}
