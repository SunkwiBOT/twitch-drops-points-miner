<p align="center">
  <img src="../../ui/public/app-icon.svg" width="112" height="112" alt="Twitch Minerin logo">
</p>

<h1 align="center">Twitch Miner</h1>

<p align="center">
  <strong>Louhii Twitch Drops -palkintoja ja kanavapisteitä usealla tilillä ilman selainta ja ilman avointa Twitch-välilehteä.</strong>
</p>

<p align="center">
  Työpöytäsovellus Linuxille, Windowsille ja macOS:lle tai Docker-image, jossa on sama verkkohallintapaneeli ja jota voi ajaa kotipalvelimella tai Raspberry Pi:llä. 4 tilillä palvelin käyttää noin 30 Mt muistia eikä juuri lainkaan suoritinta.
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-miner?label=release&color=9146FF" alt="Uusin julkaisu"></a>
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-miner/total?label=downloads&color=9146FF" alt="Lataukset"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="Alustat: Linux, Windows, macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="Docker-image alustoille amd64 ja arm64"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="MIT-lisenssi"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="Katselukerrat">
</p>

<p align="center">
  <a href="#download"><strong>Lataa</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-miner/issues/new/choose">Ilmoita bugista</a>
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
  <a href="README.ro.md">Română</a> ·
  <a href="README.bg.md">Български</a> ·
  <a href="README.el.md">Ελληνικά</a> ·
  <a href="README.tr.md">Türkçe</a> ·
  <a href="README.sv.md">Svenska</a> ·
  <a href="README.da.md">Dansk</a> ·
  <a href="README.nb.md">Norsk&nbsp;bokmål</a> ·
  <strong>Suomi</strong> ·
  <a href="README.zh-CN.md">简体中文</a> ·
  <a href="README.zh-TW.md">繁體中文</a> ·
  <a href="README.ja.md">日本語</a> ·
  <a href="README.ko.md">한국어</a> ·
  <a href="README.th.md">ไทย</a> ·
  <a href="README.vi.md">Tiếng&nbsp;Việt</a> ·
  <a href="README.id.md">Bahasa&nbsp;Indonesia</a>
</p>

<p align="center">
  <img src="../media/fi/presentation.avif" alt="Videoesittely Twitch Minerista">
</p>

## <a name="features"></a>Ominaisuudet

Nämä erottavat sen muista louhijoista. ”Katso toiminnassa” kohdan alla näyttää
ominaisuuden käytössä.

### Tilit ja streamaajat

- **Useita tilejä yhdessä sovelluksessa.** Jokainen tili louhii omillaan, omassa
  louhintatilassaan: **Drops ja pisteet** (yksi kanava parhaalle kampanjalle, toinen
  streamaajillesi), **Vain Drops** tai **Vain pisteet**.

  <details open><summary>Katso toiminnassa</summary>
  <img src="../media/fi/accounts-mining.avif" alt="Useita tilejä louhimassa samaan aikaan Etusivulla">
  </details>

- **Ohjattu käyttöönotto jokaiselle uudelle tilille.** Kirjautumisen jälkeen voit aloittaa
  oletusasetuksista tai toisen tilin asetuksista. Valitse louhintatila, kampanjat ja
  streamaajat, ja sitten ”Aloita louhinta” tai ”Tallenna ja aloita myöhemmin”.

  <details open><summary>Katso toiminnassa</summary>
  <img src="../media/fi/account-setup.avif" alt="Uuden tilin ohjattu käyttöönotto vaihe vaiheelta">
  </details>

- **Muutos muihin tileihin yhdellä napsautuksella.** Kun olet muuttanut jotain tilillä,
  vahvistusviesti tarjoaa vaihtoehdon ”Käytä muihin tileihin”. Muutos tehdään uudelleen
  valitsemillesi tileille, ja niiden muut asetukset säilyvät ennallaan.

  <details open><summary>Katso toiminnassa</summary>
  <img src="../media/fi/apply-to-accounts.avif" alt="Muutos tehdään uudelleen muille tileille toiminnolla ”Käytä muihin tileihin”">
  </details>

- **Lisää streamaaja.** Se aloittaa heti oletusvalinnoilla: sovelluksen omilla tai niillä,
  jotka olet valinnut tälle tilille. Säädä niitä tarvittaessa saman tien: bonusarkut,
  raidit, hetket, katseluputki, chat, ennustukset.

  <details open><summary>Katso toiminnassa</summary>
  <img src="../media/fi/streamer-add.avif" alt="Streamaajan lisääminen Twitch-haulla ja sen jälkeen sen valinnat">
  </details>

