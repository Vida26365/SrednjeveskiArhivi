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
    - Opis 1
    - Opis 2
        - Nisva mogla uporabiti for zanke znotraj rsx elementa
        ``` rs
        ul {
            for kljucna_beseda in Keywords {
                li {
                    input {
                        name: "keyword"
                        value: "{kljucna_beseda}"
                    }
                }
            }
        }
        ```
    - Trenutno uporabljava beta verzijo, ki še ni stabilna
- Velikokrat se nama zgodi da preveč časa porabiva za razmišljanje o implementaciji stvari, namesto da bi začela z delom


## Kaj je ša narest
- Trenutno sva se bolj ukvarjala z izgledom in implementacijo baze
- Transkripcija z OCR-jem
