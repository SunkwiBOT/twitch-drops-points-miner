<p align="center">
  <img src="../../ui/public/app-icon.svg" width="112" height="112" alt="Logotyp för Twitch Miner">
</p>

<h1 align="center">Twitch Miner</h1>

<p align="center">
  <strong>Minar Twitch Drops och kanalpoäng på flera konton, utan webbläsare och utan någon öppen Twitch-flik.</strong>
</p>

<p align="center">
  En skrivbordsapp för Linux, Windows och macOS, eller en Docker-avbild med samma webböversikt, att köra på en hemserver eller på en Raspberry Pi. Med 4 konton använder servern cirka 30 MB minne och nästan ingen CPU.
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-miner?label=release&color=9146FF" alt="Senaste versionen"></a>
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-miner/total?label=downloads&color=9146FF" alt="Nedladdningar"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="Plattformar: Linux, Windows, macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="Docker-avbild för amd64 och arm64"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="MIT-licens"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="Visningar">
</p>

<p align="center">
  <a href="#download"><strong>Ladda ner</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-miner/issues/new/choose">Rapportera en bugg</a>
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
  <strong>Svenska</strong> ·
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
  <img src="../media/sv/presentation.avif" alt="Videopresentation av Twitch Miner">
</p>

## <a name="features"></a>Funktioner

Det här skiljer den från andra miners. ”Se det i aktion” under en punkt visar funktionen i
arbete.

### Konton och streamers

- **Flera konton i en app.** Varje konto minar för sig, i sitt eget läge:
  **Drops och poäng** (en kanal för den bästa kampanjen, den andra för dina streamers),
  **Endast Drops** eller **Endast poäng**.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/sv/accounts-mining.avif" alt="Flera konton som minar samtidigt, på sidan Hem">
  </details>

- **En guide för varje nytt konto.** Efter inloggningen utgår du från
  standardinställningarna eller från ett annat kontos. Välj läge, kampanjer och streamers,
  och sedan ”Börja mina” eller ”Spara och starta senare”.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/sv/account-setup.avif" alt="Inställningsguiden för ett nytt konto, steg för steg">
  </details>

- **En ändring tillämpad på andra konton med ett klick.** När du har ändrat något på ett
  konto erbjuder bekräftelsemeddelandet ”Tillämpa på andra konton”. Ändringen görs om på de
  konton du kryssar i, och de behåller resten av sina inställningar.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/sv/apply-to-accounts.avif" alt="En ändring som görs om på andra konton med ”Tillämpa på andra konton”">
  </details>

- **Lägg till en streamer.** Den startar direkt med standardalternativen: appens, eller de
  som du har valt för det här kontot. Justera dem direkt vid behov: bonuskistor, raids,
  moments, tittarsvit, chatt, förutsägelser.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/sv/streamer-add.avif" alt="En streamer läggs till via Twitch-sökningen, sedan dess alternativ">
  </details>

- **Redigera en streamer.** ”Redigera” öppnar dess alternativ: bonuskistor, raids,
  moments, tittarsvit, chatt, förutsägelser… Ett reglage slår på eller av den utan att ta
  bort den från listan.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/sv/streamer-edit.avif" alt="En streamers alternativ, öppnade från dess rad">
  </details>

- **Redigera flera streamers samtidigt.** Kryssa i dem och välj sedan ”Redigera N valda
  streamers”. Bara de alternativ du ändrar tillämpas. Ett alternativ som skiljer sig mellan
  dem visar ”Blandat”, och dess reglage blir två knappar, ”Ja” och ”Nej”. Appen erbjuder
  sedan att göra dem till standardalternativ för nya streamers.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/sv/streamers-group-edit.avif" alt="Redigering av flera streamers, med alternativ som visar ”Blandat”">
  </details>

- **Importera streamers.** Klistra in en lista (upp till 500 namn), eller återanvänd ett
  annat kontos lista eller kanalerna som kontot följer. Listan kontrolleras så att bara
  kanaler som finns läggs till.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/sv/streamers-import.avif" alt="Import av streamers: inklistrad lista, annat konto, följda kanaler">
  </details>

