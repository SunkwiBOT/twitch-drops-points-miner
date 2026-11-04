<p align="center">
  <img src="../../ui/public/app-icon.svg" width="112" height="112" alt="Logo van Twitch Drops &amp; Points Miner">
</p>

<h1 align="center">Twitch Drops &amp; Points Miner</h1>

<p align="center">
  <strong>Mint Twitch Drops en kanaalpunten op meerdere accounts, zonder browser en zonder open Twitch-tabblad.</strong>
</p>

<p align="center">
  Een desktop-app voor Linux, Windows en macOS, of een Docker-image met hetzelfde webdashboard, om te draaien op een thuisserver of op een Raspberry Pi. Met 4 accounts gebruikt de server ongeveer 30 MB geheugen en bijna geen CPU.
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-drops-points-miner?label=release&color=9146FF" alt="Nieuwste release"></a>
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-drops-points-miner/total?label=downloads&color=9146FF" alt="Downloads"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="Platforms: Linux, Windows, macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-drops-points-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="Docker-image voor amd64 en arm64"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="MIT-licentie"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="Weergaven">
</p>

<p align="center">
  <a href="#download"><strong>Downloaden</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/issues/new/choose">Bug melden</a>
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
  <strong>Nederlands</strong> ·
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
  <img src="../media/nl/presentation.avif" alt="Video-overzicht van Twitch Drops &amp; Points Miner">
</p>

## <a name="features"></a>Functies

Wat hem onderscheidt van andere miners. “Bekijk het in actie” onder een punt laat zien hoe
het werkt.

### Accounts en streamers

- **Meerdere accounts in één app.** Elk account mint zelfstandig, in zijn eigen modus:
  **Drops en punten** (één kanaal voor de beste campagne, het andere voor je streamers),
  **Alleen Drops** of **Alleen punten**.

  <details open><summary>Bekijk het in actie</summary>
  <img src="../media/nl/accounts-mining.avif" alt="Meerdere accounts die tegelijk minen, op de pagina Home">
  </details>

- **Een wizard voor elk nieuw account.** Begin na het aanmelden met de standaardinstellingen
  of met die van een ander account. Kies de modus, de campagnes en de streamers, en daarna
  “Beginnen met minen” of “Opslaan en later starten”.

  <details open><summary>Bekijk het in actie</summary>
  <img src="../media/nl/account-setup.avif" alt="De instelwizard van een nieuw account, stap voor stap">
  </details>

- **Eén wijziging met één klik op andere accounts toepassen.** Nadat je iets op een account
  hebt gewijzigd, biedt de bevestigingsmelding “Toepassen op andere accounts” aan. De
  wijziging wordt herhaald op de accounts die je aanvinkt, en die behouden de rest van hun
  instellingen.

  <details open><summary>Bekijk het in actie</summary>
  <img src="../media/nl/apply-to-accounts.avif" alt="Een wijziging die met “Toepassen op andere accounts” op andere accounts wordt herhaald">
  </details>

- **Een streamer toevoegen.** Hij start meteen met de standaardopties: die van de app, of
  die je voor dit account hebt gekozen. Pas ze zo nodig meteen aan: bonuskisten, raids,
  momenten, kijkreeks, chat, voorspellingen.

  <details open><summary>Bekijk het in actie</summary>
  <img src="../media/nl/streamer-add.avif" alt="Een streamer toevoegen via de zoekfunctie van Twitch, daarna zijn opties">
  </details>

- **Een streamer bewerken.** “Bewerken” opent zijn opties: bonuskisten, raids, momenten,
  kijkreeks, chat, voorspellingen… Met een schakelaar zet je hem aan of uit, zonder hem uit
  de lijst te halen.

  <details open><summary>Bekijk het in actie</summary>
  <img src="../media/nl/streamer-edit.avif" alt="De opties van een streamer, geopend vanuit zijn rij">
  </details>