- **Muokkaa streamaajaa.** ”Muokkaa” avaa sen valinnat: bonusarkut, raidit, hetket,
  katseluputki, chat, ennustukset… Kytkin ottaa sen käyttöön tai poistaa sen käytöstä
  poistamatta sitä listalta.

  <details open><summary>Katso toiminnassa</summary>
  <img src="../media/fi/streamer-edit.avif" alt="Streamaajan valinnat avattuna sen riviltä">
  </details>

- **Muokkaa useita streamaajia kerralla.** Valitse ne ja sitten ”Muokkaa N valittua
  streamaajaa”. Vain muuttamasi valinnat otetaan käyttöön. Valinta, joka vaihtelee niiden
  välillä, näyttää tekstin ”Vaihtelee”, ja sen kytkin muuttuu kahdeksi painikkeeksi,
  ”Kyllä” ja ”Ei”. Sovellus tarjoutuu sen jälkeen tallentamaan ne uusien streamaajien
  oletusvalinnoiksi.

  <details open><summary>Katso toiminnassa</summary>
  <img src="../media/fi/streamers-group-edit.avif" alt="Useiden streamaajien muokkaus, valintoina ”Vaihtelee”">
  </details>

- **Tuo streamaajia.** Liitä lista (enintään 500 nimeä) tai käytä toisen tilin listaa tai
  tilin seuraamia kanavia. Lista tarkistetaan, jotta vain olemassa olevat kanavat
  lisätään.

  <details open><summary>Katso toiminnassa</summary>
  <img src="../media/fi/streamers-import.avif" alt="Streamaajien tuonti: liitetty lista, toinen tili, seuratut kanavat">
  </details>

- **Spam Points.** Lunasta kanavan oma palkinto useita kertoja peräkkäin. Valitse kanava
  Pisteet-listastasi tai mikä tahansa Twitch-haulla löytyvä kanava: sen palkinnot näkyvät
  tilin saldon kanssa, jota seurataan reaaliajassa. Aseta määrä (enintään 25, saldon
  rajoissa) ja valitse sitten ”Lunasta”. Tulos näkyy sarjan päätyttyä, ja jokaisesta
  sarjasta jää oma rivinsä Päiväkirjaan.

  <details open><summary>Katso toiminnassa</summary>
  <img src="../media/fi/spam-points.avif" alt="Spam Points: valittu kanava, 10 × lunastettu viestillinen palkinto, lunastukset ilmestyvät kanavan Twitch-chattiin ja sitten rivi Päiväkirjassa">
  </details>

### Drops

