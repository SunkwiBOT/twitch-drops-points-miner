<p align="center">
  <img src="../../ui/public/app-icon.svg" width="112" height="112" alt="Logo Twitch Drops &amp; Points Miner">
</p>

<h1 align="center">Twitch Drops &amp; Points Miner</h1>

<p align="center">
  <strong>Těží Twitch Drops a body kanálu na více účtech, bez prohlížeče a bez otevřené karty Twitche.</strong>
</p>

<p align="center">
  Desktopová aplikace pro Linux, Windows a macOS, nebo obraz pro Docker se stejným webovým panelem, který spustíte na domácím serveru nebo na Raspberry Pi. Se 4 účty zabere server asi 30 MB paměti a procesor téměř nezatěžuje.
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-drops-points-miner?label=release&color=9146FF" alt="Nejnovější vydání"></a>
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-drops-points-miner/total?label=downloads&color=9146FF" alt="Stažení"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="Platformy: Linux, Windows, macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-drops-points-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="Obraz pro Docker pro amd64 a arm64"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="Licence MIT"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="Zobrazení">
</p>

<p align="center">
  <a href="#download"><strong>Stáhnout</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/issues/new/choose">Nahlásit chybu</a>
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
  <strong>Čeština</strong> ·
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
  <img src="../media/cs/presentation.avif" alt="Videoukázka aplikace Twitch Drops &amp; Points Miner">
</p>

## <a name="features"></a>Funkce

Čím se liší od ostatních minerů. „Ukázka v akci“ pod každým bodem předvede, jak funguje.

### Účty a streameři

- **Více účtů v jedné aplikaci.** Každý účet těží samostatně, ve vlastním režimu:
  **Drops a body** (jeden kanál pro nejlepší kampaň, druhý pro vaše streamery),
  **Jen Drops** nebo **Jen body**.

  <details open><summary>Ukázka v akci</summary>
  <img src="../media/cs/accounts-mining.avif" alt="Několik účtů těží současně, na stránce Domů">
  </details>

- **Průvodce pro každý nový účet.** Po přihlášení začněte z výchozího nastavení, nebo
  z nastavení jiného účtu. Zvolte režim, kampaně a streamery a pak „Začít těžit“ nebo
  „Uložit a spustit později“.

  <details open><summary>Ukázka v akci</summary>
  <img src="../media/cs/account-setup.avif" alt="Průvodce nastavením nového účtu, krok za krokem">
  </details>

- **Jedna změna přenesená na další účty jedním kliknutím.** Když na účtu něco změníte,
  potvrzovací zpráva nabídne „Použít na další účty“. Stejná změna se provede na zaškrtnutých
  účtech a zbytek jejich nastavení zůstane beze změny.

  <details open><summary>Ukázka v akci</summary>
  <img src="../media/cs/apply-to-accounts.avif" alt="Změna zopakovaná na dalších účtech přes „Použít na další účty“">
  </details>

- **Přidání streamera.** Hned se rozběhne s výchozími možnostmi: s těmi z aplikace, nebo
  s těmi, které jste zvolili pro tento účet. V případě potřeby je rovnou upravte: bonusové
  truhly, raidy, momenty, série sledování, chat, předpovědi.

  <details open><summary>Ukázka v akci</summary>
  <img src="../media/cs/streamer-add.avif" alt="Přidání streamera přes vyhledávání na Twitchi a pak jeho možnosti">
  </details>

- **Úprava streamera.** „Upravit“ otevře jeho možnosti: bonusové truhly, raidy, momenty,
  série sledování, chat, předpovědi… Přepínač ho zapne nebo vypne, aniž by ho odebral ze
  seznamu.

  <details open><summary>Ukázka v akci</summary>
  <img src="../media/cs/streamer-edit.avif" alt="Možnosti streamera otevřené z jeho řádku">
  </details>

- **Úprava více streamerů najednou.** Zaškrtněte je a pak „Upravit N vybraných streamerů“.
  Použijí se jen možnosti, které změníte. Možnost, která se mezi nimi liší, ukazuje
  „Smíšené“ a její přepínač se změní na dvě tlačítka, „Ano“ a „Ne“. Aplikace pak nabídne,
  že z nich udělá výchozí možnosti pro nové streamery.

  <details open><summary>Ukázka v akci</summary>
  <img src="../media/cs/streamers-group-edit.avif" alt="Úprava několika streamerů s možnostmi „Smíšené“">
  </details>

