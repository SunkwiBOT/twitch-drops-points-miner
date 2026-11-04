<p align="center">
  <img src="ui/public/app-icon.svg" width="112" height="112" alt="Twitch Drops &amp; Points Miner logo">
</p>

<h1 align="center">Twitch Drops &amp; Points Miner</h1>

<p align="center">
  <strong>Mines Twitch Drops and channel points on several accounts, with no browser and no Twitch tab open.</strong>
</p>

<p align="center">
  A desktop app for Linux, Windows and macOS, or a Docker image with the same web dashboard, to run on a home server or on a Raspberry Pi. With 4 accounts, the server uses about 30 MB of memory and almost no CPU.
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-drops-points-miner?label=release&color=9146FF" alt="Latest release"></a>
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-drops-points-miner/total?label=downloads&color=9146FF" alt="Downloads"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="Platforms: Linux, Windows, macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-drops-points-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="Docker image for amd64 and arm64"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="MIT license"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="Views">
</p>

<p align="center">
  <a href="#download"><strong>Download</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/issues/new/choose">Report a bug</a>
</p>

<p align="center">
  <strong>English</strong> ·
  <a href="docs/readme/README.fr.md">Français</a> ·
  <a href="docs/readme/README.de.md">Deutsch</a> ·
  <a href="docs/readme/README.es-ES.md">Español&nbsp;(España)</a> ·
  <a href="docs/readme/README.es-419.md">Español&nbsp;(Latinoamérica)</a> ·
  <a href="docs/readme/README.pt-PT.md">Português&nbsp;(Portugal)</a> ·
  <a href="docs/readme/README.pt-BR.md">Português&nbsp;(Brasil)</a> ·
  <a href="docs/readme/README.it.md">Italiano</a> ·
  <a href="docs/readme/README.nl.md">Nederlands</a> ·
  <a href="docs/readme/README.pl.md">Polski</a> ·
  <a href="docs/readme/README.ru.md">Русский</a> ·
  <a href="docs/readme/README.uk.md">Українська</a> ·
  <a href="docs/readme/README.cs.md">Čeština</a> ·
  <a href="docs/readme/README.hu.md">Magyar</a> ·
  <a href="docs/readme/README.ro.md">Română</a> ·
  <a href="docs/readme/README.bg.md">Български</a> ·
  <a href="docs/readme/README.el.md">Ελληνικά</a> ·
  <a href="docs/readme/README.tr.md">Türkçe</a> ·
  <a href="docs/readme/README.sv.md">Svenska</a> ·
  <a href="docs/readme/README.da.md">Dansk</a> ·
  <a href="docs/readme/README.nb.md">Norsk&nbsp;bokmål</a> ·
  <a href="docs/readme/README.fi.md">Suomi</a> ·
  <a href="docs/readme/README.zh-CN.md">简体中文</a> ·
  <a href="docs/readme/README.zh-TW.md">繁體中文</a> ·
  <a href="docs/readme/README.ja.md">日本語</a> ·
  <a href="docs/readme/README.ko.md">한국어</a> ·
  <a href="docs/readme/README.th.md">ไทย</a> ·
  <a href="docs/readme/README.vi.md">Tiếng&nbsp;Việt</a> ·
  <a href="docs/readme/README.id.md">Bahasa&nbsp;Indonesia</a>
</p>

<p align="center">
  <img src="docs/media/en/presentation.avif" alt="Video overview of Twitch Drops &amp; Points Miner">
</p>

## <a name="features"></a>Features

What sets it apart from other miners. “See it in action” under a point shows it at work.

### Accounts and streamers

- **Several accounts in one app.** Each account mines on its own, in its own mode:
  **Drops and points** (one channel for the best campaign, the other for your streamers),
  **Drops only** or **Points only**.

  <details open><summary>See it in action</summary>
  <img src="docs/media/en/accounts-mining.avif" alt="Several accounts mining at the same time, on the Home page">
  </details>

- **A wizard for each new account.** After signing in, start from the default settings or
  from another account's. Choose the mode, the campaigns and the streamers, then
  “Start mining” or “Save and start later”.

  <details open><summary>See it in action</summary>
  <img src="docs/media/en/account-setup.avif" alt="The setup wizard of a new account, step by step">
  </details>

