<p align="center">
  <img src="../../ui/public/app-icon.svg" width="112" height="112" alt="Logoul Twitch Miner">
</p>

<h1 align="center">Twitch Miner</h1>

<p align="center">
  <strong>Minează Twitch Drops și puncte de canal pe mai multe conturi, fără browser și fără vreo filă Twitch deschisă.</strong>
</p>

<p align="center">
  O aplicație desktop pentru Linux, Windows și macOS sau o imagine Docker cu același panou web, de rulat pe un server de acasă sau pe un Raspberry Pi. Cu 4 conturi, serverul folosește în jur de 30 MB de memorie și aproape deloc procesor.
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-miner?label=release&color=9146FF" alt="Cea mai recentă versiune"></a>
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-miner/total?label=downloads&color=9146FF" alt="Descărcări"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="Platforme: Linux, Windows, macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="Imagine Docker pentru amd64 și arm64"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="Licență MIT"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="Vizualizări">
</p>

<p align="center">
  <a href="#download"><strong>Descarcă</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-miner/issues/new/choose">Raportează o eroare</a>
</p>

<p align="center">
  <a href="../../README.md">English</a> ·
  <a href="README.fr.md">Français</a> ·
  <a href="README.de.md">Deutsch</a> ·
  <a href="README.es-ES.md">Español&nbsp;(España)</a> ·
  <a href="README.es-419.md">Español&nbsp;(Latinoamérica)</a> ·
  <a href="README.pt-PT.md">Português&nbsp;(Portugal)</a> ·
  <a href="README.pt-BR.md">Português&nbsp;(Brasil)</a> ·
  <a href="README.it.md">Italiano</a> ·
  <a href="README.nl.md">Nederlands</a> ·
  <a href="README.pl.md">Polski</a> ·
  <a href="README.ru.md">Русский</a> ·
  <a href="README.uk.md">Українська</a> ·
  <a href="README.cs.md">Čeština</a> ·
  <a href="README.hu.md">Magyar</a> ·
  <strong>Română</strong> ·
  <a href="README.bg.md">Български</a> ·
  <a href="README.el.md">Ελληνικά</a> ·
  <a href="README.tr.md">Türkçe</a> ·
  <a href="README.sv.md">Svenska</a> ·
  <a href="README.da.md">Dansk</a> ·
  <a href="README.nb.md">Norsk&nbsp;bokmål</a> ·
  <a href="README.fi.md">Suomi</a> ·
  <a href="README.zh-CN.md">简体中文</a> ·
  <a href="README.zh-TW.md">繁體中文</a> ·
  <a href="README.ja.md">日本語</a> ·
  <a href="README.ko.md">한국어</a> ·
  <a href="README.th.md">ไทย</a> ·
  <a href="README.vi.md">Tiếng&nbsp;Việt</a> ·
  <a href="README.id.md">Bahasa&nbsp;Indonesia</a>
</p>

<p align="center">
  <img src="../media/ro/presentation.avif" alt="Prezentare video a Twitch Miner">
</p>

## <a name="features"></a>Funcționalități

Ce îl deosebește de alte instrumente de minare. „Vezi în acțiune”, sub un punct, îl arată
la lucru.

### Conturi și streameri

- **Mai multe conturi într-o singură aplicație.** Fiecare cont minează independent, în
  propriul mod: **Drops și puncte** (un canal pentru cea mai bună campanie, celălalt pentru
  streamerii tăi), **Doar Drops** sau **Doar puncte**.

  <details open><summary>Vezi în acțiune</summary>
  <img src="../media/ro/accounts-mining.avif" alt="Mai multe conturi care minează în același timp, pe pagina Acasă">
  </details>

- **Un asistent pentru fiecare cont nou.** După autentificare, pornește de la setările
  implicite sau de la cele ale altui cont. Alege modul, campaniile și streamerii, apoi
  „Începe minarea” sau „Salvează și pornește mai târziu”.

  <details open><summary>Vezi în acțiune</summary>
  <img src="../media/ro/account-setup.avif" alt="Asistentul de configurare al unui cont nou, pas cu pas">
  </details>

