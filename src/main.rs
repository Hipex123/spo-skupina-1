#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use rand::Rng;
use rand::thread_rng;
use manganis::*;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},

    #[route("/POST")]
    POST {},

    #[nest("/Napake")]
    #[layout(NapRedir)]
        #[route("/")]
        Napake {},
    #[end_layout]
    #[end_nest]


    #[nest("/Diagnostika")]
    #[layout(DiagRedir)]
        #[route("/")]
        Diagnostika {},
        #[route("/Basic")]
        OsnovnoPregled{},
        #[route("/Test")]
        Testiranje{},
        #[route("/BIOS")]
        BIOS{},
    #[end_layout]
    #[end_nest]

    #[route("/SpremeniIme")]
    SpremeniIme {},

}


const IMAGES: [manganis::ImageAsset; 5] = [
    mg!(image("./assets/meme-images/jotaro.jpg")),
    mg!(image("./assets/meme-images/bucciarati.jpg")),
    mg!(image("./assets/meme-images/kira.jpg")),
    mg!(image("./assets/meme-images/wind.png")),
    mg!(image("./assets/meme-images/josuke.jpg")),
];

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    use_context_provider(|| Signal::new(NameHandler::new()));
    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        Router::<Route> {}
    }
}

#[component]
fn Nav() -> Element {
    let mut option = use_signal(|| "".to_string());
    //let name = use_context::<Signal<NameHandler>>()().name;
    let mut name_handler = use_context::<Signal<NameHandler>>();

    rsx!(
        div {
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
                to: "/SpremeniIme",
                "U) Spremeni ime"
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
            Link{
                class: "inputs",
                to: "/Diagnostika/Basic",
                "3) Osnovni pregled"
            }
                Link{
                class: "inputs",
                to: "/Diagnostika/Test",
                "4) Testiranje"
            }
                Link{
                class: "inputs",
                to: "/Diagnostika/BIOS",
                "5) BIOS"
            }
        }
        div{
            class: "terminal-container",
            div{"[{name_handler().name}@arch ~]$"}
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
                                "U" => {navigator().push(Route::SpremeniIme{});},
                                "1" => {navigator().push(Route::Napake{});},
                                "2" => {navigator().push(Route::Diagnostika{});},
                                "3" => {navigator().push(Route::OsnovnoPregled{});},
                                "4" => {navigator().push(Route::Testiranje{});},
                                "5" => {navigator().push(Route::BIOS{});},
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
        image{

        }
    )
}

#[component]
fn Home() -> Element {
    const MB_IMG: ImageAsset = mg!(image("./assets/images/motherboard-cpu-computer-processor-preview.jpg"));

    

    rsx!(
        div{
        class:"width-limiter",

        Intro {}
            img{src:"{MB_IMG}"}
            Jojo {}
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
                Jojo {}
                Jojo {}
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
            Jojo {}
            Jojo {}
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

#[component]
fn SpremeniIme() -> Element {
    let mut option = use_signal(|| "".to_string());
    let mut name_handler = use_context::<Signal<NameHandler>>();

    rsx!(div{
        class:"width-limiter",


         div{
            class: "terminal-container",
            div{"Novo Ime: "}
         input{
                class:"terminal-input",
                tabindex: 0,
                autofocus: true,
                oninput: move |event| {
                    option.set(event.value().to_string());
                },
                onkeydown: move |event: KeyboardEvent| {
                    if event.key() == Key::Enter{
                        name_handler.write().name = option();
                        option.set("".to_string());
                        navigator().push(Route::Home {});
                    }
                },
                span {}
            }
            Jojo {}
        }
        }
    )

}


#[derive(Clone)]
struct NameHandler{
    name: String,
    option: String,
}
impl NameHandler {
    fn new() -> Self{
        Self{
            name: "user".to_string(),
            option: "".to_string(),
        }
    }
}


fn OsnovnoPregled() -> Element{
    rsx!(div {
        class: "width-limiter",
        Jojo {}
        h1 { "Osnovni Pregled in Preverjanje Napajanja" }
        p {
            "Pred začetkom podrobnejše diagnostike je ključnega pomena, da se opravi osnovni pregled
            in preverjanje napajanja. To vključuje naslednje korake:"
        }
        ul {
            li {
                b { "Vizualni Pregled: " }
                "Preveri fizične poškodbe, kot so ožgani deli, izbočeni kondenzatorji,
                korozija na komponentah ali poškodovani konektorji."
            }
            li {
                b { "Preverjanje Povezav in Napajanja: " }
                "Preveri, da so vsi kabli, vključno z napajalnimi in podatkovnimi,
                pravilno priključeni. Za preizkus napajalnika uporabi multimeter
                ali ga zamenjaj z znanim delujočim."
            }
            li {
                b { "POST in Diagnostični Signali: " }
                "Če ima matična plošča zvočnik, poslušaj za piske in preveri, kaj ti
                pomenijo. Diagnostične LED lučke na plošči ti lahko tudi pomagajo pri
                prepoznavanju napake."
            }
        }
        Nav {}
    })
}

#[component]
fn Testiranje() -> Element{
    rsx!(
        div {
            Jojo {}
    h1 { "Izolacija Komponent in Testiranje" }
    p {
        "Če osnovni pregled ne razkrije težave, je naslednji korak izolacija komponent
        in testiranje sistema z minimalno konfiguracijo."
    }
    ul {
        li {
            b { "Odstranitev Nebistvenih Komponent: " }
            "Odklopi vse, kar ni nujno za zagon sistema, kot so dodatne kartice,
            trdi diski in zunanje naprave. Poskusi zagnati sistem samo s procesorjem,
            enim RAM modulom in integrirano grafiko (če je na voljo)."
        }
        li {
            b { "Preverjanje RAM-a: " }
            "Preizkusi različne RAM module in jih vstavljaš v različne reže,
            da izključiš možnost, da je okvarjen RAM krivec za težave."
        }
        li {
            b { "Preizkus Zunaj Ohišja: " }
            "Postavi matično ploščo na neprevodno površino (kot je karton) in
            jo zaženi zunaj ohišja. Tako izključiš možnost kratkega stika z ohišjem."
        }
    }
}
        Nav{}
    )
}

#[component]
fn BIOS() -> Element{
    rsx!(h1 { "Pregled BIOS-a in Posodobitev" }
    Jojo {}
    p {
        "Če lahko dostopaš do BIOS-a, lahko s tem pridobiš dodatne informacije o stanju
        sistema in morebitnih težavah z zaznavanjem komponent."
    }
    ul {
        li {
            b { "Dostop do BIOS-a: " }
            "Če sistem pride do BIOS-a, preveri, ali so vse komponente pravilno zaznane.
            Preveri nastavitve in po potrebi ponastavi BIOS na privzete vrednosti."
        }
        li {
            b { "Posodobitev BIOS-a: " }
            "Če imaš težave z združljivostjo ali zaznavanjem komponent, preveri spletno
            stran proizvajalca matične plošče za morebitne posodobitve BIOS-a. Posodobi,
            če je na voljo novejša različica."
        }
    }
        Nav {}
    )
}

#[component]
fn NapRedir() -> Element {
    rsx! {
        h1 { "Napake" }
        Outlet::<Route> {}
    }
}

#[component]
fn DiagRedir() -> Element {
    rsx! {
        h1 { "Diagnostika" }
        Outlet::<Route> {}
    }
}

fn get_random_image() -> &'static manganis::ImageAsset {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..IMAGES.len());
    &IMAGES[index]
}

#[component]
fn Jojo() -> Element {
    let mut rng = rand::thread_rng();

    let random_left: u32 = rng.gen_range(0..1601);
    let random_top: u32 = rng.gen_range(0..601);

    let meme_images_style = format!(r#"
            margin-left: {random_left}px;
            margin-top: {random_top}px;
            opacity: 0.05;
            border: none;
            position: absolute;
            box-shadow: none;
            z-index: 1                        
        "#);
    rsx!(img{src:"{get_random_image()}", style:"{meme_images_style}"})
}