- **One change applied to other accounts in one click.** After you change something on an
  account, the confirmation message offers “Apply to other accounts”. The change is made
  again on the accounts you tick, and they keep the rest of their settings.

  <details open><summary>See it in action</summary>
  <img src="docs/media/en/apply-to-accounts.avif" alt="A change made again on other accounts with “Apply to other accounts”">
  </details>

- **Add a streamer.** It starts right away with the default options: the app's, or the
  ones you chose for this account. Adjust them on the spot if needed: bonus chests, raids,
  moments, watch streak, chat, predictions.

  <details open><summary>See it in action</summary>
  <img src="docs/media/en/streamer-add.avif" alt="Adding a streamer through the Twitch search, then its options">
  </details>

- **Edit a streamer.** “Edit” opens its options: bonus chests, raids, moments, watch
  streak, chat, predictions… A switch turns it on or off without removing it from the
  list.

  <details open><summary>See it in action</summary>
  <img src="docs/media/en/streamer-edit.avif" alt="A streamer's options, opened from its row">
  </details>

- **Edit several streamers at once.** Tick them, then “Edit the N selected streamers”.
  Only the options you change are applied. An option that differs between them shows
  “Mixed”, and its switch becomes two buttons, “Yes” and “No”. The app then offers to make
  them the default options for new streamers.

  <details open><summary>See it in action</summary>
  <img src="docs/media/en/streamers-group-edit.avif" alt="Editing several streamers, with “Mixed” options">
  </details>

- **Import streamers.** Paste a list (up to 500 names), or reuse another account's list or
  the channels the account follows. The list is checked so that only channels that exist
  are added.

  <details open><summary>See it in action</summary>
  <img src="docs/media/en/streamers-import.avif" alt="Importing streamers: a pasted list, another account, followed channels">
  </details>

- **Spam Points.** Redeem a channel's custom reward several times in a row. Pick a channel
  from your Points list, or any channel found with the Twitch search: its rewards show
  with the account's balance, read live. Set the quantity (up to 25, within the balance),
  then “Redeem”. The result shows when the burst ends, and each burst has its line in the
  Journal.

  <details open><summary>See it in action</summary>
  <img src="docs/media/en/spam-points.avif" alt="Spam Points: a channel picked, 5 × a reward redeemed, then the Journal line">
  </details>

### Drops

