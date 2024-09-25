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

    #[route("/Diagnostika")]
    Diagnostika {},
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
                "H) Domov"
            }
            Link{
                class: "inputs",
                to: "/Napake",
                "1) Napake"
            }
            Link{
                class: "inputs",
                to: "/Diagnostika",
                "2) Diagnostika"
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
                            "2" => {navigator().push(Route::Diagnostika{});},
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
            class:"width-limiter",
            div {
                h1 { "Možne težave z matično ploščo" }

                ul {
                    li {
                        strong { "Napaka POST:" }
                        br{}
                        "Postopek samopreizkusa ob vklopu lahko ne uspe, kar prepreči zagon računalnika. Označeno s kodami piskov ali lučkami."
                    }
                    li {
                        strong { "Brez napajanja:" }
                        br{}
                        "Matična plošča se morda sploh ne vklopi zaradi težav z napajanjem, okvare matične plošče ali napačnih povezav."
                    }
                    li {
                        strong { "Pregrevanje:" }
                        br{}
                        "Pregrevanje lahko poškoduje matično ploščo, pogosto zaradi slabega hlajenja, termalne paste ali ventilatorjev."
                    }
                    li {
                        strong { "Naključne zamrznitve in zrušitve:" }
                        br{}
                        "Lahko jih povzročijo nezdružljiva strojna oprema, gonilniki ali okvarjen RAM."
                    }
                    li {
                        strong { "Nezdružljivost strojne opreme:" }
                        br{}
                        "Nove komponente morda niso združljive s starejšo matično ploščo."
                    }
                    li {
                        strong { "Okvarjeni kondenzatorji:" }
                        br{}
                        "Izbočeni ali puščajoči kondenzatorji lahko povzročijo težave z napajanjem in stabilnostjo."
                    }
                    li {
                        strong { "Slabe vtičnice ali reže:" }
                        br{}
                        "Težave z vtičnicami CPU, RAM ali PCIe lahko preprečijo pravilno delovanje komponent."
                    }
                    li {
                        strong { "Korozija ali poškodbe:" }
                        br{}
                        "Fizične poškodbe ali korozija lahko povzročijo kratke stike ali težave s povezavo."
                    }
                    li {
                        strong { "Težave z BIOS-om:" }
                        br{}
                        "Težave z nastavitvami BIOS-a, posodobitvami ali poškodbami lahko povzročijo težave pri zagonu."
                    }
                    li {
                        strong { "Ponastavitev ali okvara baterije CMOS:" }
                        br{}
                        "Izpraznjena baterija lahko povzroči izgubo nastavitev BIOS-a in sistemskega časa."
                    }
                    li {
                        strong { "Napaka integrirane komponente:" }
                        br{}
                        "Težave z integriranimi komponentami, kot so zvok, LAN ali grafika, lahko vplivajo na delovanje sistema."
                    }
                    li {
                        strong { "Težave z gonilniki in vdelano programsko opremo:" }
                        br{}
                        "Zastareli ali poškodovani gonilniki ali vdelana programska oprema lahko povzročijo nestabilnost sistema."
                    }
                }

                p {
                    "Diagnosticiranje težav z matično ploščo je zahtevno, saj lahko številne komponente prispevajo k težavam. Pomaga strukturiran pristop k reševanju."
                }
            }

        }
        Nav{}
    )
}



