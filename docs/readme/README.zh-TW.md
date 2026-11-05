<p align="center">
  <img src="../../ui/public/app-icon.svg" width="112" height="112" alt="Twitch Drops &amp; Points Miner 標誌">
</p>

<h1 align="center">Twitch Drops &amp; Points Miner</h1>

<p align="center">
  <strong>在多個帳號上挖取 Twitch Drops 和頻道點數，不需要瀏覽器，也不必開著 Twitch 分頁。</strong>
</p>

<p align="center">
  支援 Linux、Windows 和 macOS 的桌面應用程式；也有內含相同網頁儀表板的 Docker 映像檔，可在家用伺服器或 Raspberry Pi 上執行。執行 4 個帳號時，伺服器約使用 30 MB 記憶體，幾乎不佔用 CPU。
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-drops-points-miner?label=release&color=9146FF" alt="最新版本"></a>
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-drops-points-miner/total?label=downloads&color=9146FF" alt="下載次數"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="平台：Linux、Windows、macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-drops-points-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="支援 amd64 和 arm64 的 Docker 映像檔"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="MIT 授權"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="瀏覽次數">
</p>

<p align="center">
  <a href="#download"><strong>下載</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/issues/new/choose">回報錯誤</a>
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
  <a href="README.fi.md">Suomi</a> ·
  <a href="README.zh-CN.md">简体中文</a> ·
  <strong>繁體中文</strong> ·
  <a href="README.ja.md">日本語</a> ·
  <a href="README.ko.md">한국어</a> ·
  <a href="README.th.md">ไทย</a> ·
  <a href="README.vi.md">Tiếng&nbsp;Việt</a> ·
  <a href="README.id.md">Bahasa&nbsp;Indonesia</a>
</p>

<p align="center">
  <img src="../media/zh-TW/presentation.avif" alt="Twitch Drops &amp; Points Miner 影片介紹">
</p>

## <a name="features"></a>功能

以下是它與其他挖掘器不同的地方。展開每一項下方的「觀看示範」，就能看到它實際運作的樣子。

### 帳號與實況主

- **一個應用程式，多個帳號。** 每個帳號依照自己的模式各自挖取：**Drops 和點數**（一個頻道用於最佳活動，另一個用於你的實況主）、**僅 Drops** 或**僅點數**。

  <details open><summary>觀看示範</summary>
  <img src="../media/zh-TW/accounts-mining.avif" alt="首頁上多個帳號同時挖取">
  </details>

- **每個新帳號都有設定精靈。** 登入後，可以從預設設定開始，也可以沿用另一個帳號的設定。選好模式、活動和實況主，再按「開始挖取」或「儲存並稍後開始」。

  <details open><summary>觀看示範</summary>
  <img src="../media/zh-TW/account-setup.avif" alt="新帳號的設定精靈，逐步示範">
  </details>

- **一次變更，一鍵套用到其他帳號。** 在某個帳號上做了變更後，確認訊息會提供「套用到其他帳號」。這項變更會在你勾選的帳號上再做一次，這些帳號的其他設定則維持不變。

  <details open><summary>觀看示範</summary>
  <img src="../media/zh-TW/apply-to-accounts.avif" alt="透過「套用到其他帳號」把一項變更同步到其他帳號">
  </details>

- **新增實況主。** 實況主會立即以預設選項開始挖取：應用程式的預設選項，或你為這個帳號選定的選項。需要的話可以當場調整：獎勵寶箱、Raid、時刻、連續觀看紀錄、聊天室、預測。

  <details open><summary>觀看示範</summary>
  <img src="../media/zh-TW/streamer-add.avif" alt="透過 Twitch 搜尋新增實況主，接著查看它的選項">
  </details>

- **編輯實況主。** 「編輯」會開啟它的選項：獎勵寶箱、Raid、時刻、連續觀看紀錄、聊天室、預測……用一個開關就能啟用或停用它，不必把它從清單中移除。

  <details open><summary>觀看示範</summary>
  <img src="../media/zh-TW/streamer-edit.avif" alt="從實況主所在列開啟它的選項">
  </details>