- **Every running Drops campaign.** Each campaign has its own details: rewards, game
  account linking, “Mine first”. The list comes from the
  [Twitch Drops API](https://drops.sunkwi.com) project and refreshes every 10 minutes.

  <details open><summary>See it in action</summary>
  <img src="docs/media/en/drops-catalogue.avif" alt="The Catalogue as a calendar, the search and a campaign's details">
  </details>

- **Your mining order.** Drag the campaigns into the order you want. New ones find their
  own place: your priority games first, then the soonest end or the fewest minutes left.
  Exclude games, words, a campaign or a single reward, and skip the campaigns that cannot
  finish in time. “Mine first”, in a campaign's details, puts it at the top.

  <details open><summary>See it in action</summary>
  <img src="docs/media/en/drops-order.avif" alt="The mining order, the priority games and the exclusions">
  </details>

- **Inventory and “Claim all”.** The Inventory page sorts rewards into “To claim”,
  “In progress” and “Claimed”. Claim them one by one, all at once, or turn on
  “Claim drops automatically”.

  <details open><summary>See it in action</summary>
  <img src="docs/media/en/drops-inventory.avif" alt="The inventory and the “Claim all” button">
  </details>

- **“My streamers first for Drops”.** For the current campaign, a streamer from your list
  whose stream counts for that campaign comes before other channels. The same channel
  moves the drop forward and earns you its channel points. On by default.

  <details open><summary>See it in action</summary>
  <img src="docs/media/en/drops-streamers-first.avif" alt="A streamer from the list, watched for the Drops and for its points">
  </details>

### Tracking

- **A journal of everything, and every request.** The Journal keeps every event: drops,
  points earned and spent, chests, bets, raids, streams going live… Filter it by account,
  by tab (Drops, Points, Other) and by type. **Technical logs › Requests** lists every
  request sent to Twitch and to other services, with its result and how long it took.
  Logs and requests contain no token or secret: “Copy for a bug report” copies them, ready
  to paste into a bug report on
  [GitHub](https://github.com/SunkwiBOT/twitch-drops-points-miner/issues) or
  [Discord](https://discord.gg/support-bots-891685895781113886).

  <details open><summary>See it in action</summary>
  <img src="docs/media/en/journal-requests.avif" alt="The filtered Journal, then Technical logs › Requests">
  </details>

### On the desktop or on a server

- **The same interface everywhere.** The desktop app (Linux, Windows, macOS) and the
  Docker image show the same dashboard. Docker runs on a VPS, a NAS or a Raspberry Pi,
  with no screen or browser on that machine: the dashboard opens in any browser.
- **Light.** About 30 MB of memory for 4 accounts, almost no CPU, and a Docker image of
  about 12 MB.

### Security and updates

- **Your sessions, protected.** No Twitch password: you sign in on `twitch.tv/activate`,
  from your browser. Sessions are encrypted on disk, and you can revoke them in your
  [Twitch settings](https://www.twitch.tv/settings/connections). On the desktop, an
  optional app password locks the app: nothing is mined until it is typed. With Docker,
  the dashboard has its own password, locks itself after a few minutes without activity
  and runs over HTTPS behind a proxy. Five wrong passwords in a row block the IP address
  for 30 seconds.

  <details open><summary>See it in action</summary>
  <img src="docs/media/en/security-lock.avif" alt="The padlock: the app locked, then unlocked with its password">
  </details>

- **Signed updates, never automatic.** An “Update” badge with the version number appears
  in the top bar. “Install and restart” first asks you to confirm, then the app installs
  the version and restarts. With an app password, it restarts locked: type it again to
  resume mining. Only a version signed by the project is installed. The Docker version
  tells you too, with the commands to run.

  <details open><summary>See it in action</summary>
  <img src="docs/media/en/updates.avif" alt="The “Update” badge, the confirmation, then the install">
  </details>

### And also

- Like other miners: bonus chests, raids, moments, community goals, predictions
  (strategies, when to bet, conditions), watch streaks and subscribed channels first,
  chat presence (can be turned off), default options for new streamers, Discord, Telegram
  and webhook notifications with images, a global or per-account proxy.
- 29 languages, dark and light themes, desktop notifications, a tray icon, start with the
  system.

  <details open><summary>See it in action</summary>
  <img src="docs/media/en/basics.avif" alt="Languages, light and dark themes, a desktop notification and the tray icon">
  </details>

> [!WARNING]
> This project is not affiliated with Twitch. Automating an account may break Twitch's
> terms or a campaign's rules.

## Contents

- [Comparison](#comparison)
- [Download](#download)
- [Docker](#docker)
- [How it works](#how-it-works)
- [FAQ](#faq)
- [Related projects](#related-projects)
- [Security](#security)
- [Build from source](#build)
- [Licence](#license)

## <a name="comparison"></a>Comparison

How [Twitch Drops & Points Miner](https://github.com/SunkwiBOT/twitch-drops-points-miner)
compares with [Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
(channel points) and [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner) (Drops).
Checked against their code and READMEs in November 2026.

| | This project | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Drops campaigns: finds, mines and claims them | ✅ | ❌ ¹ | ✅ |
| Channel points: chests, raids, moments, predictions | ✅ | ✅ | ❌ |
| Drops and channel points at the same time on one account | ✅ | partly ¹ | ❌ |
| Several accounts in one app | ✅ | ❌ ² | ❌ |
| Change the settings of several accounts at once | ✅ | ❌ | ❌ |
| Graphical interface | ✅ desktop and web ³ | ❌ ⁴ | ✅ desktop |
| Web dashboard for a home server (Docker), with a password, reachable over HTTPS | ✅ | ❌ ⁴ | ❌ |
| Sessions encrypted on disk, optional app password | ✅ | ❌ | ❌ |
| Interface in 29 languages | ✅ | ❌ | partly ⁵ |
| Signed updates from inside the app (desktop) | ✅ | ❌ | ❌ |
| Setup wizard for each new account | ✅ | ❌ ⁴ | ❌ |
| Streamers added and edited in the interface, one at a time or several at once | ✅ | ❌ ⁴ | — |
| Streamers imported by search, a list, another account or followed channels | ✅ | partly ⁶ | — |
| Discord, Telegram and webhook notifications with images | ✅ | partly ⁷ | ❌ |
| Calendar of every Drops campaign, with search | ✅ | ❌ | partly ⁸ |
| Mining order by campaign, excluded words, ignored campaign or reward | ✅ | ❌ | ❌ ⁹ |
| Your listed streamers watched first for a campaign's Drops | ✅ | partly ¹⁰ | ❌ |
| Filterable event journal and a list of every request, in the interface | ✅ | ❌ ⁴ | ❌ |
| About 30 MB of memory (Docker server, 4 accounts) | ✅ | — ¹¹ | — ¹¹ |

¹ It only watches the streamers on its list: it makes progress on the Drops of their
streams and claims them, without choosing a campaign.\
² One process per account.\
³ On your PC as a desktop app, and/or on a server (VPS, NAS, Raspberry Pi) with Docker,
with no graphical environment and no browser on that machine: the dashboard opens in any
browser.\
⁴ Configured in a Python file; its web page shows points charts and the log file.\
⁵ 21 languages.\
⁶ It can follow the channels the account follows.\
⁷ Text only, without images. It also has Matrix, Pushover and Gotify, which this project
does not.\
⁸ A list of campaigns with filters, without a calendar or search.\
⁹ It has priority games and excluded games, chosen by game name.\
¹⁰ By default, it first watches the streamers on its list whose stream carries Drops (its
`DROPS` priority), but never another channel.\
¹¹ Not measured.

## <a name="download"></a>Download

Download the file for your system from the
[latest release](https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest).

| System | Package |
|---|---|
| **Windows** · x64 | Installer (`setup.exe`) or [portable executable](https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest/download/TwitchDropsPointsMiner-portable.exe) |
| **macOS** 10.15 or later · Intel and Apple Silicon | Disk image (`.dmg`) |
| **Linux** · x86-64 | `.AppImage`, `.deb` (Debian, Ubuntu) or `.rpm` (Fedora, openSUSE) |

The macOS build is not notarized. The first time, right-click the app and choose
**Open**.

Updates are signed. The app checks for a new version when it starts, then every hour
(you can change how often in **Settings › About**). It never installs one by itself: you
install it in one click, from the message that appears or from **Settings › About**.

Data lives in the user data folder:

- Linux: `~/.local/share/twitch-drops-points-miner`
- Windows: `%APPDATA%\twitch-drops-points-miner`
- macOS: `~/Library/Application Support/twitch-drops-points-miner`

## <a name="docker"></a>Docker

The server shows the same dashboard in a browser. The image is
`ghcr.io/sunkwibot/twitch-drops-points-miner`, for `amd64` and `arm64`. When a new
version is out, the dashboard says so: `docker compose pull`, then
`docker compose up -d`.

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-drops-points-miner:latest
    pull_policy: always   # take the newest image at every start
    restart: unless-stopped
    ports: ["8080:8080"]   # behind an HTTPS reverse proxy: remove, or "127.0.0.1:8080:8080"
    environment:
      MINER_PASSWORD: "change-me"   # required, 12 characters or more
      # MINER_PUBLIC_URL: "https://miner.example.com"   # only behind an HTTPS reverse proxy
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

Open <http://localhost:8080> (or the machine's address on your network) and enter the
password.

### Variables

| Variable | Default | Purpose |
|---|---|---|
| `MINER_PASSWORD` | none | Dashboard password, required: the server refuses to start without it. 12 characters or more (a shorter one shows a warning at start). |
| `MINER_PASSWORD_FILE` | none | Reads `MINER_PASSWORD` from a file, such as a Docker secret. One or the other, not both. |
| `MINER_PUBLIC_URL` | none | Only behind an HTTPS reverse proxy (Caddy, nginx, Traefik, Cloudflare Tunnel…): the dashboard's `https://` address, without a path, like `https://miner.example.com`. The proxy must forward `X-Forwarded-Proto`, `X-Forwarded-For` and websockets, which common proxies do by default, and the container's port must not be reachable directly. The dashboard then refuses anything that does not come through the proxy over HTTPS, and locks out an address for 30 seconds after 5 wrong passwords. |
| `MINER_TRUSTED_HOPS` | `1` | With `MINER_PUBLIC_URL` only: how many proxies add an entry to `X-Forwarded-For`, from 1 to 10. `2` with two proxies one behind the other, like Cloudflare in front of Traefik. |
| `MINER_VAULT_PASSPHRASE` | none | Only for a data folder copied from the desktop app with an app password: unlocks it at start. A server set up from scratch never needs it. |
| `MINER_VAULT_PASSPHRASE_FILE` | none | Reads `MINER_VAULT_PASSPHRASE` from a file. One or the other, not both. |
| `DATA_DIR` | `/data` in the image, otherwise `data` in the working directory | Database, key file and logs. |
| `LOG_DIR` | `$DATA_DIR/logs` | Log files: one a day, the last 14 kept. |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | Listen address: an IP address and a port from 1 to 65535. |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | Log level. |

The server reads and checks the variables once, at start. An invalid value stops the
server with a message, instead of a silent fallback.

## <a name="how-it-works"></a>How it works

Every 20 seconds, each account picks its two channels: the best live channel for the top
campaign of its queue, then the streamers of its points list.

Twitch credits **one Drops campaign** and **two channels for points** at a time per
account. Since October 2026, a drop can only be claimed when the game account is linked.
Unlinked campaigns still make progress, and the dashboard shows the link.

The campaign list comes from the [Twitch Drops API](https://drops.sunkwi.com) project
(refreshed every 10 minutes), because Twitch hides it from the TV client.

## <a name="faq"></a>FAQ

### Risks and Twitch

**Could my account get banned?**\
Nobody can promise it won't. This project is not affiliated with Twitch, and automating an
account may break its terms or a campaign's rules. The miner watches at most two channels
per account, Twitch's limit, without downloading any video. It's your call.

**Is the video really watched?**\
No, no video is downloaded. For Drops, every 15 seconds the miner reads the stream's
playlist at the lowest quality and only requests the header of each new segment. For
points, it sends Twitch's “minute watched” signal every minute. Each watched channel shows
how many segments were requested.

**Why two channels at most?**\
That is Twitch's limit: it only credits two channels watched at the same time per account,
and only one Drops campaign at a time. In **Drops and points** mode, one channel serves the
campaign and the other your streamers. The “One channel” setting, in **Account settings**,
is quieter.

### Drops

**A campaign is not being mined, or a drop is not progressing.**\
Open the account's **Drops** page. In “Mining order”, each waiting campaign says why:
“No live channel”, “Waiting (one campaign at a time)” or “No drop available yet”. A
campaign that does not appear in that list was left out: by the “Which campaigns”
setting, by a rule in the **Exclusions** tab, or because it cannot finish in time. A reward that
requires a subscription is never mined, and a drop that follows another one waits until
the first is done. In **Points only** mode, the miner picks no campaign. “Mine first”, in a
campaign's details, puts it at the top; an ignored campaign can come back with
“Mine again”, in the **Exclusions** tab.

**Do I need to link the game account?**\
To claim, yes: since October 2026, Twitch only lets you claim a drop when the game account
is linked. By default, an unlinked campaign still makes progress and the claim waits for
the link. The “Not linked” badge flags it, and “Link the game account”, in the campaign's
details, opens the linking page. To mine linked games only, choose
“Only games whose account is linked”.

### Install and data

**Desktop app or Docker?**\
The desktop app mines while your PC is on, with its tray icon, an optional password and
one-click updates. Docker mines day and night on a machine that is always on (VPS, NAS,
Raspberry Pi); you open it in a browser, protected by `MINER_PASSWORD`. The interface is the
same.

**Where is my data?**\
In the data folder: `~/.local/share/twitch-drops-points-miner` on Linux,
`%APPDATA%\twitch-drops-points-miner` on Windows,
`~/Library/Application Support/twitch-drops-points-miner` on macOS, and the `/data` volume
with Docker. On the desktop, **Settings › Desktop** opens it in one click. It holds
`miner.db` (accounts, settings, journal), `master.key` (the session key, when no app
password is set) and the logs.

**How do I update?**\
The desktop app checks for a new version when it starts, then every hour (you can change
this in **Settings › About**). Click the “Update” badge, then “Install and restart”. With
Docker, the dashboard tells you: run `docker compose pull`, then `docker compose up -d`.

**I forgot the app password.**\
It cannot be recovered: it is what encrypts your Twitch sessions. Quit the app, delete the
data folder, then add your accounts again; their settings have to be set up again. With
Docker, the dashboard password is changed in `MINER_PASSWORD`, then
`docker compose up -d`.

### Points and predictions

**How many points per hour?**\
It depends on Twitch and on each channel. The miner earns what Twitch gives a viewer:
watch-time points, bonus chests (about one every 15 minutes) and the watch streak bonus. A
subscription multiplies the points, from ×1.2 to ×2 depending on the tier. The Home page
shows the points earned, and the Journal each gain.

**Are bets risky?**\
Yes, points can be lost. Predictions are off by default, streamer by streamer. You set the
strategy, when to bet, the stake (5% of the balance by default, 50,000 points at most), a
minimum balance and, if you want, a condition, such as odds of at least 1.5. Keep the
stake small.

**A streamer is not being watched.**\
Only streamers that are turned on and live are watched, and only on a free channel: in
**Drops and points** mode, one of the two serves the current campaign; in **Drops only**
mode, no streamer is watched; a paused account watches nothing. The slots go in this
order, shown as a badge on each watched channel: “Streak bonus”, “Streak at risk”,
“Subscribed”, then “List order”. Move the streamer up the list, or turn off
“Watch streaks first” and “Subscribed channels first”.

## <a name="related-projects"></a>Related projects

Other projects by the same author:

- [**Twitch Drops API**](https://drops.sunkwi.com): a free public API of every running
  Twitch Drops and reward campaign, updated every minute. This app reads its campaign
  catalogue from it.
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com): a Discord bot that announces Twitch
  live streams and Drops campaigns in your server.
- [**Kick Alerts 🔔**](https://kick.sunkwi.com): a Discord bot that announces Kick live
  streams and Drops campaigns in your server.

## <a name="security"></a>Security

- Tokens are sealed with AES-256-GCM. The key is either a random `master.key` next to the
  database (default, nothing to do) or derived from the app password with Argon2id.
- The HTTP dashboard checks the password in constant time and uses HttpOnly,
  SameSite=Strict cookies (`Secure`, with the `__Host-` prefix, behind HTTPS). The
  server keeps only a SHA-256 of each session. It ends a session after 30 days or at
  sign-out. It checks the session before reading any request body and limits bodies to
  256 KiB.
- After 5 wrong passwords in a row from one IP address (sign-in form or bearer), that
  address waits 30 seconds. The server answers `429` with `Retry-After` without looking at
  the password. Each wrong password also takes half a second to answer.
- Outside your network, put the dashboard behind an HTTPS reverse proxy with
  `MINER_PUBLIC_URL` (see [Docker](#docker)).
- The miner only talks to `twitch.tv`, `ttvnw.net`, `jtvnw.net`, `twitchcdn.net`,
  `twitch-drops-api.sunkwi.com` and the notification targets you configure. The app
  (desktop and Docker) checks for new versions on `github.com`, and the desktop app
  downloads them from the project's GitHub releases. They are signed: only the version
  shown is installed.

## <a name="build"></a>Build from source

**Prerequisites:** Rust, Node.js and the
[Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) for your system.

```bash
npm ci
npm run build          # dashboard
cargo build --release -p miner-server   # server: target/release/twitch-miner
npm run bundle:linux   # desktop bundles (Tauri's Linux dependencies required)
```

Code checks:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier, oxlint (type-aware) and svelte-check
```

The TypeScript types of the interface are generated from the Rust types with
[ts-rs](https://github.com/Aleph-Alpha/ts-rs) into `ui/src/lib/bindings`. After changing a
type the interface sees, run `npm run bindings`.

## <a name="license"></a>Licence

[MIT](LICENSE). Inspired by [DevilXD's TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner),
[Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
and their forks.