- **Meerdere streamers tegelijk bewerken.** Vink ze aan en kies “De N geselecteerde
  streamers bewerken”. Alleen de opties die je wijzigt, worden toegepast. Een optie die
  tussen hen verschilt, toont “Gemengd”, en de schakelaar wordt twee knoppen, “Ja” en
  “Nee”. Daarna stelt de app voor om er de standaardopties voor nieuwe streamers van te
  maken.

  <details open><summary>Bekijk het in actie</summary>
  <img src="../media/nl/streamers-group-edit.avif" alt="Meerdere streamers bewerken, met opties op “Gemengd”">
  </details>

- **Streamers importeren.** Plak een lijst (tot 500 namen), of neem de lijst van een ander
  account of de kanalen die het account volgt over. De lijst wordt gecontroleerd, zodat
  alleen kanalen die echt bestaan worden toegevoegd.

  <details open><summary>Bekijk het in actie</summary>
  <img src="../media/nl/streamers-import.avif" alt="Streamers importeren: geplakte lijst, ander account, gevolgde kanalen">
  </details>

- **Spam Points.** Wissel een aangepaste beloning van een kanaal meerdere keren achter
  elkaar in. Kies een kanaal uit je Puntenlijst, of een willekeurig kanaal dat je via de
  Twitch-zoekfunctie vindt: de beloningen verschijnen met het saldo van het account, live
  opnieuw gelezen. Stel het aantal in (tot 25, binnen het saldo), daarna “Inwisselen”. Het
  resultaat verschijnt aan het einde van de reeks, en elke reeks krijgt een eigen regel
  in het Journaal.

  <details open><summary>Bekijk het in actie</summary>
  <img src="../media/nl/spam-points.avif" alt="Spam Points: een kanaal gekozen, 10 × een beloning met bericht ingewisseld, de inwisselingen verschijnen in de Twitch-chat van het kanaal, daarna de regel in het Journaal">
  </details>

### Drops

