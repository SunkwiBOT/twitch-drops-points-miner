<p align="center">
  <img src="../../ui/public/app-icon.svg" width="112" height="112" alt="Logo for Twitch Drops &amp; Points Miner">
</p>

<h1 align="center">Twitch Drops &amp; Points Miner</h1>

<p align="center">
  <strong>Miner Twitch Drops og kanalpoeng på flere kontoer, uten nettleser og uten noen åpen Twitch-fane.</strong>
</p>

<p align="center">
  En skrivebordsapp for Linux, Windows og macOS, eller et Docker-image med den samme weboversikten, til å kjøre på en hjemmeserver eller på en Raspberry Pi. Med 4 kontoer bruker serveren rundt 30 MB minne og nesten ingen CPU.
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-drops-points-miner?label=release&color=9146FF" alt="Siste versjon"></a>
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-drops-points-miner/total?label=downloads&color=9146FF" alt="Nedlastinger"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="Plattformer: Linux, Windows, macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-drops-points-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="Docker-image for amd64 og arm64"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="MIT-lisens"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="Visninger">
</p>

<p align="center">
  <a href="#download"><strong>Last ned</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/issues/new/choose">Rapporter en feil</a>
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
  <strong>Norsk&nbsp;bokmål</strong> ·
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
  <img src="../media/nb/presentation.avif" alt="Videopresentasjon av Twitch Drops &amp; Points Miner">
</p>

## <a name="features"></a>Funksjoner

Dette skiller den fra andre minere. «Se det i aksjon» under et punkt viser funksjonen i
bruk.

### Kontoer og streamere

- **Flere kontoer i én app.** Hver konto miner på egen hånd, i sin egen modus:
  **Drops og poeng** (én kanal for den beste kampanjen, den andre for streamerne dine),
  **Bare Drops** eller **Bare poeng**.

  <details open><summary>Se det i aksjon</summary>
  <img src="../media/nb/accounts-mining.avif" alt="Flere kontoer som miner samtidig, på Hjem-siden">
  </details>

- **En veiviser for hver nye konto.** Etter innloggingen tar du utgangspunkt i
  standardinnstillingene eller i innstillingene til en annen konto. Velg modus, kampanjer
  og streamere, og deretter «Begynn å mine» eller «Lagre og start senere».

  <details open><summary>Se det i aksjon</summary>
  <img src="../media/nb/account-setup.avif" alt="Oppsettveiviseren for en ny konto, trinn for trinn">
  </details>

- **Én endring brukt på andre kontoer med ett klikk.** Når du har endret noe på en konto,
  tilbyr bekreftelsesmeldingen «Bruk på andre kontoer». Endringen gjøres på nytt på
  kontoene du krysser av, og de beholder resten av innstillingene sine.

  <details open><summary>Se det i aksjon</summary>
  <img src="../media/nb/apply-to-accounts.avif" alt="En endring som gjøres på nytt på andre kontoer med «Bruk på andre kontoer»">
  </details>

- **Legg til en streamer.** Den starter med en gang med standardalternativene: appens,
  eller de du har valgt for denne kontoen. Juster dem med en gang ved behov: bonuskister,
  raids, øyeblikk, seerrekke, chat, spådommer.

  <details open><summary>Se det i aksjon</summary>
  <img src="../media/nb/streamer-add.avif" alt="En streamer legges til via Twitch-søket, deretter alternativene">
  </details>

- **Rediger en streamer.** «Rediger» åpner alternativene: bonuskister, raids, øyeblikk,
  seerrekke, chat, spådommer … En bryter slår den på eller av uten å fjerne den fra
  listen.

  <details open><summary>Se det i aksjon</summary>
  <img src="../media/nb/streamer-edit.avif" alt="Alternativene til en streamer, åpnet fra raden">
  </details>

- **Rediger flere streamere samtidig.** Kryss av for dem, og velg deretter
  «Rediger de N valgte streamerne». Bare alternativene du endrer, blir brukt. Et
  alternativ som er forskjellig mellom dem, viser «Blandet», og bryteren blir til to
  knapper, «Ja» og «Nei». Appen tilbyr deretter å gjøre dem til standardalternativer for
  nye streamere.

  <details open><summary>Se det i aksjon</summary>
  <img src="../media/nb/streamers-group-edit.avif" alt="Redigering av flere streamere, med alternativer som viser «Blandet»">
  </details>

