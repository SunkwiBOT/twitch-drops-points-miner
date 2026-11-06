<p align="center">
  <img src="../../ui/public/app-icon.svg" width="112" height="112" alt="Logo for Twitch Miner">
</p>

<h1 align="center">Twitch Miner</h1>

<p align="center">
  <strong>Miner Twitch Drops og kanalpoint på flere konti uden browser og uden en åben Twitch-fane.</strong>
</p>

<p align="center">
  En skrivebordsapp til Linux, Windows og macOS eller et Docker-image med den samme weboversigt, der kan køre på en hjemmeserver eller på en Raspberry Pi. Med 4 konti bruger serveren omkring 30 MB hukommelse og næsten ingen CPU.
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-miner?label=release&color=9146FF" alt="Seneste udgivelse"></a>
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-miner/total?label=downloads&color=9146FF" alt="Downloads"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="Platforme: Linux, Windows, macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="Docker-image til amd64 og arm64"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="MIT-licens"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="Visninger">
</p>

<p align="center">
  <a href="#download"><strong>Download</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-miner/issues/new/choose">Rapportér en fejl</a>
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
  <strong>Dansk</strong> ·
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
  <img src="../media/da/presentation.avif" alt="Videopræsentation af Twitch Miner">
</p>

## <a name="features"></a>Funktioner

Det her adskiller den fra andre minere. »Se det i aktion« under et punkt viser funktionen i
brug.

### Konti og streamere

- **Flere konti i én app.** Hver konto miner for sig selv, i sin egen tilstand:
  **Drops og point** (én kanal til den bedste kampagne, den anden til dine streamere),
  **Kun Drops** eller **Kun point**.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/da/accounts-mining.avif" alt="Flere konti, der miner samtidig, på siden Hjem">
  </details>

- **En guide til hver ny konto.** Efter login tager du udgangspunkt i
  standardindstillingerne eller i en anden kontos. Vælg tilstand, kampagner og streamere,
  og derefter »Start mining« eller »Gem og start senere«.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/da/account-setup.avif" alt="Opsætningsguiden for en ny konto, trin for trin">
  </details>

- **Én ændring anvendt på andre konti med ét klik.** Når du har ændret noget på en konto,
  tilbyder bekræftelsesbeskeden »Anvend på andre konti«. Ændringen foretages igen på de
  konti, du sætter flueben ved, og de beholder resten af deres indstillinger.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/da/apply-to-accounts.avif" alt="En ændring, der foretages igen på andre konti med »Anvend på andre konti«">
  </details>

- **Tilføj en streamer.** Den starter med det samme med standardindstillingerne: appens
  eller dem, du har valgt til denne konto. Justér dem med det samme efter behov:
  bonuskister, raids, moments, seerstime, chat, forudsigelser.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/da/streamer-add.avif" alt="En streamer tilføjes via Twitch-søgningen, derefter dens indstillinger">
  </details>

- **Redigér en streamer.** »Redigér« åbner dens indstillinger: bonuskister, raids, moments,
  seerstime, chat, forudsigelser … En kontakt slår den til eller fra uden at fjerne den fra
  listen.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/da/streamer-edit.avif" alt="En streamers indstillinger, åbnet fra dens række">
  </details>

- **Redigér flere streamere på én gang.** Sæt flueben ved dem, og vælg derefter
  »Redigér de N valgte streamere«. Kun de indstillinger, du ændrer, bliver anvendt. En
  indstilling, der er forskellig mellem dem, viser »Blandet«, og dens kontakt bliver til to
  knapper, »Ja« og »Nej«. Appen tilbyder derefter at gøre dem til standardindstillinger for
  nye streamere.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/da/streamers-group-edit.avif" alt="Redigering af flere streamere med indstillinger, der viser »Blandet«">
  </details>

- **Importér streamere.** Indsæt en liste (op til 500 navne), eller genbrug en anden kontos
  liste eller de kanaler, kontoen følger. Listen kontrolleres, så kun kanaler, der findes,
  bliver tilføjet.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/da/streamers-import.avif" alt="Import af streamere: indsat liste, anden konto, fulgte kanaler">
  </details>