- **Alle lopende Drops-campagnes.** Elke campagne heeft haar eigen details: beloningen,
  koppeling van het game-account, “Eerst minen”. De lijst komt van het project
  [Twitch Drops API](https://drops.sunkwi.com) en wordt elke 10 minuten ververst.

  <details open><summary>Bekijk het in actie</summary>
  <img src="../media/nl/drops-catalogue.avif" alt="De Catalogus als kalender, de zoekfunctie en de details van een campagne">
  </details>

- **Jouw minevolgorde.** Sleep de campagnes in de volgorde die je wilt. Nieuwe campagnes
  vinden zelf hun plek: eerst je prioriteitsgames, daarna het dichtstbijzijnde einde of de
  minste resterende minuten. Sluit games, woorden, een campagne of één enkele beloning uit,
  en sla de campagnes over die niet op tijd af kunnen. “Eerst minen”, in de details van een
  campagne, zet haar bovenaan.

  <details open><summary>Bekijk het in actie</summary>
  <img src="../media/nl/drops-order.avif" alt="De minevolgorde, de prioriteitsgames en de uitsluitingen">
  </details>

- **Inventaris en “Alles claimen”.** De pagina Inventaris verdeelt de beloningen over
  “Te claimen”, “Lopend” en “Geclaimd”. Claim ze een voor een of allemaal tegelijk, of zet
  “Drops automatisch claimen” aan.

  <details open><summary>Bekijk het in actie</summary>
  <img src="../media/nl/drops-inventory.avif" alt="De inventaris en de knop “Alles claimen”">
  </details>

- **“Mijn streamers eerst voor Drops”.** Voor de huidige campagne gaat een streamer uit je
  lijst wiens stream voor die campagne telt, voor op andere kanalen. Hetzelfde kanaal
  brengt de drop verder en levert je zijn kanaalpunten op. Standaard aan.

  <details open><summary>Bekijk het in actie</summary>
  <img src="../media/nl/drops-streamers-first.avif" alt="Een streamer uit de lijst, bekeken voor de Drops en voor zijn punten">
  </details>

### Bijhouden

- **Een journaal van alles, en elk verzoek.** Het Journaal houdt elke gebeurtenis bij:
  drops, verdiende en uitgegeven punten, kisten, weddenschappen, raids, streams die live
  gaan… Filter het op account, op tabblad (Drops, Punten, Overig) en op type.
  **Technisch logboek › Verzoeken** toont elk verzoek aan Twitch en aan andere diensten,
  met het resultaat en hoe lang het duurde. Logboek en verzoeken bevatten geen tokens of
  geheimen: “Kopiëren voor een bugrapport” kopieert ze, klaar om te plakken in een
  bugrapport op
  [GitHub](https://github.com/SunkwiBOT/twitch-drops-points-miner/issues) of
  [Discord](https://discord.gg/support-bots-891685895781113886).

  <details open><summary>Bekijk het in actie</summary>
  <img src="../media/nl/journal-requests.avif" alt="Het gefilterde Journaal, daarna Technisch logboek › Verzoeken">
  </details>

### Op de desktop of op een server

- **Overal dezelfde interface.** De desktop-app (Linux, Windows, macOS) en de Docker-image
  tonen hetzelfde dashboard. Docker draait op een VPS, een NAS of een Raspberry Pi, zonder
  scherm of browser op die machine: het dashboard opent in elke browser.
- **Licht.** Ongeveer 30 MB geheugen voor 4 accounts, bijna geen CPU, en een Docker-image
  van ongeveer 12 MB.

### Beveiliging en updates

- **Je sessies, beschermd.** Geen Twitch-wachtwoord: je meldt je aan op
  `twitch.tv/activate`, vanuit je eigen browser. Sessies staan versleuteld op schijf en je
  kunt ze intrekken in je [Twitch-instellingen](https://www.twitch.tv/settings/connections).
  Op de desktop vergrendelt een optioneel app-wachtwoord de app: er wordt niets gemined tot
  het is ingevoerd. Met Docker heeft het dashboard een eigen wachtwoord, vergrendelt het
  zichzelf na een paar minuten zonder activiteit en draait het via HTTPS achter een proxy.
  Vijf verkeerde wachtwoorden op rij blokkeren het IP-adres 30 seconden lang.

  <details open><summary>Bekijk het in actie</summary>
  <img src="../media/nl/security-lock.avif" alt="Het slotje: de app vergrendeld, daarna ontgrendeld met het wachtwoord">
  </details>

- **Ondertekende updates, nooit automatisch.** In de bovenste balk verschijnt een badge
  “Update” met het versienummer. “Installeren en herstarten” vraagt eerst om bevestiging,
  daarna installeert de app de versie en start ze opnieuw op. Met een app-wachtwoord start
  ze vergrendeld op: voer het opnieuw in om verder te minen. Alleen een door het project
  ondertekende versie wordt geïnstalleerd. De Docker-versie laat het ook weten, met de
  commando's die je moet uitvoeren.

  <details open><summary>Bekijk het in actie</summary>
  <img src="../media/nl/updates.avif" alt="De badge “Update”, de bevestiging, daarna de installatie">
  </details>

### En verder

- Net als andere miners: bonuskisten, raids, momenten, communitydoelen, voorspellingen
  (strategieën, moment van inzetten, voorwaarden), kijkreeksen en geabonneerde kanalen
  eerst, aanwezigheid in de chat (uit te zetten), standaardopties voor nieuwe streamers,
  meldingen via Discord, Telegram en webhook met afbeeldingen, een globale proxy of een
  proxy per account.
- 29 talen, donker en licht thema, bureaubladmeldingen, een pictogram in het systeemvak,
  starten met het systeem.

  <details open><summary>Bekijk het in actie</summary>
  <img src="../media/nl/basics.avif" alt="Talen, licht en donker thema, een bureaubladmelding en het pictogram in het systeemvak">
  </details>

> [!WARNING]
> Dit project is niet gelieerd aan Twitch. Een account automatiseren kan in strijd zijn
> met de voorwaarden van Twitch of de regels van een campagne.

## Inhoud

- [Vergelijking](#comparison)
- [Downloaden](#download)
- [Docker](#docker)
- [Hoe het werkt](#how-it-works)
- [Veelgestelde vragen](#faq)
- [Gerelateerde projecten](#related-projects)
- [Beveiliging](#security)
- [Bouwen vanuit de broncode](#build)
- [Licentie](#license)

## <a name="comparison"></a>Vergelijking

Hoe [Twitch Drops & Points Miner](https://github.com/SunkwiBOT/twitch-drops-points-miner)
zich verhoudt tot [Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
(kanaalpunten) en [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner) (Drops).
Gecontroleerd aan de hand van hun code en README's in november 2026.

| | Dit project | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Drops-campagnes: vindt, mint en claimt ze | ✅ | ❌ ¹ | ✅ |
| Kanaalpunten: kisten, raids, momenten, voorspellingen | ✅ | ✅ | ❌ |
| Spam Points: een aangepaste beloning van een kanaal meerdere keren achter elkaar ingewisseld | ✅ | ❌ | ❌ |
| Drops en kanaalpunten tegelijk op één account | ✅ | deels ¹ | ❌ |
| Meerdere accounts in één app | ✅ | ❌ ² | ❌ |
| Instellingen van meerdere accounts tegelijk wijzigen | ✅ | ❌ | ❌ |
| Grafische interface | ✅ desktop en web ³ | ❌ ⁴ | ✅ desktop |
| Webdashboard voor een thuisserver (Docker), met wachtwoord, bereikbaar via HTTPS | ✅ | ❌ ⁴ | ❌ |
| Sessies versleuteld op schijf, optioneel app-wachtwoord | ✅ | ❌ | ❌ |
| Interface in 29 talen | ✅ | ❌ | deels ⁵ |
| Ondertekende updates vanuit de app (desktop) | ✅ | ❌ | ❌ |
| Instelwizard voor elk nieuw account | ✅ | ❌ ⁴ | ❌ |
| Streamers toevoegen en bewerken in de interface, één voor één of meerdere tegelijk | ✅ | ❌ ⁴ | — |
| Streamers importeren via zoeken, een lijst, een ander account of gevolgde kanalen | ✅ | deels ⁶ | — |
| Meldingen via Discord, Telegram en webhook, met afbeeldingen | ✅ | deels ⁷ | ❌ |
| Kalender van alle Drops-campagnes, met zoekfunctie | ✅ | ❌ | deels ⁸ |
| Minevolgorde per campagne, uitgesloten woorden, genegeerde campagne of beloning | ✅ | ❌ | ❌ ⁹ |
| Streamers uit je lijst eerst bekeken voor de Drops van een campagne | ✅ | deels ¹⁰ | ❌ |
| Filterbaar gebeurtenissenjournaal en een lijst van elk verzoek, in de interface | ✅ | ❌ ⁴ | ❌ |
| Ongeveer 30 MB geheugen (Docker-server, 4 accounts) | ✅ | — ¹¹ | — ¹¹ |

¹ Het kijkt alleen naar de streamers op zijn lijst: het brengt de Drops van hun streams
verder en claimt ze, zonder een campagne te kiezen.\
² Eén proces per account.\
³ Op je pc als desktop-app, en/of op een server (VPS, NAS, Raspberry Pi) met Docker,
zonder grafische omgeving en zonder browser op die machine: het dashboard opent in elke
browser.\
⁴ Ingesteld in een Python-bestand; de webpagina toont puntengrafieken en het logbestand.\
⁵ 21 talen.\
⁶ Het kan de kanalen volgen die het account volgt.\
⁷ Alleen tekst, zonder afbeeldingen. Het heeft ook Matrix, Pushover en Gotify, die dit
project niet heeft.\
⁸ Een lijst van campagnes met filters, zonder kalender of zoekfunctie.\
⁹ Het heeft prioriteitsgames en uitgesloten games, gekozen op naam van de game.\
¹⁰ Standaard kijkt het eerst naar de streamers op zijn lijst wier stream Drops heeft (zijn
prioriteit `DROPS`), maar nooit naar een ander kanaal.\
¹¹ Niet gemeten.

## <a name="download"></a>Downloaden

Download het bestand voor je systeem uit de
[nieuwste release](https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest).

| Systeem | Pakket |
|---|---|
| **Windows** · x64 | Installatieprogramma (`setup.exe`) of [portable versie](https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest/download/TwitchDropsPointsMiner-portable.exe) |
| **macOS** 10.15 of nieuwer · Intel en Apple Silicon | Schijfkopie (`.dmg`) |
| **Linux** · x86-64 | `.AppImage`, `.deb` (Debian, Ubuntu) of `.rpm` (Fedora, openSUSE) |

De macOS-build is niet genotariseerd. Klik de eerste keer met de rechtermuisknop op de app
en kies **Open**.

Updates zijn ondertekend. De app zoekt bij het starten naar een nieuwe versie, en daarna
elk uur (hoe vaak stel je in onder **Instellingen › Over**). Hij installeert er nooit zelf
een: je installeert die met één klik, via de melding die verschijnt of via
**Instellingen › Over**.

De gegevens staan in de map met gebruikersgegevens:

- Linux: `~/.local/share/twitch-drops-points-miner`
- Windows: `%APPDATA%\twitch-drops-points-miner`
- macOS: `~/Library/Application Support/twitch-drops-points-miner`

## <a name="docker"></a>Docker

De server toont hetzelfde dashboard in een browser. De image is
`ghcr.io/sunkwibot/twitch-drops-points-miner`, voor `amd64` en `arm64`. Is er een nieuwe
versie, dan meldt het dashboard dat: `docker compose pull`, daarna `docker compose up -d`.

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-drops-points-miner:latest
    pull_policy: always   # bij elke start de nieuwste image ophalen
    restart: unless-stopped
    ports: ["8080:8080"]   # achter een HTTPS-reverse-proxy: weghalen, of "127.0.0.1:8080:8080"
    environment:
      MINER_PASSWORD: "change-me"   # verplicht, minstens 12 tekens
      # MINER_PUBLIC_URL: "https://miner.example.com"   # alleen achter een HTTPS-reverse-proxy
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

Open <http://localhost:8080> (of het adres van de machine op je netwerk) en voer het
wachtwoord in.

### Variabelen

| Variabele | Standaard | Doel |
|---|---|---|
| `MINER_PASSWORD` | geen | Dashboardwachtwoord, verplicht: zonder wachtwoord weigert de server te starten. Minstens 12 tekens (bij een korter wachtwoord verschijnt bij het starten een waarschuwing). De voorbeeldwaarde `change-me` wordt geweigerd. |
| `MINER_PASSWORD_FILE` | geen | Leest `MINER_PASSWORD` uit een bestand, zoals een Docker-secret. Het een of het ander, niet allebei. |
| `MINER_PUBLIC_URL` | geen | Alleen achter een HTTPS-reverse-proxy (Caddy, nginx, Traefik, Cloudflare Tunnel…): het `https://`-adres van het dashboard, zonder pad, zoals `https://miner.example.com`. De proxy moet `X-Forwarded-Proto`, `X-Forwarded-For` en websockets doorgeven, wat gangbare proxy's standaard doen, en de poort van de container mag niet direct bereikbaar zijn. Het dashboard weigert dan alles wat niet via de proxy over HTTPS binnenkomt, en blokkeert een adres 30 seconden lang na 5 verkeerde wachtwoorden. De proxy moet op dezelfde machine of in het lokale netwerk ervan draaien: een verbinding vanaf een openbaar adres wordt geweigerd. |
| `MINER_TRUSTED_HOPS` | `1` | Alleen met `MINER_PUBLIC_URL`: hoeveel proxy's een vermelding toevoegen aan `X-Forwarded-For`, van 1 tot 10. `2` met twee proxy's achter elkaar, zoals Cloudflare vóór Traefik. |
| `MINER_PROXY_NETWORKS` | geen | Alleen met `MINER_PUBLIC_URL`: andere netwerken van waaruit de proxy mag verbinden, naast deze machine en het privé lokale netwerk ervan, gescheiden door komma's: één adres of een netwerk zoals `2001:db8:1::/64`, voor een lokaal netwerk met globale IPv6-adressen. |
| `MINER_VAULT_PASSPHRASE` | geen | Alleen voor een gegevensmap die is gekopieerd uit de desktop-app met een ingesteld app-wachtwoord: ontgrendelt die bij het starten. Een server die vanaf nul is opgezet, heeft hem nooit nodig. |
| `MINER_VAULT_PASSPHRASE_FILE` | geen | Leest `MINER_VAULT_PASSPHRASE` uit een bestand. Het een of het ander, niet allebei. |
| `DATA_DIR` | `/data` in de image, anders `data` in de werkmap | Database, sleutelbestand en logboek. |
| `LOG_DIR` | `$DATA_DIR/logs` | Logbestanden: één per dag, de laatste 14 blijven bewaard. |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | Luisteradres: een IP-adres en een poort van 1 tot 65535. |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | Logniveau. |

De server leest en controleert de variabelen één keer, bij het starten. Een ongeldige
waarde stopt de server met een melding, in plaats van stil terug te vallen op iets anders.

## <a name="how-it-works"></a>Hoe het werkt

Elke 20 seconden kiest elk account zijn twee kanalen: het beste live kanaal voor de
bovenste campagne in zijn wachtrij, daarna de streamers van zijn puntenlijst.

Twitch telt per account **één Drops-campagne** en **twee kanalen voor punten** tegelijk
mee. Sinds oktober 2026 kan een drop alleen worden geclaimd als het game-account gekoppeld
is. Niet-gekoppelde campagnes maken toch voortgang, en het dashboard toont de koppellink.

De campagnelijst komt van het project [Twitch Drops API](https://drops.sunkwi.com) (elke
10 minuten ververst), omdat Twitch die verbergt voor de tv-client.

## <a name="faq"></a>Veelgestelde vragen

### Risico's en Twitch

**Kan mijn account worden geblokkeerd?**\
Niemand kan beloven dat het niet gebeurt. Dit project is niet gelieerd aan Twitch, en een
account automatiseren kan in strijd zijn met de voorwaarden van Twitch of de regels van
een campagne. De miner kijkt naar hoogstens twee kanalen per account, de limiet van
Twitch, zonder video te downloaden. De keuze is aan jou.

**Wordt de video echt bekeken?**\
Nee, er wordt geen video gedownload. Voor Drops leest de miner elke 15 seconden de
afspeellijst van de stream in de laagste kwaliteit en vraagt hij van elk nieuw segment
alleen de header op. Voor punten stuurt hij elke minuut het Twitch-signaal “minuut
bekeken”. Elk bekeken kanaal toont hoeveel segmenten er zijn opgevraagd.

**Waarom hoogstens twee kanalen?**\
Dat is de limiet van Twitch: per account telt het maar twee tegelijk bekeken kanalen mee,
en maar één Drops-campagne tegelijk. In de modus **Drops en punten** dient het ene kanaal
de campagne en het andere je streamers. De instelling “Eén kanaal”, in
**Accountinstellingen**, valt minder op.

### Drops

**Een campagne wordt niet gemined, of een drop maakt geen voortgang.**\
Open de pagina **Drops** van het account. Onder “Minevolgorde” staat bij elke wachtende
campagne waarom: “Geen kanaal live”, “Wacht (één campagne tegelijk)” of “Nog geen drop
beschikbaar”. Een campagne die niet in die lijst staat, is buiten beschouwing gelaten:
door de instelling “Welke campagnes”, door een regel op het tabblad **Uitsluitingen**, of
omdat ze niet op tijd af kan. Een beloning waarvoor een abonnement nodig is, wordt nooit
gemined, en een drop die op een andere volgt, wacht tot de eerste klaar is. In de modus
**Alleen punten** kiest de miner geen campagne. “Eerst minen”, in de details van een
campagne, zet haar bovenaan; een genegeerde campagne komt terug met “Weer minen”, op het
tabblad **Uitsluitingen**.

**Moet ik het game-account koppelen?**\
Om te claimen wel: sinds oktober 2026 laat Twitch je een drop alleen claimen als het
game-account gekoppeld is. Standaard maakt een niet-gekoppelde campagne toch voortgang en
wacht het claimen op de koppeling. De badge “Niet gekoppeld” geeft dat aan, en
“Game-account koppelen”, in de details van de campagne, opent de koppelpagina. Wil je
alleen gekoppelde games minen, kies dan “Alleen games met een gekoppeld account”.

### Installatie en gegevens

**Desktop-app of Docker?**\
De desktop-app mint zolang je pc aanstaat, met een pictogram in het systeemvak, een
optioneel wachtwoord en updates met één klik. Docker mint dag en nacht op een machine die
altijd aanstaat (VPS, NAS, Raspberry Pi); je opent het in een browser, beveiligd met
`MINER_PASSWORD`. De interface is dezelfde.

**Waar staan mijn gegevens?**\
In de gegevensmap: `~/.local/share/twitch-drops-points-miner` op Linux,
`%APPDATA%\twitch-drops-points-miner` op Windows,
`~/Library/Application Support/twitch-drops-points-miner` op macOS, en het volume `/data`
bij Docker. Op de desktop opent **Instellingen › Bureaublad** die map met één klik. Er
staan `miner.db` (accounts, instellingen, journaal), `master.key` (de sessiesleutel, als
er geen app-wachtwoord is ingesteld) en het logboek in.

**Hoe werk ik bij?**\
De desktop-app zoekt bij het starten naar een nieuwe versie, en daarna elk uur (aan te
passen in **Instellingen › Over**). Klik op de badge “Update” en daarna op
“Installeren en herstarten”. Bij Docker meldt het dashboard het: voer
`docker compose pull` uit en daarna `docker compose up -d`.

**Ik ben het app-wachtwoord vergeten.**\
Het kan niet worden hersteld: juist dit wachtwoord versleutelt je Twitch-sessies. Sluit de
app af, verwijder de gegevensmap en voeg je accounts opnieuw toe; hun instellingen moet je
opnieuw invoeren. Bij Docker wijzig je het dashboardwachtwoord in `MINER_PASSWORD` en voer
je daarna `docker compose up -d` uit.

### Punten en voorspellingen

**Hoeveel punten per uur?**\
Dat hangt af van Twitch en van elk kanaal. De miner verdient wat Twitch een kijker geeft:
punten voor kijktijd, bonuskisten (ongeveer één per 15 minuten) en de kijkreeksbonus. Een
abonnement vermenigvuldigt de punten, van ×1,2 tot ×2, afhankelijk van het niveau. De
pagina Home toont de verdiende punten, en het Journaal elke opbrengst.

**Zijn weddenschappen riskant?**\
Ja, je kunt punten verliezen. Voorspellingen staan standaard uit, per streamer. Je stelt
de strategie in, het moment van inzetten, de inzet (standaard 5% van het saldo, hoogstens
50.000 punten), een minimumsaldo en, als je wilt, een voorwaarde, zoals een quotering van
minstens 1,5. Houd de inzet klein.

**Een streamer wordt niet bekeken.**\
Alleen streamers die aanstaan en live zijn, worden bekeken, en alleen op een vrij kanaal:
in de modus **Drops en punten** dient een van de twee de huidige campagne; in de modus
**Alleen Drops** wordt geen enkele streamer bekeken; een gepauzeerd account bekijkt niets.
De plaatsen worden in deze volgorde verdeeld, getoond als badge op elk bekeken kanaal:
“Reeksbonus”, “Reeks in gevaar”, “Geabonneerd”, daarna “Lijstvolgorde”. Zet de streamer
hoger in de lijst, of zet “Kijkreeksen eerst” en “Geabonneerde kanalen eerst” uit.

## <a name="related-projects"></a>Gerelateerde projecten

Andere projecten van dezelfde auteur:

- [**Twitch Drops API**](https://drops.sunkwi.com): een gratis openbare API met alle lopende
  Twitch Drops- en beloningscampagnes, elke minuut ververst. Deze app haalt er zijn
  campagnecatalogus uit.
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com): een Discord-bot die livestreams en
  Drops-campagnes op Twitch aankondigt in je server.
- [**Kick Alerts 🔔**](https://kick.sunkwi.com): een Discord-bot die livestreams en
  Drops-campagnes op Kick aankondigt in je server.

## <a name="security"></a>Beveiliging

- Tokens worden verzegeld met AES-256-GCM. De sleutel is ofwel een willekeurige `master.key`
  naast de database (standaard, niets te doen), ofwel afgeleid van het app-wachtwoord met
  Argon2id.
- Het HTTP-dashboard controleert het wachtwoord in constante tijd en gebruikt cookies met
  HttpOnly en SameSite=Strict (`Secure`, met het voorvoegsel `__Host-`, achter HTTPS). De
  server bewaart van elke sessie alleen een SHA-256-hash. Hij beëindigt een sessie na
  30 dagen of bij het afmelden. Hij controleert de sessie voordat hij een body leest en
  beperkt bodies tot 256 KiB.
- Na 5 verkeerde wachtwoorden op rij vanaf één IP-adres (aanmeldformulier of bearer) moet
  dat adres 30 seconden wachten. De server antwoordt dan `429` met `Retry-After`, zonder
  naar het wachtwoord te kijken. Op elk verkeerd wachtwoord komt bovendien pas na een halve
  seconde antwoord.
- Zet het dashboard buiten je netwerk achter een HTTPS-reverse-proxy met
  `MINER_PUBLIC_URL` (zie [Docker](#docker)).
- De miner praat alleen met `twitch.tv`, `ttvnw.net`, `jtvnw.net`, `twitchcdn.net`,
  `twitch-drops-api.sunkwi.com` en de meldingsdoelen die je instelt. De app (desktop en
  Docker) zoekt naar nieuwe versies op `github.com`, en de desktop-app downloadt ze uit de
  GitHub-releases van het project. Ze zijn ondertekend: alleen de getoonde versie wordt
  geïnstalleerd.

## <a name="build"></a>Bouwen vanuit de broncode

**Vereisten:** Rust, Node.js en de
[vereisten van Tauri](https://v2.tauri.app/start/prerequisites/) voor je systeem.

```bash
npm ci
npm run build          # dashboard
cargo build --release -p miner-server   # server: target/release/twitch-miner
npm run bundle:linux   # desktopbundels (de Linux-afhankelijkheden van Tauri zijn vereist)
```

Codecontroles:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier, oxlint (met typecontrole) en svelte-check
```

De TypeScript-typen van de interface worden met
[ts-rs](https://github.com/Aleph-Alpha/ts-rs) uit de Rust-typen gegenereerd, in
`ui/src/lib/bindings`. Voer na een wijziging aan een type dat de interface ziet
`npm run bindings` uit.

## <a name="license"></a>Licentie

[MIT](../../LICENSE). Geïnspireerd door [TwitchDropsMiner van DevilXD](https://github.com/DevilXD/TwitchDropsMiner),
[Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
en hun forks.
