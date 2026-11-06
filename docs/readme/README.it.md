<p align="center">
  <img src="../../ui/public/app-icon.svg" width="112" height="112" alt="Logo di Twitch Miner">
</p>

<h1 align="center">Twitch Miner</h1>

<p align="center">
  <strong>Mina i Drops di Twitch e i punti canale su più account, senza browser e senza schede di Twitch aperte.</strong>
</p>

<p align="center">
  Un'app desktop per Linux, Windows e macOS, oppure un'immagine Docker con la stessa dashboard web, da eseguire su un server domestico o su un Raspberry Pi. Con 4 account, il server usa circa 30 MB di memoria e quasi niente CPU.
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-miner?label=release&color=9146FF" alt="Ultima versione"></a>
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-miner/total?label=downloads&color=9146FF" alt="Download"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="Piattaforme: Linux, Windows, macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="Immagine Docker per amd64 e arm64"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="Licenza MIT"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="Visualizzazioni">
</p>

<p align="center">
  <a href="#download"><strong>Scarica</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-miner/issues/new/choose">Segnala un bug</a>
</p>

<p align="center">
  <a href="../../README.md">English</a> ·
  <a href="README.fr.md">Français</a> ·
  <a href="README.de.md">Deutsch</a> ·
  <a href="README.es-ES.md">Español&nbsp;(España)</a> ·
  <a href="README.es-419.md">Español&nbsp;(Latinoamérica)</a> ·
  <a href="README.pt-PT.md">Português&nbsp;(Portugal)</a> ·
  <a href="README.pt-BR.md">Português&nbsp;(Brasil)</a> ·
  <strong>Italiano</strong> ·
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
  <img src="../media/it/presentation.avif" alt="Presentazione video di Twitch Miner">
</p>

## <a name="features"></a>Funzionalità

Cosa lo distingue dagli altri miner. «Vedilo in azione», sotto un punto, lo mostra
all'opera.

### Account e streamer