- **O modificare aplicată altor conturi dintr-un clic.** După ce schimbi ceva la un cont,
  mesajul de confirmare îți oferă „Aplică la alte conturi”. Modificarea se repetă pe
  conturile bifate, care își păstrează restul setărilor.

  <details open><summary>Vezi în acțiune</summary>
  <img src="../media/ro/apply-to-accounts.avif" alt="O modificare repetată pe alte conturi cu „Aplică la alte conturi”">
  </details>

- **Adaugă un streamer.** Pornește imediat cu opțiunile implicite: cele ale aplicației sau
  cele alese de tine pentru acest cont. Ajustează-le pe loc, dacă e nevoie: cufere bonus,
  raiduri, momente, serie de vizionare, chat, predicții.

  <details open><summary>Vezi în acțiune</summary>
  <img src="../media/ro/streamer-add.avif" alt="Un streamer adăugat prin căutarea Twitch, apoi opțiunile lui">
  </details>

- **Editează un streamer.** „Editează” îi deschide opțiunile: cufere bonus, raiduri,
  momente, serie de vizionare, chat, predicții… Un comutator îl activează sau îl
  dezactivează fără să-l scoată din listă.

  <details open><summary>Vezi în acțiune</summary>
  <img src="../media/ro/streamer-edit.avif" alt="Opțiunile unui streamer, deschise din rândul lui">
  </details>

- **Editează mai mulți streameri deodată.** Bifează-i, apoi „Editează cei N streameri
  selectați”. Se aplică doar opțiunile pe care le schimbi. O opțiune care diferă între ei
  apare ca „Mixt”, iar comutatorul ei devine două butoane, „Da” și „Nu”. Apoi aplicația îți
  propune să le faci opțiunile implicite pentru streamerii noi.

  <details open><summary>Vezi în acțiune</summary>
  <img src="../media/ro/streamers-group-edit.avif" alt="Editarea mai multor streameri, cu opțiuni „Mixt”">
  </details>

- **Importă streameri.** Lipește o listă (până la 500 de nume) sau refolosește lista altui
  cont ori canalele pe care le urmărește contul. Lista este verificată, ca să fie adăugate
  doar canale care există.

  <details open><summary>Vezi în acțiune</summary>
  <img src="../media/ro/streamers-import.avif" alt="Importul de streameri: o listă lipită, alt cont, canalele urmărite">
  </details>

- **Spam Points.** Revendică o recompensă personalizată a unui canal de mai multe ori la
  rând. Alege un canal din lista ta de Puncte sau orice canal găsit prin căutarea Twitch:
  recompensele lui apar împreună cu soldul contului, citit în timp real. Setează cantitatea
  (până la 25, în limita soldului), apoi „Revendică”. Rezultatul apare la sfârșitul seriei,
  iar fiecare serie are rândul ei în Jurnal.

  <details open><summary>Vezi în acțiune</summary>
  <img src="../media/ro/spam-points.avif" alt="Spam Points: un canal ales, 10 × o recompensă cu mesaj revendicată, revendicările apar în chatul Twitch al canalului, apoi rândul din Jurnal">
  </details>

### Drops