- **Import streamerů.** Vložte seznam (až 500 jmen), nebo převezměte seznam jiného účtu či
  kanály, které účet sleduje. Seznam se zkontroluje, takže se přidají jen existující kanály.

  <details open><summary>Ukázka v akci</summary>
  <img src="../media/cs/streamers-import.avif" alt="Import streamerů: vložený seznam, jiný účet, sledované kanály">
  </details>

- **Spam Points.** Uplatněte vlastní odměnu kanálu několikrát za sebou. Vyberte kanál ze
  svého seznamu Bodů, nebo jakýkoli kanál nalezený vyhledáváním na Twitchi: jeho odměny se
  zobrazí spolu se zůstatkem účtu, který se průběžně načítá. Nastavte počet (až 25, v rámci
  zůstatku) a pak „Uplatnit“. Výsledek se zobrazí na konci série a každá série má v Deníku
  svůj řádek.

  <details open><summary>Ukázka v akci</summary>
  <img src="../media/cs/spam-points.avif" alt="Spam Points: vybraný kanál, 10 × uplatněná odměna se zprávou, uplatnění se objevují v chatu kanálu na Twitchi, pak řádek v Deníku">
  </details>

### Drops

- **Všechny probíhající kampaně Drops.** Každá kampaň má vlastní kartu s podrobnostmi:
  odměny, propojení herního účtu, „Těžit přednostně“. Seznam pochází z projektu
  [Twitch Drops API](https://drops.sunkwi.com) a obnovuje se každých 10 minut.

  <details open><summary>Ukázka v akci</summary>
  <img src="../media/cs/drops-catalogue.avif" alt="Katalog jako kalendář, vyhledávání a podrobnosti kampaně">
  </details>

- **Vaše pořadí těžby.** Přetáhněte kampaně v požadovaném pořadí. Nové si své místo najdou
  samy: nejdřív vaše prioritní hry, pak nejbližší konec nebo nejméně zbývajících minut.
  Vylučte hry, slova, kampaň nebo jedinou odměnu a přeskakujte kampaně, které nejde
  dokončit včas. „Těžit přednostně“ v podrobnostech kampaně ji přesune na začátek.

  <details open><summary>Ukázka v akci</summary>
  <img src="../media/cs/drops-order.avif" alt="Pořadí těžby, prioritní hry a vyloučení">
  </details>

- **Inventář a „Vyzvednout vše“.** Stránka Inventář řadí odměny do skupin „K vyzvednutí“,
  „Probíhá“ a „Vyzvednuto“. Vyzvedávejte je jednu po druhé, všechny najednou, nebo zapněte
  „Vyzvedávat drops automaticky“.

  <details open><summary>Ukázka v akci</summary>
  <img src="../media/cs/drops-inventory.avif" alt="Inventář a tlačítko „Vyzvednout vše“">
  </details>

- **„Moji streameři první pro Drops“.** Pro aktuální kampaň má streamer z vašeho seznamu,
  jehož vysílání se do této kampaně počítá, přednost před ostatními kanály. Tentýž kanál
  posouvá drop dopředu a přináší vám své body kanálu. Ve výchozím stavu zapnuto.

  <details open><summary>Ukázka v akci</summary>
  <img src="../media/cs/drops-streamers-first.avif" alt="Streamer ze seznamu sledovaný kvůli Drops i kvůli svým bodům">
  </details>

### Záznamy

- **Deník všeho a každý požadavek.** Deník uchovává každou událost: dropy, získané
  a utracené body, truhly, sázky, raidy, začátky vysílání… Filtrujte ho podle účtu, karty
  (Drops, Body, Ostatní) a typu. **Technické protokoly › Požadavky** vypíše každý požadavek
  odeslaný na Twitch a další služby, s jeho výsledkem a dobou trvání. Protokoly ani
  požadavky neobsahují žádný token ani tajný údaj: „Kopírovat pro hlášení chyby“ je
  zkopíruje, připravené k vložení do hlášení chyby na
  [GitHubu](https://github.com/SunkwiBOT/twitch-drops-points-miner/issues) nebo
  [Discordu](https://discord.gg/support-bots-891685895781113886).

  <details open><summary>Ukázka v akci</summary>
  <img src="../media/cs/journal-requests.avif" alt="Filtrovaný Deník, pak Technické protokoly › Požadavky">
  </details>

### Na počítači nebo na serveru

- **Všude stejné rozhraní.** Desktopová aplikace (Linux, Windows, macOS) i obraz pro Docker
  zobrazují stejný panel. Docker běží na VPS, NAS nebo Raspberry Pi, bez obrazovky
  a prohlížeče na tom stroji: panel se otevře v jakémkoli prohlížeči.
- **Nenáročný.** Asi 30 MB paměti pro 4 účty, téměř žádná zátěž procesoru a obraz pro
  Docker o velikosti asi 12 MB.

### Zabezpečení a aktualizace

- **Vaše relace v bezpečí.** Žádné heslo k Twitchi: přihlásíte se na `twitch.tv/activate`
  ve svém prohlížeči. Relace jsou na disku šifrované a můžete je odvolat
  v [nastavení Twitche](https://www.twitch.tv/settings/connections). Na počítači aplikaci
  zamyká volitelné heslo aplikace: dokud ho nezadáte, nic se netěží. V Dockeru má panel
  vlastní heslo, po několika minutách nečinnosti se sám zamkne a za proxy běží přes HTTPS.
  Pět špatných hesel po sobě zablokuje IP adresu na 30 sekund.

  <details open><summary>Ukázka v akci</summary>
  <img src="../media/cs/security-lock.avif" alt="Zámek: aplikace zamčená a pak odemčená heslem">
  </details>

- **Podepsané aktualizace, nikdy automatické.** V horní liště se objeví odznak
  „Aktualizace“ s číslem verze. „Nainstalovat a restartovat“ si nejdřív vyžádá potvrzení,
  pak aplikace verzi nainstaluje a restartuje se. S heslem aplikace se restartuje zamčená:
  zadejte ho znovu a těžba bude pokračovat. Nainstaluje se jen verze podepsaná projektem.
  Verze pro Docker vám to také oznámí, i s příkazy, které je třeba spustit.

  <details open><summary>Ukázka v akci</summary>
  <img src="../media/cs/updates.avif" alt="Odznak „Aktualizace“, potvrzení a pak instalace">
  </details>

### A také

- Stejně jako jiné minery: bonusové truhly, raidy, momenty, komunitní cíle, předpovědi
  (strategie, kdy vsadit, podmínky), přednost sériím sledování a předplaceným kanálům,
  přítomnost v chatu (lze vypnout), výchozí možnosti pro nové streamery, oznámení přes
  Discord, Telegram a webhook s obrázky, globální proxy nebo proxy pro každý účet.
- 29 jazyků, tmavý a světlý motiv, oznámení na ploše, ikona v liště, spouštění se systémem.

  <details open><summary>Ukázka v akci</summary>
  <img src="../media/cs/basics.avif" alt="Jazyky, světlý a tmavý motiv, oznámení na ploše a ikona v liště">
  </details>

> [!WARNING]
> Tento projekt není spojen s Twitchem. Automatizace účtu může porušovat podmínky Twitche
> nebo pravidla kampaně.

## Obsah

- [Srovnání](#comparison)
- [Stažení](#download)
- [Docker](#docker)
- [Jak to funguje](#how-it-works)
- [Časté otázky](#faq)
- [Související projekty](#related-projects)
- [Zabezpečení](#security)
- [Sestavení ze zdrojového kódu](#build)
- [Licence](#license)

## <a name="comparison"></a>Srovnání

Jak si [Twitch Drops & Points Miner](https://github.com/SunkwiBOT/twitch-drops-points-miner)
stojí vedle [Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
(body kanálu) a [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner) (Drops).
Ověřeno v jejich kódu a souborech README v listopadu 2026.

| | Tento projekt | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Kampaně Drops: najde je, těží a vyzvedává | ✅ | ❌ ¹ | ✅ |
| Body kanálu: truhly, raidy, momenty, předpovědi | ✅ | ✅ | ❌ |
| Spam Points: vlastní odměna kanálu uplatněná několikrát za sebou | ✅ | ❌ | ❌ |
| Drops a body kanálu současně na jednom účtu | ✅ | částečně ¹ | ❌ |
| Více účtů v jedné aplikaci | ✅ | ❌ ² | ❌ |
| Změna nastavení více účtů najednou | ✅ | ❌ | ❌ |
| Grafické rozhraní | ✅ desktopové i webové ³ | ❌ ⁴ | ✅ desktopové |
| Webový panel pro domácí server (Docker), chráněný heslem, dostupný přes HTTPS | ✅ | ❌ ⁴ | ❌ |
| Relace šifrované na disku, volitelné heslo aplikace | ✅ | ❌ | ❌ |
| Rozhraní ve 29 jazycích | ✅ | ❌ | částečně ⁵ |
| Podepsané aktualizace přímo z aplikace (desktop) | ✅ | ❌ | ❌ |
| Průvodce nastavením pro každý nový účet | ✅ | ❌ ⁴ | ❌ |
| Streameři přidávaní a upravovaní v rozhraní, po jednom nebo několik najednou | ✅ | ❌ ⁴ | — |
| Import streamerů vyhledáním, seznamem, z jiného účtu nebo ze sledovaných kanálů | ✅ | částečně ⁶ | — |
| Oznámení přes Discord, Telegram a webhook s obrázky | ✅ | částečně ⁷ | ❌ |
| Kalendář všech kampaní Drops s vyhledáváním | ✅ | ❌ | částečně ⁸ |
| Pořadí těžby podle kampaní, vyloučená slova, ignorovaná kampaň nebo odměna | ✅ | ❌ | ❌ ⁹ |
| Streameři z vašeho seznamu sledovaní přednostně pro Drops kampaně | ✅ | částečně ¹⁰ | ❌ |
| Filtrovatelný deník událostí a seznam všech požadavků v rozhraní | ✅ | ❌ ⁴ | ❌ |
| Asi 30 MB paměti (server Docker, 4 účty) | ✅ | — ¹¹ | — ¹¹ |

¹ Sleduje jen streamery ze svého seznamu: posouvá a vyzvedává Drops z jejich streamů, ale
kampaň nevybírá.\
² Jeden proces na účet.\
³ Na vašem PC jako desktopová aplikace a/nebo na serveru (VPS, NAS, Raspberry Pi)
s Dockerem, bez grafického prostředí a bez prohlížeče na tomto stroji: panel se otevře
v jakémkoli prohlížeči.\
⁴ Nastavuje se v souboru Pythonu; jeho webová stránka ukazuje grafy bodů a soubor
s protokolem.\
⁵ 21 jazyků.\
⁶ Umí sledovat kanály, které sleduje účet.\
⁷ Jen text, bez obrázků. Má také Matrix, Pushover a Gotify, které tento projekt nemá.\
⁸ Seznam kampaní s filtry, bez kalendáře a vyhledávání.\
⁹ Má prioritní a vyloučené hry, vybírané podle názvu hry.\
¹⁰ Ve výchozím stavu nejdřív sleduje streamery ze svého seznamu, jejichž stream nese Drops
(jeho priorita `DROPS`), ale nikdy jiný kanál.\
¹¹ Neměřeno.

## <a name="download"></a>Stažení

Stáhněte si soubor pro svůj systém z
[nejnovějšího vydání](https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest).

| Systém | Balíček |
|---|---|
| **Windows** · x64 | Instalátor (`setup.exe`) nebo [přenosná verze](https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest/download/TwitchDropsPointsMiner-portable.exe) |
| **macOS** 10.15 nebo novější · Intel a Apple Silicon | Obraz disku (`.dmg`) |
| **Linux** · x86-64 | `.AppImage`, `.deb` (Debian, Ubuntu) nebo `.rpm` (Fedora, openSUSE) |

Sestavení pro macOS není notarizované. Poprvé klikněte na aplikaci pravým tlačítkem myši
a zvolte **Otevřít**.

Aktualizace jsou podepsané. Aplikace zkontroluje novou verzi při spuštění a pak každou
hodinu (jak často, změníte v **Nastavení › O aplikaci**). Sama ji nikdy nenainstaluje:
nainstalujete ji jedním kliknutím, ze zobrazené zprávy nebo z **Nastavení › O aplikaci**.

Data jsou uložena ve složce s uživatelskými daty:

- Linux: `~/.local/share/twitch-drops-points-miner`
- Windows: `%APPDATA%\twitch-drops-points-miner`
- macOS: `~/Library/Application Support/twitch-drops-points-miner`

## <a name="docker"></a>Docker

Server zobrazuje stejný panel v prohlížeči. Obraz je
`ghcr.io/sunkwibot/twitch-drops-points-miner`, pro `amd64` a `arm64`. Když vyjde nová
verze, panel to oznámí: `docker compose pull`, pak `docker compose up -d`.

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-drops-points-miner:latest
    pull_policy: always   # při každém startu stáhnout nejnovější obraz
    restart: unless-stopped
    ports: ["8080:8080"]   # za reverzní proxy HTTPS: odstraňte, nebo "127.0.0.1:8080:8080"
    environment:
      MINER_PASSWORD: "change-me"   # povinné, alespoň 12 znaků
      # MINER_PUBLIC_URL: "https://miner.example.com"   # jen za reverzní proxy HTTPS
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

Otevřete <http://localhost:8080> (nebo adresu stroje ve vaší síti) a zadejte heslo.

### Proměnné

| Proměnná | Výchozí | Účel |
|---|---|---|
| `MINER_PASSWORD` | žádná | Heslo panelu, povinné: bez něj se server odmítne spustit. Alespoň 12 znaků (kratší heslo zobrazí při startu varování). |
| `MINER_PASSWORD_FILE` | žádná | Načte `MINER_PASSWORD` ze souboru, například z Docker secretu. Nastavte jen jednu z nich, ne obě. |
| `MINER_PUBLIC_URL` | žádná | Jen za reverzní proxy HTTPS (Caddy, nginx, Traefik, Cloudflare Tunnel…): adresa `https://` panelu, bez cesty, například `https://miner.example.com`. Proxy musí předávat `X-Forwarded-Proto`, `X-Forwarded-For` a websockety, což běžné proxy ve výchozím stavu dělají, a port kontejneru nesmí být přímo dostupný. Panel pak odmítne vše, co nepřichází přes proxy po HTTPS, a po 5 špatných heslech zablokuje adresu na 30 sekund. |
| `MINER_TRUSTED_HOPS` | `1` | Jen s `MINER_PUBLIC_URL`: kolik proxy přidává záznam do `X-Forwarded-For`, od 1 do 10. `2` se dvěma proxy za sebou, například Cloudflare před Traefikem. |
| `MINER_VAULT_PASSPHRASE` | žádná | Jen pro složku s daty zkopírovanou z desktopové aplikace s nastaveným heslem aplikace: při startu ji odemkne. Server postavený od nuly ji nikdy nepotřebuje. |
| `MINER_VAULT_PASSPHRASE_FILE` | žádná | Načte `MINER_VAULT_PASSPHRASE` ze souboru. Nastavte jen jednu z nich, ne obě. |
| `DATA_DIR` | `/data` v obrazu, jinak `data` v pracovním adresáři | Databáze, soubor s klíčem a protokoly. |
| `LOG_DIR` | `$DATA_DIR/logs` | Soubory protokolů: jeden denně, uchovává se posledních 14. |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | Adresa pro naslouchání: IP adresa a port od 1 do 65535. |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | Úroveň protokolování. |

Server načte a zkontroluje proměnné jednou, při startu. Neplatná hodnota server zastaví se
zprávou, místo aby ji potichu nahradil náhradní hodnotou.

## <a name="how-it-works"></a>Jak to funguje

Každých 20 sekund si každý účet vybere své dva kanály: nejlepší živý kanál pro první
kampaň ve své frontě, pak streamery ze svého seznamu pro body.

Twitch na jeden účet současně započítává **jednu kampaň Drops** a **dva kanály pro body**.
Od října 2026 lze drop vyzvednout, jen když je herní účet propojený. Nepropojené kampaně
přesto postupují a panel zobrazuje odkaz pro propojení.

Seznam kampaní pochází z projektu [Twitch Drops API](https://drops.sunkwi.com) (obnovuje
se každých 10 minut), protože ho Twitch před klientem pro TV skrývá.

## <a name="faq"></a>Časté otázky

### Rizika a Twitch

**Může být můj účet zablokován?**\
Nikdo nemůže slíbit, že ne. Tento projekt není spojen s Twitchem a automatizace účtu může
porušovat jeho podmínky nebo pravidla kampaně. Miner sleduje nejvýš dva kanály na účet,
což je limit Twitche, a nestahuje žádné video. Rozhodnutí je na vás.

**Sleduje se video doopravdy?**\
Ne, žádné video se nestahuje. Pro Drops miner každých 15 sekund načte playlist streamu
v nejnižší kvalitě a vyžádá si jen hlavičku každého nového segmentu. Pro body posílá každou
minutu signál Twitche „sledovaná minuta“. U každého sledovaného kanálu je vidět, kolik
segmentů si vyžádal.

**Proč nejvýš dva kanály?**\
To je limit Twitche: na účet započítává jen dva současně sledované kanály a jen jednu
kampaň Drops najednou. V režimu **Drops a body** jeden kanál slouží kampani a druhý vašim
streamerům. Nastavení „Jeden kanál“ v **Nastavení účtu** je nenápadnější.

### Drops

**Kampaň se netěží nebo drop nepostupuje.**\
Otevřete stránku **Drops** daného účtu. V části „Pořadí těžby“ každá čekající kampaň uvádí
důvod: „Žádný kanál nevysílá“, „Čeká (jedna kampaň najednou)“ nebo
„Zatím žádný drop k dispozici“. Kampaň, která se v tomto seznamu neobjevuje, byla
vynechána: nastavením „Které kampaně“, pravidlem na kartě **Vyloučení**, nebo proto, že ji
nejde dokončit včas. Odměna, která vyžaduje předplatné, se nikdy netěží, a drop, který
navazuje na jiný, čeká, dokud není ten první hotový. V režimu **Jen body** miner nevybírá
žádnou kampaň. „Těžit přednostně“ v podrobnostech kampaně ji přesune na začátek;
ignorovaná kampaň se může vrátit přes „Znovu těžit“ na kartě **Vyloučení**.

**Musím propojit herní účet?**\
Kvůli vyzvednutí ano: od října 2026 Twitch dovolí vyzvednout drop, jen když je herní účet
propojený. Ve výchozím stavu nepropojená kampaň přesto postupuje a vyzvednutí počká na
propojení. Upozorní na to odznak „Nepropojeno“ a „Propojit herní účet“ v podrobnostech
kampaně otevře stránku pro propojení. Chcete-li těžit jen propojené hry, zvolte
„Jen hry s propojeným účtem“.

### Instalace a data

**Desktopová aplikace, nebo Docker?**\
Desktopová aplikace těží, dokud je váš počítač zapnutý, s ikonou v liště, volitelným
heslem a aktualizacemi na jedno kliknutí. Docker těží ve dne v noci na stroji, který je
stále zapnutý (VPS, NAS, Raspberry Pi); otevíráte ho v prohlížeči a chrání ho
`MINER_PASSWORD`. Rozhraní je stejné.

**Kde jsou moje data?**\
Ve složce s daty: `~/.local/share/twitch-drops-points-miner` na Linuxu,
`%APPDATA%\twitch-drops-points-miner` ve Windows,
`~/Library/Application Support/twitch-drops-points-miner` na macOS a svazek `/data`
v Dockeru. Na počítači ji jedním kliknutím otevře **Nastavení › Plocha**. Obsahuje
`miner.db` (účty, nastavení, deník), `master.key` (klíč k relacím, pokud není nastavené
heslo aplikace) a protokoly.

**Jak aktualizovat?**\
Desktopová aplikace kontroluje novou verzi při spuštění a pak každou hodinu (lze změnit
v **Nastavení › O aplikaci**). Klikněte na odznak „Aktualizace“ a pak na
„Nainstalovat a restartovat“. U Dockeru vám to oznámí panel: spusťte `docker compose pull`
a pak `docker compose up -d`.

**Nepamatuji si heslo aplikace.**\
Nelze ho obnovit: právě ono šifruje vaše relace Twitch. Ukončete aplikaci, smažte složku
s daty a pak účty přidejte znovu; jejich nastavení bude potřeba udělat znovu. U Dockeru se
heslo panelu mění v `MINER_PASSWORD` a pak se spustí `docker compose up -d`.

### Body a předpovědi

**Kolik bodů za hodinu?**\
Záleží na Twitchi a na každém kanálu. Miner získá to, co Twitch dává divákovi: body za
sledování, bonusové truhly (zhruba jednu každých 15 minut) a bonus za sérii sledování.
Předplatné body násobí, od ×1,2 do ×2 podle úrovně. Stránka Domů ukazuje získané body
a Deník každý zisk.

**Jsou sázky riskantní?**\
Ano, body se dají prohrát. Předpovědi jsou ve výchozím stavu vypnuté, u každého streamera
zvlášť. Nastavíte strategii, kdy vsadit, sázku (ve výchozím stavu 5 % zůstatku, nejvýš
50 000 bodů), minimální zůstatek a případně podmínku, například kurz alespoň 1,5. Držte
sázku nízkou.

**Streamer se nesleduje.**\
Sledují se jen zapnutí streameři, kteří právě vysílají, a jen na volném kanálu: v režimu
**Drops a body** jeden ze dvou slouží aktuální kampani; v režimu **Jen Drops** se žádný
streamer nesleduje; pozastavený účet nesleduje nic. Místa se přidělují v tomto pořadí,
zobrazeném jako odznak u každého sledovaného kanálu: „Bonus za sérii“, „Série ohrožena“,
„Předplaceno“, pak „Pořadí seznamu“. Posuňte streamera v seznamu výš, nebo vypněte
„Série sledování první“ a „Předplacené kanály první“.

## <a name="related-projects"></a>Související projekty

Další projekty stejného autora:

- [**Twitch Drops API**](https://drops.sunkwi.com): bezplatné veřejné API všech probíhajících
  kampaní Twitch Drops a kampaní s odměnami, aktualizované každou minutu. Tato aplikace
  z něj načítá svůj katalog kampaní.
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com): bot pro Discord, který na vašem serveru
  oznamuje živá vysílání na Twitchi a kampaně Drops.
- [**Kick Alerts 🔔**](https://kick.sunkwi.com): bot pro Discord, který na vašem serveru
  oznamuje živá vysílání na Kicku a kampaně Drops.

## <a name="security"></a>Zabezpečení

- Tokeny jsou zapečetěné pomocí AES-256-GCM. Klíčem je buď náhodný `master.key` vedle
  databáze (výchozí, bez zásahu uživatele), nebo klíč odvozený z hesla aplikace pomocí
  Argon2id.
- Panel HTTP ověřuje heslo v konstantním čase a používá cookies HttpOnly, SameSite=Strict
  (`Secure`, s předponou `__Host-`, za HTTPS). Server uchovává z každé relace
  jen otisk SHA-256. Relaci ukončí po 30 dnech nebo při odhlášení. Relaci kontroluje dřív,
  než přečte tělo požadavku, a těla omezuje na 256 KiB.
- Po 5 špatných heslech po sobě z jedné IP adresy (přihlašovací formulář nebo bearer)
  musí tato adresa 30 sekund čekat. Server odpoví `429` s `Retry-After`, aniž by se na
  heslo podíval. Každé špatné heslo navíc dostane odpověď až po půl sekundě.
- Mimo svou síť umístěte panel za reverzní proxy HTTPS s `MINER_PUBLIC_URL`
  (viz [Docker](#docker)).
- Miner komunikuje jen s `twitch.tv`, `ttvnw.net`, `jtvnw.net`, `twitchcdn.net`,
  `twitch-drops-api.sunkwi.com` a s cíli oznámení, které nastavíte. Aplikace (desktopová
  i Docker) kontroluje nové verze na `github.com` a desktopová aplikace je stahuje
  z vydání projektu na GitHubu. Jsou podepsané: nainstaluje se jen zobrazená verze.

## <a name="build"></a>Sestavení ze zdrojového kódu

**Předpoklady:** Rust, Node.js a
[předpoklady pro Tauri](https://v2.tauri.app/start/prerequisites/) pro váš systém.

```bash
npm ci
npm run build          # panel
cargo build --release -p miner-server   # server: target/release/twitch-miner
npm run bundle:linux   # desktopové balíčky (vyžadují linuxové závislosti Tauri)
```

Kontroly kódu:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier, oxlint (s ohledem na typy) a svelte-check
```

Typy TypeScriptu pro rozhraní se generují z typů Rustu pomocí
[ts-rs](https://github.com/Aleph-Alpha/ts-rs) do `ui/src/lib/bindings`. Po změně typu,
který rozhraní vidí, spusťte `npm run bindings`.

## <a name="license"></a>Licence

[MIT](../../LICENSE). Inspirováno projekty [TwitchDropsMiner od DevilXD](https://github.com/DevilXD/TwitchDropsMiner),
[Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
a jejich forky.