- **Spam Points.** Indløs en kanals tilpassede belønning flere gange i træk. Vælg en kanal
  fra din Point-liste, eller en hvilken som helst kanal, du finder med Twitch-søgningen:
  dens belønninger vises med kontoens saldo, der læses igen live. Angiv antallet (op til
  25, inden for saldoen), og derefter »Indløs«. Resultatet vises, når serien er slut, og
  hver serie får sin egen linje i Journalen.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/da/spam-points.avif" alt="Spam Points: en kanal valgt, 10 × en belønning med besked indløst, indløsningerne dukker op i kanalens Twitch-chat, derefter linjen i Journalen">
  </details>

### Drops

- **Alle igangværende Drops-kampagner.** Hver kampagne har sine egne detaljer:
  belønninger, tilknytning af spilkontoen, »Sæt først i mining-rækkefølgen«. Listen kommer
  fra projektet [Twitch Drops API](https://drops.sunkwi.com) og opdateres hvert 10. minut.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/da/drops-catalogue.avif" alt="Kataloget som kalender, søgningen og en kampagnes detaljer">
  </details>

- **Din mining-rækkefølge.** Træk kampagnerne i den rækkefølge, du vil have. Nye finder
  selv deres plads: dine prioriterede spil først, derefter den nærmeste slutdato eller
  færrest resterende minutter. Udeluk spil, ord, en kampagne eller en enkelt belønning, og
  spring de kampagner over, der ikke kan nå at blive færdige. »Sæt først i
  mining-rækkefølgen« i en kampagnes detaljer sætter den øverst.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/da/drops-order.avif" alt="Mining-rækkefølgen, de prioriterede spil og udelukkelserne">
  </details>

- **Inventar og »Hent alle«.** Siden Inventar sorterer belønningerne i
  »Klar til hentning«, »I gang« og »Hentet«. Hent dem én ad gangen eller alle på én gang,
  eller slå »Hent drops automatisk« til.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/da/drops-inventory.avif" alt="Inventaret og knappen »Hent alle«">
  </details>

- **»Mine streamere først til Drops«.** For den aktuelle kampagne går en streamer fra din
  liste, hvis stream tæller for kampagnen, forud for andre kanaler. Den samme kanal bringer
  dit drop videre og giver dig sine kanalpoint. Slået til som standard.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/da/drops-streamers-first.avif" alt="En streamer fra listen, der ses for Drops og for sine point">
  </details>

### Overblik

- **En journal over alt, og hver forespørgsel.** Journalen gemmer hver hændelse: drops,
  optjente og brugte point, kister, væddemål, raids, streams, der går live … Filtrér den
  efter konto, efter fane (Drops, Point, Andet) og efter type.
  **Tekniske logfiler › Forespørgsler** viser hver forespørgsel, der er sendt til Twitch og
  til andre tjenester, med resultatet og hvor lang tid den tog. Logfiler og forespørgsler
  indeholder hverken tokens eller hemmeligheder: »Kopiér til fejlrapport« kopierer dem, klar
  til at blive indsat i en fejlrapport på
  [GitHub](https://github.com/SunkwiBOT/twitch-miner/issues) eller
  [Discord](https://discord.gg/support-bots-891685895781113886).

  <details open><summary>Se det i aktion</summary>
  <img src="../media/da/journal-requests.avif" alt="Den filtrerede journal, derefter Tekniske logfiler › Forespørgsler">
  </details>

### På skrivebordet eller på en server

- **Den samme brugerflade overalt.** Skrivebordsappen (Linux, Windows, macOS) og
  Docker-imaget viser den samme oversigt. Docker kører på en VPS, en NAS eller en Raspberry
  Pi uden skærm eller browser på den maskine: oversigten åbner i enhver browser.
  På en telefon tilpasser oversigten sig skærmen, og via HTTPS kan den installeres som en app.
- **Let.** Omkring 30 MB hukommelse til 4 konti, næsten ingen CPU og et Docker-image på
  omkring 12 MB.

### Sikkerhed og opdateringer

- **Dine sessioner, beskyttet.** Ingen Twitch-adgangskode: du logger ind på
  `twitch.tv/activate` fra din browser. Sessionerne krypteres på disken, og du kan
  tilbagekalde dem i dine
  [Twitch-indstillinger](https://www.twitch.tv/settings/connections). På skrivebordet
  låser en valgfri appadgangskode appen: der mines intet, før den er indtastet. Med Docker
  har oversigten sin egen adgangskode, låser sig selv efter nogle minutter uden aktivitet
  og kører over HTTPS bag en proxy. Fem forkerte adgangskoder i træk spærrer IP-adressen i
  30 sekunder.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/da/security-lock.avif" alt="Hængelåsen: appen låst og derefter låst op med sin adgangskode">
  </details>

- **Signerede opdateringer, aldrig automatiske.** Mærket »Opdatering« vises med
  versionsnummeret i den øverste bjælke. »Installer og genstart« beder dig først bekræfte,
  derefter installerer appen versionen og genstarter. Med en appadgangskode genstarter den
  låst: indtast den igen for at fortsætte med at mine. Kun en version, der er signeret af
  projektet, bliver installeret. Docker-versionen giver også besked, med de kommandoer, der
  skal køres.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/da/updates.avif" alt="Mærket »Opdatering«, bekræftelsen og derefter installationen">
  </details>

### Og desuden

- Som andre minere: bonuskister, raids, moments, fællesskabsmål, forudsigelser
  (strategier, hvornår der satses, betingelser), seerstimer og abonnerede kanaler først,
  tilstedeværelse i chatten (kan slås fra), standardindstillinger for nye streamere,
  notifikationer via Discord, Telegram og webhook med billeder, en global proxy eller én pr.
  konto.
- **Kommer snart: mining af badges.** De Twitch-badges, man optjener ved at se med, mines ligesom Drops.
- 29 sprog, mørkt og lyst tema, skrivebordsnotifikationer, et ikon i proceslinjen, start
  sammen med systemet.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/da/basics.avif" alt="Sprog, lyst og mørkt tema, en skrivebordsnotifikation og ikonet i proceslinjen">
  </details>

> [!WARNING]
> Dette projekt er ikke tilknyttet Twitch. Automatisering af en konto kan bryde Twitchs
> vilkår eller en kampagnes regler.

## Indhold

- [Sammenligning](#comparison)
- [Download](#download)
- [Docker](#docker)
- [Sådan virker det](#how-it-works)
- [Ofte stillede spørgsmål](#faq)
- [Relaterede projekter](#related-projects)
- [Sikkerhed](#security)
- [Byg fra kildekoden](#build)
- [Licens](#license)

## <a name="comparison"></a>Sammenligning

Sådan står
[Twitch Miner](https://github.com/SunkwiBOT/twitch-miner) i
forhold til
[Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
(kanalpoint) og [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner) (Drops).
Kontrolleret ud fra deres kode og README-filer i november 2026.

| | Dette projekt | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Drops-kampagner: finder, miner og henter dem | ✅ | ❌ ¹ | ✅ |
| Kanalpoint: kister, raids, moments, forudsigelser | ✅ | ✅ | ❌ |
| Spam Points: en kanals tilpassede belønning indløst flere gange i træk | ✅ | ❌ | ❌ |
| Drops og kanalpoint på samme tid på én konto | ✅ | delvist ¹ | ❌ |
| Flere konti i én app | ✅ | ❌ ² | ❌ |
| Ændr indstillinger på flere konti på én gang | ✅ | ❌ | ❌ |
| Grafisk brugerflade | ✅ skrivebord og web ³ | ❌ ⁴ | ✅ skrivebord |
| Weboversigt til en hjemmeserver (Docker), med adgangskode, tilgængelig over HTTPS | ✅ | ❌ ⁴ | ❌ |
| Sessioner krypteret på disken, valgfri appadgangskode | ✅ | ❌ | ❌ |
| Brugerflade på 29 sprog | ✅ | ❌ | delvist ⁵ |
| Signerede opdateringer inde fra appen (skrivebord) | ✅ | ❌ | ❌ |
| Opsætningsguide til hver ny konto | ✅ | ❌ ⁴ | ❌ |
| Streamere tilføjes og redigeres i brugerfladen, én ad gangen eller flere på én gang | ✅ | ❌ ⁴ | — |
| Streamere importeret via søgning, en liste, en anden konto eller fulgte kanaler | ✅ | delvist ⁶ | — |
| Discord-, Telegram- og webhook-notifikationer med billeder | ✅ | delvist ⁷ | ❌ |
| Kalender over alle Drops-kampagner, med søgning | ✅ | ❌ | delvist ⁸ |
| Mining-rækkefølge pr. kampagne, udelukkede ord, ignoreret kampagne eller belønning | ✅ | ❌ | ❌ ⁹ |
| Streamerne på din liste ses først for en kampagnes Drops | ✅ | delvist ¹⁰ | ❌ |
| Filtrérbar hændelsesjournal og en liste over hver forespørgsel, i brugerfladen | ✅ | ❌ ⁴ | ❌ |
| Omkring 30 MB hukommelse (Docker-server, 4 konti) | ✅ | — ¹¹ | — ¹¹ |

¹ Den ser kun streamerne på sin liste: den bringer Drops i deres streams videre og henter
dem uden at vælge en kampagne.\
² Én proces pr. konto.\
³ På din pc som skrivebordsapp og/eller på en server (VPS, NAS, Raspberry Pi) med Docker,
uden grafisk miljø og uden browser på den maskine: oversigten åbner i enhver browser.\
⁴ Konfigureres i en Python-fil; dens webside viser pointdiagrammer og logfilen.\
⁵ 21 sprog.\
⁶ Den kan følge de kanaler, kontoen følger.\
⁷ Kun tekst, uden billeder. Den har også Matrix, Pushover og Gotify, som dette projekt ikke
har.\
⁸ En liste over kampagner med filtre, uden kalender eller søgning.\
⁹ Den har prioriterede spil og udelukkede spil, valgt efter spillets navn.\
¹⁰ Som standard ser den først de streamere på sin liste, hvis stream har Drops (dens
prioritet `DROPS`), men aldrig en anden kanal.\
¹¹ Ikke målt.

## <a name="download"></a>Download

Hent filen til dit system fra den
[seneste udgivelse](https://github.com/SunkwiBOT/twitch-miner/releases/latest).

| System | Pakke |
|---|---|
| **Windows** · x64 | Installationsprogram (`setup.exe`) eller [portabel eksekverbar fil](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/TwitchMiner-portable.exe) |
| **macOS** 10.15 eller nyere · Intel og Apple Silicon | Diskbillede (`.dmg`) |
| **Linux** · x86-64 | `.AppImage`, `.deb` (Debian, Ubuntu) eller `.rpm` (Fedora, openSUSE) |

macOS-versionen er ikke notariseret. Første gang skal du højreklikke på appen og vælge
**Åbn**.

Opdateringer er signerede. Appen søger efter en ny version, når den starter, og derefter
hver time (hvor ofte, ændrer du i **Indstillinger › Om**). Den installerer aldrig en
version af sig selv: du installerer den med ét klik, fra den besked, der vises, eller fra
**Indstillinger › Om**.

Data ligger i brugerens datamappe:

- Linux: `~/.local/share/twitch-miner`
- Windows: `%APPDATA%\twitch-miner`
- macOS: `~/Library/Application Support/twitch-miner`

## <a name="docker"></a>Docker

Serveren viser den samme oversigt i en browser. Imaget er
`ghcr.io/sunkwibot/twitch-miner`, til `amd64` og `arm64`. Når en ny version
er ude, siger oversigten det: `docker compose pull` og derefter `docker compose up -d`.

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-miner:latest
    pull_policy: always   # hent det nyeste image ved hver start
    restart: unless-stopped
    ports: ["8080:8080"]   # bag en reverse proxy til HTTPS: fjern, eller "127.0.0.1:8080:8080"
    environment:
      MINER_PASSWORD: "change-me"   # påkrævet, mindst 12 tegn
      # MINER_PUBLIC_URL: "https://miner.example.com"   # kun bag en reverse proxy til HTTPS
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

Åbn <http://localhost:8080> (eller maskinens adresse på dit netværk), og indtast
adgangskoden.

### Variabler

| Variabel | Standard | Formål |
|---|---|---|
| `MINER_PASSWORD` | ingen | Adgangskode til oversigten, påkrævet: serveren nægter at starte uden. Mindst 12 tegn (en kortere giver en advarsel ved start). Eksempelværdien `change-me` afvises. |
| `MINER_PASSWORD_FILE` | ingen | Læser `MINER_PASSWORD` fra en fil, f.eks. en Docker-secret. Den ene eller den anden, ikke begge. |
| `MINER_PUBLIC_URL` | ingen | Kun bag en reverse proxy til HTTPS (Caddy, nginx, Traefik, Cloudflare Tunnel …): oversigtens `https://`-adresse, uden sti, f.eks. `https://miner.example.com`. Proxyen skal videresende `X-Forwarded-Proto`, `X-Forwarded-For` og websockets, hvilket almindelige proxyer gør som standard, og containerens port må ikke kunne nås direkte. Oversigten afviser så alt, der ikke kommer gennem proxyen over HTTPS, og spærrer en adresse i 30 sekunder efter 5 forkerte adgangskoder. Proxyen skal køre på samme maskine eller dens lokale netværk: en forbindelse fra en offentlig adresse afvises. |
| `MINER_TRUSTED_HOPS` | `1` | Kun med `MINER_PUBLIC_URL`: hvor mange proxyer der tilføjer en post til `X-Forwarded-For`, fra 1 til 10. `2` med to proxyer bag hinanden, f.eks. Cloudflare foran Traefik. |
| `MINER_PROXY_NETWORKS` | ingen | Kun med `MINER_PUBLIC_URL`: andre netværk, proxyen må forbinde fra, ud over denne maskine og dens private lokale netværk, adskilt med kommaer: en enkelt adresse eller et netværk som `2001:db8:1::/64`, for et lokalt netværk med globale IPv6-adresser. |
| `MINER_VAULT_PASSPHRASE` | ingen | Kun til en datamappe kopieret fra skrivebordsappen med en appadgangskode sat: låser den op ved start. En server bygget fra bunden har aldrig brug for den. |
| `MINER_VAULT_PASSPHRASE_FILE` | ingen | Læser `MINER_VAULT_PASSPHRASE` fra en fil. Den ene eller den anden, ikke begge. |
| `DATA_DIR` | `/data` i imaget, ellers `data` i arbejdsmappen | Database, nøglefil og logfiler. |
| `LOG_DIR` | `$DATA_DIR/logs` | Logfiler: én om dagen, de seneste 14 gemmes. |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | Lytteadresse: en IP-adresse og en port fra 1 til 65535. |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | Logniveau. |

Serveren læser og kontrollerer variablerne én gang, ved start. En ugyldig værdi stopper
serveren med en besked i stedet for en stille reserveværdi.

## <a name="how-it-works"></a>Sådan virker det

Hvert 20. sekund vælger hver konto sine to kanaler: den bedste livekanal til den øverste
kampagne i sin kø og derefter streamerne på sin pointliste.

Twitch krediterer **én Drops-kampagne** og **to kanaler til point** ad gangen pr. konto.
Siden oktober 2026 kan et drop kun hentes, når spilkontoen er tilknyttet. Kampagner uden
tilknytning giver stadig fremskridt, og oversigten viser linket.

Kampagnelisten kommer fra projektet [Twitch Drops API](https://drops.sunkwi.com)
(opdateret hvert 10. minut), fordi Twitch skjuler den for tv-klienten.

## <a name="faq"></a>Ofte stillede spørgsmål

### Risici og Twitch

**Kan min konto blive spærret?**\
Ingen kan love, at det ikke sker. Dette projekt er ikke tilknyttet Twitch, og
automatisering af en konto kan bryde Twitchs vilkår eller en kampagnes regler. Mineren ser
højst to kanaler pr. konto, Twitchs grænse, uden at downloade nogen video. Det er dit
valg.

**Ser den virkelig videoen?**\
Nej, der downloades ingen video. For Drops læser mineren hvert 15. sekund streamens
playliste i laveste kvalitet og beder kun om headeren for hvert nyt segment. For point
sender den hvert minut Twitchs signal »minut set«. Hver kanal, der ses, viser, hvor mange
segmenter der er anmodet om.

**Hvorfor højst to kanaler?**\
Det er Twitchs grænse: Twitch krediterer kun to kanaler, der ses samtidig pr. konto, og
kun én Drops-kampagne ad gangen. I tilstanden **Drops og point** bruges den ene kanal til
kampagnen og den anden til dine streamere. Indstillingen »Én kanal« i
**Kontoindstillinger** er mere diskret.

### Drops

**En kampagne bliver ikke minet, eller et drop gør ingen fremskridt.**\
Åbn kontoens side **Drops**. Under »Mining-rækkefølge« står der ved hver ventende kampagne
hvorfor: »Ingen kanal live«, »Venter (én kampagne ad gangen)« eller »Ingen drop
tilgængelig endnu«. En kampagne, der ikke vises i listen, er sorteret fra: af
indstillingen »Hvilke kampagner«, af en regel på fanen **Udelukkelser**, eller fordi den
ikke kan nå at blive færdig. En belønning, der kræver et abonnement, bliver aldrig minet,
og et drop, der følger efter et andet, venter, til det første er færdigt. I tilstanden
**Kun point** vælger mineren ingen kampagne. »Sæt først i mining-rækkefølgen« i en
kampagnes detaljer sætter den øverst; en ignoreret kampagne kan komme tilbage med
»Medtag i mining igen« på fanen **Udelukkelser**.

**Skal jeg tilknytte spilkontoen?**\
For at hente, ja: siden oktober 2026 lader Twitch dig kun hente et drop, når spilkontoen
er tilknyttet. Som standard giver en kampagne uden tilknytning stadig fremskridt, og
hentningen venter på tilknytningen. Mærket »Ikke tilknyttet« viser det, og
»Tilknyt spilkontoen« i kampagnens detaljer åbner tilknytningssiden. Vil du kun mine
tilknyttede spil, så vælg »Kun spil med tilknyttet konto«.

### Installation og data

**Skrivebordsapp eller Docker?**\
Skrivebordsappen miner, mens din pc er tændt, med ikon i proceslinjen, valgfri adgangskode
og opdateringer med ét klik. Docker miner døgnet rundt på en maskine, der altid er tændt
(VPS, NAS, Raspberry Pi); du åbner oversigten i en browser, beskyttet af `MINER_PASSWORD`.
Brugerfladen er den samme.

**Hvor ligger mine data?**\
I datamappen: `~/.local/share/twitch-miner` på Linux,
`%APPDATA%\twitch-miner` på Windows,
`~/Library/Application Support/twitch-miner` på macOS og volumen `/data`
med Docker. På skrivebordet åbner **Indstillinger › Skrivebord** den med ét klik. Den
indeholder `miner.db` (konti, indstillinger, journal), `master.key` (sessionsnøglen, når
der ikke er sat en appadgangskode) og logfilerne.

**Hvordan opdaterer jeg?**\
Skrivebordsappen søger efter en ny version, når den starter, og derefter hver time (kan
ændres i **Indstillinger › Om**). Klik på mærket »Opdatering« og derefter på
»Installer og genstart«. Med Docker giver oversigten besked: kør `docker compose pull` og
derefter `docker compose up -d`.

**Jeg har glemt appadgangskoden.**\
Den kan ikke gendannes: det er den, der krypterer dine Twitch-sessioner. Luk appen, slet
datamappen, og tilføj derefter dine konti igen; deres indstillinger skal sættes op igen.
Med Docker ændrer du oversigtens adgangskode i `MINER_PASSWORD` og kører derefter
`docker compose up -d`.

### Point og forudsigelser

**Hvor mange point i timen?**\
Det afhænger af Twitch og af den enkelte kanal. Mineren optjener det, Twitch giver en
seer: point for seertid, bonuskister (omkring én hvert 15. minut) og bonussen for
seerstimen. Et abonnement ganger pointene, fra ×1,2 til ×2 afhængigt af niveauet. Siden
Hjem viser de optjente point og siden Journal hver enkelt gevinst.

**Er væddemål risikable?**\
Ja, point kan gå tabt. Forudsigelser er slået fra som standard, streamer for streamer. Du
indstiller strategien, hvornår der satses, indsatsen (5 % af saldoen som standard, højst
50.000 point), en minimumssaldo og, hvis du vil, en betingelse, f.eks. odds på mindst
1,5. Hold indsatsen lav.

**En streamer bliver ikke set.**\
Kun streamere, der er slået til og er live, bliver set, og kun på en ledig kanal: i
tilstanden **Drops og point** bruges den ene af de to til den aktuelle kampagne; i
tilstanden **Kun Drops** ses ingen streamer; en konto på pause ser ingenting. Pladserne
fordeles i denne rækkefølge, vist som et mærke på hver kanal, der ses: »Stimebonus«,
»Stime i fare«, »Abonneret« og til sidst »Listerækkefølge«. Ryk streameren op på listen,
eller slå »Seerstimer først« og »Abonnerede kanaler først« fra.

## <a name="related-projects"></a>Relaterede projekter

Andre projekter fra samme udvikler:

- [**Twitch Drops API**](https://drops.sunkwi.com): et gratis, offentligt API med alle
  igangværende Twitch Drops- og belønningskampagner, opdateret hvert minut. Denne app
  læser sit kampagnekatalog derfra.
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com): en Discord-bot, der annoncerer
  livestreams på Twitch og Drops-kampagner på din server.
- [**Kick Alerts 🔔**](https://kick.sunkwi.com): en Discord-bot, der annoncerer livestreams
  på Kick og Drops-kampagner på din server.

## <a name="security"></a>Sikkerhed

- Tokens forsegles med AES-256-GCM. Nøglen er enten en tilfældig `master.key` ved siden af
  databasen (standard, intet at gøre) eller afledt af appadgangskoden med Argon2id.
- HTTP-oversigten kontrollerer adgangskoden i konstant tid og bruger HttpOnly-,
  SameSite=Strict-cookies (`Secure`, med præfikset `__Host-`, bag HTTPS). Serveren gemmer
  kun en SHA-256 af hver session. Den afslutter en session efter 30 dage eller ved logout.
  Den kontrollerer sessionen, før den læser noget anmodningsindhold, og begrænser
  indholdet til 256 KiB.
- Efter 5 forkerte adgangskoder i træk fra én IP-adresse (loginformular eller bearer) skal
  den adresse vente 30 sekunder. Serveren svarer `429` med `Retry-After` uden at se på
  adgangskoden. Hver forkert adgangskode tager desuden et halvt sekund at besvare.
- Uden for dit netværk skal du placere oversigten bag en reverse proxy til HTTPS med
  `MINER_PUBLIC_URL` (se [Docker](#docker)).
- Mineren taler kun med `twitch.tv`, `ttvnw.net`, `jtvnw.net`, `twitchcdn.net`,
  `drops.sunkwi.com` og de notifikationsmål, du konfigurerer. Appen (skrivebord
  og Docker) søger efter nye versioner på `github.com`, og skrivebordsappen henter dem fra
  projektets GitHub-udgivelser. De er signerede: kun den viste version bliver installeret.

## <a name="build"></a>Byg fra kildekoden

**Forudsætninger:** Rust, Node.js og
[Tauris forudsætninger](https://v2.tauri.app/start/prerequisites/) til dit system.

```bash
npm ci
npm run build          # oversigten
cargo build --release -p miner-server   # server: target/release/twitch-miner
npm run bundle:linux   # skrivebordspakker (Tauris Linux-afhængigheder kræves)
```

Kodetjek:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier, oxlint (typebevidst) og svelte-check
```

Brugerfladens TypeScript-typer genereres ud fra Rust-typerne med
[ts-rs](https://github.com/Aleph-Alpha/ts-rs) i `ui/src/lib/bindings`. Efter en ændring af
en type, som brugerfladen ser, skal du køre `npm run bindings`.

## <a name="license"></a>Licens

[MIT](../../LICENSE). Inspireret af
[DevilXDs TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner),
[Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
og deres forks.