- **Importer streamere.** Lim inn en liste (opptil 500 navn), eller gjenbruk listen til en
  annen konto eller kanalene kontoen følger. Listen kontrolleres, slik at bare kanaler som
  finnes, blir lagt til.

  <details open><summary>Se det i aksjon</summary>
  <img src="../media/nb/streamers-import.avif" alt="Import av streamere: innlimt liste, annen konto, fulgte kanaler">
  </details>

- **Spam Points.** Løs inn en kanals tilpassede belønning flere ganger på rad. Velg en
  kanal fra Poeng-listen din, eller en hvilken som helst kanal du finner med Twitch-søket:
  belønningene vises med kontoens saldo, som leses på nytt fortløpende. Angi antallet
  (opptil 25, innenfor saldoen), og deretter «Løs inn». Resultatet vises når serien er
  ferdig, og hver serie får sin egen linje i Journalen.

  <details open><summary>Se det i aksjon</summary>
  <img src="../media/nb/spam-points.avif" alt="Spam Points: en kanal valgt, 10 × en belønning med melding innløst, innløsningene dukker opp i kanalens Twitch-chat, deretter linjen i Journalen">
  </details>

### Drops

- **Alle pågående Drops-kampanjer.** Hver kampanje har sine egne detaljer: belønninger,
  kobling av spillkontoen, «Sett først i mining-rekkefølgen». Listen kommer fra prosjektet
  [Twitch Drops API](https://drops.sunkwi.com) og oppdateres hvert 10. minutt.

  <details open><summary>Se det i aksjon</summary>
  <img src="../media/nb/drops-catalogue.avif" alt="Katalogen som kalender, søket og detaljene til en kampanje">
  </details>

- **Din mining-rekkefølge.** Dra kampanjene i den rekkefølgen du vil. Nye finner plassen
  sin selv: de prioriterte spillene dine først, deretter nærmeste slutt eller færrest
  gjenværende minutter. Utelukk spill, ord, en kampanje eller én enkelt belønning, og hopp
  over kampanjer som ikke rekker å bli ferdige i tide. «Sett først i mining-rekkefølgen»,
  i detaljene til en kampanje, setter den øverst.

  <details open><summary>Se det i aksjon</summary>
  <img src="../media/nb/drops-order.avif" alt="Mining-rekkefølgen, de prioriterte spillene og unntakene">
  </details>

- **Inventar og «Hent alle».** Inventar-siden sorterer belønningene i
  «Klar til henting», «Pågår» og «Hentet». Hent dem én og én eller alle samtidig, eller
  slå på «Hent drops automatisk».

  <details open><summary>Se det i aksjon</summary>
  <img src="../media/nb/drops-inventory.avif" alt="Inventaret og knappen «Hent alle»">
  </details>

- **«Mine streamere først for Drops».** For den gjeldende kampanjen går en streamer fra
  listen din, der sendingen teller for kampanjen, foran andre kanaler. Den samme kanalen
  gir fremgang på dropen og gir deg kanalpoengene sine. På som standard.

  <details open><summary>Se det i aksjon</summary>
  <img src="../media/nb/drops-streamers-first.avif" alt="En streamer fra listen, sett på for Drops og for poengene sine">
  </details>

### Oppfølging

- **En journal over alt, og hver forespørsel.** Journalen tar vare på hver hendelse:
  drops, opptjente og brukte poeng, kister, veddemål, raids, sendinger som går direkte …
  Filtrer den etter konto, etter fane (Drops, Poeng, Annet) og etter type.
  **Tekniske logger › Forespørsler** viser hver forespørsel som er sendt til Twitch og til
  andre tjenester, med resultatet og hvor lang tid den tok. Logger og forespørsler
  inneholder ingen tokens eller hemmeligheter: «Kopier til feilrapport» kopierer dem, klare
  til å limes inn i en feilrapport på
  [GitHub](https://github.com/SunkwiBOT/twitch-drops-points-miner/issues) eller
  [Discord](https://discord.gg/support-bots-891685895781113886).

  <details open><summary>Se det i aksjon</summary>
  <img src="../media/nb/journal-requests.avif" alt="Den filtrerte journalen, deretter Tekniske logger › Forespørsler">
  </details>

### På skrivebordet eller på en server

- **Det samme grensesnittet overalt.** Skrivebordsappen (Linux, Windows, macOS) og
  Docker-imaget viser den samme oversikten. Docker kjører på en VPS, en NAS eller en
  Raspberry Pi, uten skjerm eller nettleser på den maskinen: oversikten åpnes i hvilken som
  helst nettleser.
- **Lett.** Rundt 30 MB minne for 4 kontoer, nesten ingen CPU og et Docker-image på rundt
  12 MB.

### Sikkerhet og oppdateringer

- **Øktene dine, beskyttet.** Ikke noe Twitch-passord: du logger inn på
  `twitch.tv/activate` fra nettleseren din. Øktene krypteres på disken, og du kan
  tilbakekalle dem i
  [Twitch-innstillingene](https://www.twitch.tv/settings/connections) dine. På
  skrivebordet låser et valgfritt app-passord appen: ingenting mines før det er skrevet
  inn. Med Docker har oversikten sitt eget passord, låser seg selv etter noen minutter uten
  aktivitet og kjører over HTTPS bak en proxy. Fem feil passord på rad sperrer IP-adressen
  i 30 sekunder.

  <details open><summary>Se det i aksjon</summary>
  <img src="../media/nb/security-lock.avif" alt="Hengelåsen: appen låst, deretter låst opp med passordet">
  </details>

- **Signerte oppdateringer, aldri automatiske.** Merket «Oppdatering» vises med
  versjonsnummeret i den øverste linjen. «Installer og start på nytt» ber deg først
  bekrefte, deretter installerer appen versjonen og starter på nytt. Med app-passord
  starter den låst: skriv det inn igjen for å fortsette å mine. Bare en versjon signert av
  prosjektet blir installert. Docker-versjonen sier også fra, med kommandoene som skal
  kjøres.

  <details open><summary>Se det i aksjon</summary>
  <img src="../media/nb/updates.avif" alt="Merket «Oppdatering», bekreftelsen, deretter installasjonen">
  </details>

### Og dessuten

- Som andre minere: bonuskister, raids, øyeblikk, fellesskapsmål, spådommer (strategier,
  når det skal satses, betingelser), seerrekker og abonnerte kanaler først,
  tilstedeværelse i chatten (kan slås av), standardalternativer for nye streamere, varsler
  via Discord, Telegram og webhook med bilder, en global proxy eller én per konto.
- 29 språk, mørkt og lyst tema, skrivebordsvarsler, et ikon i systemstatusfeltet, start
  med systemet.

  <details open><summary>Se det i aksjon</summary>
  <img src="../media/nb/basics.avif" alt="Språk, lyst og mørkt tema, et skrivebordsvarsel og ikonet i systemstatusfeltet">
  </details>

> [!WARNING]
> Dette prosjektet er ikke tilknyttet Twitch. Å automatisere en konto kan bryte Twitchs
> vilkår eller reglene for en kampanje.

## Innhold

- [Sammenligning](#comparison)
- [Last ned](#download)
- [Docker](#docker)
- [Slik fungerer det](#how-it-works)
- [Vanlige spørsmål](#faq)
- [Relaterte prosjekter](#related-projects)
- [Sikkerhet](#security)
- [Bygg fra kildekode](#build)
- [Lisens](#license)

## <a name="comparison"></a>Sammenligning

Slik står
[Twitch Drops & Points Miner](https://github.com/SunkwiBOT/twitch-drops-points-miner)
seg mot
[Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
(kanalpoeng) og [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner) (Drops).
Kontrollert mot koden og README-filene deres i november 2026.

| | Dette prosjektet | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Drops-kampanjer: finner, miner og henter dem | ✅ | ❌ ¹ | ✅ |
| Kanalpoeng: kister, raids, øyeblikk, spådommer | ✅ | ✅ | ❌ |
| Spam Points: en kanals tilpassede belønning løst inn flere ganger på rad | ✅ | ❌ | ❌ |
| Drops og kanalpoeng samtidig på én konto | ✅ | delvis ¹ | ❌ |
| Flere kontoer i én app | ✅ | ❌ ² | ❌ |
| Endre innstillinger på flere kontoer samtidig | ✅ | ❌ | ❌ |
| Grafisk grensesnitt | ✅ skrivebord og web ³ | ❌ ⁴ | ✅ skrivebord |
| Weboversikt for en hjemmeserver (Docker), med passord, tilgjengelig over HTTPS | ✅ | ❌ ⁴ | ❌ |
| Økter kryptert på disk, valgfritt app-passord | ✅ | ❌ | ❌ |
| Grensesnitt på 29 språk | ✅ | ❌ | delvis ⁵ |
| Signerte oppdateringer fra selve appen (skrivebord) | ✅ | ❌ | ❌ |
| Oppsettveiviser for hver nye konto | ✅ | ❌ ⁴ | ❌ |
| Streamere legges til og redigeres i grensesnittet, én og én eller flere samtidig | ✅ | ❌ ⁴ | — |
| Streamere importert via søk, en liste, en annen konto eller fulgte kanaler | ✅ | delvis ⁶ | — |
| Varsler via Discord, Telegram og webhook med bilder | ✅ | delvis ⁷ | ❌ |
| Kalender over alle Drops-kampanjer, med søk | ✅ | ❌ | delvis ⁸ |
| Mining-rekkefølge per kampanje, utelukkede ord, ignorert kampanje eller belønning | ✅ | ❌ | ❌ ⁹ |
| Streamerne på listen din ses først for en kampanjes Drops | ✅ | delvis ¹⁰ | ❌ |
| Filtrerbar hendelsesjournal og en liste over hver forespørsel, i grensesnittet | ✅ | ❌ ⁴ | ❌ |
| Rundt 30 MB minne (Docker-server, 4 kontoer) | ✅ | — ¹¹ | — ¹¹ |

¹ Den ser bare på streamerne på listen sin: den gir fremgang på Drops i sendingene deres og
henter dem, uten å velge kampanje.\
² Én prosess per konto.\
³ På PC-en din som skrivebordsapp, og/eller på en server (VPS, NAS, Raspberry Pi) med
Docker, uten grafisk miljø og uten nettleser på den maskinen: oversikten åpnes i hvilken
som helst nettleser.\
⁴ Konfigureres i en Python-fil; nettsiden viser poengdiagrammer og loggfilen.\
⁵ 21 språk.\
⁶ Den kan følge kanalene kontoen følger.\
⁷ Bare tekst, uten bilder. Den har også Matrix, Pushover og Gotify, som dette prosjektet
ikke har.\
⁸ En liste over kampanjer med filtre, uten kalender eller søk.\
⁹ Den har prioriterte spill og ekskluderte spill, valgt etter spillnavn.\
¹⁰ Som standard ser den først på streamerne på listen sin der sendingen har Drops
(prioriteten `DROPS`), men aldri på en annen kanal.\
¹¹ Ikke målt.

## <a name="download"></a>Last ned

Last ned filen for systemet ditt fra den
[nyeste versjonen](https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest).

| System | Pakke |
|---|---|
| **Windows** · x64 | Installasjonsprogram (`setup.exe`) eller [bærbar kjørbar fil](https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest/download/TwitchDropsPointsMiner-portable.exe) |
| **macOS** 10.15 eller nyere · Intel og Apple Silicon | Diskbilde (`.dmg`) |
| **Linux** · x86-64 | `.AppImage`, `.deb` (Debian, Ubuntu) eller `.rpm` (Fedora, openSUSE) |

macOS-versjonen er ikke notarisert. Første gang høyreklikker du på appen og velger
**Åpne**.

Oppdateringene er signert. Appen ser etter en ny versjon når den starter, og deretter hver
time (hvor ofte endrer du i **Innstillinger › Om**). Den installerer aldri en versjon av
seg selv: du installerer den med ett klikk, fra meldingen som vises, eller fra
**Innstillinger › Om**.

Dataene ligger i brukerens datamappe:

- Linux: `~/.local/share/twitch-drops-points-miner`
- Windows: `%APPDATA%\twitch-drops-points-miner`
- macOS: `~/Library/Application Support/twitch-drops-points-miner`

## <a name="docker"></a>Docker

Serveren viser den samme oversikten i en nettleser. Imaget er
`ghcr.io/sunkwibot/twitch-drops-points-miner`, for `amd64` og `arm64`. Når en ny versjon
er ute, sier oversikten fra: `docker compose pull`, deretter `docker compose up -d`.

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-drops-points-miner:latest
    pull_policy: always   # hent det nyeste imaget ved hver oppstart
    restart: unless-stopped
    ports: ["8080:8080"]   # bak en omvendt HTTPS-proxy: fjern, eller "127.0.0.1:8080:8080"
    environment:
      MINER_PASSWORD: "change-me"   # påkrevd, minst 12 tegn
      # MINER_PUBLIC_URL: "https://miner.example.com"   # bare bak en omvendt HTTPS-proxy
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

Åpne <http://localhost:8080> (eller maskinens adresse på nettverket ditt), og skriv inn
passordet.

### Variabler

| Variabel | Standard | Formål |
|---|---|---|
| `MINER_PASSWORD` | ingen | Passord for oversikten, påkrevd: serveren nekter å starte uten. Minst 12 tegn (et kortere gir en advarsel ved oppstart). Eksempelverdien `change-me` avvises. |
| `MINER_PASSWORD_FILE` | ingen | Leser `MINER_PASSWORD` fra en fil, for eksempel en Docker-secret. Den ene eller den andre, ikke begge. |
| `MINER_PUBLIC_URL` | ingen | Bare bak en omvendt HTTPS-proxy (Caddy, nginx, Traefik, Cloudflare Tunnel …): oversiktens `https://`-adresse, uten sti, for eksempel `https://miner.example.com`. Proxyen må videresende `X-Forwarded-Proto`, `X-Forwarded-For` og websockets, noe vanlige proxyer gjør som standard, og containerens port må ikke kunne nås direkte. Oversikten avviser da alt som ikke kommer gjennom proxyen over HTTPS, og sperrer en adresse i 30 sekunder etter 5 feil passord. Proxyen må kjøre på samme maskin eller i det lokale nettverket dens: en tilkobling fra en offentlig adresse avvises. |
| `MINER_TRUSTED_HOPS` | `1` | Bare med `MINER_PUBLIC_URL`: hvor mange proxyer som legger til en oppføring i `X-Forwarded-For`, fra 1 til 10. `2` med to proxyer etter hverandre, for eksempel Cloudflare foran Traefik. |
| `MINER_PROXY_NETWORKS` | ingen | Bare med `MINER_PUBLIC_URL`: andre nettverk proxyen kan koble til fra, i tillegg til denne maskinen og det private lokale nettverket, skilt med komma: én adresse eller et nettverk som `2001:db8:1::/64`, for et lokalt nettverk med globale IPv6-adresser. |
| `MINER_VAULT_PASSPHRASE` | ingen | Bare for en datamappe kopiert fra skrivebordsappen med et app-passord satt: låser den opp ved oppstart. En server bygget fra bunnen av trenger den aldri. |
| `MINER_VAULT_PASSPHRASE_FILE` | ingen | Leser `MINER_VAULT_PASSPHRASE` fra en fil. Den ene eller den andre, ikke begge. |
| `DATA_DIR` | `/data` i imaget, ellers `data` i arbeidskatalogen | Database, nøkkelfil og logger. |
| `LOG_DIR` | `$DATA_DIR/logs` | Loggfiler: én per dag, de siste 14 beholdes. |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | Lytteadresse: en IP-adresse og en port fra 1 til 65535. |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | Loggnivå. |

Serveren leser og kontrollerer variablene én gang, ved oppstart. En ugyldig verdi stopper
serveren med en melding i stedet for en stille reserveverdi.

## <a name="how-it-works"></a>Slik fungerer det

Hvert 20. sekund velger hver konto sine to kanaler: den beste direktekanalen for den
øverste kampanjen i køen, deretter streamerne på poenglisten.

Twitch godskriver **én Drops-kampanje** og **to kanaler for poeng** om gangen per konto.
Siden oktober 2026 kan en drop bare hentes når spillkontoen er koblet. Ukoblede kampanjer
gir fortsatt fremgang, og oversikten viser lenken.

Kampanjelisten kommer fra prosjektet [Twitch Drops API](https://drops.sunkwi.com)
(oppdatert hvert 10. minutt), fordi Twitch skjuler den for TV-klienten.

## <a name="faq"></a>Vanlige spørsmål

### Risiko og Twitch

**Kan kontoen min bli utestengt?**\
Ingen kan love at det ikke skjer. Dette prosjektet er ikke tilknyttet Twitch, og å
automatisere en konto kan bryte Twitchs vilkår eller reglene for en kampanje. Mineren ser
på høyst to kanaler per konto, Twitchs grense, uten å laste ned noen video. Det er ditt
valg.

**Ser den virkelig på videoen?**\
Nei, ingen video lastes ned. For Drops leser mineren hvert 15. sekund spillelisten til
sendingen i laveste kvalitet og ber bare om headeren til hvert nytt segment. For poeng
sender den Twitchs signal «minutt sett» hvert minutt. Hver kanal som ses på, viser hvor
mange segmenter som er forespurt.

**Hvorfor høyst to kanaler?**\
Det er Twitchs grense: Twitch godskriver bare to kanaler som ses samtidig per konto, og
bare én Drops-kampanje om gangen. I modusen **Drops og poeng** brukes den ene kanalen til
kampanjen og den andre til streamerne dine. Innstillingen «Én kanal», i
**Kontoinnstillinger**, er mer diskret.

### Drops

**En kampanje mines ikke, eller en drop gjør ingen fremgang.**\
Åpne siden **Drops** for kontoen. Under «Mining-rekkefølge» står det ved hver ventende
kampanje hvorfor: «Ingen kanal direkte», «Venter (én kampanje om gangen)» eller «Ingen
drop tilgjengelig ennå». En kampanje som ikke vises i listen, er sortert bort: av
innstillingen «Hvilke kampanjer», av en regel i fanen **Unntak**, eller fordi den ikke
rekker å bli ferdig i tide. En belønning som krever abonnement, mines aldri, og en drop som
følger etter en annen, venter til den første er ferdig. I modusen **Bare poeng** velger
mineren ingen kampanje. «Sett først i mining-rekkefølgen», i detaljene til en kampanje,
setter den øverst; en ignorert kampanje kan komme tilbake med «Ta med i mining igjen», i
fanen **Unntak**.

**Må jeg koble spillkontoen?**\
For å hente, ja: siden oktober 2026 lar Twitch deg bare hente en drop når spillkontoen er
koblet. Som standard gir en ukoblet kampanje likevel fremgang, og hentingen venter på
koblingen. Merket «Ikke koblet» viser det, og «Koble spillkontoen», i detaljene til
kampanjen, åpner koblingssiden. Vil du bare mine koblede spill, velger du
«Bare spill med koblet konto».

### Installasjon og data

**Skrivebordsapp eller Docker?**\
Skrivebordsappen miner mens PC-en din er på, med ikon i systemstatusfeltet, valgfritt
passord og oppdateringer med ett klikk. Docker miner døgnet rundt på en maskin som alltid
er på (VPS, NAS, Raspberry Pi); du åpner oversikten i en nettleser, beskyttet av
`MINER_PASSWORD`. Grensesnittet er det samme.

**Hvor ligger dataene mine?**\
I datamappen: `~/.local/share/twitch-drops-points-miner` på Linux,
`%APPDATA%\twitch-drops-points-miner` på Windows,
`~/Library/Application Support/twitch-drops-points-miner` på macOS, og volumet `/data`
med Docker. På skrivebordet åpner **Innstillinger › Skrivebord** den med ett klikk. Den
inneholder `miner.db` (kontoer, innstillinger, journal), `master.key` (øktnøkkelen, når
ikke noe app-passord er satt) og loggene.

**Hvordan oppdaterer jeg?**\
Skrivebordsappen ser etter en ny versjon når den starter, og deretter hver time (kan
endres i **Innstillinger › Om**). Klikk på merket «Oppdatering», deretter på
«Installer og start på nytt». Med Docker sier oversikten fra: kjør `docker compose pull`
og deretter `docker compose up -d`.

**Jeg har glemt app-passordet.**\
Det kan ikke gjenopprettes: det er det som krypterer Twitch-øktene dine. Avslutt appen,
slett datamappen, og legg deretter til kontoene dine på nytt; innstillingene deres må
settes opp på nytt. Med Docker endrer du passordet for oversikten i `MINER_PASSWORD` og
kjører deretter `docker compose up -d`.

### Poeng og spådommer

**Hvor mange poeng i timen?**\
Det avhenger av Twitch og av hver kanal. Mineren tjener det Twitch gir en seer: poeng for
seertid, bonuskister (omtrent én hvert 15. minutt) og bonusen for seerrekken. Et
abonnement ganger poengene, fra ×1,2 til ×2 avhengig av nivå. Hjem-siden viser opptjente
poeng, og Journal-siden hver enkelt gevinst.

**Er veddemål risikabelt?**\
Ja, poeng kan gå tapt. Spådommer er slått av som standard, streamer for streamer. Du
stiller inn strategien, når det skal satses, innsatsen (5 % av saldoen som standard,
høyst 50 000 poeng), en minstesaldo og, om du vil, en betingelse, for eksempel odds på
minst 1,5. Hold innsatsen lav.

**En streamer blir ikke sett på.**\
Bare streamere som er slått på og er direkte, blir sett på, og bare på en ledig kanal: i
modusen **Drops og poeng** brukes én av de to til den gjeldende kampanjen; i modusen
**Bare Drops** ses ingen streamer på; en konto på pause ser ikke på noe. Plassene fordeles
i denne rekkefølgen, vist som et merke på hver kanal som ses på: «Rekkebonus»,
«Rekke i fare», «Abonnert» og til slutt «Listerekkefølge». Flytt streameren opp i listen,
eller slå av «Seerrekker først» og «Abonnerte kanaler først».

## <a name="related-projects"></a>Relaterte prosjekter

Andre prosjekter fra samme utvikler:

- [**Twitch Drops API**](https://drops.sunkwi.com): et gratis, offentlig API med alle
  pågående Twitch Drops- og belønningskampanjer, oppdatert hvert minutt. Denne appen leser
  kampanjekatalogen sin derfra.
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com): en Discord-bot som kunngjør
  direktesendinger på Twitch og Drops-kampanjer på serveren din.
- [**Kick Alerts 🔔**](https://kick.sunkwi.com): en Discord-bot som kunngjør
  direktesendinger på Kick og Drops-kampanjer på serveren din.

## <a name="security"></a>Sikkerhet

- Tokenene forsegles med AES-256-GCM. Nøkkelen er enten en tilfeldig `master.key` ved
  siden av databasen (standard, ingenting å gjøre) eller avledet fra app-passordet med
  Argon2id.
- HTTP-oversikten sjekker passordet på konstant tid og bruker HttpOnly-,
  SameSite=Strict-informasjonskapsler (`Secure`, med prefikset `__Host-`, bak HTTPS).
  Serveren lagrer bare en SHA-256 av hver økt. Den avslutter en økt etter 30 dager eller
  ved utlogging. Den sjekker økten før den leser innholdet i en forespørsel, og begrenser
  innholdet til 256 KiB.
- Etter 5 feil passord på rad fra én IP-adresse (innloggingsskjema eller bearer) må den
  adressen vente i 30 sekunder. Serveren svarer `429` med `Retry-After` uten å se på
  passordet. Hvert feil passord tar dessuten et halvt sekund å besvare.
- Utenfor nettverket ditt plasserer du oversikten bak en omvendt HTTPS-proxy med
  `MINER_PUBLIC_URL` (se [Docker](#docker)).
- Mineren snakker bare med `twitch.tv`, `ttvnw.net`, `jtvnw.net`, `twitchcdn.net`,
  `drops.sunkwi.com` og varslingsmålene du konfigurerer. Appen (skrivebord og
  Docker) ser etter nye versjoner på `github.com`, og skrivebordsappen laster dem ned fra
  prosjektets GitHub-utgivelser. De er signert: bare den viste versjonen blir installert.

## <a name="build"></a>Bygg fra kildekode

**Forutsetninger:** Rust, Node.js og
[forutsetningene for Tauri](https://v2.tauri.app/start/prerequisites/) for systemet ditt.

```bash
npm ci
npm run build          # oversikten
cargo build --release -p miner-server   # server: target/release/twitch-miner
npm run bundle:linux   # skrivebordspakker (krever Tauris Linux-avhengigheter)
```

Kodesjekker:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier, oxlint (typebevisst) og svelte-check
```

TypeScript-typene til grensesnittet genereres fra Rust-typene med
[ts-rs](https://github.com/Aleph-Alpha/ts-rs) i `ui/src/lib/bindings`. Etter en endring i
en type som grensesnittet ser, kjører du `npm run bindings`.

## <a name="license"></a>Lisens

[MIT](../../LICENSE). Inspirert av
[DevilXDs TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner),
[Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
og forkene deres.