- **Più account in un'unica app.** Ogni account mina per conto suo, nella sua modalità:
  **Drops e punti** (un canale per la campagna migliore, l'altro per i tuoi streamer),
  **Solo Drops** o **Solo punti**.

  <details open><summary>Vedilo in azione</summary>
  <img src="../media/it/accounts-mining.avif" alt="Più account che minano contemporaneamente, nella pagina Home">
  </details>

- **Una procedura guidata per ogni nuovo account.** Dopo l'accesso, parti dalle
  impostazioni predefinite o da quelle di un altro account. Scegli la modalità, le
  campagne e gli streamer, poi «Inizia il mining» o «Salva e avvia più tardi».

  <details open><summary>Vedilo in azione</summary>
  <img src="../media/it/account-setup.avif" alt="La procedura guidata di configurazione di un nuovo account, passo dopo passo">
  </details>

- **Una modifica applicata ad altri account con un clic.** Dopo aver cambiato qualcosa su
  un account, il messaggio di conferma propone «Applica ad altri account». La modifica
  viene ripetuta sugli account che spunti, che mantengono il resto delle loro impostazioni.

  <details open><summary>Vedilo in azione</summary>
  <img src="../media/it/apply-to-accounts.avif" alt="Una modifica ripetuta su altri account con «Applica ad altri account»">
  </details>

- **Aggiungere uno streamer.** Parte subito con le opzioni predefinite: quelle dell'app o
  quelle che hai scelto per questo account. Regolale al momento, se serve: forzieri bonus,
  raid, momenti, serie di visione, chat, previsioni.

  <details open><summary>Vedilo in azione</summary>
  <img src="../media/it/streamer-add.avif" alt="Uno streamer aggiunto con la ricerca di Twitch, poi le sue opzioni">
  </details>

- **Modificare uno streamer.** «Modifica» apre le sue opzioni: forzieri bonus, raid,
  momenti, serie di visione, chat, previsioni… Un interruttore lo attiva o lo disattiva
  senza toglierlo dalla lista.

  <details open><summary>Vedilo in azione</summary>
  <img src="../media/it/streamer-edit.avif" alt="Le opzioni di uno streamer, aperte dalla sua riga">
  </details>

- **Modificare più streamer insieme.** Spuntali, poi «Modifica i N streamer selezionati».
  Si applicano solo le opzioni che cambi. Un'opzione che differisce tra loro appare come
  «Misto», e il suo interruttore diventa due pulsanti, «Sì» e «No». L'app propone poi di
  farne le opzioni predefinite per i nuovi streamer.

  <details open><summary>Vedilo in azione</summary>
  <img src="../media/it/streamers-group-edit.avif" alt="Modifica di più streamer, con opzioni «Misto»">
  </details>

- **Importare streamer.** Incolla una lista (fino a 500 nomi), oppure riprendi quella di un
  altro account o i canali che l'account segue. La lista viene verificata per aggiungere
  solo canali che esistono.

  <details open><summary>Vedilo in azione</summary>
  <img src="../media/it/streamers-import.avif" alt="Importazione di streamer: una lista incollata, un altro account, i canali seguiti">
  </details>

- **Spam Points.** Riscatta più volte di fila un premio personalizzato di un canale.
  Scegli un canale dalla tua lista Punti, o qualsiasi canale trovato con la ricerca di
  Twitch: i suoi premi compaiono con il saldo dell'account, letto in tempo reale. Imposta
  la quantità (fino a 25, nei limiti del saldo), poi «Riscatta». Il risultato compare alla
  fine della raffica, e ogni raffica ha la sua riga nel Registro.

  <details open><summary>Vedilo in azione</summary>
  <img src="../media/it/spam-points.avif" alt="Spam Points: un canale scelto, 10 × un premio con messaggio riscattato, i riscatti arrivano nella chat Twitch del canale, poi la riga del Registro">
  </details>

### Drops

- **Tutte le campagne Drops in corso.** Ogni campagna ha i suoi dettagli: ricompense,
  collegamento dell'account di gioco, «Mina per prima». L'elenco arriva dal progetto
  [Twitch Drops API](https://drops.sunkwi.com) e si aggiorna ogni 10 minuti.

  <details open><summary>Vedilo in azione</summary>
  <img src="../media/it/drops-catalogue.avif" alt="Il Catalogo in vista calendario, la ricerca e i dettagli di una campagna">
  </details>

- **Il tuo ordine di mining.** Trascina le campagne nell'ordine che preferisci. Le nuove
  trovano posto da sole: prima i tuoi giochi prioritari, poi quelle che finiscono prima o
  a cui mancano meno minuti. Escludi giochi, parole, una campagna o una singola
  ricompensa, e salta le campagne che non possono finire in tempo. «Mina per prima», nei
  dettagli di una campagna, la porta in cima.

  <details open><summary>Vedilo in azione</summary>
  <img src="../media/it/drops-order.avif" alt="L'ordine di mining, i giochi prioritari e le esclusioni">
  </details>

- **Inventario e «Riscatta tutto».** La pagina Inventario divide le ricompense in
  «Da riscattare», «In corso» e «Riscattate». Riscattale una alla volta, tutte insieme,
  oppure attiva «Riscatta i drop automaticamente».

  <details open><summary>Vedilo in azione</summary>
  <img src="../media/it/drops-inventory.avif" alt="L'inventario e il pulsante «Riscatta tutto»">
  </details>

- **«Prima i miei streamer per i Drops».** Per la campagna in corso, uno streamer della
  tua lista la cui live vale per quella campagna passa davanti agli altri canali. Lo
  stesso canale fa avanzare il drop e ti fa guadagnare i suoi punti canale. Attivo per
  impostazione predefinita.

  <details open><summary>Vedilo in azione</summary>
  <img src="../media/it/drops-streamers-first.avif" alt="Uno streamer della lista, guardato per i Drops e per i suoi punti">
  </details>

### Monitoraggio

- **Un registro di tutto, e ogni richiesta.** Il Registro conserva ogni evento: drop, punti
  guadagnati e spesi, forzieri, scommesse, raid, live che iniziano… Filtralo per account,
  per scheda (Drops, Punti, Altro) e per tipo. **Log tecnici › Richieste** elenca ogni
  richiesta inviata a Twitch e ad altri servizi, con il suo esito e la sua durata. Log e
  richieste non contengono alcun token né segreto: «Copia per una segnalazione di bug» li
  copia, pronti da incollare in una segnalazione di bug su
  [GitHub](https://github.com/SunkwiBOT/twitch-miner/issues) o
  [Discord](https://discord.gg/support-bots-891685895781113886).

  <details open><summary>Vedilo in azione</summary>
  <img src="../media/it/journal-requests.avif" alt="Il Registro filtrato, poi Log tecnici › Richieste">
  </details>

### Sul desktop o su un server

- **La stessa interfaccia ovunque.** L'app desktop (Linux, Windows, macOS) e l'immagine
  Docker mostrano la stessa dashboard. Docker gira su un VPS, un NAS o un Raspberry Pi,
  senza schermo né browser su quella macchina: la dashboard si apre in qualsiasi browser.
  Sul telefono la dashboard si adatta allo schermo e, in HTTPS, si installa come un'app.
- **Leggero.** Circa 30 MB di memoria per 4 account, quasi niente CPU e un'immagine Docker
  di circa 12 MB.

### Sicurezza e aggiornamenti

- **Le tue sessioni, protette.** Nessuna password di Twitch: accedi su
  `twitch.tv/activate`, dal tuo browser. Le sessioni sono cifrate su disco e puoi
  revocarle nelle tue [impostazioni di Twitch](https://www.twitch.tv/settings/connections).
  Sul desktop, una password dell'app facoltativa la blocca: non si mina nulla finché non
  viene inserita. Con Docker, la dashboard ha la sua password, si blocca da sola dopo
  qualche minuto di inattività e passa in HTTPS dietro un proxy. Cinque password errate di
  fila bloccano l'indirizzo IP per 30 secondi.

  <details open><summary>Vedilo in azione</summary>
  <img src="../media/it/security-lock.avif" alt="Il lucchetto: l'app bloccata, poi sbloccata con la sua password">
  </details>

- **Aggiornamenti firmati, mai automatici.** Nella barra in alto compare un badge
  «Aggiornamento» con il numero di versione. «Installa e riavvia» chiede prima una
  conferma, poi l'app installa la versione e si riavvia. Con una password dell'app, si
  riavvia bloccata: reinseriscila per riprendere il mining. Viene installata solo una
  versione firmata dal progetto. Anche la versione Docker avvisa, con i comandi da
  eseguire.

  <details open><summary>Vedilo in azione</summary>
  <img src="../media/it/updates.avif" alt="Il badge «Aggiornamento», la conferma, poi l'installazione">
  </details>

### E inoltre

- Come altri miner: forzieri bonus, raid, momenti, obiettivi della community, previsioni
  (strategie, momento della scommessa, condizioni), priorità alle serie di visione e ai
  canali abbonati, presenza in chat (disattivabile), opzioni predefinite per i nuovi
  streamer, notifiche Discord, Telegram e webhook con immagini, proxy globale o per
  account.
- **Presto: il mining dei badge.** I badge di Twitch che si ottengono guardando, minati come i Drops.
- 29 lingue, temi scuro e chiaro, notifiche desktop, icona nell'area di notifica, avvio con
  il sistema.

  <details open><summary>Vedilo in azione</summary>
  <img src="../media/it/basics.avif" alt="Lingue, tema chiaro e scuro, una notifica desktop e l'icona nell'area di notifica">
  </details>

> [!WARNING]
> Questo progetto non è affiliato a Twitch. Automatizzare un account può violare i termini
> di Twitch o le regole di una campagna.

## Indice

- [Confronto](#comparison)
- [Download](#download)
- [Docker](#docker)
- [Come funziona](#how-it-works)
- [Domande frequenti](#faq)
- [Progetti correlati](#related-projects)
- [Sicurezza](#security)
- [Compilare dai sorgenti](#build)
- [Licenza](#license)

## <a name="comparison"></a>Confronto

Come si posiziona [Twitch Miner](https://github.com/SunkwiBOT/twitch-miner)
rispetto a [Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
(punti canale) e [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner) (Drops).
Verificato sul loro codice e sui loro README a novembre 2026.

| | Questo progetto | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Campagne Drops: le trova, le mina e le riscatta | ✅ | ❌ ¹ | ✅ |
| Punti canale: forzieri, raid, momenti, previsioni | ✅ | ✅ | ❌ |
| Spam Points: un premio personalizzato di un canale riscattato più volte di fila | ✅ | ❌ | ❌ |
| Drops e punti canale contemporaneamente su un solo account | ✅ | in parte ¹ | ❌ |
| Più account in un'unica app | ✅ | ❌ ² | ❌ |
| Modificare le impostazioni di più account in una volta | ✅ | ❌ | ❌ |
| Interfaccia grafica | ✅ desktop e web ³ | ❌ ⁴ | ✅ desktop |
| Dashboard web per un server domestico (Docker), con password e raggiungibile in HTTPS | ✅ | ❌ ⁴ | ❌ |
| Sessioni cifrate su disco, password dell'app facoltativa | ✅ | ❌ | ❌ |
| Interfaccia in 29 lingue | ✅ | ❌ | in parte ⁵ |
| Aggiornamenti firmati direttamente dall'app (desktop) | ✅ | ❌ | ❌ |
| Procedura guidata di configurazione per ogni nuovo account | ✅ | ❌ ⁴ | ❌ |
| Streamer aggiunti e modificati nell'interfaccia, uno alla volta o più insieme | ✅ | ❌ ⁴ | — |
| Streamer importati tramite ricerca, lista, altro account o canali seguiti | ✅ | in parte ⁶ | — |
| Notifiche Discord, Telegram e webhook con immagini | ✅ | in parte ⁷ | ❌ |
| Calendario di tutte le campagne Drops, con ricerca | ✅ | ❌ | in parte ⁸ |
| Ordine di mining per campagna, parole escluse, campagna o ricompensa ignorata | ✅ | ❌ | ❌ ⁹ |
| Gli streamer della tua lista guardati per primi per i Drops di una campagna | ✅ | in parte ¹⁰ | ❌ |
| Registro degli eventi filtrabile ed elenco di ogni richiesta, nell'interfaccia | ✅ | ❌ ⁴ | ❌ |
| Circa 30 MB di memoria (server Docker, 4 account) | ✅ | — ¹¹ | — ¹¹ |

¹ Guarda solo gli streamer della sua lista: fa avanzare i Drops dei loro stream e li
riscatta, senza scegliere una campagna.\
² Un processo per account.\
³ Sul tuo PC come app desktop, e/o su un server (VPS, NAS, Raspberry Pi) con Docker, senza
ambiente grafico né browser su quella macchina: la dashboard si apre in qualsiasi browser.\
⁴ Si configura in un file Python; la sua pagina web mostra grafici dei punti e il file di
log.\
⁵ 21 lingue.\
⁶ Può seguire i canali che l'account segue.\
⁷ Solo testo, senza immagini. Ha anche Matrix, Pushover e Gotify, che questo progetto non
ha.\
⁸ Un elenco di campagne con filtri, senza calendario né ricerca.\
⁹ Ha giochi prioritari e giochi esclusi, scelti per nome del gioco.\
¹⁰ Per impostazione predefinita, guarda prima gli streamer della sua lista la cui live ha
i Drops (la sua priorità `DROPS`), ma mai un altro canale.\
¹¹ Non misurato.

## <a name="download"></a>Download

Scarica il file per il tuo sistema
dall'[ultima versione](https://github.com/SunkwiBOT/twitch-miner/releases/latest).

| Sistema | Pacchetto |
|---|---|
| **Windows** · x64 | Programma di installazione (`setup.exe`) o [eseguibile portatile](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/TwitchMiner-portable.exe) |
| **macOS** 10.15 o successivo · Intel e Apple Silicon | Immagine disco (`.dmg`) |
| **Linux** · x86-64 | `.AppImage`, `.deb` (Debian, Ubuntu) o `.rpm` (Fedora, openSUSE) |

La build per macOS non è notarizzata. La prima volta, fai clic destro sull'app e scegli
**Apri**.

Gli aggiornamenti sono firmati. L'app controlla se c'è una nuova versione all'avvio, poi
ogni ora (puoi cambiare la frequenza in **Impostazioni › Informazioni**). Non ne installa
mai una da sola: la installi tu con un clic, dal messaggio che compare o da
**Impostazioni › Informazioni**.

I dati si trovano nella cartella dei dati dell'utente:

- Linux: `~/.local/share/twitch-miner`
- Windows: `%APPDATA%\twitch-miner`
- macOS: `~/Library/Application Support/twitch-miner`

## <a name="docker"></a>Docker

Il server mostra la stessa dashboard in un browser. L'immagine è
`ghcr.io/sunkwibot/twitch-miner`, per `amd64` e `arm64`. Quando esce una
nuova versione, la dashboard lo segnala: `docker compose pull`, poi
`docker compose up -d`.

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-miner:latest
    pull_policy: always   # scarica l'immagine più recente a ogni avvio
    restart: unless-stopped
    ports: ["8080:8080"]   # dietro un reverse proxy HTTPS: da togliere, o "127.0.0.1:8080:8080"
    environment:
      MINER_PASSWORD: "change-me"   # obbligatoria, 12 caratteri o più
      # MINER_PUBLIC_URL: "https://miner.example.com"   # solo dietro un reverse proxy HTTPS
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

Apri <http://localhost:8080> (o l'indirizzo della macchina sulla tua rete) e inserisci la
password.

### Variabili

| Variabile | Predefinito | Scopo |
|---|---|---|
| `MINER_PASSWORD` | nessuno | Password della dashboard, obbligatoria: senza, il server si rifiuta di avviarsi. Di 12 caratteri o più (una più corta mostra un avviso all'avvio). Il valore di esempio `change-me` viene rifiutato. |
| `MINER_PASSWORD_FILE` | nessuno | Legge `MINER_PASSWORD` da un file, per esempio un secret Docker. L'una o l'altra, non entrambe. |
| `MINER_PUBLIC_URL` | nessuno | Solo dietro un reverse proxy HTTPS (Caddy, nginx, Traefik, Cloudflare Tunnel…): l'indirizzo `https://` della dashboard, senza percorso, come `https://miner.example.com`. Il proxy deve inoltrare `X-Forwarded-Proto`, `X-Forwarded-For` e i websocket, cosa che i proxy più diffusi fanno già di default, e la porta del container non deve essere raggiungibile direttamente. La dashboard rifiuta quindi tutto ciò che non passa dal proxy in HTTPS, e blocca un indirizzo per 30 secondi dopo 5 password errate. Il proxy deve girare sulla stessa macchina o sulla sua rete locale: una connessione da un indirizzo pubblico viene rifiutata. |
| `MINER_TRUSTED_HOPS` | `1` | Solo con `MINER_PUBLIC_URL`: quanti proxy aggiungono una voce a `X-Forwarded-For`, da 1 a 10. `2` con due proxy uno dietro l'altro, come Cloudflare davanti a Traefik. |
| `MINER_PROXY_NETWORKS` | nessuno | Solo con `MINER_PUBLIC_URL`: altre reti da cui il proxy può collegarsi, oltre a questa macchina e alla sua rete locale privata, separate da virgole: un singolo indirizzo o una rete come `2001:db8:1::/64`, per una rete locale con indirizzi IPv6 globali. |
| `MINER_VAULT_PASSPHRASE` | nessuno | Solo per una cartella dei dati copiata dall'app desktop con una password dell'app: la sblocca all'avvio. Un server creato da zero non ne ha mai bisogno. |
| `MINER_VAULT_PASSPHRASE_FILE` | nessuno | Legge `MINER_VAULT_PASSPHRASE` da un file. L'una o l'altra, non entrambe. |
| `DATA_DIR` | `/data` nell'immagine, altrimenti `data` nella directory di lavoro | Database, file della chiave e log. |
| `LOG_DIR` | `$DATA_DIR/logs` | File di log: uno al giorno, vengono conservati gli ultimi 14. |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | Indirizzo di ascolto: un indirizzo IP e una porta da 1 a 65535. |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | Livello di log. |

Il server legge e controlla le variabili una sola volta, all'avvio. Un valore non valido
arresta il server con un messaggio, invece di ripiegare in silenzio su un valore di
riserva.

## <a name="how-it-works"></a>Come funziona

Ogni 20 secondi, ogni account sceglie i suoi due canali: il miglior canale in diretta per
la prima campagna della sua coda, poi gli streamer della sua lista dei punti.

Twitch accredita **una campagna Drops** e **due canali per i punti** alla volta per
account. Da ottobre 2026, un drop si può riscattare solo se l'account di gioco è
collegato. Le campagne non collegate avanzano comunque, e la dashboard mostra il link per
collegarlo.

L'elenco delle campagne arriva dal progetto [Twitch Drops API](https://drops.sunkwi.com)
(aggiornato ogni 10 minuti), perché Twitch lo nasconde al client TV.

## <a name="faq"></a>Domande frequenti

### Rischi e Twitch

**Il mio account può essere bannato?**\
Nessuno può garantire di no. Questo progetto non è affiliato a Twitch, e automatizzare un
account può violarne i termini o le regole di una campagna. Il miner guarda al massimo due
canali per account, il limite di Twitch, senza scaricare alcun video. Sta a te decidere.

**Il video viene davvero guardato?**\
No, nessun video viene scaricato. Per i Drops, ogni 15 secondi il miner legge la playlist
della live alla qualità più bassa e chiede solo l'header di ogni nuovo segmento. Per i
punti, invia ogni minuto il segnale «minuto guardato» di Twitch. Ogni canale guardato
mostra quanti segmenti sono stati richiesti.

**Perché al massimo due canali?**\
È il limite di Twitch: accredita solo due canali guardati contemporaneamente per account,
e una sola campagna Drops alla volta. In modalità **Drops e punti**, un canale serve la
campagna e l'altro i tuoi streamer. L'impostazione «Un canale», in
**Impostazioni account**, è più discreta.

### Drops

**Una campagna non viene minata, o un drop non avanza.**\
Apri la pagina **Drops** dell'account. In «Ordine di mining», ogni campagna in attesa
spiega perché: «Nessun canale in diretta», «In attesa (una campagna alla volta)» o
«Nessun drop ancora disponibile». Una campagna che non compare in quell'elenco è stata
scartata: dall'impostazione «Quali campagne», da una regola della scheda **Esclusioni**,
oppure perché non può finire in tempo. Una ricompensa che richiede un abbonamento non
viene mai minata, e un drop che ne segue un altro aspetta che il primo sia completato. In
modalità **Solo punti**, il miner non sceglie alcuna campagna. «Mina per prima», nei
dettagli di una campagna, la porta in cima; una campagna ignorata può tornare con
«Mina di nuovo», nella scheda **Esclusioni**.

**Devo collegare l'account di gioco?**\
Per riscattare, sì: da ottobre 2026 Twitch permette di riscattare un drop solo se
l'account di gioco è collegato. Per impostazione predefinita, una campagna non collegata
avanza comunque e il riscatto aspetta il collegamento. Il badge «Non collegato» lo
segnala, e «Collega l'account di gioco», nei dettagli della campagna, apre la pagina di
collegamento. Per minare solo i giochi collegati, scegli
«Solo giochi con l'account collegato».

### Installazione e dati

**App desktop o Docker?**\
L'app desktop mina finché il PC è acceso, con la sua icona nell'area di notifica, una
password facoltativa e gli aggiornamenti con un clic. Docker mina giorno e notte su una
macchina sempre accesa (VPS, NAS, Raspberry Pi); la apri in un browser, protetta da
`MINER_PASSWORD`. L'interfaccia è la stessa.

**Dove sono i miei dati?**\
Nella cartella dei dati: `~/.local/share/twitch-miner` su Linux,
`%APPDATA%\twitch-miner` su Windows,
`~/Library/Application Support/twitch-miner` su macOS, e il volume `/data`
con Docker. Sul desktop, **Impostazioni › Desktop** la apre con un clic. Contiene
`miner.db` (account, impostazioni, registro), `master.key` (la chiave delle sessioni,
quando non è impostata alcuna password dell'app) e i log.

**Come si aggiorna?**\
L'app desktop controlla se c'è una nuova versione all'avvio, poi ogni ora (si può cambiare
in **Impostazioni › Informazioni**). Fai clic sul badge «Aggiornamento», poi su
«Installa e riavvia». Con Docker, la dashboard ti avvisa: esegui `docker compose pull`,
poi `docker compose up -d`.

**Ho dimenticato la password dell'app.**\
Non si può recuperare: è proprio lei a cifrare le tue sessioni Twitch. Chiudi l'app,
elimina la cartella dei dati, poi aggiungi di nuovo i tuoi account; le loro impostazioni
vanno rifatte. Con Docker, la password della dashboard si cambia in `MINER_PASSWORD`, poi
`docker compose up -d`.

### Punti e previsioni

**Quanti punti all'ora?**\
Dipende da Twitch e da ogni canale. Il miner guadagna quello che Twitch dà a uno
spettatore: i punti per il tempo di visione, i forzieri bonus (circa uno ogni 15 minuti) e
il bonus della serie di visione. Un abbonamento moltiplica i punti, da ×1,2 a ×2 secondo
il livello. La pagina Home mostra i punti guadagnati, e il Registro ogni guadagno.

**Le scommesse sono rischiose?**\
Sì, si possono perdere punti. Le previsioni sono disattivate per impostazione
predefinita, streamer per streamer. Scegli tu la strategia, quando scommettere, la
puntata (5% del saldo per impostazione predefinita, al massimo 50.000 punti), un saldo
minimo e, se vuoi, una condizione, come una quota di almeno 1,5. Tieni bassa la puntata.

**Uno streamer non viene guardato.**\
Vengono guardati solo gli streamer attivi e in diretta, e solo su un canale libero: in
modalità **Drops e punti**, uno dei due serve la campagna in corso; in modalità
**Solo Drops**, nessuno streamer viene guardato; un account in pausa non guarda niente. I
posti seguono quest'ordine, mostrato come badge su ogni canale guardato: «Bonus serie»,
«Serie a rischio», «Abbonato», poi «Ordine della lista». Sposta lo streamer più in alto
nella lista, oppure disattiva «Prima le serie di visione» e «Prima i canali abbonati».

## <a name="related-projects"></a>Progetti correlati

Altri progetti dello stesso autore:

- [**Twitch Drops API**](https://drops.sunkwi.com): un'API pubblica e gratuita con tutte
  le campagne Drops e di ricompense di Twitch in corso, aggiornata ogni minuto. Questa app
  ne legge il catalogo delle campagne.
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com): un bot Discord che annuncia nel tuo
  server le live su Twitch e le campagne Drops.
- [**Kick Alerts 🔔**](https://kick.sunkwi.com): un bot Discord che annuncia nel tuo
  server le live su Kick e le campagne Drops.

## <a name="security"></a>Sicurezza

- I token sono sigillati con AES-256-GCM. La chiave è un file `master.key` casuale accanto
  al database (predefinito, niente da fare) oppure è derivata dalla password dell'app con
  Argon2id.
- La dashboard HTTP controlla la password in tempo costante e usa cookie HttpOnly e
  SameSite=Strict (`Secure`, con il prefisso `__Host-`, dietro HTTPS). Il server
  conserva solo uno SHA-256 di ogni sessione. Chiude una sessione dopo 30 giorni o al
  logout. Controlla la sessione prima di leggere qualsiasi corpo della richiesta e limita
  i corpi a 256 KiB.
- Dopo 5 password errate di fila dallo stesso indirizzo IP (modulo di accesso o bearer),
  quell'indirizzo deve attendere 30 secondi. Il server risponde `429` con `Retry-After`
  senza guardare la password. Inoltre ogni password errata impiega mezzo secondo a
  ricevere risposta.
- Fuori dalla tua rete, metti la dashboard dietro un reverse proxy HTTPS con
  `MINER_PUBLIC_URL` (vedi [Docker](#docker)).
- Il miner comunica solo con `twitch.tv`, `ttvnw.net`, `jtvnw.net`, `twitchcdn.net`,
  `drops.sunkwi.com` e con le destinazioni delle notifiche che configuri. L'app
  (desktop e Docker) controlla le nuove versioni su `github.com`, e l'app desktop le
  scarica dalle release GitHub del progetto. Sono firmate: viene installata solo la
  versione mostrata.

## <a name="build"></a>Compilare dai sorgenti

**Prerequisiti:** Rust, Node.js e i
[prerequisiti di Tauri](https://v2.tauri.app/start/prerequisites/) per il tuo sistema.

```bash
npm ci
npm run build          # dashboard
cargo build --release -p miner-server   # server: target/release/twitch-miner
npm run bundle:linux   # pacchetti desktop (richiede le dipendenze Linux di Tauri)
```

Controlli del codice:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier, oxlint (con i tipi) e svelte-check
```

I tipi TypeScript dell'interfaccia sono generati dai tipi Rust con
[ts-rs](https://github.com/Aleph-Alpha/ts-rs) in `ui/src/lib/bindings`. Dopo aver
modificato un tipo visibile all'interfaccia, esegui `npm run bindings`.

## <a name="license"></a>Licenza

[MIT](../../LICENSE). Ispirato a
[TwitchDropsMiner di DevilXD](https://github.com/DevilXD/TwitchDropsMiner), a
[Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
e ai loro fork.
