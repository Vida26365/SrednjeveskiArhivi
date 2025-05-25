# Srednejveški Arhivi

## Zanimivi deli implementacije
- dioxus knjižnjica, kjer lahko sestavljaš komponente, kot v html-ju
    - Za to sva se odločila, kljub temu, da dokumentacija ni popolna, zaradi bolj ali manj preproste uporabe
    -Iskanje primerov  implementacije v repozitorijih naključnih ljudi
- baza ![struktura_baze](predstavitev_slike/srednjeveski-arhivi.png)
- automatično dodajanje polj ob kliku enter

## Težave
- Iskanje primerov uporabe knjižnjice dioxus
- grda implementacija
- Uporaba JavaScripta za drsnike
    - V rustu to ni mogoče
    - [grid.js](assets/scripts/grid.js)
- Prevajalniški hrošč. In to celo dva
    - Pri prevajanju ring zaboja (ki ga najin projekt posredno uporablja za kriptografijo) s prevajalnikom GCC z orodjarno MinGW je prišlo do notranje napake prevajalnika, zaradi česar se projekt ni prevedel. Težavo sva prijavila razvijalcem projekta MinGW Builds (https://github.com/niXman/mingw-builds/issues/706), nato pa še neposredno razvijalcem prevajalnika GCC (https://gcc.gnu.org/bugzilla/show_bug.cgi?id=120192). Izkazalo se je, da je to resničen hrošč v prevajalniku GCC z orodjarno MinGW, ki se ga da ponoviti z nekaj preprostimi vrsticami kode:

    ``` rs
    typedef char a;
    void b() {
        typedef a c __attribute__((vector_size(32), aligned));
        c d = {};
    }
    ```

    Napaka žal še ni bila odprevljena, zato sva težavo rešila tako, da uporabljava MSVC orodjarno.

    - Pri prevajanju Dioxus komponent, ki vsebujejo veliko gnezdenih blokov in zank, je prišlo do puščanja pomnilnika v prevajalniku Rustc. To je povzročilo veliko porabo pomnilnika (tudi do 30 GB) in posledično notranjo napako prevajalnika, ko sistem ni mogel več dodeliti dodatnega pomnilnika. Izkazalo se je, da je tudi to resničen hrošč v prevajalniku Rustc (https://github.com/rust-lang/rust/issues/139142), ki se lahko zgodi pri postopnem prevajanju kode. Tudi to težavo se da ponoviti z nekaj preprostimi vrsticami kode:

    ``` rs
    fn main() {
        #[cfg(a)]
        || ();

        || {
            Some(())
                .map(|_| ())
                .map(|y| y);
        };

        async || {};
    }```

To težavo so sicer že odpravili, vendar bo objavljena šele v različici Rust 1.88.0, ki bo izšla 25. 6. 2025. Do tedaj morava pri razvoju projekta uporabljati nočno različico prevajalnika, ki ta popravek že ima.
- Velikokrat se nama zgodi da preveč časa porabiva za razmišljanje o implementaciji stvari, namesto da bi začela z delom


## Kaj je ša narest
- Trenutno sva se bolj ukvarjala z izgledom in implementacijo baze
- Transkripcija z OCR-jem
    - Najde prave okenčke
    - odpraviti nepotrebne nove vrstice
    - Čez poženemo še en model za prepoznavo besed, ki popravi nepravilnosti
- OCR-ji
    - Tessract:
        - Če hočeš na hitro zalaufat OCR in te ne moti slabša kvaliteta,
        - Delno je to že implementirano v Rustu (obstaja ena funkcije extract nekje, ki vzame PDF in vrne tekst),
        - Prednosti: Je hiter, ni ga težko naložit, dela na vseh računalnikih,
        - Slabosti: Slabša kvaliteta, ne vem če se da naredit ločevanje na dele vsebine,
    - OpenAI-compatible API:
    - Tukaj lahko uporabljamo bolj napredne modele za OCR (npr. Qwen 2.5 VL), ki omogočajo boljšo kvaliteto,
    - To dela tako, da nekje obstaja en ločen API strežnik, ti njemu pošlješ dokument, nazaj pa dobiš tekst,
    - V tem načinu naj bi se že dalo samodejno ločevat tekst na dele, tko da pač maš ne prompt ki to naredi,
    - Ta API kličeš prek Rusta, recimo z https://github.com/graniet/llm,
    - Kako pa dobiš ta API sta pa dve možnosti:
        - Lahko plačaš in uporabiš že nek komercialen API, recimo od OpenAIja alpa Google,
        - Če maš dovolj dober računalnik (z dovolj dobro grafično), lahko uporabiš Ollama (https://ollama.com/) in sam laufaš te modele,
    - Prednosti: Boljša kvaliteta, podpira ločevanje vsebine,
    - Slabosti: Stane (če uporabljaš komercialen API); težje za postavit, rabi dober računalnik, traja precej časa (če laufaš sam)

- Obdelava s superračunalnikom:
    - To je uporabno, če moraš obdelati veliko količino dokumentov, recimo na začetku, ker bi bila prejšnja opcija prepočasna/predraga,
    - To bi delovalo tako, da ti aplikacija pripravi en ZIP file, kamor zapakira vse PDFje, in en Python script,
    - Ta ZIP in script lahko potem naložiš na superračunalnik, kjer se vse obdela z velikimi modeli (spet Qwen),
    - Potem kot rezultat dobiš recimo spet nek ZIP s tekstom, ki ga lahko spet uvoziš v aplikacijo,
    - Tukaj tudi lahko podpiramo ločevanje vsebine,
    - Neki za to sem že js naredil par tednov nazaj, ampak ni še čist,
    - Also link do SLINGa: https://www.sling.si/,
    - Prednosti: Je zastonj (če maš dostop do SLINGa), lahko uporabiš večje in kvalitetnejše modele, za večje količine dokumentov bi moral bit hitrejše,
    - Slabosti: Rabiš dostop do SLINGa, mal več dela uporabnika je (pač to izvažanje pa uvažanje, ampak itak se tega ne dela pogosto)