#[component]
fn Diagnostika() ->  Element{
    rsx!{
        div{
            class:"flex width-limiter",
            div{
                class:"left-spacer"
            }
            div {
                h1 { "Diagnostika težav z matično ploščo" }

                h1 { "Začetni vizualni pregled" }
                p {
                    "Fizične poškodbe: Preverite, ali so ožgane komponente, izbočeni kondenzatorji ali vidni znaki poškodb ali korozije." br{}
                    "Povezave: Prepričajte se, da so vsi kabli (napajalni, podatkovni itd.) dobro povezani." br{}
                    "Reže in vtičnice: Preverite reže RAM, reže PCIe in vtičnice CPU za prah ali poškodbe."
                }

                h1 { "Preverite napajanje" }
                p {
                    "Napajalna enota (PSU): Preverite, ali napajalnik deluje pravilno, tako da ga preizkusite z multimetrom ali zamenjate z znano dobro enoto." br{}
                    "Povezave: Preverite, ali sta 24-pinski ATX in 4/8-pinski napajalni konektor CPU pravilno priključena."
                }

                h1 { "POST matične plošče" }
                p {
                    "Poslušaj piske: Če ima vaš sistem priključen zvočnik, poslušajte kode piskov ob zagonu. Oglejte si priročnik za matično ploščo, kaj označujejo specifične kode piskov." br{}
                    "Preverite LED-lučke: Nekatere matične plošče imajo diagnostične LED-lučke, ki označujejo težave z napajanjem, CPU-jem, RAM-om ali drugimi komponentami."
                }

                h1 { "Izolirajte komponente" }
                p {
                    "Odstranite nebistvene komponente: Odklopite vse zunanje naprave, dodatne pogone in razširitvene kartice. Poskusite zagnati samo s procesorjem, enim pomnilnikom RAM in integrirano grafiko (če je na voljo)." br{}
                    "Testiranje RAM-a: Preizkusite z različnimi ključki RAM in konfiguracijami (ena ključka, različne reže), da izključite okvarjen pomnilnik."
                }

                h1 { "Preverite kratke stike" }
                p {
                    "Stojala: Zagotovite, da je matična plošča pravilno nameščena z držali in ni v stiku z ohišjem." br{}
                    "Preizkušanje plošče: Preizkusite matično ploščo zunaj ohišja (na neprevodni površini), da izključite kratke stike z ohišjem."
                }

                h1 { "Preglejte CPU in hlajenje" }
                p {
                    "Namestitev CPU: Ponovno namestite CPU in se prepričajte, da ni upognjenih zatičev (če obstaja)." br{}
                    "Hlajenje: Zagotovite, da je hladilnik procesorja pravilno pritrjen in deluje, da preprečite pregrevanje."
                }

                h1 { "Dostop do BIOS-a/UEFI" }
                p {
                    "Poskus dostopa do BIOS-a: Če lahko dostopate do BIOS-a/UEFI, preverite nastavitve in zagotovite, da so zaznane komponente (CPU, RAM, pogoni)." br{}
                    "Ponastavitev BIOS-a: To lahko storite tako, da odstranite baterijo CMOS ali uporabite mostiček na matični plošči."
                }

                h1 { "Posodobite/flash BIOS" }
                p {
                    "Težave z BIOS-om: Če je mogoče, preverite, ali so na voljo posodobitve BIOS-a pri proizvajalcu matične plošče. Včasih težave nastanejo zaradi zastarele vdelane programske opreme."
                }

                h1 { "Preizkusite z znano dobrimi komponentami" }
                p {
                    "Znani dobri deli: Zamenjajte komponente z znanimi delujočimi deli (kot so RAM, GPU ali napajalniki), da odstranite drugo strojno opremo kot vzrok."
                }

                h1 { "Posvetujte se z diagnostiko proizvajalca" }
                p {
                    "Priročnik in viri: Glejte priročnik za matično ploščo za posebne nasvete za odpravljanje težav ali diagnostična orodja, ki jih ponuja proizvajalec."
                }

                h1 { "Strokovna pomoč" }
                p {
                    "Če vse drugo odpove: Če težave ne morete rešiti, razmislite o tem, da bi matično ploščo odnesli k profesionalnemu tehniku ali preverite garancijski servis, če je na voljo."
                }

                h1 { "Končne opombe" }
                p {
                    "Zabeležite si podrobne teste, ki ste jih opravili, in rezultate." br{}
                    "Bodite potrpežljivi, saj lahko diagnosticiranje težav z matično ploščo traja dolgo in lahko vključuje poskuse in napake." br{}
                    "Če sistematično sledite tem metodam, lahko učinkovito zožite možnosti in prepoznate težavo z matično ploščo."
                }
            }
         }
        Nav{}
    }
}