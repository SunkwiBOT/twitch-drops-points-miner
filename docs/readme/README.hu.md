<p align="center">
  <img src="../../ui/public/app-icon.svg" width="112" height="112" alt="A Twitch Drops &amp; Points Miner logója">
</p>

<h1 align="center">Twitch Drops &amp; Points Miner</h1>

<p align="center">
  <strong>Twitch Drops-ot és csatornapontokat bányászik több fiókon, böngésző és nyitott Twitch-lap nélkül.</strong>
</p>

<p align="center">
  Asztali alkalmazás Linuxra, Windowsra és macOS-re, vagy Docker-image ugyanazzal a webes irányítópulttal, otthoni szerveren vagy Raspberry Pi-on való futtatásra. 4 fiókkal a szerver nagyjából 30 MB memóriát használ, és alig terheli a processzort.
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-drops-points-miner?label=release&color=9146FF" alt="Legújabb kiadás"></a>
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-drops-points-miner/total?label=downloads&color=9146FF" alt="Letöltések"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="Platformok: Linux, Windows, macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-drops-points-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="Docker-image amd64 és arm64 architektúrára"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="MIT licenc"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="Megtekintések">
</p>

<p align="center">
  <a href="#download"><strong>Letöltés</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/issues/new/choose">Hiba bejelentése</a>
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
  <strong>Magyar</strong> ·
  <a href="README.ro.md">Română</a> ·
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
  <img src="../media/hu/presentation.avif" alt="A Twitch Drops &amp; Points Miner videós bemutatója">
</p>

## <a name="features"></a>Funkciók

Ami megkülönbözteti a többi bányásztól. Az egyes pontok alatt a „Működés közben” résznél
láthatod, hogyan működnek.

### Fiókok és streamerek

- **Több fiók egy alkalmazásban.** Minden fiók önállóan bányászik, a saját módjában:
  **Drops és pontok** (egy csatorna a legjobb kampányért, a másik a streamereidért),
  **Csak Drops** vagy **Csak pontok**.

  <details open><summary>Működés közben</summary>
  <img src="../media/hu/accounts-mining.avif" alt="Több fiók bányászik egyszerre a Kezdőlapon">
  </details>

- **Varázsló minden új fiókhoz.** Bejelentkezés után indulj ki az alapértelmezett
  beállításokból vagy egy másik fiók beállításaiból. Válaszd ki a módot, a kampányokat és
  a streamereket, majd „Bányászás indítása” vagy „Mentés és indítás később”.

  <details open><summary>Működés közben</summary>
  <img src="../media/hu/account-setup.avif" alt="Egy új fiók beállítási varázslója, lépésről lépésre">
  </details>

- **Egy módosítás más fiókokra is, egyetlen kattintással.** Ha egy fiókon módosítasz
  valamit, a megerősítő üzenet felajánlja az „Alkalmazás más fiókokra” lehetőséget.
  A módosítás a bejelölt fiókokon is megtörténik, a többi beállításuk pedig változatlan
  marad.

  <details open><summary>Működés közben</summary>
  <img src="../media/hu/apply-to-accounts.avif" alt="Más fiókokon is elvégzett módosítás az „Alkalmazás más fiókokra” gombbal">
  </details>

- **Streamer hozzáadása.** Azonnal elindul az alapértelmezett beállításokkal: az
  alkalmazáséval, vagy azokkal, amelyeket ehhez a fiókhoz választottál. Ha kell, rögtön
  módosíthatod őket: bónuszládák, raidek, pillanatok, nézési sorozat, chat, előrejelzések.

  <details open><summary>Működés közben</summary>
  <img src="../media/hu/streamer-add.avif" alt="Streamer hozzáadása a Twitch-keresővel, majd a beállításai">
  </details>

- **Streamer szerkesztése.** A „Szerkesztés” megnyitja a beállításait: bónuszládák, raidek,
  pillanatok, nézési sorozat, chat, előrejelzések… Egy kapcsolóval be- vagy kikapcsolhatod
  anélkül, hogy eltávolítanád a listáról.

  <details open><summary>Működés közben</summary>
  <img src="../media/hu/streamer-edit.avif" alt="Egy streamer beállításai, a sorából megnyitva">
  </details>

