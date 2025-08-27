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

Za nalaganje dokumentov je desno zgoraj gumb _Dodajanje_. Odprla se bo stran, kamor lahko izberete kateri `pdf` dokument želite naložiti. Podpiramo pa tudi mehaniko "drag and drop".

Da pridete do seznama dokumentov, zgoraj v navigacijski vrstici kliknete zavihel **Dokumenti**.
Da so ogledate posamezen dokument, kliknite na gumb **Poglej** ob želenem dokumentu.

Odpre se vam stran razdeljena na tri dele. Razmerje med predeli lahko zamenjate tako da z desnim klikom držite na rob med dvema deloma in z miško povlečete na tisto mesto kjer želite da rob je.

V srednjem panelu na dnu lahko najdete gumb **OCR** (_Optical character recognition_). Ko ga kliknete, bo program poskusil ugotoviti besedilo dokumenta. To lahko vzame nekaj časa (predvideno ne več kot minuto). Med tem časom morate ostati na tej srani, drugače se bo proces prekinil.
Celotno besedilo dokmenta bo zapisano v polje **Vsebina**. Spodbudajmo vas, da preverite če je besedilo pravilno zapisano, kajti skoraj gotovo ne bo.

V levem panelu lahko spreminjate vse podatke, razen **Naslov dokumenta**.
Če želite dodati več oseb/organizacij/lokacij/ključnih besed, pritisnite tipko enter, ko je vaš kurzor v zadnjem od vnosnih polj.
Osebam, organizacijam in lokacijam lahko dodate variacije imen, tako da na desni strani vnosnih prostorčkov kliknete gumb **+**. Pojavilo se bo še eno vnosno polje, kamor zapišete variacijo.

Če želite odstraniti vnosni prostorček, na desni strani prostorčka kliknite ikono za koš za smeti.

Na koncu ne pozabite pritisniti gumba shrani, čisto na dnu levega **IN** srednjega panela.

Pod zavihkom **Organizacije** si lahko ogledate vse omenjene oragnizacije v dokumentih.