- **Toate campaniile Drops în desfășurare.** Fiecare campanie are fișa ei: recompense,
  conectarea contului de joc, „Minează cu prioritate”. Lista vine de la proiectul
  [Twitch Drops API](https://drops.sunkwi.com) și se reîmprospătează la fiecare 10 minute.

  <details open><summary>Vezi în acțiune</summary>
  <img src="../media/ro/drops-catalogue.avif" alt="Catalogul sub formă de calendar, căutarea și fișa unei campanii">
  </details>

- **Ordinea ta de minare.** Trage campaniile în ordinea dorită. Cele noi își găsesc singure
  locul: întâi jocurile tale prioritare, apoi cele care se termină mai devreme sau cărora
  le-au rămas cele mai puține minute. Exclude jocuri, cuvinte, o campanie sau o singură
  recompensă și sari peste campaniile care nu se pot termina la timp. „Minează cu
  prioritate”, în fișa unei campanii, o pune pe primul loc.

  <details open><summary>Vezi în acțiune</summary>
  <img src="../media/ro/drops-order.avif" alt="Ordinea de minare, jocurile prioritare și excluderile">
  </details>

- **Inventar și „Revendică tot”.** Pagina Inventar împarte recompensele în „De revendicat”,
  „În desfășurare” și „Revendicate”. Revendică-le pe rând, pe toate odată sau activează
  „Revendică drops automat”.

  <details open><summary>Vezi în acțiune</summary>
  <img src="../media/ro/drops-inventory.avif" alt="Inventarul și butonul „Revendică tot”">
  </details>

- **„Streamerii mei mai întâi pentru Drops”.** Pentru campania curentă, un streamer din
  lista ta al cărui live contează pentru acea campanie trece înaintea celorlalte canale.
  Același canal avansează drop-ul și îți aduce punctele lui de canal. Activat implicit.

  <details open><summary>Vezi în acțiune</summary>
  <img src="../media/ro/drops-streamers-first.avif" alt="Un streamer din listă, urmărit pentru Drops și pentru punctele lui">
  </details>

### Monitorizare

- **Un jurnal pentru tot și fiecare cerere.** Jurnalul păstrează fiecare eveniment: drops,
  puncte câștigate și cheltuite, cufere, pariuri, raiduri, live-uri care încep…
  Filtrează-l după cont, după filă (Drops, Puncte, Altele) și după tip.
  **Loguri tehnice › Cereri** listează fiecare cerere trimisă către Twitch și către alte
  servicii, cu rezultatul și durata ei. Logurile și cererile nu conțin niciun token sau
  secret: „Copiază pentru un raport de eroare” le copiază, gata de lipit într-un raport de
  eroare pe [GitHub](https://github.com/SunkwiBOT/twitch-miner/issues) sau
  [Discord](https://discord.gg/support-bots-891685895781113886).

  <details open><summary>Vezi în acțiune</summary>
  <img src="../media/ro/journal-requests.avif" alt="Jurnalul filtrat, apoi Loguri tehnice › Cereri">
  </details>

### Pe desktop sau pe un server

- **Aceeași interfață peste tot.** Aplicația desktop (Linux, Windows, macOS) și imaginea
  Docker afișează același panou. Docker rulează pe un VPS, un NAS sau un Raspberry Pi, fără
  ecran și fără browser pe acea mașină: panoul se deschide în orice browser.
- **Consum redus.** În jur de 30 MB de memorie pentru 4 conturi, aproape deloc procesor și
  o imagine Docker de circa 12 MB.

### Securitate și actualizări

- **Sesiunile tale, protejate.** Fără parola Twitch: te autentifici pe
  `twitch.tv/activate`, din browserul tău. Sesiunile sunt criptate pe disc și le poți
  revoca din [setările Twitch](https://www.twitch.tv/settings/connections). Pe desktop, o
  parolă opțională a aplicației o blochează: nu se minează nimic până nu este introdusă. Cu
  Docker, panoul are propria parolă, se blochează singur după câteva minute de inactivitate
  și funcționează prin HTTPS în spatele unui proxy. Cinci parole greșite la rând blochează
  adresa IP timp de 30 de secunde.

  <details open><summary>Vezi în acțiune</summary>
  <img src="../media/ro/security-lock.avif" alt="Lacătul: aplicația blocată, apoi deblocată cu parola ei">
  </details>

- **Actualizări semnate, niciodată automate.** În bara de sus apare o insignă
  „Actualizare” cu numărul versiunii. „Instalează și repornește” cere întâi o confirmare,
  apoi aplicația instalează versiunea și repornește. Dacă are o parolă a aplicației,
  repornește blocată: introdu-o din nou ca să reiei minarea. Se instalează doar o versiune
  semnată de proiect. Și versiunea Docker te anunță, cu comenzile de rulat.

  <details open><summary>Vezi în acțiune</summary>
  <img src="../media/ro/updates.avif" alt="Insigna „Actualizare”, confirmarea, apoi instalarea">
  </details>

### Și în plus

- Ca alte instrumente de minare: cufere bonus, raiduri, momente, obiective ale comunității,
  predicții (strategii, momentul pariului, condiții), prioritate pentru seriile de
  vizionare și pentru canalele cu abonament, prezență în chat (se poate dezactiva), opțiuni
  implicite pentru streamerii noi, notificări Discord, Telegram și webhook cu imagini, proxy
  global sau per cont.
- 29 de limbi, teme întunecată și luminoasă, notificări desktop, pictogramă în bara de
  sistem, pornire odată cu sistemul.

  <details open><summary>Vezi în acțiune</summary>
  <img src="../media/ro/basics.avif" alt="Limbi, temele luminoasă și întunecată, o notificare desktop și pictograma din bara de sistem">
  </details>

> [!WARNING]
> Acest proiect nu este afiliat cu Twitch. Automatizarea unui cont poate încălca termenii
> Twitch sau regulile unei campanii.

## Cuprins

- [Comparație](#comparison)
- [Descărcare](#download)
- [Docker](#docker)
- [Cum funcționează](#how-it-works)
- [Întrebări frecvente](#faq)
- [Proiecte înrudite](#related-projects)
- [Securitate](#security)
- [Compilare din sursă](#build)
- [Licență](#license)

## <a name="comparison"></a>Comparație

Cum se situează [Twitch Miner](https://github.com/SunkwiBOT/twitch-miner)
față de [Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
(puncte de canal) și [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner) (Drops).
Verificat în codul și în README-urile lor în noiembrie 2026.

| | Acest proiect | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Campanii Drops: le găsește, le minează și le revendică | ✅ | ❌ ¹ | ✅ |
| Puncte de canal: cufere, raiduri, momente, predicții | ✅ | ✅ | ❌ |
| Spam Points: o recompensă personalizată a unui canal revendicată de mai multe ori la rând | ✅ | ❌ | ❌ |
| Drops și puncte de canal simultan pe același cont | ✅ | parțial ¹ | ❌ |
| Mai multe conturi într-o singură aplicație | ✅ | ❌ ² | ❌ |
| Modificarea setărilor mai multor conturi deodată | ✅ | ❌ | ❌ |
| Interfață grafică | ✅ desktop și web ³ | ❌ ⁴ | ✅ desktop |
| Panou web pentru un server de acasă (Docker), cu parolă și accesibil prin HTTPS | ✅ | ❌ ⁴ | ❌ |
| Sesiuni criptate pe disc, parolă opțională a aplicației | ✅ | ❌ | ❌ |
| Interfață în 29 de limbi | ✅ | ❌ | parțial ⁵ |
| Actualizări semnate direct din aplicație (desktop) | ✅ | ❌ | ❌ |
| Asistent de configurare pentru fiecare cont nou | ✅ | ❌ ⁴ | ❌ |
| Streameri adăugați și editați din interfață, unul câte unul sau mai mulți deodată | ✅ | ❌ ⁴ | — |
| Streameri importați prin căutare, dintr-o listă, de la alt cont sau din canalele urmărite | ✅ | parțial ⁶ | — |
| Notificări Discord, Telegram și webhook, cu imagini | ✅ | parțial ⁷ | ❌ |
| Calendarul tuturor campaniilor Drops, cu căutare | ✅ | ❌ | parțial ⁸ |
| Ordine de minare pe campanie, cuvinte excluse, campanie sau recompensă ignorată | ✅ | ❌ | ❌ ⁹ |
| Streamerii din lista ta, urmăriți primii pentru Drops-urile unei campanii | ✅ | parțial ¹⁰ | ❌ |
| Jurnal de evenimente filtrabil și lista fiecărei cereri, în interfață | ✅ | ❌ ⁴ | ❌ |
| În jur de 30 MB de memorie (server Docker, 4 conturi) | ✅ | — ¹¹ | — ¹¹ |

¹ Urmărește doar streamerii din lista lui: avansează Drops-urile din streamurile lor și le
revendică, fără să aleagă o campanie.\
² Un proces per cont.\
³ Pe PC-ul tău ca aplicație desktop și/sau pe un server (VPS, NAS, Raspberry Pi) cu
Docker, fără mediu grafic și fără browser pe acea mașină: panoul se deschide în orice
browser.\
⁴ Configurat într-un fișier Python; pagina sa web arată grafice cu puncte și fișierul de
log.\
⁵ 21 de limbi.\
⁶ Poate urmări canalele pe care le urmărește contul.\
⁷ Doar text, fără imagini. Are și Matrix, Pushover și Gotify, pe care acest proiect nu le
are.\
⁸ O listă de campanii cu filtre, fără calendar și fără căutare.\
⁹ Are jocuri prioritare și jocuri excluse, alese după numele jocului.\
¹⁰ Implicit, urmărește întâi streamerii din lista lui al căror stream are Drops
(prioritatea lui `DROPS`), dar niciodată alt canal.\
¹¹ Nemăsurat.

## <a name="download"></a>Descărcare

Descarcă fișierul pentru sistemul tău din
[cea mai recentă versiune](https://github.com/SunkwiBOT/twitch-miner/releases/latest).

| Sistem | Pachet |
|---|---|
| **Windows** · x64 | Program de instalare (`setup.exe`) sau [executabil portabil](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/TwitchMiner-portable.exe) |
| **macOS** 10.15 sau mai nou · Intel și Apple Silicon | Imagine de disc (`.dmg`) |
| **Linux** · x86-64 | `.AppImage`, `.deb` (Debian, Ubuntu) sau `.rpm` (Fedora, openSUSE) |

Versiunea pentru macOS nu este notarizată. Prima dată, fă clic dreapta pe aplicație și
alege **Deschide**.

Actualizările sunt semnate. Aplicația verifică la pornire dacă există o versiune nouă, apoi
la fiecare oră (frecvența o poți schimba în **Setări › Despre**). Nu instalează niciodată
singură o versiune: o instalezi tu cu un singur clic, din mesajul care apare sau din
**Setări › Despre**.

Datele se află în dosarul de date al utilizatorului:

- Linux: `~/.local/share/twitch-miner`
- Windows: `%APPDATA%\twitch-miner`
- macOS: `~/Library/Application Support/twitch-miner`

## <a name="docker"></a>Docker

Serverul afișează același panou într-un browser. Imaginea este
`ghcr.io/sunkwibot/twitch-miner`, pentru `amd64` și `arm64`. Când apare o
versiune nouă, panoul te anunță: `docker compose pull`, apoi `docker compose up -d`.

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-miner:latest
    pull_policy: always   # descarcă cea mai nouă imagine la fiecare pornire
    restart: unless-stopped
    ports: ["8080:8080"]   # în spatele unui reverse proxy HTTPS: de eliminat, sau "127.0.0.1:8080:8080"
    environment:
      MINER_PASSWORD: "change-me"   # obligatorie, cel puțin 12 caractere
      # MINER_PUBLIC_URL: "https://miner.example.com"   # doar în spatele unui reverse proxy HTTPS
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

Deschide <http://localhost:8080> (sau adresa mașinii în rețeaua ta) și introdu parola.

### Variabile

| Variabilă | Implicit | Rol |
|---|---|---|
| `MINER_PASSWORD` | niciuna | Parola panoului, obligatorie: fără ea, serverul refuză să pornească. De cel puțin 12 caractere (una mai scurtă afișează un avertisment la pornire). Valoarea de exemplu `change-me` este refuzată. |
| `MINER_PASSWORD_FILE` | niciuna | Citește `MINER_PASSWORD` dintr-un fișier, de exemplu un secret Docker. Una sau alta, nu amândouă. |
| `MINER_PUBLIC_URL` | niciuna | Doar în spatele unui reverse proxy HTTPS (Caddy, nginx, Traefik, Cloudflare Tunnel…): adresa `https://` a panoului, fără cale, de exemplu `https://miner.example.com`. Proxy-ul trebuie să transmită `X-Forwarded-Proto`, `X-Forwarded-For` și websocketurile, ceea ce proxy-urile obișnuite fac implicit, iar portul containerului nu trebuie să fie accesibil direct. Panoul refuză atunci tot ce nu vine prin proxy, prin HTTPS, și blochează o adresă timp de 30 de secunde după 5 parole greșite. Proxy-ul trebuie să ruleze pe aceeași mașină sau în rețeaua ei locală: o conexiune de la o adresă publică este refuzată. |
| `MINER_TRUSTED_HOPS` | `1` | Doar cu `MINER_PUBLIC_URL`: câte proxy-uri adaugă o intrare în `X-Forwarded-For`, de la 1 la 10. `2` cu două proxy-uri unul în spatele celuilalt, de exemplu Cloudflare în fața Traefik. |
| `MINER_PROXY_NETWORKS` | niciuna | Doar cu `MINER_PUBLIC_URL`: alte rețele din care se poate conecta proxy-ul, pe lângă această mașină și rețeaua ei locală privată, separate prin virgule: o singură adresă sau o rețea ca `2001:db8:1::/64`, pentru o rețea locală cu adrese IPv6 globale. |
| `MINER_VAULT_PASSPHRASE` | niciuna | Doar pentru un dosar de date copiat din aplicația desktop care are o parolă a aplicației: îl deblochează la pornire. Un server creat de la zero nu are niciodată nevoie de ea. |
| `MINER_VAULT_PASSPHRASE_FILE` | niciuna | Citește `MINER_VAULT_PASSPHRASE` dintr-un fișier. Una sau alta, nu amândouă. |
| `DATA_DIR` | `/data` în imagine, altfel `data` în directorul de lucru | Baza de date, fișierul cu cheia și logurile. |
| `LOG_DIR` | `$DATA_DIR/logs` | Fișierele de log: câte unul pe zi, se păstrează ultimele 14. |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | Adresa de ascultare: o adresă IP și un port de la 1 la 65535. |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | Nivelul de logare. |

Serverul citește și verifică variabilele o singură dată, la pornire. O valoare invalidă
oprește serverul cu un mesaj, în loc să treacă tacit la o valoare de rezervă.

## <a name="how-it-works"></a>Cum funcționează

La fiecare 20 de secunde, fiecare cont își alege cele două canale: cel mai bun canal live
pentru prima campanie din coada sa, apoi streamerii din lista sa de puncte.

Twitch creditează simultan, per cont, **o singură campanie Drops** și **două canale pentru
puncte**. Din octombrie 2026, un drop poate fi revendicat doar dacă este conectat contul de
joc. Campaniile neconectate progresează oricum, iar panoul afișează linkul de conectare.

Lista campaniilor vine de la proiectul [Twitch Drops API](https://drops.sunkwi.com)
(reîmprospătată la fiecare 10 minute), pentru că Twitch o ascunde de clientul TV.

## <a name="faq"></a>Întrebări frecvente

### Riscuri și Twitch

**Îmi poate fi banat contul?**\
Nimeni nu poate garanta că nu. Acest proiect nu este afiliat cu Twitch, iar automatizarea
unui cont poate încălca termenii săi sau regulile unei campanii. Minerul urmărește cel mult
două canale per cont, limita Twitch, fără să descarce vreun videoclip. Decizia îți
aparține.

**Chiar se urmărește videoclipul?**\
Nu, nu se descarcă niciun videoclip. Pentru Drops, la fiecare 15 secunde minerul citește
playlistul streamului la cea mai mică calitate și cere doar antetul fiecărui segment nou.
Pentru puncte, trimite în fiecare minut semnalul Twitch de „minut vizionat”. Fiecare canal
urmărit arată câte segmente au fost cerute.

**De ce cel mult două canale?**\
Aceasta e limita Twitch: creditează doar două canale urmărite în același timp per cont și
o singură campanie Drops odată. În modul **Drops și puncte**, un canal servește campania,
iar celălalt streamerii tăi. Setarea „Un canal”, din **Setări cont**, e mai discretă.

### Drops

**O campanie nu este minată sau un drop nu avansează.**\
Deschide pagina **Drops** a contului. În „Ordinea de minare”, fiecare campanie în așteptare
spune de ce: „Niciun canal live”, „În așteptare (o campanie pe rând)” sau „Niciun drop
disponibil încă”. O campanie care nu apare în acea listă a fost lăsată deoparte: de
setarea „Ce campanii”, de o regulă din fila **Excluderi** sau pentru că nu se poate
termina la timp. O recompensă care cere un abonament nu este minată niciodată, iar un drop
care urmează după altul așteaptă ca primul să se termine. În modul **Doar puncte**, minerul
nu alege nicio campanie. „Minează cu prioritate”, în fișa unei campanii, o pune pe primul
loc; o campanie ignorată poate reveni cu „Minează din nou”, din fila **Excluderi**.

**Trebuie să conectez contul de joc?**\
Pentru revendicare, da: din octombrie 2026, Twitch permite revendicarea unui drop doar
dacă contul de joc este conectat. Implicit, o campanie neconectată progresează oricum, iar
revendicarea așteaptă conectarea. Insigna „Neconectat” semnalează asta, iar „Conectează
contul de joc”, din fișa campaniei, deschide pagina de conectare. Ca să minezi doar
jocurile conectate, alege „Doar jocurile cu contul conectat”.

### Instalare și date

**Aplicație desktop sau Docker?**\
Aplicația desktop minează cât timp PC-ul tău e pornit, cu pictograma din bara de sistem, o
parolă opțională și actualizări dintr-un clic. Docker minează zi și noapte pe o mașină
mereu pornită (VPS, NAS, Raspberry Pi); îl deschizi într-un browser, protejat de
`MINER_PASSWORD`. Interfața este aceeași.

**Unde sunt datele mele?**\
În dosarul de date: `~/.local/share/twitch-miner` pe Linux,
`%APPDATA%\twitch-miner` pe Windows,
`~/Library/Application Support/twitch-miner` pe macOS și volumul `/data` cu
Docker. Pe desktop, **Setări › Desktop** îl deschide dintr-un clic. Conține `miner.db`
(conturi, setări, jurnal), `master.key` (cheia sesiunilor, când nu e setată nicio parolă a
aplicației) și logurile.

**Cum actualizez?**\
Aplicația desktop verifică la pornire dacă există o versiune nouă, apoi la fiecare oră
(poți schimba asta în **Setări › Despre**). Dă clic pe insigna „Actualizare”, apoi pe
„Instalează și repornește”. Cu Docker, panoul te anunță: rulează `docker compose pull`,
apoi `docker compose up -d`.

**Am uitat parola aplicației.**\
Nu poate fi recuperată: ea criptează sesiunile tale Twitch. Închide aplicația, șterge
dosarul de date, apoi adaugă din nou conturile; setările lor trebuie refăcute. Cu Docker,
parola panoului se schimbă în `MINER_PASSWORD`, urmată de `docker compose up -d`.

### Puncte și predicții

**Câte puncte pe oră?**\
Depinde de Twitch și de fiecare canal. Minerul câștigă ce îi dă Twitch unui spectator:
punctele pentru timpul de vizionare, cuferele bonus (cam unul la fiecare 15 minute) și
bonusul pentru seria de vizionare. Un abonament înmulțește punctele, de la ×1,2 la ×2, în
funcție de nivel. Pagina Acasă arată punctele câștigate, iar Jurnalul, fiecare câștig.

**Sunt riscante pariurile?**\
Da, se pot pierde puncte. Predicțiile sunt dezactivate implicit, streamer cu streamer. Tu
alegi strategia, momentul pariului, miza (5 % din sold implicit, cel mult 50.000 de
puncte), un sold minim și, dacă vrei, o condiție, de exemplu o cotă de cel puțin 1,5.
Păstrează miza mică.

**Un streamer nu este urmărit.**\
Sunt urmăriți doar streamerii activați și live, și doar pe un canal liber: în modul
**Drops și puncte**, unul dintre cele două servește campania curentă; în modul
**Doar Drops**, nu este urmărit niciun streamer; un cont pus pe pauză nu urmărește nimic.
Locurile se ocupă în această ordine, afișată ca insignă pe fiecare canal urmărit: „Bonus
de serie”, „Serie în pericol”, „Abonat”, apoi „Ordinea listei”. Mută streamerul mai sus în
listă sau dezactivează „Seriile de vizionare mai întâi” și „Canalele cu abonament mai
întâi”.

## <a name="related-projects"></a>Proiecte înrudite

Alte proiecte ale aceluiași autor:

- [**Twitch Drops API**](https://drops.sunkwi.com): un API public și gratuit cu toate
  campaniile Twitch Drops și de recompense în desfășurare, actualizat în fiecare minut.
  Această aplicație își citește de acolo catalogul de campanii.
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com): un bot Discord care anunță pe serverul
  tău live-urile Twitch și campaniile Drops.
- [**Kick Alerts 🔔**](https://kick.sunkwi.com): un bot Discord care anunță pe serverul tău
  live-urile Kick și campaniile Drops.

## <a name="security"></a>Securitate

- Tokenurile sunt sigilate cu AES-256-GCM. Cheia este fie un `master.key` aleator lângă
  baza de date (implicit, nimic de făcut), fie derivată din parola aplicației cu Argon2id.
- Panoul HTTP verifică parola în timp constant și folosește cookie-uri HttpOnly,
  SameSite=Strict (`Secure`, cu prefixul `__Host-`, în spatele HTTPS). Serverul păstrează
  doar un hash SHA-256 al fiecărei sesiuni. Încheie o sesiune după 30 de zile sau la
  deconectare. Verifică sesiunea înainte de a citi orice corp de cerere și limitează
  corpurile la 256 KiB.
- După 5 parole greșite la rând de la aceeași adresă IP (formularul de autentificare sau
  bearer), acea adresă așteaptă 30 de secunde. Serverul răspunde `429` cu `Retry-After`,
  fără să se uite la parolă. În plus, fiecare parolă greșită primește răspuns abia după
  o jumătate de secundă.
- În afara rețelei tale, pune panoul în spatele unui reverse proxy HTTPS cu
  `MINER_PUBLIC_URL` (vezi [Docker](#docker)).
- Minerul comunică doar cu `twitch.tv`, `ttvnw.net`, `jtvnw.net`, `twitchcdn.net`,
  `drops.sunkwi.com` și cu destinațiile de notificare pe care le configurezi.
  Aplicația (desktop și Docker) verifică versiunile noi pe `github.com`, iar aplicația
  desktop le descarcă din versiunile proiectului pe GitHub. Sunt semnate: se instalează
  doar versiunea afișată.

## <a name="build"></a>Compilare din sursă

**Cerințe:** Rust, Node.js și
[cerințele preliminare Tauri](https://v2.tauri.app/start/prerequisites/) pentru sistemul
tău.

```bash
npm ci
npm run build          # panoul
cargo build --release -p miner-server   # serverul: target/release/twitch-miner
npm run bundle:linux   # pachetele desktop (necesită dependențele Tauri pentru Linux)
```

Verificări ale codului:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier, oxlint (cu analiză de tipuri) și svelte-check
```

Tipurile TypeScript ale interfeței sunt generate din tipurile Rust cu
[ts-rs](https://github.com/Aleph-Alpha/ts-rs) în `ui/src/lib/bindings`. După o modificare
a unui tip pe care îl vede interfața, rulează `npm run bindings`.

## <a name="license"></a>Licență

[MIT](../../LICENSE). Inspirat de [TwitchDropsMiner al lui DevilXD](https://github.com/DevilXD/TwitchDropsMiner),
[Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
și de forkurile lor.