- **一次編輯多位實況主。** 勾選他們，再按「編輯所選的 N 位實況主」。只有你變更的選項才會套用。各實況主之間設定不同的選項會顯示為「混合」，它的開關會變成「是」和「否」兩個按鈕。接著，應用程式會詢問是否要把這些選項設為新實況主的預設選項。

  <details open><summary>觀看示範</summary>
  <img src="../media/zh-TW/streamers-group-edit.avif" alt="同時編輯多位實況主，其中有顯示為「混合」的選項">
  </details>

- **匯入實況主。** 貼上一份清單（最多 500 個名稱），或沿用另一個帳號的清單，或使用該帳號追隨的頻道。清單會先經過檢查，只加入確實存在的頻道。

  <details open><summary>觀看示範</summary>
  <img src="../media/zh-TW/streamers-import.avif" alt="匯入實況主：貼上的清單、另一個帳號、追隨的頻道">
  </details>

- **Spam Points。** 將某個頻道的自訂獎勵連續兌換多次。從你的點數清單中選擇頻道，或透過 Twitch 搜尋找到任何頻道：它的獎勵會和帳號的餘額一起顯示，餘額即時讀取。設定數量（最多 25 次，且不超過餘額），再按「兌換」。連發結束時會顯示結果，每次連發都會在「紀錄」中留下一筆。

  <details open><summary>觀看示範</summary>
  <img src="../media/zh-TW/spam-points.avif" alt="Spam Points：選擇一個頻道，將一項附留言的獎勵兌換 10 次，兌換出現在頻道的 Twitch 聊天室中，接著是紀錄中的那一筆">
  </details>

### Drops