- **Spam Points.** Lös in en kanals anpassade belöning flera gånger i rad. Välj en kanal
  från din Poänglista, eller vilken kanal som helst som du hittar med Twitch-sökningen:
  dess belöningar visas med kontots saldo, som läses om live. Ange antalet (upp till 25,
  inom saldot) och sedan ”Lös in”. Resultatet visas när serien är klar, och varje serie
  får en egen rad i Journalen.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/sv/spam-points.avif" alt="Spam Points: en kanal vald, 10 × en belöning med meddelande inlöst, inlösningarna dyker upp i kanalens Twitch-chatt, sedan raden i Journalen">
  </details>

### Drops

- **Alla pågående Drops-kampanjer.** Varje kampanj har sina egna detaljer: belöningar,
  länkning av spelkontot, ”Mina först”. Listan kommer från projektet
  [Twitch Drops API](https://drops.sunkwi.com) och uppdateras var 10:e minut.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/sv/drops-catalogue.avif" alt="Katalogen som kalender, sökningen och en kampanjs detaljer">
  </details>

- **Din miningordning.** Dra kampanjerna i den ordning du vill. Nya hittar sin plats
  själva: dina prioriterade spel först, sedan det närmaste slutet eller minst återstående
  minuter. Uteslut spel, ord, en kampanj eller en enskild belöning, och hoppa över
  kampanjer som inte hinner bli klara i tid. ”Mina först”, i en kampanjs detaljer, sätter
  den överst.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/sv/drops-order.avif" alt="Miningordningen, de prioriterade spelen och undantagen">
  </details>

- **Inventarie och ”Hämta alla”.** Sidan Inventarie sorterar belöningarna i ”Att hämta”,
  ”Pågår” och ”Hämtade”. Hämta dem en i taget eller alla på en gång, eller slå på
  ”Hämta drops automatiskt”.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/sv/drops-inventory.avif" alt="Inventariet och knappen ”Hämta alla”">
  </details>

- **”Mina streamers först för Drops”.** För den aktuella kampanjen går en streamer från
  din lista, vars sändning räknas för kampanjen, före andra kanaler. Samma kanal ger
  framsteg på din drop och ger dig sina kanalpoäng. På som standard.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/sv/drops-streamers-first.avif" alt="En streamer från listan som tittas på för Drops och för sina poäng">
  </details>

### Uppföljning

- **En journal över allt, och varje förfrågan.** Journalen sparar varje händelse: drops,
  intjänade och spenderade poäng, kistor, vadslagningar, raids, sändningar som går live…
  Filtrera den på konto, på flik (Drops, Poäng, Övrigt) och på typ.
  **Tekniska loggar › Förfrågningar** listar varje förfrågan som skickats till Twitch och
  till andra tjänster, med resultatet och hur lång tid den tog. Loggar och förfrågningar
  innehåller inga tokens eller hemligheter: ”Kopiera för buggrapport” kopierar dem, redo att
  klistras in i en buggrapport på
  [GitHub](https://github.com/SunkwiBOT/twitch-miner/issues) eller
  [Discord](https://discord.gg/support-bots-891685895781113886).

  <details open><summary>Se det i aktion</summary>
  <img src="../media/sv/journal-requests.avif" alt="Den filtrerade journalen, sedan Tekniska loggar › Förfrågningar">
  </details>

### På skrivbordet eller på en server

- **Samma gränssnitt överallt.** Skrivbordsappen (Linux, Windows, macOS) och
  Docker-avbilden visar samma översikt. Docker körs på en VPS, en NAS eller en Raspberry
  Pi, utan skärm eller webbläsare på den maskinen: översikten öppnas i valfri webbläsare.
  På en telefon anpassar sig översikten efter skärmen, och via HTTPS kan den installeras som en app.
- **Lätt.** Cirka 30 MB minne för 4 konton, nästan ingen CPU och en Docker-avbild på
  cirka 12 MB.

### Säkerhet och uppdateringar

- **Dina sessioner, skyddade.** Inget Twitch-lösenord: du loggar in på
  `twitch.tv/activate` från din webbläsare. Sessionerna krypteras på disken, och du kan
  återkalla dem i dina [Twitch-inställningar](https://www.twitch.tv/settings/connections).
  På skrivbordet låser ett valfritt applösenord appen: ingenting minas förrän det har
  skrivits in. Med Docker har översikten ett eget lösenord, låser sig själv efter några
  minuter utan aktivitet och körs över HTTPS bakom en proxy. Fem felaktiga lösenord i rad
  spärrar IP-adressen i 30 sekunder.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/sv/security-lock.avif" alt="Hänglåset: appen låst och sedan upplåst med sitt lösenord">
  </details>

- **Signerade uppdateringar, aldrig automatiska.** Märket ”Uppdatering” visas med
  versionsnumret i det övre fältet. ”Installera och starta om” ber dig först bekräfta,
  sedan installerar appen versionen och startar om. Med ett applösenord startar den om
  låst: skriv in det igen för att fortsätta mina. Bara en version som signerats av
  projektet installeras. Docker-versionen säger också till, med kommandona som ska köras.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/sv/updates.avif" alt="Märket ”Uppdatering”, bekräftelsen och sedan installationen">
  </details>

### Och dessutom

- Som andra miners: bonuskistor, raids, moments, communitymål, förutsägelser
  (strategier, när det ska satsas, villkor), tittarsviter och prenumererade kanaler först,
  närvaro i chatten (kan stängas av), standardalternativ för nya streamers, aviseringar via
  Discord, Telegram och webhook med bilder, en global proxy eller en per konto.
- **Kommer snart: mining av märken.** Twitch-märkena man får genom att titta, minade som Drops.
- 29 språk, mörkt och ljust tema, skrivbordsaviseringar, en ikon i systemfältet, start med
  systemet.

  <details open><summary>Se det i aktion</summary>
  <img src="../media/sv/basics.avif" alt="Språk, ljust och mörkt tema, en skrivbordsavisering och ikonen i systemfältet">
  </details>

> [!WARNING]
> Det här projektet är inte kopplat till Twitch. Att automatisera ett konto kan bryta mot
> Twitchs villkor eller en kampanjs regler.

## Innehåll

- [Jämförelse](#comparison)
- [Ladda ner](#download)
- [Docker](#docker)
- [Så fungerar det](#how-it-works)
- [Vanliga frågor](#faq)
- [Relaterade projekt](#related-projects)
- [Säkerhet](#security)
- [Bygga från källkod](#build)
- [Licens](#license)

## <a name="comparison"></a>Jämförelse

Så står sig
[Twitch Miner](https://github.com/SunkwiBOT/twitch-miner)
mot
[Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
(kanalpoäng) och [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner) (Drops).
Kontrollerat mot deras kod och README-filer i november 2026.

| | Det här projektet | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Drops-kampanjer: hittar, minar och hämtar dem | ✅ | ❌ ¹ | ✅ |
| Kanalpoäng: kistor, raids, moments, förutsägelser | ✅ | ✅ | ❌ |
| Spam Points: en kanals anpassade belöning inlöst flera gånger i rad | ✅ | ❌ | ❌ |
| Drops och kanalpoäng samtidigt på ett konto | ✅ | delvis ¹ | ❌ |
| Flera konton i en app | ✅ | ❌ ² | ❌ |
| Ändra inställningar på flera konton samtidigt | ✅ | ❌ | ❌ |
| Grafiskt gränssnitt | ✅ skrivbord och webb ³ | ❌ ⁴ | ✅ skrivbord |
| Webböversikt för en hemserver (Docker), med lösenord, nåbar över HTTPS | ✅ | ❌ ⁴ | ❌ |
| Sessioner krypterade på disk, valfritt applösenord | ✅ | ❌ | ❌ |
| Gränssnitt på 29 språk | ✅ | ❌ | delvis ⁵ |
| Signerade uppdateringar inifrån appen (skrivbord) | ✅ | ❌ | ❌ |
| Inställningsguide för varje nytt konto | ✅ | ❌ ⁴ | ❌ |
| Streamers läggs till och redigeras i gränssnittet, en i taget eller flera samtidigt | ✅ | ❌ ⁴ | — |
| Streamers importerade via sökning, en lista, ett annat konto eller följda kanaler | ✅ | delvis ⁶ | — |
| Aviseringar via Discord, Telegram och webhook med bilder | ✅ | delvis ⁷ | ❌ |
| Kalender över alla Drops-kampanjer, med sökning | ✅ | ❌ | delvis ⁸ |
| Miningordning per kampanj, uteslutna ord, ignorerad kampanj eller belöning | ✅ | ❌ | ❌ ⁹ |
| Streamers från din lista tittas på först för en kampanjs Drops | ✅ | delvis ¹⁰ | ❌ |
| Filtrerbar händelsejournal och en lista över varje förfrågan, i gränssnittet | ✅ | ❌ ⁴ | ❌ |
| Cirka 30 MB minne (Docker-server, 4 konton) | ✅ | — ¹¹ | — ¹¹ |

¹ Den tittar bara på streamers på sin lista: den ger framsteg på Drops i deras sändningar
och hämtar dem, utan att välja kampanj.\
² En process per konto.\
³ På din dator som skrivbordsapp, och/eller på en server (VPS, NAS, Raspberry Pi) med
Docker, utan grafisk miljö och utan webbläsare på den maskinen: översikten öppnas i valfri
webbläsare.\
⁴ Konfigureras i en Python-fil; dess webbsida visar poängdiagram och loggfilen.\
⁵ 21 språk.\
⁶ Den kan följa kanalerna som kontot följer.\
⁷ Bara text, utan bilder. Den har också Matrix, Pushover och Gotify, som det här projektet
saknar.\
⁸ En lista över kampanjer med filter, utan kalender eller sökning.\
⁹ Den har prioriterade spel och uteslutna spel, valda efter spelnamn.\
¹⁰ Som standard tittar den först på streamers på sin lista vars sändning har Drops (dess
prioritet `DROPS`), men aldrig på en annan kanal.\
¹¹ Inte uppmätt.

## <a name="download"></a>Ladda ner

Ladda ner filen för ditt system från den
[senaste versionen](https://github.com/SunkwiBOT/twitch-miner/releases/latest).

| System | Paket |
|---|---|
| **Windows** · x64 | Installationsprogram (`setup.exe`) eller [portabel körbar fil](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/TwitchMiner-portable.exe) |
| **macOS** 10.15 eller senare · Intel och Apple Silicon | Skivavbild (`.dmg`) |
| **Linux** · x86-64 | `.AppImage`, `.deb` (Debian, Ubuntu) eller `.rpm` (Fedora, openSUSE) |

macOS-versionen är inte notariserad. Första gången högerklickar du på appen och väljer
**Öppna**.

Uppdateringarna är signerade. Appen söker efter en ny version när den startar och sedan
varje timme (hur ofta ändrar du i **Inställningar › Om**). Den installerar aldrig en
version på egen hand: du installerar den med ett klick, från meddelandet som visas eller
från **Inställningar › Om**.

Data lagras i användarens datamapp:

- Linux: `~/.local/share/twitch-miner`
- Windows: `%APPDATA%\twitch-miner`
- macOS: `~/Library/Application Support/twitch-miner`

## <a name="docker"></a>Docker

Servern visar samma översikt i en webbläsare. Avbilden är
`ghcr.io/sunkwibot/twitch-miner`, för `amd64` och `arm64`. När en ny
version finns säger översikten till: `docker compose pull`, sedan `docker compose up -d`.

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-miner:latest
    pull_policy: always   # hämta den senaste avbilden vid varje start
    restart: unless-stopped
    ports: ["8080:8080"]   # bakom en omvänd HTTPS-proxy: ta bort, eller "127.0.0.1:8080:8080"
    environment:
      MINER_PASSWORD: "change-me"   # krävs, minst 12 tecken
      # MINER_PUBLIC_URL: "https://miner.example.com"   # bara bakom en omvänd HTTPS-proxy
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

Öppna <http://localhost:8080> (eller datorns adress i ditt nätverk) och ange lösenordet.

### Variabler

| Variabel | Standard | Syfte |
|---|---|---|
| `MINER_PASSWORD` | inget | Lösenord för översikten, krävs: servern vägrar starta utan det. Minst 12 tecken (ett kortare ger en varning vid start). Exempelvärdet `change-me` nekas. |
| `MINER_PASSWORD_FILE` | inget | Läser `MINER_PASSWORD` från en fil, till exempel en Docker-secret. Den ena eller den andra, inte båda. |
| `MINER_PUBLIC_URL` | inget | Bara bakom en omvänd HTTPS-proxy (Caddy, nginx, Traefik, Cloudflare Tunnel…): översiktens `https://`-adress, utan sökväg, till exempel `https://miner.example.com`. Proxyn måste vidarebefordra `X-Forwarded-Proto`, `X-Forwarded-For` och websockets, vilket vanliga proxyer gör som standard, och containerns port får inte gå att nå direkt. Översikten nekar då allt som inte kommer via proxyn över HTTPS, och spärrar en adress i 30 sekunder efter 5 felaktiga lösenord. Proxyn måste köras på samma dator eller i dess lokala nätverk: en anslutning från en offentlig adress nekas. |
| `MINER_TRUSTED_HOPS` | `1` | Bara med `MINER_PUBLIC_URL`: hur många proxyer som lägger till en post i `X-Forwarded-For`, från 1 till 10. `2` med två proxyer efter varandra, till exempel Cloudflare framför Traefik. |
| `MINER_PROXY_NETWORKS` | inget | Bara med `MINER_PUBLIC_URL`: andra nätverk som proxyn får ansluta från, utöver den här datorn och dess privata lokala nätverk, åtskilda med kommatecken: en enda adress eller ett nätverk som `2001:db8:1::/64`, för ett lokalt nätverk med globala IPv6-adresser. |
| `MINER_VAULT_PASSPHRASE` | inget | Bara för en datamapp som kopierats från skrivbordsappen med ett applösenord satt: låser upp den vid start. En server som byggts från grunden behöver den aldrig. |
| `MINER_VAULT_PASSPHRASE_FILE` | inget | Läser `MINER_VAULT_PASSPHRASE` från en fil. Den ena eller den andra, inte båda. |
| `DATA_DIR` | `/data` i avbilden, annars `data` i arbetskatalogen | Databas, nyckelfil och loggar. |
| `LOG_DIR` | `$DATA_DIR/logs` | Loggfiler: en per dag, de senaste 14 sparas. |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | Lyssningsadress: en IP-adress och en port från 1 till 65535. |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | Loggnivå. |

Servern läser och kontrollerar variablerna en gång, vid start. Ett ogiltigt värde stoppar
servern med ett meddelande, i stället för att tyst falla tillbaka på ett standardvärde.

## <a name="how-it-works"></a>Så fungerar det

Var 20:e sekund väljer varje konto sina två kanaler: den bästa livekanalen för den översta
kampanjen i sin kö, sedan streamers på sin poänglista.

Twitch krediterar **en Drops-kampanj** och **två kanaler för poäng** åt gången per konto.
Sedan oktober 2026 kan en drop bara hämtas när spelkontot är länkat. Olänkade kampanjer
ger fortfarande framsteg, och översikten visar länken.

Kampanjlistan kommer från projektet [Twitch Drops API](https://drops.sunkwi.com)
(uppdateras var 10:e minut), eftersom Twitch döljer den för tv-klienten.

## <a name="faq"></a>Vanliga frågor

### Risker och Twitch

**Kan mitt konto bli avstängt?**\
Ingen kan lova att det inte händer. Det här projektet är inte kopplat till Twitch, och att
automatisera ett konto kan bryta mot Twitchs villkor eller en kampanjs regler. Minern
tittar på högst två kanaler per konto, Twitchs gräns, utan att ladda ner någon video. Det
är ditt beslut.

**Tittar den verkligen på videon?**\
Nej, ingen video laddas ner. För Drops läser minern var 15:e sekund sändningens spellista
i lägsta kvalitet och begär bara huvudet för varje nytt segment. För poäng skickar den
varje minut Twitchs signal ”minut tittad”. Varje kanal som tittas på visar hur många
segment som har begärts.

**Varför högst två kanaler?**\
Det är Twitchs gräns: per konto krediteras bara två kanaler som tittas på samtidigt, och
bara en Drops-kampanj åt gången. I läget **Drops och poäng** används den ena kanalen för
kampanjen och den andra för dina streamers. Inställningen ”En kanal”, i
**Kontoinställningar**, är mer diskret.

### Drops

**En kampanj minas inte, eller en drop gör inga framsteg.**\
Öppna kontots sida **Drops**. Under ”Miningordning” anger varje väntande kampanj varför:
”Ingen kanal live”, ”Väntar (en kampanj i taget)” eller ”Ingen drop tillgänglig än”. En
kampanj som inte syns i listan har sorterats bort: av inställningen ”Vilka kampanjer”, av
en regel på fliken **Undantag**, eller för att den inte hinner bli klar i tid. En belöning
som kräver en prenumeration minas aldrig, och en drop som följer efter en annan väntar
tills den första är klar. I läget **Endast poäng** väljer minern ingen kampanj.
”Mina först”, i en kampanjs detaljer, sätter den överst; en ignorerad kampanj kan komma
tillbaka med ”Mina igen”, på fliken **Undantag**.

**Måste jag länka spelkontot?**\
För att hämta, ja: sedan oktober 2026 låter Twitch dig bara hämta en drop när spelkontot
är länkat. Som standard ger en olänkad kampanj ändå framsteg, och hämtningen väntar på
länkningen. Märket ”Inte länkat” visar det, och ”Länka spelkontot”, i kampanjens
detaljer, öppnar länkningssidan. Vill du bara mina länkade spel väljer du
”Bara spel med länkat konto”.

### Installation och data

**Skrivbordsapp eller Docker?**\
Skrivbordsappen minar medan din dator är på, med ikon i systemfältet, valfritt lösenord och
uppdateringar med ett klick. Docker minar dygnet runt på en maskin som alltid är på (VPS,
NAS, Raspberry Pi); du öppnar översikten i en webbläsare, skyddad av `MINER_PASSWORD`.
Gränssnittet är detsamma.

**Var finns mina data?**\
I datamappen: `~/.local/share/twitch-miner` på Linux,
`%APPDATA%\twitch-miner` på Windows,
`~/Library/Application Support/twitch-miner` på macOS, och volymen `/data`
med Docker. På skrivbordet öppnar **Inställningar › Skrivbord** den med ett klick. Den
innehåller `miner.db` (konton, inställningar, journal), `master.key` (sessionsnyckeln,
när inget applösenord har angetts) och loggarna.

**Hur uppdaterar jag?**\
Skrivbordsappen söker efter en ny version när den startar och sedan varje timme (kan
ändras i **Inställningar › Om**). Klicka på märket ”Uppdatering” och sedan på
”Installera och starta om”. Med Docker säger översikten till: kör `docker compose pull`
och sedan `docker compose up -d`.

**Jag har glömt applösenordet.**\
Det går inte att återställa: det är det som krypterar dina Twitch-sessioner. Avsluta
appen, radera datamappen och lägg sedan till dina konton igen; deras inställningar måste
göras om. Med Docker ändrar du översiktens lösenord i `MINER_PASSWORD` och kör sedan
`docker compose up -d`.

### Poäng och förutsägelser

**Hur många poäng per timme?**\
Det beror på Twitch och på varje kanal. Minern tjänar det som Twitch ger en tittare: poäng
för tittartid, bonuskistor (ungefär en var 15:e minut) och tittarsvitens bonus. En
prenumeration multiplicerar poängen, från ×1,2 till ×2 beroende på nivå. Sidan Hem visar
intjänade poäng och sidan Journal varje enskild vinst.

**Är vadslagning riskabelt?**\
Ja, poäng kan gå förlorade. Förutsägelser är avstängda som standard, streamer för
streamer. Du ställer in strategin, när det ska satsas, insatsen (5 % av saldot som
standard, högst 50 000 poäng), ett minsta saldo och, om du vill, ett villkor, till exempel
odds på minst 1,5. Håll insatsen låg.

**En streamer tittas inte på.**\
Bara streamers som är påslagna och live tittas på, och bara på en ledig kanal: i läget
**Drops och poäng** används en av de två för den aktuella kampanjen; i läget
**Endast Drops** tittas ingen streamer på; ett pausat konto tittar inte på något.
Platserna fördelas i den här ordningen, som visas som ett märke på varje kanal som tittas
på: ”Svitbonus”, ”Svit i fara”, ”Prenumererad” och sist ”Listordning”. Flytta upp
streamern i listan, eller stäng av ”Tittarsviter först” och
”Prenumererade kanaler först”.

## <a name="related-projects"></a>Relaterade projekt

Andra projekt av samma upphovsperson:

- [**Twitch Drops API**](https://drops.sunkwi.com): ett gratis, offentligt API med alla
  pågående Twitch Drops- och belöningskampanjer, uppdaterat varje minut. Den här appen
  läser sin kampanjkatalog därifrån.
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com): en Discord-bot som annonserar
  livesändningar på Twitch och Drops-kampanjer på din server.
- [**Kick Alerts 🔔**](https://kick.sunkwi.com): en Discord-bot som annonserar
  livesändningar på Kick och Drops-kampanjer på din server.

## <a name="security"></a>Säkerhet

- Tokens förseglas med AES-256-GCM. Nyckeln är antingen en slumpmässig `master.key`
  bredvid databasen (standard, inget att göra) eller härleds från applösenordet med
  Argon2id.
- HTTP-översikten kontrollerar lösenordet i konstant tid och använder HttpOnly-,
  SameSite=Strict-kakor (`Secure`, med prefixet `__Host-`, bakom HTTPS). Servern sparar
  bara en SHA-256 av varje session. Den avslutar en session efter 30 dagar eller vid
  utloggning. Den kontrollerar sessionen innan den läser någon begärandekropp och
  begränsar kropparna till 256 KiB.
- Efter 5 felaktiga lösenord i rad från en IP-adress (inloggningsformulär eller bearer)
  får den adressen vänta 30 sekunder. Servern svarar `429` med `Retry-After` utan att
  titta på lösenordet. Varje felaktigt lösenord tar dessutom en halv sekund att besvara.
- Utanför ditt nätverk placerar du översikten bakom en omvänd HTTPS-proxy med
  `MINER_PUBLIC_URL` (se [Docker](#docker)).
- Minern pratar bara med `twitch.tv`, `ttvnw.net`, `jtvnw.net`, `twitchcdn.net`,
  `drops.sunkwi.com` och de aviseringsmål du konfigurerar. Appen (skrivbord och
  Docker) söker efter nya versioner på `github.com`, och skrivbordsappen laddar ner dem
  från projektets GitHub-releaser. De är signerade: bara den visade versionen installeras.

## <a name="build"></a>Bygga från källkod

**Förutsättningar:** Rust, Node.js och
[Tauris förutsättningar](https://v2.tauri.app/start/prerequisites/) för ditt system.

```bash
npm ci
npm run build          # översikten
cargo build --release -p miner-server   # server: target/release/twitch-miner
npm run bundle:linux   # skrivbordspaket (Tauris Linux-beroenden krävs)
```

Kodkontroller:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier, oxlint (typmedveten) och svelte-check
```

Gränssnittets TypeScript-typer genereras från Rust-typerna med
[ts-rs](https://github.com/Aleph-Alpha/ts-rs) till `ui/src/lib/bindings`. Efter en ändring
av en typ som gränssnittet ser kör du `npm run bindings`.

## <a name="license"></a>Licens

[MIT](../../LICENSE). Inspirerat av
[DevilXD:s TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner),
[Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
och deras forkar.
