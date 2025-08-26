# Uporabniška navodila

## priprava

Naložite si naslednje programe:
* [Rust & Cargo](https://www.rust-lang.org/tools/install)
* [Node.js & npm](https://nodejs.org/en/download/)
* [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started/#install-the-dioxus-cli)
* [WebView](https://dioxuslabs.com/learn/0.6/getting_started/#platform-specific-dependencies)

Nato v terminalu poženite ukaz
```bash
npm install
```
## Zagon programa

Da zaženete program, odrpite dva zavihka v **Terminalu**. V enem poženite ukaz
```bash
npm run watch
```
S tem boste zagtovili lep izgled aplikacije.

V drugem zavihku pa poženite ukaz
```bash
dx serve
```

## Uporaba programa

```html
<svg fill="none">
  <path d= "M3 16.5V18.75C3 19.9926 4.00736 21 5.25 21H18.75C19.9926 21 21 19.9926 21 18.75V16.5M7.5 7.5L12 3M12 3L16.5 7.5M12 3L12 16.5" stroke-linecap="round" </path>
</svg>```



### Dokumenti

Da so ogledate posamezen dokument, kliknite na gumb `poglej` ob želenem dokumentu.

Odpre se vam stran razdeljena na tri dele. Razmerje med predeli lahko zamenjate tako da z desnim klikom držite na rob med dvema deloma in z miško povlečete na tisto mesto kjer želite da rob je.

Na levi strani lahko spreminjate vse podatke, razen `Naslov dokumenta`

Če želite dodati več `oseb/organizacij/lokacij/ključnih besed`, pritisnite tipko enter, ko je vaš kurzor v zadnjem vnosnem polju.

Če želite odstraniti vnosni prostorček, na desni strani kliknite ikono
`<svg fill="none">
  <path d= "M14.7404 9L14.3942 18M9.60577 18L9.25962 9M19.2276 5.79057C19.5696 5.84221 19.9104 5.89747 20.25 5.95629M19.2276 5.79057L18.1598 19.6726C18.0696 20.8448 17.0921 21.75 15.9164 21.75H8.08357C6.90786 21.75 5.93037 20.8448 5.8402 19.6726L4.77235 5.79057M19.2276 5.79057C18.0812 5.61744 16.9215 5.48485 15.75 5.39432M3.75 5.95629C4.08957 5.89747 4.43037 5.84221 4.77235 5.79057M4.77235 5.79057C5.91878 5.61744 7.07849 5.48485 8.25 5.39432M15.75 5.39432V4.47819C15.75 3.29882 14.8393 2.31423 13.6606 2.27652C13.1092 2.25889 12.5556 2.25 12 2.25C11.4444 2.25 10.8908 2.25889 10.3394 2.27652C9.16065 2.31423 8.25 3.29882 8.25 4.47819V5.39432M15.75 5.39432C14.5126 5.2987 13.262 5.25 12 5.25C10.738 5.25 9.48744 5.2987 8.25 5.39432" stroke-linecap="round" </path>
</svg>`.

\red[**Pozor**]: Če imata dva polja isto ime, ko odstraniš enega, se odstrani tudi drugi.


Če želite shraniti imena oseb, organizacij in lokacij z njihovimi variacijami, je na desni strani vnosnih prostorčkov gumb `+`. Ko ga kliknete, se bo pojavil še eno vnosno polje, kamor gre lahko variacija imena.

Na koncu ne pozabite pritisniti gumba shrani, čisto na dnu levega **IN** srednjega panela.

Da pridete nazaj do lista dokumentov, zgoraj v navigacijski vrstici kliknete zavihel `Dokumenti`. Na voljo pa imete še list organizacij ter list oseb (pod zavihkoma `Organizacije` in `Osebe`).