- **Több streamer szerkesztése egyszerre.** Jelöld be őket, majd
  „N kiválasztott streamer szerkesztése”. Csak a módosított beállítások érvényesülnek. Az
  a beállítás, amely eltér közöttük, „Vegyes” értéket mutat, a kapcsolója pedig két gombbá
  válik: „Igen” és „Nem”. Ezután az alkalmazás felajánlja, hogy ezek legyenek az új
  streamerek alapértelmezett beállításai.

  <details open><summary>Működés közben</summary>
  <img src="../media/hu/streamers-group-edit.avif" alt="Több streamer szerkesztése „Vegyes” beállításokkal">
  </details>

- **Streamerek importálása.** Illessz be egy listát (legfeljebb 500 névvel), vagy vedd át
  egy másik fiók listáját, illetve a fiók által követett csatornákat. A listát ellenőrzi,
  így csak létező csatornák kerülnek fel.

  <details open><summary>Működés közben</summary>
  <img src="../media/hu/streamers-import.avif" alt="Streamerek importálása: beillesztett lista, másik fiók, követett csatornák">
  </details>

- **Spam Points.** Válts be egy csatorna egyedi jutalmát többször egymás után. Válassz
  egy csatornát a Pontok listádról, vagy bármelyik csatornát, amelyet a Twitch-keresővel
  találsz: a jutalmai a fiók élőben frissülő egyenlegével együtt jelennek meg. Állítsd be
  a mennyiséget (legfeljebb 25-öt, az egyenleg erejéig), majd „Beváltás”. Az eredmény
  a sorozat végén jelenik meg, és minden sorozatnak saját sora van a Naplóban.

  <details open><summary>Működés közben</summary>
  <img src="../media/hu/spam-points.avif" alt="Spam Points: egy kiválasztott csatorna, 10 × beváltott üzenetes jutalom, a beváltások megjelennek a csatorna Twitch-csevegőjében, majd a Napló sora">
  </details>

### Drops