- **Kaikki käynnissä olevat Drops-kampanjat.** Jokaisella kampanjalla on omat tietonsa:
  palkinnot, pelitilin linkitys, ”Louhi ensin”. Lista tulee
  [Twitch Drops API](https://drops.sunkwi.com) -projektista ja päivittyy 10 minuutin
  välein.

  <details open><summary>Katso toiminnassa</summary>
  <img src="../media/fi/drops-catalogue.avif" alt="Luettelo kalenterina, haku ja kampanjan tiedot">
  </details>

- **Oma louhintajärjestyksesi.** Vedä kampanjat haluamaasi järjestykseen. Uudet löytävät
  paikkansa itse: ensin prioriteettipelisi, sitten lähimpänä päättyvät tai ne, joissa on
  vähiten minuutteja jäljellä. Sulje pois pelejä, sanoja, kampanja tai yksittäinen
  palkinto, ja ohita kampanjat, jotka eivät ehdi valmistua ajoissa. ”Louhi ensin”
  kampanjan tiedoissa nostaa sen kärkeen.

  <details open><summary>Katso toiminnassa</summary>
  <img src="../media/fi/drops-order.avif" alt="Louhintajärjestys, prioriteettipelit ja ohitukset">
  </details>

- **Varasto ja ”Lunasta kaikki”.** Varasto-sivu jakaa palkinnot ryhmiin
  ”Lunastettavissa”, ”Kesken” ja ”Lunastetut”. Lunasta ne yksi kerrallaan tai kaikki
  kerralla, tai ota käyttöön ”Lunasta dropsit automaattisesti”.

  <details open><summary>Katso toiminnassa</summary>
  <img src="../media/fi/drops-inventory.avif" alt="Varasto ja ”Lunasta kaikki” -painike">
  </details>

- **”Omat streamaajat ensin Dropseissa”.** Nykyisessä kampanjassa listasi streamaaja,
  jonka lähetys lasketaan kampanjaan, menee muiden kanavien edelle. Sama kanava edistää
  dropia ja kerryttää sinulle kanavansa pisteitä. Oletuksena käytössä.

  <details open><summary>Katso toiminnassa</summary>
  <img src="../media/fi/drops-streamers-first.avif" alt="Listan streamaaja, jota katsotaan sekä Dropsien että pisteiden vuoksi">
  </details>

### Seuranta

- **Päiväkirja kaikesta ja jokainen pyyntö.** Päiväkirja tallentaa jokaisen tapahtuman:
  dropit, ansaitut ja käytetyt pisteet, arkut, vedot, raidit, alkavat lähetykset…
  Suodata sitä tilin, välilehden (Drops, Pisteet, Muut) ja tyypin mukaan.
  **Tekniset lokit › Pyynnöt** listaa jokaisen Twitchille ja muille palveluille lähetetyn
  pyynnön tuloksineen ja kestoineen. Lokit ja pyynnöt eivät sisällä tokeneita tai
  salaisuuksia: ”Kopioi bugiraporttia varten” kopioi ne valmiiksi liitettäviksi
  bugiraporttiin
  [GitHubissa](https://github.com/SunkwiBOT/twitch-miner/issues) tai
  [Discordissa](https://discord.gg/support-bots-891685895781113886).

  <details open><summary>Katso toiminnassa</summary>
  <img src="../media/fi/journal-requests.avif" alt="Suodatettu Päiväkirja ja sitten Tekniset lokit › Pyynnöt">
  </details>

### Työpöydällä tai palvelimella

- **Sama käyttöliittymä kaikkialla.** Työpöytäsovellus (Linux, Windows, macOS) ja
  Docker-image näyttävät saman hallintapaneelin. Docker toimii VPS:llä, NAS-laitteella tai
  Raspberry Pi:llä ilman näyttöä tai selainta kyseisellä koneella: hallintapaneeli
  avautuu missä tahansa selaimessa.
- **Kevyt.** Noin 30 Mt muistia 4 tilille, tuskin lainkaan suoritinta ja noin 12 Mt:n
  Docker-image.

### Tietoturva ja päivitykset

- **Istuntosi suojattuina.** Ei Twitch-salasanaa: kirjaudut osoitteessa
  `twitch.tv/activate` omasta selaimestasi. Istunnot tallennetaan levylle salattuina, ja
  voit perua ne [Twitchin asetuksista](https://www.twitch.tv/settings/connections).
  Työpöydällä valinnainen sovelluksen salasana lukitsee sovelluksen: mitään ei louhita
  ennen kuin se on kirjoitettu. Dockerissa hallintapaneelilla on oma salasanansa, se
  lukittuu muutaman minuutin käyttämättömyyden jälkeen ja toimii HTTPS:n yli proxyn
  takana. Viisi väärää salasanaa peräkkäin estää IP-osoitteen 30 sekunniksi.

  <details open><summary>Katso toiminnassa</summary>
  <img src="../media/fi/security-lock.avif" alt="Riippulukko: sovellus lukittuna ja sitten avattuna salasanallaan">
  </details>

- **Allekirjoitetut päivitykset, ei koskaan automaattisesti.** Yläpalkkiin ilmestyy
  ”Päivitys”-merkki ja versionumero. ”Asenna ja käynnistä uudelleen” pyytää ensin
  vahvistuksen, minkä jälkeen sovellus asentaa version ja käynnistyy uudelleen. Jos
  sovelluksen salasana on asetettu, se käynnistyy lukittuna: kirjoita salasana uudelleen
  jatkaaksesi louhintaa. Vain projektin allekirjoittama versio asennetaan. Docker-versio
  ilmoittaa myös ja kertoo suoritettavat komennot.

  <details open><summary>Katso toiminnassa</summary>
  <img src="../media/fi/updates.avif" alt="”Päivitys”-merkki, vahvistus ja sitten asennus">
  </details>

### Lisäksi

- Kuten muissa louhijoissa: bonusarkut, raidit, hetket, yhteisötavoitteet, ennustukset
  (strategiat, panostuksen ajoitus, ehdot), katseluputket ja tilatut kanavat ensin,
  läsnäolo chatissa (voi poistaa käytöstä), uusien streamaajien oletusvalinnat, Discord-,
  Telegram- ja webhook-ilmoitukset kuvineen, yleinen tai tilikohtainen proxy.
- **Tulossa pian: merkkien louhinta.** Katsomalla ansaittavat Twitch-merkit louhitaan kuten Dropsit.
- 29 kieltä, tumma ja vaalea teema, työpöytäilmoitukset, ilmoitusalueen kuvake,
  käynnistys järjestelmän mukana.

  <details open><summary>Katso toiminnassa</summary>
  <img src="../media/fi/basics.avif" alt="Kielet, vaalea ja tumma teema, työpöytäilmoitus ja ilmoitusalueen kuvake">
  </details>

> [!WARNING]
> Tämä projekti ei ole yhteydessä Twitchiin. Tilin automatisointi voi rikkoa Twitchin
> käyttöehtoja tai kampanjan sääntöjä.

## Sisältö

- [Vertailu](#comparison)
- [Lataus](#download)
- [Docker](#docker)
- [Näin se toimii](#how-it-works)
- [Usein kysytyt kysymykset](#faq)
- [Liittyvät projektit](#related-projects)
- [Tietoturva](#security)
- [Kääntäminen lähdekoodista](#build)
- [Lisenssi](#license)

## <a name="comparison"></a>Vertailu

Näin
[Twitch Miner](https://github.com/SunkwiBOT/twitch-miner)
vertautuu
[Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
-ohjelmaan (kanavapisteet) ja
[TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner) -ohjelmaan (Drops).
Tarkistettu niiden koodista ja README-tiedostoista marraskuussa 2026.

| | Tämä projekti | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Drops-kampanjat: löytää, louhii ja lunastaa ne | ✅ | ❌ ¹ | ✅ |
| Kanavapisteet: arkut, raidit, hetket, ennustukset | ✅ | ✅ | ❌ |
| Spam Points: kanavan oma palkinto lunastettuna useita kertoja peräkkäin | ✅ | ❌ | ❌ |
| Drops ja kanavapisteet samanaikaisesti yhdellä tilillä | ✅ | osittain ¹ | ❌ |
| Useita tilejä yhdessä sovelluksessa | ✅ | ❌ ² | ❌ |
| Usean tilin asetusten muuttaminen kerralla | ✅ | ❌ | ❌ |
| Graafinen käyttöliittymä | ✅ työpöytä ja verkko ³ | ❌ ⁴ | ✅ työpöytä |
| Verkkohallintapaneeli kotipalvelimelle (Docker), salasanalla suojattu, käytettävissä HTTPS:n kautta | ✅ | ❌ ⁴ | ❌ |
| Levyllä salatut istunnot, valinnainen sovelluksen salasana | ✅ | ❌ | ❌ |
| Käyttöliittymä 29 kielellä | ✅ | ❌ | osittain ⁵ |
| Allekirjoitetut päivitykset suoraan sovelluksesta (työpöytä) | ✅ | ❌ | ❌ |
| Ohjattu käyttöönotto jokaiselle uudelle tilille | ✅ | ❌ ⁴ | ❌ |
| Streamaajien lisäys ja muokkaus käyttöliittymässä, yksi kerrallaan tai useita kerralla | ✅ | ❌ ⁴ | — |
| Streamaajien tuonti haulla, listasta, toiselta tililtä tai seuratuista kanavista | ✅ | osittain ⁶ | — |
| Discord-, Telegram- ja webhook-ilmoitukset kuvineen | ✅ | osittain ⁷ | ❌ |
| Kalenteri kaikista Drops-kampanjoista, haulla | ✅ | ❌ | osittain ⁸ |
| Louhintajärjestys kampanjoittain, poissuljetut sanat, ohitettu kampanja tai palkinto | ✅ | ❌ | ❌ ⁹ |
| Listasi streamaajia katsotaan ensin kampanjan Dropseja varten | ✅ | osittain ¹⁰ | ❌ |
| Suodatettava tapahtumapäiväkirja ja lista jokaisesta pyynnöstä käyttöliittymässä | ✅ | ❌ ⁴ | ❌ |
| Noin 30 Mt muistia (Docker-palvelin, 4 tiliä) | ✅ | — ¹¹ | — ¹¹ |

¹ Se katsoo vain listansa streamaajia: se edistää heidän lähetystensä Dropseja ja
lunastaa ne valitsematta kampanjaa.\
² Yksi prosessi tiliä kohden.\
³ Tietokoneellasi työpöytäsovelluksena ja/tai palvelimella (VPS, NAS, Raspberry Pi)
Dockerilla, ilman graafista ympäristöä ja ilman selainta kyseisellä koneella:
hallintapaneeli avautuu missä tahansa selaimessa.\
⁴ Määritetään Python-tiedostossa; sen verkkosivu näyttää pistekaavioita ja
lokitiedoston.\
⁵ 21 kieltä.\
⁶ Se voi seurata kanavia, joita tili seuraa.\
⁷ Pelkkää tekstiä ilman kuvia. Siinä on myös Matrix, Pushover ja Gotify, joita tässä
projektissa ei ole.\
⁸ Kampanjalista suodattimilla, ilman kalenteria ja hakua.\
⁹ Siinä on prioriteettipelit ja ohitetut pelit, jotka valitaan pelin nimen mukaan.\
¹⁰ Oletuksena se katsoo ensin niitä listansa streamaajia, joiden lähetyksessä on Dropseja
(sen prioriteetti `DROPS`), mutta ei koskaan muuta kanavaa.\
¹¹ Ei mitattu.

## <a name="download"></a>Lataus

Lataa järjestelmällesi sopiva tiedosto
[uusimmasta julkaisusta](https://github.com/SunkwiBOT/twitch-miner/releases/latest).

| Järjestelmä | Paketti |
|---|---|
| **Windows** · x64 | Asennusohjelma (`setup.exe`) tai [siirrettävä suoritettava tiedosto](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/TwitchMiner-portable.exe) |
| **macOS** 10.15 tai uudempi · Intel ja Apple Silicon | Levykuva (`.dmg`) |
| **Linux** · x86-64 | `.AppImage`, `.deb` (Debian, Ubuntu) tai `.rpm` (Fedora, openSUSE) |

macOS-versiota ei ole notarisoitu. Napsauta ensimmäisellä kerralla sovellusta hiiren
oikealla painikkeella ja valitse **Avaa**.

Päivitykset on allekirjoitettu. Sovellus tarkistaa käynnistyessään ja sen jälkeen tunnin
välein, onko uutta versiota (tiheyttä voi muuttaa kohdassa **Asetukset › Tietoja**). Se ei
koskaan asenna versiota itse: asennat sen yhdellä napsautuksella näkyviin tulevasta
ilmoituksesta tai kohdasta **Asetukset › Tietoja**.

Tiedot sijaitsevat käyttäjän datakansiossa:

- Linux: `~/.local/share/twitch-miner`
- Windows: `%APPDATA%\twitch-miner`
- macOS: `~/Library/Application Support/twitch-miner`

## <a name="docker"></a>Docker

Palvelin näyttää saman hallintapaneelin selaimessa. Image on
`ghcr.io/sunkwibot/twitch-miner` alustoille `amd64` ja `arm64`. Kun uusi
versio julkaistaan, hallintapaneeli kertoo sen: `docker compose pull` ja sitten
`docker compose up -d`.

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-miner:latest
    pull_policy: always   # hakee uusimman imagen joka käynnistyksessä
    restart: unless-stopped
    ports: ["8080:8080"]   # HTTPS-käänteisproxyn takana: poista, tai "127.0.0.1:8080:8080"
    environment:
      MINER_PASSWORD: "change-me"   # pakollinen, vähintään 12 merkkiä
      # MINER_PUBLIC_URL: "https://miner.example.com"   # vain HTTPS-käänteisproxyn takana
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

Avaa <http://localhost:8080> (tai koneen osoite verkossasi) ja syötä salasana.

### Muuttujat

| Muuttuja | Oletus | Tarkoitus |
|---|---|---|
| `MINER_PASSWORD` | ei mitään | Hallintapaneelin salasana, pakollinen: ilman sitä palvelin kieltäytyy käynnistymästä. Vähintään 12 merkkiä (lyhyemmästä näytetään varoitus käynnistyksessä). Esimerkkiarvo `change-me` hylätään. |
| `MINER_PASSWORD_FILE` | ei mitään | Lukee `MINER_PASSWORD`-arvon tiedostosta, esimerkiksi Docker-secretistä. Vain toinen, ei molempia. |
| `MINER_PUBLIC_URL` | ei mitään | Vain HTTPS-käänteisproxyn takana (Caddy, nginx, Traefik, Cloudflare Tunnel…): hallintapaneelin `https://`-osoite ilman polkua, esimerkiksi `https://miner.example.com`. Proxyn on välitettävä `X-Forwarded-Proto`, `X-Forwarded-For` ja websocket-yhteydet, minkä yleiset proxyt tekevät oletuksena, eikä kontin porttiin saa päästä suoraan. Hallintapaneeli hylkää silloin kaiken, mikä ei tule proxyn kautta HTTPS:llä, ja estää osoitteen 30 sekunniksi 5 väärän salasanan jälkeen. Välityspalvelimen on toimittava samalla koneella tai sen lähiverkossa: yhteys julkisesta osoitteesta hylätään. |
| `MINER_TRUSTED_HOPS` | `1` | Vain `MINER_PUBLIC_URL`-muuttujan kanssa: kuinka moni proxy lisää merkinnän `X-Forwarded-For`-otsakkeeseen, 1–10. `2`, kun proxyja on kaksi peräkkäin, esimerkiksi Cloudflare Traefikin edessä. |
| `MINER_PROXY_NETWORKS` | ei mitään | Vain `MINER_PUBLIC_URL`-asetuksen kanssa: muut verkot, joista välityspalvelin saa yhdistää tämän koneen ja sen yksityisen lähiverkon lisäksi, pilkuilla erotettuina: yksittäinen osoite tai verkko kuten `2001:db8:1::/64` lähiverkolle, jonka IPv6-osoitteet ovat julkisia. |
| `MINER_VAULT_PASSPHRASE` | ei mitään | Vain työpöytäsovelluksesta kopioidulle datakansiolle, jossa on asetettu sovelluksen salasana: avaa sen lukituksen käynnistyksessä. Tyhjästä pystytetty palvelin ei koskaan tarvitse sitä. |
| `MINER_VAULT_PASSPHRASE_FILE` | ei mitään | Lukee `MINER_VAULT_PASSPHRASE`-arvon tiedostosta. Vain toinen, ei molempia. |
| `DATA_DIR` | imagessa `/data`, muuten `data` työhakemistossa | Tietokanta, avaintiedosto ja lokit. |
| `LOG_DIR` | `$DATA_DIR/logs` | Lokitiedostot: yksi päivässä, viimeiset 14 säilytetään. |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | Kuunteluosoite: IP-osoite ja portti väliltä 1–65535. |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | Lokitaso. |

Palvelin lukee ja tarkistaa muuttujat kerran, käynnistyksessä. Virheellinen arvo pysäyttää
palvelimen ja näyttää viestin sen sijaan, että palvelin siirtyisi hiljaa varavaihtoehtoon.

## <a name="how-it-works"></a>Näin se toimii

Joka 20. sekunti jokainen tili valitsee kaksi kanavaansa: parhaan live-kanavan jononsa
kärjessä olevalle kampanjalle ja sitten pistelistansa streamaajat.

Twitch hyvittää tiliä kohden kerrallaan **yhden Drops-kampanjan** ja
**kaksi kanavaa pisteille**. Lokakuusta 2026 lähtien dropin voi lunastaa vain, kun
pelitili on linkitetty. Linkittämättömät kampanjat edistyvät silti, ja hallintapaneeli
näyttää linkin.

Kampanjaluettelo tulee [Twitch Drops API](https://drops.sunkwi.com) -projektista
(päivittyy 10 minuutin välein), koska Twitch piilottaa sen TV-asiakkaalta.

## <a name="faq"></a>Usein kysytyt kysymykset

### Riskit ja Twitch

**Voiko tilini saada porttikiellon?**\
Kukaan ei voi luvata, ettei niin käy. Tämä projekti ei ole yhteydessä Twitchiin, ja tilin
automatisointi voi rikkoa Twitchin käyttöehtoja tai kampanjan sääntöjä. Louhija katsoo
enintään kahta kanavaa tiliä kohden, mikä on Twitchin raja, eikä lataa videota. Päätös on
sinun.

**Katsotaanko videota oikeasti?**\
Ei, videota ei ladata. Dropseja varten louhija lukee 15 sekunnin välein lähetyksen
soittolistan heikoimmalla laadulla ja pyytää jokaisesta uudesta segmentistä vain
otsakkeen. Pisteitä varten se lähettää joka minuutti Twitchin ”katsottu minuutti”
-signaalin. Jokainen katsottu kanava näyttää, kuinka monta segmenttiä on pyydetty.

**Miksi enintään kaksi kanavaa?**\
Se on Twitchin raja: Twitch hyvittää tiliä kohden vain kaksi samanaikaisesti katsottua
kanavaa ja vain yhden Drops-kampanjan kerrallaan. **Drops ja pisteet** -tilassa toinen
kanava palvelee kampanjaa ja toinen streamaajiasi. ”Yksi kanava” -asetus kohdassa
**Tilin asetukset** on huomaamattomampi.

### Drops

**Kampanjaa ei louhita, tai drop ei edisty.**\
Avaa tilin **Drops**-sivu. Kohdassa ”Louhintajärjestys” jokainen odottava kampanja kertoo
syyn: ”Ei kanavaa livenä”, ”Odottaa (yksi kampanja kerrallaan)” tai ”Ei vielä dropia
saatavilla”. Kampanja, joka ei näy tässä listassa, on jätetty pois: ”Mitkä kampanjat”
-asetuksella, **Ohitukset**-välilehden säännöllä tai siksi, ettei se ehdi valmistua
ajoissa. Tilausta vaativaa palkintoa ei koskaan louhita, ja toista seuraava drop odottaa,
kunnes ensimmäinen on valmis. **Vain pisteet** -tilassa louhija ei valitse kampanjaa.
”Louhi ensin” kampanjan tiedoissa nostaa sen kärkeen; ohitetun kampanjan saa takaisin
valinnalla ”Louhi taas” **Ohitukset**-välilehdellä.

**Pitääkö pelitili linkittää?**\
Lunastamista varten kyllä: lokakuusta 2026 lähtien Twitch antaa lunastaa dropin vain, kun
pelitili on linkitetty. Oletuksena linkittämätön kampanja edistyy silti, ja lunastus
odottaa linkitystä. ”Ei linkitetty” -merkki kertoo tästä, ja ”Linkitä pelitili”
kampanjan tiedoissa avaa linkityssivun. Jos haluat louhia vain linkitettyjä pelejä,
valitse ”Vain pelit, joiden tili on linkitetty”.

### Asennus ja tiedot

**Työpöytäsovellus vai Docker?**\
Työpöytäsovellus louhii, kun tietokoneesi on päällä, ja siinä on ilmoitusalueen kuvake,
valinnainen salasana ja päivitykset yhdellä napsautuksella. Docker louhii yötä päivää
koneella, joka on aina päällä (VPS, NAS, Raspberry Pi); avaat hallintapaneelin
selaimessa, ja sitä suojaa `MINER_PASSWORD`. Käyttöliittymä on sama.

**Missä tietoni ovat?**\
Datakansiossa: Linuxilla `~/.local/share/twitch-miner`, Windowsilla
`%APPDATA%\twitch-miner`, macOS:llä
`~/Library/Application Support/twitch-miner` ja Dockerilla `/data`-taltiossa.
Työpöydällä **Asetukset › Työpöytä** avaa sen yhdellä napsautuksella. Siellä ovat
`miner.db` (tilit, asetukset, päiväkirja), `master.key` (istuntoavain, kun sovelluksen
salasanaa ei ole asetettu) ja lokit.

**Miten päivitän?**\
Työpöytäsovellus tarkistaa uuden version käynnistyessään ja sen jälkeen tunnin välein
(muutettavissa kohdassa **Asetukset › Tietoja**). Napsauta ”Päivitys”-merkkiä ja sitten
”Asenna ja käynnistä uudelleen”. Dockerilla hallintapaneeli ilmoittaa asiasta: suorita
`docker compose pull` ja sitten `docker compose up -d`.

**Unohdin sovelluksen salasanan.**\
Sitä ei voi palauttaa: juuri se salaa Twitch-istuntosi. Sulje sovellus, poista datakansio
ja lisää tilisi uudelleen; niiden asetukset on tehtävä uudelleen. Dockerilla
hallintapaneelin salasana vaihdetaan `MINER_PASSWORD`-muuttujassa, minkä jälkeen ajetaan
`docker compose up -d`.

### Pisteet ja ennustukset

**Montako pistettä tunnissa?**\
Se riippuu Twitchistä ja kustakin kanavasta. Louhija ansaitsee sen, minkä Twitch antaa
katsojalle: katselusta kertyvät pisteet, bonusarkut (noin yksi 15 minuutin välein) ja
katseluputken bonuksen. Tilaus kasvattaa pisteet tasosta riippuen 1,2–2-kertaisiksi.
Etusivu näyttää ansaitut pisteet ja Päiväkirja jokaisen ansion.

**Ovatko vedot riskialttiita?**\
Kyllä, pisteitä voi menettää. Ennustukset ovat oletuksena pois päältä, streamaajakohtaisesti.
Asetat strategian, panostuksen ajoituksen, panoksen (oletuksena 5 % saldosta, enintään
50 000 pistettä), vähimmäissaldon ja halutessasi ehdon, kuten vähintään 1,5:n kertoimen.
Pidä panos pienenä.

**Streamaajaa ei katsota.**\
Vain käytössä olevia ja livenä olevia streamaajia katsotaan, ja vain vapaalla kanavalla:
**Drops ja pisteet** -tilassa toinen kahdesta palvelee nykyistä kampanjaa;
**Vain Drops** -tilassa streamaajia ei katsota lainkaan; tauolla oleva tili ei katso
mitään. Paikat jaetaan tässä järjestyksessä, joka näkyy merkkinä jokaisella katsotulla
kanavalla: ”Putkibonus”, ”Putki vaarassa”, ”Tilattu” ja sitten ”Listan järjestys”.
Siirrä streamaajaa ylemmäs listalla tai poista käytöstä ”Katseluputket ensin” ja
”Tilatut kanavat ensin”.

## <a name="related-projects"></a>Liittyvät projektit

Saman tekijän muita projekteja:

- [**Twitch Drops API**](https://drops.sunkwi.com): ilmainen julkinen API kaikista
  käynnissä olevista Twitch Drops- ja palkintokampanjoista, päivittyy minuutin välein.
  Tämä sovellus lukee kampanjaluettelonsa siitä.
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com): Discord-botti, joka ilmoittaa
  palvelimellasi Twitchin livelähetyksistä ja Drops-kampanjoista.
- [**Kick Alerts 🔔**](https://kick.sunkwi.com): Discord-botti, joka ilmoittaa
  palvelimellasi Kickin livelähetyksistä ja Drops-kampanjoista.

## <a name="security"></a>Tietoturva

- Tokenit sinetöidään AES-256-GCM:llä. Avain on joko satunnainen `master.key` tietokannan
  vieressä (oletus, ei vaadi toimia) tai johdetaan sovelluksen salasanasta Argon2id:llä.
- HTTP-hallintapaneeli tarkistaa salasanan vakioajassa ja käyttää HttpOnly- ja
  SameSite=Strict-evästeitä (HTTPS:n takana `Secure` ja `__Host-`-etuliite). Palvelin
  säilyttää jokaisesta istunnosta vain SHA-256-tiivisteen. Se päättää istunnon 30 päivän
  jälkeen tai uloskirjautuessa. Se tarkistaa istunnon ennen kuin lukee pyynnön runkoa ja
  rajoittaa rungot 256 KiB:hen.
- Kun samasta IP-osoitteesta tulee 5 väärää salasanaa peräkkäin (kirjautumislomake tai
  bearer), osoite joutuu odottamaan 30 sekuntia. Palvelin vastaa `429` ja `Retry-After`
  katsomatta salasanaa. Lisäksi vastaus jokaiseen väärään salasanaan kestää puoli
  sekuntia.
- Oman verkkosi ulkopuolella sijoita hallintapaneeli HTTPS-käänteisproxyn taakse ja aseta
  `MINER_PUBLIC_URL` (katso [Docker](#docker)).
- Louhija on yhteydessä vain kohteisiin `twitch.tv`, `ttvnw.net`, `jtvnw.net`,
  `twitchcdn.net`, `drops.sunkwi.com` sekä määrittämiisi ilmoituskohteisiin.
  Sovellus (työpöytä ja Docker) tarkistaa uudet versiot `github.com`-palvelusta, ja
  työpöytäsovellus lataa ne projektin GitHub-julkaisuista. Ne on allekirjoitettu: vain
  näytetty versio asennetaan.

## <a name="build"></a>Kääntäminen lähdekoodista

**Esivaatimukset:** Rust, Node.js ja järjestelmällesi
[Taurin esivaatimukset](https://v2.tauri.app/start/prerequisites/).

```bash
npm ci
npm run build          # hallintapaneeli
cargo build --release -p miner-server   # palvelin: target/release/twitch-miner
npm run bundle:linux   # työpöytäpaketit (vaatii Taurin Linux-riippuvuudet)
```

Kooditarkistukset:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier, oxlint (tyyppitietoinen) ja svelte-check
```

Käyttöliittymän TypeScript-tyypit generoidaan Rust-tyypeistä
[ts-rs](https://github.com/Aleph-Alpha/ts-rs) -työkalulla hakemistoon
`ui/src/lib/bindings`. Kun muutat tyyppiä, jonka käyttöliittymä näkee, suorita
`npm run bindings`.

## <a name="license"></a>Lisenssi

[MIT](../../LICENSE). Inspiraationa
[DevilXD:n TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner),
[Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
ja niiden forkit.
