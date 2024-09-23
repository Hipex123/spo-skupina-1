#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use log::trace;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/POST")]
    POST {},

    #[route("/Napake")]
    Napake {},
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
fn Nav() -> Element {
    let mut option = use_signal(|| "".to_string());


    rsx!(div {
        h2 {
            "Izberite opcijo:"
        }
        div{
            class:"input-options",
            Link{
                class: "inputs",
                to: "/",
                "H) Domol"
            }
            Link{
                class: "inputs",
                to: "/Napake",
                "1) Napake"
            }
            Link{
                class: "inputs",
                to: "/POST",
                "2) POST"
            }
        }
        div{
            class: "terminal-container",
            div{"[gapi@arch ~]$"}
            input{
                class:"terminal-input",
                tabindex: 0,
                autofocus: true,
                oninput: move |event| {
                    option.set(event.value().to_string());
                },
                onkeydown: move |event: KeyboardEvent| {
                    if event.key() == Key::Enter{
                        match option().as_str().to_uppercase().as_str() {
                            "H" => {navigator().push(Route::Home{});},
                            "1" => {navigator().push(Route::Napake{});},
                            "2" => {navigator().push(Route::POST{});},
                            _ => {}
                        }
                        option.set("".to_string());
                    }
                },
                span {}
            }
        }
    }
    )
}

#[component]
fn Intro() -> Element {
    rsx!(
        h1 {
            "Diagnostika/Napake Matične Plošče "
        }
        article {
            h2 {
                "Uvod"
            }
            p {
                "Matična plošča je ključni del računalnika,
                ki povezuje vse komponente sistema in zagotavlja njegovo stabilno delovanje.
                Kljub svoji pomembnosti je pogosto izpostavljena napakam,
                ki lahko vplivajo na celoten sistem.
                Diagnostika napak na matični plošči je zapletena,
                saj vključuje analizo fizičnih in funkcionalnih težav,
                ki izvirajo iz okvar elektronike, povezav ali mehanskih poškodb.
                V tej nalogi bomo raziskali vrste napak na matični plošči ter
                metode njihove diagnostike, z namenom izboljšanja razumevanja
                in vzdrževanja računalniških sistemov."
            }
        }
    )
}

#[component]
fn Home() -> Element {
    rsx!(div{
        class:"width-limiter",

        Intro {}
        Nav {}
        }
    )

}

#[component]
fn POST() -> Element {
    rsx!(
        div {
            "Hello from POST"
        }
        Nav{}
    )
}


#[component]
fn Napake() -> Element {
    rsx!(
        div{
            "hello"
        }
        Nav{}
    )
}