- **Minden futó Drops-kampány.** Minden kampánynak saját adatlapja van: jutalmak,
  játékfiók összekapcsolása, „Bányászás elsőként”. A lista a
  [Twitch Drops API](https://drops.sunkwi.com) projektből származik, és 10 percenként
  frissül.

  <details open><summary>Működés közben</summary>
  <img src="../media/hu/drops-catalogue.avif" alt="A Katalógus naptárnézetben, a keresés és egy kampány adatlapja">
  </details>

- **A saját bányászási sorrended.** Húzd a kampányokat a kívánt sorrendbe. Az újak maguktól
  a helyükre kerülnek: előbb a prioritást élvező játékaid, aztán a leghamarabb záruló vagy
  a legkevesebb hátralévő percet igénylő kampány. Zárj ki játékokat, szavakat, egy kampányt
  vagy egyetlen jutalmat, és hagyd ki azokat a kampányokat, amelyek nem fejezhetők be
  időben. A kampány adatlapján a „Bányászás elsőként” a lista élére teszi.

  <details open><summary>Működés közben</summary>
  <img src="../media/hu/drops-order.avif" alt="A bányászási sorrend, a prioritás játékok és a kizárások">
  </details>

- **Leltár és „Összes begyűjtése”.** A Leltár oldal a jutalmakat a „Begyűjthető”,
  a „Folyamatban” és a „Begyűjtve” csoportba sorolja. Gyűjtsd be őket egyenként, egyszerre
  mindet, vagy kapcsold be a „Drops automatikus begyűjtése” beállítást.

  <details open><summary>Működés közben</summary>
  <img src="../media/hu/drops-inventory.avif" alt="A leltár és az „Összes begyűjtése” gomb">
  </details>

- **„Saját streamereim előre a Dropsnál”.** Az aktuális kampánynál a listádon szereplő
  streamer, akinek az adása beleszámít ebbe a kampányba, megelőzi a többi csatornát.
  Ugyanaz a csatorna viszi előre a dropot, és hozza neked a csatornapontjait. Alapértelmezés
  szerint be van kapcsolva.

  <details open><summary>Működés közben</summary>
  <img src="../media/hu/drops-streamers-first.avif" alt="A listán szereplő streamer, akit a Dropsért és a pontjaiért is néz">
  </details>

### Nyomon követés

- **Napló mindenről, és minden kérés.** A Napló minden eseményt megőriz: dropok, szerzett
  és elköltött pontok, ládák, fogadások, raidek, induló adások… Szűrheted fiók, lap (Drops,
  Pontok, Egyéb) és típus szerint. A **Technikai logok › Kérések** felsorol minden kérést,
  amelyet az alkalmazás a Twitchnek és más szolgáltatásoknak küldött, az eredményével és
  az időtartamával. A logok és a kérések nem tartalmaznak tokent vagy titkot: a
  „Másolás hibajelentéshez” kimásolja őket, készen arra, hogy beilleszd egy hibajelentésbe
  a [GitHubon](https://github.com/SunkwiBOT/twitch-drops-points-miner/issues) vagy
  a [Discordon](https://discord.gg/support-bots-891685895781113886).

  <details open><summary>Működés közben</summary>
  <img src="../media/hu/journal-requests.avif" alt="A szűrt Napló, majd a Technikai logok › Kérések">
  </details>

### Asztalon vagy szerveren

- **Mindenhol ugyanaz a felület.** Az asztali alkalmazás (Linux, Windows, macOS) és
  a Docker-image ugyanazt az irányítópultot mutatja. A Docker VPS-en, NAS-on vagy
  Raspberry Pi-on fut, azon a gépen képernyő és böngésző nélkül: az irányítópult bármely
  böngészőben megnyílik.
- **Könnyű.** Nagyjából 30 MB memória 4 fiókhoz, szinte semmi processzorhasználat, és egy
  nagyjából 12 MB-os Docker-image.

### Biztonság és frissítések

- **Védett munkamenetek.** Nincs Twitch-jelszó: a böngésződből jelentkezel be
  a `twitch.tv/activate` oldalon. A munkamenetek titkosítva vannak a lemezen, és
  a [Twitch-beállításaidban](https://www.twitch.tv/settings/connections) vissza is vonhatod
  őket. Asztali gépen egy opcionális alkalmazásjelszó zárolja az alkalmazást: amíg be nem
  írod, semmit sem bányászik. Dockerrel az irányítópultnak saját jelszava van, néhány perc
  tétlenség után magától zárolódik, és proxy mögött HTTPS-en fut. Öt egymást követő hibás
  jelszó 30 másodpercre letiltja az IP-címet.

  <details open><summary>Működés közben</summary>
  <img src="../media/hu/security-lock.avif" alt="A lakat: a zárolt alkalmazás, majd feloldása a jelszavával">
  </details>

- **Aláírt frissítések, sosem automatikusan.** A felső sávban megjelenik egy „Frissítés”
  jelvény a verziószámmal. A „Telepítés és újraindítás” először megerősítést kér, majd az
  alkalmazás telepíti a verziót és újraindul. Alkalmazásjelszóval zárolva indul újra: írd
  be újra, hogy folytatódjon a bányászat. Csak a projekt által aláírt verzió települ.
  A Docker-verzió is szól, a futtatandó parancsokkal együtt.

  <details open><summary>Működés közben</summary>
  <img src="../media/hu/updates.avif" alt="A „Frissítés” jelvény, a megerősítés, majd a telepítés">
  </details>

### És még

- Mint más bányászokban: bónuszládák, raidek, pillanatok, közösségi célok, előrejelzések
  (stratégiák, a fogadás időzítése, feltételek), a nézési sorozatok és az előfizetett
  csatornák előnyben részesítése, jelenlét a chatben (kikapcsolható), alapértelmezett
  beállítások az új streamerekhez, Discord-, Telegram- és webhook-értesítések képekkel,
  globális vagy fiókonkénti proxy.
- 29 nyelv, sötét és világos téma, asztali értesítések, tálcaikon, indítás a rendszerrel.

  <details open><summary>Működés közben</summary>
  <img src="../media/hu/basics.avif" alt="Nyelvek, világos és sötét téma, egy asztali értesítés és a tálcaikon">
  </details>

> [!WARNING]
> Ez a projekt nem áll kapcsolatban a Twitchcsel. Egy fiók automatizálása sértheti
> a Twitch felhasználási feltételeit vagy egy kampány szabályait.

## Tartalom

- [Összehasonlítás](#comparison)
- [Letöltés](#download)
- [Docker](#docker)
- [Hogyan működik](#how-it-works)
- [Gyakori kérdések](#faq)
- [Kapcsolódó projektek](#related-projects)
- [Biztonság](#security)
- [Fordítás forráskódból](#build)
- [Licenc](#license)

## <a name="comparison"></a>Összehasonlítás

Hogyan áll a [Twitch Drops & Points Miner](https://github.com/SunkwiBOT/twitch-drops-points-miner)
a [Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
(csatornapontok) és a [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner) (Drops)
mellett. A kódjuk és a README-jük alapján ellenőrizve, 2026 novemberében.

| | Ez a projekt | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Drops-kampányok: megtalálja, bányássza és begyűjti őket | ✅ | ❌ ¹ | ✅ |
| Csatornapontok: ládák, raidek, pillanatok, előrejelzések | ✅ | ✅ | ❌ |
| Spam Points: egy csatorna egyedi jutalma többször egymás után beváltva | ✅ | ❌ | ❌ |
| Drops és csatornapontok egyszerre egy fiókon | ✅ | részben ¹ | ❌ |
| Több fiók egy alkalmazásban | ✅ | ❌ ² | ❌ |
| Több fiók beállításainak módosítása egyszerre | ✅ | ❌ | ❌ |
| Grafikus felület | ✅ asztali és webes ³ | ❌ ⁴ | ✅ asztali |
| Webes irányítópult otthoni szerverre (Docker), jelszóval védve, HTTPS-en elérhető | ✅ | ❌ ⁴ | ❌ |
| Lemezen titkosított munkamenetek, opcionális alkalmazásjelszó | ✅ | ❌ | ❌ |
| Felület 29 nyelven | ✅ | ❌ | részben ⁵ |
| Aláírt frissítések közvetlenül az alkalmazásból (asztali) | ✅ | ❌ | ❌ |
| Beállítási varázsló minden új fiókhoz | ✅ | ❌ ⁴ | ❌ |
| Streamerek hozzáadása és szerkesztése a felületen, egyenként vagy többet egyszerre | ✅ | ❌ ⁴ | — |
| Streamerek importálása kereséssel, listából, másik fiókból vagy követett csatornákból | ✅ | részben ⁶ | — |
| Discord-, Telegram- és webhook-értesítések képekkel | ✅ | részben ⁷ | ❌ |
| Az összes Drops-kampány naptára, kereséssel | ✅ | ❌ | részben ⁸ |
| Bányászási sorrend kampányonként, kizárt szavak, mellőzött kampány vagy jutalom | ✅ | ❌ | ❌ ⁹ |
| A listádon szereplő streamerek elsőként nézve egy kampány Dropsaiért | ✅ | részben ¹⁰ | ❌ |
| Szűrhető eseménynapló és az összes kérés listája a felületen | ✅ | ❌ ⁴ | ❌ |
| Nagyjából 30 MB memória (Docker-szerver, 4 fiók) | ✅ | — ¹¹ | — ¹¹ |

¹ Csak a listáján szereplő streamereket nézi: az ő adásaik Dropsait viszi előre és gyűjti
be, kampányt nem választ.\
² Fiókonként egy folyamat.\
³ A gépeden asztali alkalmazásként, és/vagy egy szerveren (VPS, NAS, Raspberry Pi)
Dockerrel, grafikus környezet és böngésző nélkül azon a gépen: az irányítópult bármely
böngészőben megnyílik.\
⁴ Egy Python-fájlban konfigurálható; a weboldala pontgrafikonokat és a logfájlt mutatja.\
⁵ 21 nyelv.\
⁶ Követni tudja azokat a csatornákat, amelyeket a fiók követ.\
⁷ Csak szöveg, képek nélkül. Van benne Matrix, Pushover és Gotify is, amelyek ebben
a projektben nincsenek.\
⁸ Kampánylista szűrőkkel, naptár és keresés nélkül.\
⁹ Vannak prioritás és kizárt játékai, játéknév alapján kiválasztva.\
¹⁰ Alapértelmezés szerint először a listáján szereplő azon streamereket nézi, akiknek az
adásában van Drops (a `DROPS` prioritása), de más csatornát soha.\
¹¹ Nem mértük.

## <a name="download"></a>Letöltés

Töltsd le a rendszerednek megfelelő fájlt a
[legújabb kiadásból](https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest).

| Rendszer | Csomag |
|---|---|
| **Windows** · x64 | Telepítő (`setup.exe`) vagy [hordozható futtatható fájl](https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest/download/TwitchDropsPointsMiner-portable.exe) |
| **macOS** 10.15 vagy újabb · Intel és Apple Silicon | Lemezkép (`.dmg`) |
| **Linux** · x86-64 | `.AppImage`, `.deb` (Debian, Ubuntu) vagy `.rpm` (Fedora, openSUSE) |

A macOS-es build nincs hitelesítve (notarization). Első alkalommal kattints jobb gombbal az
alkalmazásra, és válaszd a **Megnyitás** lehetőséget.

A frissítések alá vannak írva. Az alkalmazás induláskor, majd óránként ellenőrzi, van-e új
verzió (a gyakoriságot a **Beállítások › Névjegy** helyen módosíthatod). Magától soha nem
telepít: egy kattintással telepítheted a megjelenő üzenetből vagy a **Beállítások › Névjegy**
helyen.

Az adatok a felhasználói adatmappában vannak:

- Linux: `~/.local/share/twitch-drops-points-miner`
- Windows: `%APPDATA%\twitch-drops-points-miner`
- macOS: `~/Library/Application Support/twitch-drops-points-miner`

## <a name="docker"></a>Docker

A szerver ugyanazt az irányítópultot mutatja böngészőben. Az image
a `ghcr.io/sunkwibot/twitch-drops-points-miner`, `amd64` és `arm64` architektúrára. Ha új
verzió jelenik meg, az irányítópult jelzi: `docker compose pull`, majd `docker compose up -d`.

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-drops-points-miner:latest
    pull_policy: always   # minden indításkor a legújabb image letöltése
    restart: unless-stopped
    ports: ["8080:8080"]   # HTTPS fordított proxy mögött: töröld, vagy "127.0.0.1:8080:8080"
    environment:
      MINER_PASSWORD: "change-me"   # kötelező, legalább 12 karakter
      # MINER_PUBLIC_URL: "https://miner.example.com"   # csak HTTPS fordított proxy mögött
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

Nyisd meg a <http://localhost:8080> címet (vagy a gép címét a hálózatodon), és írd be
a jelszót.

### Változók

| Változó | Alapérték | Szerep |
|---|---|---|
| `MINER_PASSWORD` | nincs | Az irányítópult jelszava, kötelező: nélküle a szerver nem hajlandó elindulni. Legalább 12 karakter (rövidebb jelszónál induláskor figyelmeztetés jelenik meg). A `change-me` mintaérték nem fogadható el. |
| `MINER_PASSWORD_FILE` | nincs | A `MINER_PASSWORD` értékét fájlból olvassa be, például egy Docker-secretből. Csak az egyiket állítsd be, ne mindkettőt. |
| `MINER_PUBLIC_URL` | nincs | Csak HTTPS fordított proxy mögött (Caddy, nginx, Traefik, Cloudflare Tunnel…): az irányítópult `https://` címe, útvonal nélkül, például `https://miner.example.com`. A proxynak továbbítania kell az `X-Forwarded-Proto` és az `X-Forwarded-For` fejlécet, valamint a websocketeket, amit a szokásos proxyk alapból megtesznek, a konténer portja pedig nem lehet közvetlenül elérhető. Az irányítópult ezután elutasít mindent, ami nem a proxyn keresztül, HTTPS-en érkezik, és 5 hibás jelszó után 30 másodpercre kizárja az adott címet. A proxynak ugyanazon a gépen vagy annak helyi hálózatán kell futnia: nyilvános címről érkező kapcsolatot a szerver elutasít. |
| `MINER_TRUSTED_HOPS` | `1` | Csak `MINER_PUBLIC_URL` mellett: hány proxy ad hozzá bejegyzést az `X-Forwarded-For` fejléchez, 1 és 10 között. `2`, ha két proxy áll egymás mögött, például a Cloudflare a Traefik előtt. |
| `MINER_PROXY_NETWORKS` | nincs | Csak `MINER_PUBLIC_URL` mellett: további hálózatok, amelyekből a proxy csatlakozhat ezen a gépen és a privát helyi hálózatán kívül, vesszővel elválasztva: egyetlen cím vagy egy hálózat, például `2001:db8:1::/64`, olyan helyi hálózathoz, amelynek IPv6-címei globálisak. |
| `MINER_VAULT_PASSPHRASE` | nincs | Csak olyan adatmappához, amelyet beállított alkalmazásjelszavú asztali alkalmazásból másoltál át: induláskor feloldja. A nulláról felépített szervernek soha nincs rá szüksége. |
| `MINER_VAULT_PASSPHRASE_FILE` | nincs | A `MINER_VAULT_PASSPHRASE` értékét fájlból olvassa be. Csak az egyiket állítsd be, ne mindkettőt. |
| `DATA_DIR` | `/data` az image-ben, egyébként `data` a munkakönyvtárban | Adatbázis, kulcsfájl és logok. |
| `LOG_DIR` | `$DATA_DIR/logs` | Logfájlok: naponta egy, az utolsó 14 marad meg. |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | Figyelési cím: egy IP-cím és egy port 1 és 65535 között. |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | Naplózási szint. |

A szerver egyszer, induláskor olvassa be és ellenőrzi a változókat. Érvénytelen érték
esetén a szerver üzenettel leáll, ahelyett hogy csendben egy tartalék értékre váltana.

## <a name="how-it-works"></a>Hogyan működik

20 másodpercenként minden fiók kiválasztja a két csatornáját: a legjobb élő csatornát
a sora élén álló kampányhoz, utána a pontlistája streamereit.

A Twitch fiókonként egyszerre **egy Drops-kampányt** és **két csatornát a pontokért** ír
jóvá. 2026 októbere óta egy drop csak akkor gyűjthető be, ha a játékfiók össze van
kapcsolva. A nem összekapcsolt kampányok így is haladnak, és az irányítópult mutatja az
összekapcsolás linkjét.

A kampánylista a [Twitch Drops API](https://drops.sunkwi.com) projektből jön (10 percenként
frissül), mert a Twitch elrejti a TV-s kliens elől.

## <a name="faq"></a>Gyakori kérdések

### Kockázatok és a Twitch

**Kitilthatják a fiókomat?**\
Senki sem ígérheti, hogy nem. Ez a projekt nem áll kapcsolatban a Twitchcsel, és egy fiók
automatizálása sértheti a Twitch feltételeit vagy egy kampány szabályait. A bányász
fiókonként legfeljebb két csatornát néz, ami a Twitch korlátja, és nem tölt le videót.
A döntés a tiéd.

**Tényleg nézi a videót?**\
Nem, semmilyen videó nem töltődik le. A Drops esetében a bányász 15 másodpercenként
beolvassa az adás lejátszási listáját a legalacsonyabb minőségben, és minden új szegmensnek
csak a fejlécét kéri le. A pontokhoz percenként elküldi a Twitch „megnézett perc” jelzését.
Minden nézett csatornánál látszik, hány szegmenst kért le.

**Miért legfeljebb két csatorna?**\
Ez a Twitch korlátja: fiókonként csak két egyszerre nézett csatornát ír jóvá, és egyszerre
csak egy Drops-kampányt. **Drops és pontok** módban az egyik csatorna a kampányt szolgálja,
a másik a streamereidet. Az „Egy csatorna” beállítás a **Fiókbeállítások** alatt
feltűnésmentesebb.

### Drops

**Egy kampányt nem bányászik, vagy egy drop nem halad.**\
Nyisd meg a fiók **Drops** oldalát. A „Bányászási sorrend” részben minden várakozó kampány
megmondja, miért vár: „Nincs élő csatorna”, „Várakozik (egyszerre egy kampány)” vagy
„Még nincs elérhető drop”. Az a kampány, amely nem szerepel ebben a listában, kimaradt: a
„Mely kampányok” beállítás miatt, a **Kizárások** lap egy szabálya miatt, vagy mert nem
fejezhető be időben. Az előfizetést igénylő jutalmat soha nem bányássza, a másikat követő
drop pedig megvárja, amíg az első elkészül. **Csak pontok** módban a bányász nem választ
kampányt. A kampány adatlapján a „Bányászás elsőként” a lista élére teszi; egy mellőzött
kampány az „Újbóli bányászás” gombbal térhet vissza, a **Kizárások** lapon.

**Össze kell kapcsolnom a játékfiókot?**\
A begyűjtéshez igen: 2026 októbere óta a Twitch csak akkor enged begyűjteni egy dropot, ha
a játékfiók össze van kapcsolva. Alapértelmezés szerint a nem összekapcsolt kampány így is
halad, a begyűjtés pedig megvárja az összekapcsolást. A „Nincs összekapcsolva” jelvény
jelzi ezt, a kampány adatlapján pedig a „Játékfiók összekapcsolása” megnyitja az
összekapcsolási oldalt. Ha csak összekapcsolt játékokat szeretnél bányászni, válaszd
a „Csak összekapcsolt fiókú játékok” lehetőséget.

### Telepítés és adatok

**Asztali alkalmazás vagy Docker?**\
Az asztali alkalmazás addig bányászik, amíg a géped be van kapcsolva, tálcaikonnal,
opcionális jelszóval és egykattintásos frissítésekkel. A Docker éjjel-nappal bányászik egy
mindig bekapcsolt gépen (VPS, NAS, Raspberry Pi); böngészőben nyitod meg, a `MINER_PASSWORD`
védelmével. A felület ugyanaz.

**Hol vannak az adataim?**\
Az adatmappában: Linuxon `~/.local/share/twitch-drops-points-miner`, Windowson
`%APPDATA%\twitch-drops-points-miner`, macOS-en
`~/Library/Application Support/twitch-drops-points-miner`, Dockerrel pedig a `/data` kötet.
Asztali gépen a **Beállítások › Asztal** egy kattintással megnyitja. Ebben van a `miner.db`
(fiókok, beállítások, napló), a `master.key` (a munkamenetek kulcsa, ha nincs
alkalmazásjelszó beállítva) és a logok.

**Hogyan frissíthetek?**\
Az asztali alkalmazás induláskor, majd óránként keres új verziót (ez a
**Beállítások › Névjegy** alatt módosítható). Kattints a „Frissítés” jelvényre, majd a
„Telepítés és újraindítás” gombra. Dockerrel az irányítópult szól: futtasd a
`docker compose pull`, majd a `docker compose up -d` parancsot.

**Elfelejtettem az alkalmazásjelszót.**\
Nem állítható vissza: épp ez titkosítja a Twitch-munkameneteidet. Lépj ki az
alkalmazásból, töröld az adatmappát, majd add hozzá újra a fiókjaidat; a beállításaikat
újra el kell végezni. Dockerrel az irányítópult jelszavát a `MINER_PASSWORD` változóban
módosíthatod, utána pedig `docker compose up -d`.

### Pontok és előrejelzések

**Hány pont jön óránként?**\
Ez a Twitchtől és az adott csatornától függ. A bányász azt kapja, amit a Twitch egy
nézőnek ad: nézési pontokat, bónuszládákat (nagyjából 15 percenként egyet) és a nézési
sorozat bónuszát. Az előfizetés megsokszorozza a pontokat, szinttől függően ×1,2 és ×2
között. A Kezdőlap a szerzett pontokat mutatja, a Napló pedig minden egyes nyereséget.

**Kockázatosak a fogadások?**\
Igen, pontokat lehet veszíteni. Az előrejelzések alapértelmezés szerint ki vannak
kapcsolva, streamerenként. Te állítod be a stratégiát, a fogadás időzítését, a tétet
(alapértelmezés szerint az egyenleg 5%-a, legfeljebb 50 000 pont), a minimális egyenleget
és, ha szeretnéd, egy feltételt, például legalább 1,5-ös szorzót. Tarts alacsony tétet.

**Egy streamert nem néz.**\
Csak a bekapcsolt és éppen élőben adó streamereket nézi, és csak szabad csatornán:
**Drops és pontok** módban a kettő közül az egyik az aktuális kampányt szolgálja;
**Csak Drops** módban egyetlen streamert sem néz; a szüneteltetett fiók semmit sem néz.
A helyeket ebben a sorrendben osztja ki, amelyet minden nézett csatornán egy jelvény
mutat: „Sorozatbónusz”, „Sorozat veszélyben”, „Előfizetve”, majd „Lista sorrendje”.
Mozgasd feljebb a streamert a listán, vagy kapcsold ki a „Nézési sorozatok előre” és az
„Előfizetett csatornák előre” beállítást.

## <a name="related-projects"></a>Kapcsolódó projektek

Ugyanennek a szerzőnek más projektjei:

- [**Twitch Drops API**](https://drops.sunkwi.com): ingyenes, nyilvános API az összes futó
  Twitch Drops- és jutalomkampányról, percenként frissítve. Ez az alkalmazás innen olvassa
  a kampánykatalógusát.
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com): Discord-bot, amely bejelenti a Twitch-es
  élő adásokat és Drops-kampányokat a szervereden.
- [**Kick Alerts 🔔**](https://kick.sunkwi.com): Discord-bot, amely bejelenti a Kick-es élő
  adásokat és Drops-kampányokat a szervereden.

## <a name="security"></a>Biztonság

- A tokeneket AES-256-GCM pecsételi le. A kulcs vagy egy véletlenszerű `master.key`
  az adatbázis mellett (alapértelmezés, felügyelet nélkül), vagy az alkalmazásjelszóból
  Argon2id-vel származtatott kulcs.
- A HTTP-irányítópult állandó idejű jelszó-ellenőrzést és HttpOnly, SameSite=Strict
  sütiket használ (HTTPS mögött `Secure`, `__Host-` előtaggal). A szerver minden
  munkamenetből csak egy SHA-256 kivonatot tárol. 30 nap után vagy kijelentkezéskor
  lezárja a munkamenetet. Bármely kéréstörzs beolvasása előtt ellenőrzi a munkamenetet, és
  a törzseket 256 KiB-ra korlátozza.
- Egy IP-címről érkező 5 egymást követő hibás jelszó után (bejelentkezési űrlap vagy
  bearer) az a cím 30 másodpercig vár. A szerver ilyenkor `429` kóddal és `Retry-After`
  fejléccel válaszol, a jelszó megnézése nélkül. Minden hibás jelszóra ráadásul csak fél
  másodperc múlva jön válasz.
- A hálózatodon kívül tedd az irányítópultot egy HTTPS fordított proxy mögé, a
  `MINER_PUBLIC_URL` beállításával (lásd: [Docker](#docker)).
- A bányász csak a `twitch.tv`, `ttvnw.net`, `jtvnw.net`, `twitchcdn.net`,
  `twitch-drops-api.sunkwi.com` címekkel és az általad beállított értesítési célokkal
  kommunikál. Az alkalmazás (asztali és Docker) a `github.com` oldalon keres új
  verziókat, az asztali alkalmazás pedig a projekt GitHub-kiadásaiból tölti le őket. Ezek
  alá vannak írva: csak a megjelenített verzió települ.

## <a name="build"></a>Fordítás forráskódból

**Előfeltételek:** Rust, Node.js és a rendszeredhez tartozó
[Tauri-előfeltételek](https://v2.tauri.app/start/prerequisites/).

```bash
npm ci
npm run build          # irányítópult
cargo build --release -p miner-server   # szerver: target/release/twitch-miner
npm run bundle:linux   # asztali csomagok (a Tauri linuxos függőségei kellenek hozzá)
```

Kódellenőrzések:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier, oxlint (típusérzékeny) és svelte-check
```

A felület TypeScript-típusait a Rust-típusokból generálja a
[ts-rs](https://github.com/Aleph-Alpha/ts-rs) az `ui/src/lib/bindings` mappába. Ha olyan
típust módosítasz, amelyet a felület lát, futtasd az `npm run bindings` parancsot.

## <a name="license"></a>Licenc

[MIT](../../LICENSE). [DevilXD TwitchDropsMinere](https://github.com/DevilXD/TwitchDropsMiner),
a [Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
és ezek forkjai ihlették.