- **所有進行中的 Drops 活動。** 每個活動都有自己的詳細資料：獎勵、遊戲帳號連結、「優先挖取」。清單來自 [Twitch Drops API](https://drops.sunkwi.com) 專案，每 10 分鐘更新一次。

  <details open><summary>觀看示範</summary>
  <img src="../media/zh-TW/drops-catalogue.avif" alt="以月曆呈現的目錄、搜尋和某個活動的詳細資料">
  </details>

- **你的挖取順序。** 把活動拖曳成你想要的順序。新活動會自動排到合適的位置：先是你的優先遊戲，再來是最早結束或剩餘分鐘數最少的活動。可以排除遊戲、關鍵字、某個活動或單一獎勵，並略過來不及完成的活動。在活動的詳細資料中按「優先挖取」，就能把它排到最前面。

  <details open><summary>觀看示範</summary>
  <img src="../media/zh-TW/drops-order.avif" alt="挖取順序、優先遊戲和排除項目">
  </details>

- **收藏庫與「全部領取」。** 「收藏庫」頁面把獎勵分成「待領取」「進行中」和「已領取」三類。可以逐一領取、一次全部領取，或開啟「自動領取掉寶」。

  <details open><summary>觀看示範</summary>
  <img src="../media/zh-TW/drops-inventory.avif" alt="收藏庫和「全部領取」按鈕">
  </details>

- **「Drops 優先我的實況主」。** 對於目前的活動，如果你清單上某位實況主的直播可計入該活動，就會比其他頻道優先觀看。同一個頻道既能推進掉寶進度，又能幫你賺取它的頻道點數。預設為開啟。

  <details open><summary>觀看示範</summary>
  <img src="../media/zh-TW/drops-streamers-first.avif" alt="清單上的一位實況主，同時為了 Drops 和它的點數而觀看">
  </details>

### 追蹤

- **完整紀錄，連每個請求都看得到。** 「紀錄」頁面會保存每一個事件：掉寶、賺取和花費的點數、寶箱、下注、Raid、開台……可以依帳號、依分頁（Drops、點數、其他）和依類型篩選。**技術日誌 › 請求**會列出送往 Twitch 和其他服務的每個請求，以及結果和花費的時間。日誌和請求都不含任何權杖或密鑰：按「複製以回報錯誤」就能複製，直接貼到 [GitHub](https://github.com/SunkwiBOT/twitch-drops-points-miner/issues) 或 [Discord](https://discord.gg/support-bots-891685895781113886) 上的錯誤回報中。

  <details open><summary>觀看示範</summary>
  <img src="../media/zh-TW/journal-requests.avif" alt="篩選後的紀錄，接著是技術日誌 › 請求">
  </details>

### 桌面或伺服器

- **到哪裡都是同一個介面。** 桌面應用程式（Linux、Windows、macOS）和 Docker 映像檔顯示的是同一個儀表板。Docker 可以在 VPS、NAS 或 Raspberry Pi 上執行，那台機器不需要螢幕或瀏覽器：儀表板可以在任何瀏覽器中開啟。
- **輕巧。** 4 個帳號約使用 30 MB 記憶體，幾乎不佔用 CPU，Docker 映像檔約 12 MB。

### 安全性與更新

- **工作階段受到保護。** 不需要 Twitch 密碼：你在自己的瀏覽器中透過 `twitch.tv/activate` 登入。工作階段會加密儲存在磁碟上，你可以在 [Twitch 設定](https://www.twitch.tv/settings/connections)中撤銷。在桌面版中，可選用的應用程式密碼會鎖定應用程式：輸入密碼之前什麼都不會挖取。在 Docker 版本中，儀表板有自己的密碼，閒置幾分鐘後會自動鎖定，並在代理伺服器後方透過 HTTPS 執行。連續輸錯 5 次密碼，該 IP 位址會被封鎖 30 秒。

  <details open><summary>觀看示範</summary>
  <img src="../media/zh-TW/security-lock.avif" alt="鎖頭：應用程式已鎖定，再用它的密碼解鎖">
  </details>

- **簽章更新，絕不自動安裝。** 頂端列會出現附有版本號碼的「更新」徽章。按「安裝並重新啟動」會先請你確認，接著應用程式會安裝該版本並重新啟動。若設定了應用程式密碼，重新啟動後會處於鎖定狀態：再次輸入密碼即可繼續挖取。只有經過本專案簽章的版本才會安裝。Docker 版本同樣會通知你，並附上需要執行的指令。

  <details open><summary>觀看示範</summary>
  <img src="../media/zh-TW/updates.avif" alt="「更新」徽章、確認，接著安裝">
  </details>

### 此外

- 與其他挖掘器一樣：獎勵寶箱、Raid、時刻、社群目標、預測（策略、下注時機、條件）、連續觀看紀錄和已訂閱頻道優先、聊天室在線（可關閉）、新實況主的預設選項、附圖片的 Discord、Telegram 和 Webhook 通知、全域或個別帳號的代理伺服器。
- 29 種語言、深色和淺色主題、桌面通知、系統匣圖示、隨系統啟動。

  <details open><summary>觀看示範</summary>
  <img src="../media/zh-TW/basics.avif" alt="多種語言、淺色和深色主題、一則桌面通知和系統匣圖示">
  </details>

> [!WARNING]
> 本專案與 Twitch 無關聯。自動操作帳號可能違反 Twitch 的條款或活動規則。

## 目錄

- [比較](#comparison)
- [下載](#download)
- [Docker](#docker)
- [運作原理](#how-it-works)
- [常見問題](#faq)
- [相關專案](#related-projects)
- [安全性](#security)
- [從原始碼建置](#build)
- [授權](#license)

## <a name="comparison"></a>比較

[Twitch Drops & Points Miner](https://github.com/SunkwiBOT/twitch-drops-points-miner) 與 [Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)（頻道點數）和 [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner)（Drops）的比較如下。已於 2026 年 11 月對照它們的程式碼和 README 確認。

| | 本專案 | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Drops 活動：尋找、挖取並領取 | ✅ | ❌ ¹ | ✅ |
| 頻道點數：寶箱、Raid、時刻、預測 | ✅ | ✅ | ❌ |
| Spam Points：將某個頻道的自訂獎勵連續兌換多次 | ✅ | ❌ | ❌ |
| 同一個帳號同時挖取 Drops 和頻道點數 | ✅ | 部分 ¹ | ❌ |
| 一個應用程式管理多個帳號 | ✅ | ❌ ² | ❌ |
| 一次變更多個帳號的設定 | ✅ | ❌ | ❌ |
| 圖形介面 | ✅ 桌面版和網頁版 ³ | ❌ ⁴ | ✅ 桌面版 |
| 家用伺服器的網頁儀表板（Docker），有密碼保護，可透過 HTTPS 連線 | ✅ | ❌ ⁴ | ❌ |
| 工作階段加密儲存在磁碟上，可選用應用程式密碼 | ✅ | ❌ | ❌ |
| 介面支援 29 種語言 | ✅ | ❌ | 部分 ⁵ |
| 在應用程式內完成簽章更新（桌面版） | ✅ | ❌ | ❌ |
| 每個新帳號都有設定精靈 | ✅ | ❌ ⁴ | ❌ |
| 在介面中新增和編輯實況主，可逐一或批次處理 | ✅ | ❌ ⁴ | — |
| 透過搜尋、清單、另一個帳號或追隨的頻道匯入實況主 | ✅ | 部分 ⁶ | — |
| 附圖片的 Discord、Telegram 和 Webhook 通知 | ✅ | 部分 ⁷ | ❌ |
| 所有 Drops 活動的月曆，支援搜尋 | ✅ | ❌ | 部分 ⁸ |
| 依活動設定挖取順序、排除的詞、忽略活動或獎勵 | ✅ | ❌ | ❌ ⁹ |
| 挖取活動的 Drops 時，優先觀看你清單上的實況主 | ✅ | 部分 ¹⁰ | ❌ |
| 介面中可篩選的事件紀錄，以及每個請求的清單 | ✅ | ❌ ⁴ | ❌ |
| 約 30 MB 記憶體（Docker 伺服器，4 個帳號） | ✅ | — ¹¹ | — ¹¹ |

¹ 它只觀看自己清單上的實況主：會推進並領取這些直播的 Drops，但不會挑選活動。\
² 每個帳號一個程序。\
³ 可以當作桌面應用程式在你的 PC 上執行，也可以用 Docker 在伺服器（VPS、NAS、Raspberry Pi）上執行，或兩者並用；那台機器不需要圖形環境，也不需要瀏覽器：儀表板可以在任何瀏覽器中開啟。\
⁴ 以 Python 檔案設定；它的網頁會顯示點數圖表和日誌檔案。\
⁵ 21 種語言。\
⁶ 它可以使用該帳號追隨的頻道。\
⁷ 只有文字，沒有圖片。它另外支援 Matrix、Pushover 和 Gotify，本專案則沒有。\
⁸ 有篩選條件的活動清單，沒有月曆，也沒有搜尋。\
⁹ 它有優先遊戲和排除的遊戲，依遊戲名稱選擇。\
¹⁰ 預設會先觀看清單上直播附有 Drops 的實況主（它的 `DROPS` 優先順序），但絕不會觀看其他頻道。\
¹¹ 未測量。

## <a name="download"></a>下載

請從[最新版本](https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest)下載適合你系統的檔案。

| 系統 | 套件 |
|---|---|
| **Windows** · x64 | 安裝程式（`setup.exe`）或[免安裝執行檔](https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest/download/TwitchDropsPointsMiner-portable.exe) |
| **macOS** 10.15 或以上版本 · Intel 和 Apple Silicon | 磁碟映像檔（`.dmg`） |
| **Linux** · x86-64 | `.AppImage`、`.deb`（Debian、Ubuntu）或 `.rpm`（Fedora、openSUSE） |

macOS 版本未經 Apple 公證。第一次執行時，請在應用程式上按右鍵，然後選擇**開啟**。

更新都經過簽章。應用程式啟動時會檢查是否有新版本，之後每小時檢查一次（可在**設定 › 關於**中變更檢查頻率）。它絕不會自行安裝更新：你可以從跳出的訊息或從**設定 › 關於**一鍵安裝。

資料存放在使用者資料夾中：

- Linux：`~/.local/share/twitch-drops-points-miner`
- Windows：`%APPDATA%\twitch-drops-points-miner`
- macOS：`~/Library/Application Support/twitch-drops-points-miner`

## <a name="docker"></a>Docker

伺服器會在瀏覽器中提供同一個儀表板。映像檔為 `ghcr.io/sunkwibot/twitch-drops-points-miner`，支援 `amd64` 和 `arm64`。有新版本推出時，儀表板會告訴你：先執行 `docker compose pull`，再執行 `docker compose up -d`。

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-drops-points-miner:latest
    pull_policy: always   # 每次啟動時都取得最新的映像檔
    restart: unless-stopped
    ports: ["8080:8080"]   # 位於 HTTPS 反向代理伺服器後方時：刪除這一行，或改成 "127.0.0.1:8080:8080"
    environment:
      MINER_PASSWORD: "change-me"   # 必填，至少 12 個字元
      # MINER_PUBLIC_URL: "https://miner.example.com"   # 僅在 HTTPS 反向代理伺服器後方使用
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

開啟 <http://localhost:8080>（或這台機器在你網路中的位址），然後輸入密碼。

### 變數

| 變數 | 預設值 | 用途 |
|---|---|---|
| `MINER_PASSWORD` | 無 | 儀表板密碼，必填：沒有設定的話，伺服器會拒絕啟動。至少 12 個字元（較短的密碼會在啟動時顯示警告）。範例值 `change-me` 會被拒絕。 |
| `MINER_PASSWORD_FILE` | 無 | 從檔案讀取 `MINER_PASSWORD`，例如 Docker secret。兩者只能擇一設定。 |
| `MINER_PUBLIC_URL` | 無 | 僅在 HTTPS 反向代理伺服器（Caddy、nginx、Traefik、Cloudflare Tunnel……）後方使用：儀表板的 `https://` 位址，不含路徑，例如 `https://miner.example.com`。代理伺服器必須轉送 `X-Forwarded-Proto`、`X-Forwarded-For` 和 websocket（常見的代理伺服器預設都會這麼做），而且容器的連接埠不能被直接連到。如此一來，儀表板會拒絕一切不是經由代理伺服器透過 HTTPS 送來的請求，並在某個位址輸錯 5 次密碼後將其封鎖 30 秒。代理伺服器必須在同一台機器或其區域網路中執行：來自公開位址的連線會被拒絕。 |
| `MINER_TRUSTED_HOPS` | `1` | 僅搭配 `MINER_PUBLIC_URL` 使用：會在 `X-Forwarded-For` 加上一筆項目的代理伺服器數量，介於 1 到 10。兩台代理伺服器前後串接時（例如 Cloudflare 位於 Traefik 前方）設為 `2`。 |
| `MINER_PROXY_NETWORKS` | 無 | 僅與 `MINER_PUBLIC_URL` 一起使用：除了本機及其私有區域網路之外，代理伺服器還可從哪些網路連線，以逗號分隔：單一位址或類似 `2001:db8:1::/64` 的網路，適用於 IPv6 位址為公開位址的區域網路。 |
| `MINER_VAULT_PASSPHRASE` | 無 | 僅適用於從已設定應用程式密碼的桌面版複製過來的資料目錄：在啟動時將其解鎖。全新架設的伺服器完全不需要它。 |
| `MINER_VAULT_PASSPHRASE_FILE` | 無 | 從檔案讀取 `MINER_VAULT_PASSPHRASE`。兩者只能擇一設定。 |
| `DATA_DIR` | 映像檔中為 `/data`，否則為工作目錄下的 `data` | 資料庫、金鑰檔案和日誌。 |
| `LOG_DIR` | `$DATA_DIR/logs` | 日誌檔案：每天一個，保留最近 14 個。 |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | 監聽位址：一個 IP 位址和一個介於 1 到 65535 的連接埠。 |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | 日誌等級。 |

伺服器只會在啟動時讀取並檢查這些變數一次。值無效時，伺服器會停止並顯示訊息，而不是默默改用預設值。

## <a name="how-it-works"></a>運作原理

每個帳號每 20 秒會選定一次自己的兩個頻道：先為佇列中最前面的活動挑出最合適的直播頻道，再安排點數清單上的實況主。

Twitch 對每個帳號同一時間只會計入**一個 Drops 活動**和**兩個頻道的點數**。自 2026 年 10 月起，必須先連結遊戲帳號才能領取掉寶。未連結的活動仍會累積進度，儀表板會顯示連結的入口。

活動清單來自 [Twitch Drops API](https://drops.sunkwi.com) 專案（每 10 分鐘更新一次），因為 Twitch 不會讓 TV 用戶端看到它。

## <a name="faq"></a>常見問題

### 風險與 Twitch

**我的帳號會被停權嗎？**\
沒有人能保證不會。本專案與 Twitch 無關聯，自動操作帳號可能違反 Twitch 的條款或活動規則。挖掘器每個帳號最多觀看兩個頻道，這是 Twitch 的上限，而且不會下載任何影片。要不要使用，由你自行判斷。

**真的會觀看影片嗎？**\
不會，不會下載任何影片。為了 Drops，挖掘器每 15 秒以最低畫質讀取一次直播的播放清單，而且只請求每個新片段的標頭。為了點數，它每分鐘會送出一次 Twitch 的「已觀看一分鐘」訊號。每個正在觀看的頻道都會顯示已請求的片段數。

**為什麼最多兩個頻道？**\
這是 Twitch 的限制：每個帳號同一時間只會計入兩個正在觀看的頻道，而且一次只計入一個 Drops 活動。在 **Drops 和點數**模式下，一個頻道用於活動，另一個用於你的實況主。**帳號設定**中的「一個頻道」選項比較低調。

### Drops

**某個活動沒有被挖取，或某個掉寶沒有進度。**\
開啟該帳號的 **Drops** 頁面。在「挖取順序」中，每個等待中的活動都會說明原因：「沒有直播中的頻道」「等待中（一次一個活動）」或「尚無可用掉寶」。沒有出現在這份清單中的活動已被排除，原因可能是「挖取哪些活動」設定、**排除**分頁中的某條規則，或它來不及完成。需要訂閱的獎勵永遠不會被挖取，而接在另一個掉寶之後的掉寶，會等前一個完成後才開始。在**僅點數**模式下，挖掘器不會選擇任何活動。在活動的詳細資料中按「優先挖取」可以把它排到最前面；被忽略的活動可以在**排除**分頁中透過「恢復挖取」重新加入。

**需要連結遊戲帳號嗎？**\
要領取的話，需要：自 2026 年 10 月起，Twitch 只允許在遊戲帳號已連結時領取掉寶。預設情況下，未連結的活動仍會累積進度，領取則會等到連結之後。「未連結」徽章會標示出這種情況，活動詳細資料中的「連結遊戲帳號」會開啟連結頁面。如果只想挖取已連結的遊戲，請選擇「僅已連結帳號的遊戲」。

### 安裝與資料

**桌面應用程式還是 Docker？**\
桌面應用程式會在你的 PC 開機期間挖取，提供系統匣圖示、可選用的密碼和一鍵更新。Docker 則在一台隨時開著的機器（VPS、NAS、Raspberry Pi）上日夜挖取；你在瀏覽器中開啟它，並由 `MINER_PASSWORD` 保護。兩者的介面相同。

**我的資料在哪裡？**\
在資料目錄中：Linux 上是 `~/.local/share/twitch-drops-points-miner`，Windows 上是 `%APPDATA%\twitch-drops-points-miner`，macOS 上是 `~/Library/Application Support/twitch-drops-points-miner`，Docker 則是 `/data` 磁碟區。在桌面版中，**設定 › 桌面**可以一鍵開啟它。裡面有 `miner.db`（帳號、設定、紀錄）、`master.key`（未設定應用程式密碼時的工作階段金鑰）和日誌。

**要怎麼更新？**\
桌面應用程式啟動時會檢查是否有新版本，之後每小時檢查一次（可在**設定 › 關於**中變更）。按「更新」徽章，再按「安裝並重新啟動」。使用 Docker 時，儀表板會通知你：執行 `docker compose pull`，再執行 `docker compose up -d`。

**我忘了應用程式密碼。**\
密碼無法找回：你的 Twitch 工作階段正是用它加密的。請結束應用程式、刪除資料目錄，然後重新新增帳號；這些帳號的設定必須重新設定。使用 Docker 時，儀表板密碼要在 `MINER_PASSWORD` 中變更，再執行 `docker compose up -d`。

### 點數與預測

**每小時能賺多少點數？**\
這取決於 Twitch 和各個頻道。挖掘器能賺到的，就是 Twitch 給觀眾的那些：觀看時數點數、獎勵寶箱（大約每 15 分鐘一個）和連續觀看獎勵。訂閱會依等級讓點數變成 ×1.2 到 ×2。首頁會顯示賺到的點數，「紀錄」則列出每一筆收入。

**下注有風險嗎？**\
有，點數可能會輸掉。預測預設為關閉，需要逐一為實況主開啟。你可以設定策略、下注時機、下注額（預設為餘額的 5%，最多 50,000 點）、最低餘額，也可以加上一個條件，例如賠率至少 1.5。請控制下注額。

**某位實況主沒有被觀看。**\
只有已啟用且正在直播的實況主才會被觀看，而且只能使用空著的頻道：在 **Drops 和點數**模式下，其中一個頻道用於目前的活動；在**僅 Drops** 模式下，不會觀看任何實況主；已暫停的帳號什麼都不會觀看。頻道名額依以下順序分配，並以徽章顯示在每個正在觀看的頻道上：「連續觀看獎勵」「連續紀錄有風險」「已訂閱」，然後是「清單順序」。把這位實況主在清單中往上移，或關閉「連續觀看紀錄優先」和「已訂閱頻道優先」。

## <a name="related-projects"></a>相關專案

同一位作者的其他專案：

- [**Twitch Drops API**](https://drops.sunkwi.com)：免費的公開 API，收錄所有進行中的 Twitch Drops 和獎勵活動，每分鐘更新一次。本應用程式的活動目錄就是從這裡讀取。
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com)：一個 Discord 機器人，會在你的伺服器中通知 Twitch 直播和 Drops 活動。
- [**Kick Alerts 🔔**](https://kick.sunkwi.com)：一個 Discord 機器人，會在你的伺服器中通知 Kick 直播和 Drops 活動。

## <a name="security"></a>安全性

- 權杖以 AES-256-GCM 加密封存。金鑰可以是資料庫旁隨機產生的 `master.key`（預設，不必做任何事），也可以用 Argon2id 從應用程式密碼衍生。
- HTTP 儀表板使用固定時間的密碼比對，以及 HttpOnly、SameSite=Strict 的 Cookie（透過 HTTPS 時另加 `Secure` 和 `__Host-` 前綴）。伺服器只保存每個工作階段的 SHA-256。工作階段會在 30 天後或登出時結束。伺服器會先檢查工作階段，才讀取任何請求內容，並將請求內容限制在 256 KiB 以內。
- 同一個 IP 位址連續輸錯 5 次密碼後（登入表單或 bearer），該位址必須等待 30 秒。伺服器會直接回傳 `429` 和 `Retry-After`，不會檢查密碼。每次輸錯密碼，回應也會延遲半秒。
- 在你的網路以外使用時，請把儀表板放在 HTTPS 反向代理伺服器後方，並設定 `MINER_PUBLIC_URL`（請參閱 [Docker](#docker)）。
- 挖掘器只會與 `twitch.tv`、`ttvnw.net`、`jtvnw.net`、`twitchcdn.net`、`drops.sunkwi.com` 以及你設定的通知目標通訊。應用程式（桌面版和 Docker 版）會在 `github.com` 檢查新版本，桌面版會從本專案的 GitHub releases 下載新版本。新版本都經過簽章：只會安裝畫面上顯示的那個版本。

## <a name="build"></a>從原始碼建置

**必要條件：** Rust、Node.js，以及適用於你系統的 [Tauri 必要條件](https://v2.tauri.app/start/prerequisites/)。

```bash
npm ci
npm run build          # 儀表板
cargo build --release -p miner-server   # 伺服器：target/release/twitch-miner
npm run bundle:linux   # 桌面版安裝套件（需要 Tauri 的 Linux 相依套件）
```

程式碼檢查：

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier、oxlint（型別感知）和 svelte-check
```

介面的 TypeScript 型別是用 [ts-rs](https://github.com/Aleph-Alpha/ts-rs) 從 Rust 型別產生到 `ui/src/lib/bindings` 的。變更介面會用到的型別後，請執行 `npm run bindings`。

## <a name="license"></a>授權

[MIT](../../LICENSE)。靈感來自 [DevilXD 的 TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner)、[Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2) 及其衍生專案。
