<p align="center">
  <img src="../../ui/public/app-icon.svg" width="112" height="112" alt="Twitch Drops &amp; Points Miner 标志">
</p>

<h1 align="center">Twitch Drops &amp; Points Miner</h1>

<p align="center">
  <strong>在多个账号上挖取 Twitch Drops 和频道点数，无需浏览器，也无需打开 Twitch 标签页。</strong>
</p>

<p align="center">
  适用于 Linux、Windows 和 macOS 的桌面应用；也可以使用带有相同网页仪表盘的 Docker 镜像，在家用服务器或 Raspberry Pi 上运行。运行 4 个账号时，服务器约占用 30 MB 内存，几乎不占用 CPU。
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-drops-points-miner?label=release&color=9146FF" alt="最新版本"></a>
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-drops-points-miner/total?label=downloads&color=9146FF" alt="下载量"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="平台：Linux、Windows、macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-drops-points-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="适用于 amd64 和 arm64 的 Docker 镜像"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="MIT 许可证"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="浏览量">
</p>

<p align="center">
  <a href="#download"><strong>下载</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/issues/new/choose">报告错误</a>
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
  <strong>简体中文</strong> ·
  <a href="README.zh-TW.md">繁體中文</a> ·
  <a href="README.ja.md">日本語</a> ·
  <a href="README.ko.md">한국어</a> ·
  <a href="README.th.md">ไทย</a> ·
  <a href="README.vi.md">Tiếng&nbsp;Việt</a> ·
  <a href="README.id.md">Bahasa&nbsp;Indonesia</a>
</p>

<p align="center">
  <img src="../media/zh-CN/presentation.avif" alt="Twitch Drops &amp; Points Miner 视频介绍">
</p>

## <a name="features"></a>功能

这些是它与其他挖掘器的不同之处。展开某一项下方的“查看演示”，即可看到它的实际运行效果。

### 账号与主播

- **一个应用，多个账号。** 每个账号按自己的模式各自挖取：**Drops 和点数**（一个频道用于最佳活动，另一个用于你的主播）、**仅 Drops** 或**仅点数**。

  <details open><summary>查看演示</summary>
  <img src="../media/zh-CN/accounts-mining.avif" alt="首页上多个账号同时挖取">
  </details>

- **每个新账号都有设置向导。** 登录后，可以从默认设置开始，也可以沿用另一个账号的设置。选择模式、活动和主播，然后点击“开始挖取”或“保存并稍后开始”。

  <details open><summary>查看演示</summary>
  <img src="../media/zh-CN/account-setup.avif" alt="新账号的设置向导，一步一步演示">
  </details>

- **一处修改，一键同步到其他账号。** 在某个账号上做出修改后，确认消息会提供“应用到其他账号”。这项修改会在你勾选的账号上再执行一次，这些账号的其他设置保持不变。

  <details open><summary>查看演示</summary>
  <img src="../media/zh-CN/apply-to-accounts.avif" alt="通过“应用到其他账号”把一项修改同步到其他账号">
  </details>

- **添加主播。** 主播会立即以默认选项开始挖取：应用的默认选项，或你为这个账号选定的选项。需要的话可以当场调整：奖励宝箱、突袭、时刻、观看连胜、聊天室、预测。

  <details open><summary>查看演示</summary>
  <img src="../media/zh-CN/streamer-add.avif" alt="通过 Twitch 搜索添加主播，然后查看它的选项">
  </details>

- **编辑主播。** “编辑”会打开它的选项：奖励宝箱、突袭、时刻、观看连胜、聊天室、预测……用一个开关就能启用或停用它，而不必把它从列表中移除。

  <details open><summary>查看演示</summary>
  <img src="../media/zh-CN/streamer-edit.avif" alt="从主播所在行打开它的选项">
  </details>

- **一次编辑多个主播。** 勾选它们，然后点击“编辑所选的 N 位主播”。只有你更改的选项才会生效。各主播之间取值不同的选项会显示为“混合”，它的开关会变成“是”和“否”两个按钮。随后，应用会询问是否将这些选项设为新主播的默认选项。

  <details open><summary>查看演示</summary>
  <img src="../media/zh-CN/streamers-group-edit.avif" alt="同时编辑多个主播，其中有显示为“混合”的选项">
  </details>

- **导入主播。** 粘贴一个列表（最多 500 个名称），或者沿用另一个账号的列表，或者使用该账号关注的频道。列表会先经过检查，只添加真实存在的频道。

  <details open><summary>查看演示</summary>
  <img src="../media/zh-CN/streamers-import.avif" alt="导入主播：粘贴的列表、另一个账号、关注的频道">
  </details>

- **Spam Points。** 将某个频道的自定义奖励连续兑换多次。从你的点数列表中选择频道，或通过 Twitch 搜索找到任意频道：它的奖励会与账号的余额一起显示，余额实时读取。设置数量（最多 25 次，且不超过余额），然后点击“兑换”。连发结束时会显示结果，每次连发都会在“记录”中留下一条。

  <details open><summary>查看演示</summary>
  <img src="../media/zh-CN/spam-points.avif" alt="Spam Points：选择一个频道，将一项带留言的奖励兑换 10 次，兑换出现在频道的 Twitch 聊天中，然后是记录中的那一条">
  </details>

### Drops

- **所有进行中的 Drops 活动。** 每个活动都有自己的详情：奖励、游戏账号关联、“优先挖取”。列表来自 [Twitch Drops API](https://drops.sunkwi.com) 项目，每 10 分钟刷新一次。

  <details open><summary>查看演示</summary>
  <img src="../media/zh-CN/drops-catalogue.avif" alt="日历视图的目录、搜索和某个活动的详情">
  </details>

- **你的挖取顺序。** 把活动拖成你想要的顺序。新活动会自动排到合适的位置：先是你的优先游戏，然后是最早结束或剩余分钟数最少的活动。可以排除游戏、关键词、某个活动或单个奖励，并跳过来不及完成的活动。在活动详情中点击“优先挖取”，即可把它排到最前面。

  <details open><summary>查看演示</summary>
  <img src="../media/zh-CN/drops-order.avif" alt="挖取顺序、优先游戏和排除项">
  </details>

- **库存与“全部领取”。** “库存”页面把奖励分为“待领取”“进行中”和“已领取”三类。可以逐个领取、一次全部领取，或者开启“自动领取掉落”。

  <details open><summary>查看演示</summary>
  <img src="../media/zh-CN/drops-inventory.avif" alt="库存和“全部领取”按钮">
  </details>

- **“Drops 优先我的主播”。** 对于当前活动，如果你列表中某位主播的直播计入该活动，就会优先于其他频道观看它。同一个频道既能推进掉落进度，又能为你赚取它的频道点数。默认开启。

  <details open><summary>查看演示</summary>
  <img src="../media/zh-CN/drops-streamers-first.avif" alt="列表中的一位主播，同时为 Drops 和它的点数而观看">
  </details>

### 跟踪

- **记录一切，包括每个请求。** “记录”页面保存每一个事件：掉落、获得和花费的点数、宝箱、下注、突袭、开播……可以按账号、按标签页（Drops、点数、其他）和按类型筛选。**技术日志 › 请求**列出发往 Twitch 和其他服务的每个请求，以及它的结果和耗时。日志和请求都不含任何令牌或密钥：点击“复制用于错误报告”即可复制，直接粘贴到 [GitHub](https://github.com/SunkwiBOT/twitch-drops-points-miner/issues) 或 [Discord](https://discord.gg/support-bots-891685895781113886) 上的错误报告中。

  <details open><summary>查看演示</summary>
  <img src="../media/zh-CN/journal-requests.avif" alt="筛选后的记录，然后是技术日志 › 请求">
  </details>

### 桌面或服务器

- **处处都是同一个界面。** 桌面应用（Linux、Windows、macOS）和 Docker 镜像显示的是同一个仪表盘。Docker 可以运行在 VPS、NAS 或 Raspberry Pi 上，那台机器上不需要屏幕或浏览器：仪表盘可以在任何浏览器中打开。
- **轻量。** 4 个账号约占用 30 MB 内存，几乎不占用 CPU，Docker 镜像约 12 MB。

### 安全与更新

- **会话受到保护。** 不需要 Twitch 密码：你在自己的浏览器中通过 `twitch.tv/activate` 登录。会话在磁盘上加密保存，你可以在 [Twitch 设置](https://www.twitch.tv/settings/connections)中撤销。在桌面版中，可选的应用密码会锁定应用：输入密码之前不会挖取任何内容。在 Docker 版本中，仪表盘有自己的密码，闲置几分钟后会自动锁定，并在代理之后通过 HTTPS 运行。连续输错 5 次密码，该 IP 地址会被封锁 30 秒。

  <details open><summary>查看演示</summary>
  <img src="../media/zh-CN/security-lock.avif" alt="锁：应用被锁定，然后用它的密码解锁">
  </details>

- **签名更新，从不自动安装。** 顶部栏会出现带版本号的“更新”徽章。“安装并重启”会先请你确认，然后应用会安装该版本并重启。设置了应用密码时，应用重启后处于锁定状态：重新输入密码即可继续挖取。只有经过项目签名的版本才会被安装。Docker 版本同样会提示你，并给出需要运行的命令。

  <details open><summary>查看演示</summary>
  <img src="../media/zh-CN/updates.avif" alt="“更新”徽章、确认，然后安装">
  </details>

### 此外

- 与其他挖掘器一样：奖励宝箱、突袭、时刻、社区目标、预测（策略、下注时机、条件）、观看连胜和已订阅频道优先、聊天室在线（可关闭）、新主播的默认选项、带图片的 Discord、Telegram 和 Webhook 通知、全局或按账号设置的代理。
- 29 种语言、深色和浅色主题、桌面通知、托盘图标、随系统启动。

  <details open><summary>查看演示</summary>
  <img src="../media/zh-CN/basics.avif" alt="多种语言、浅色和深色主题、一条桌面通知和托盘图标">
  </details>

> [!WARNING]
> 本项目与 Twitch 无关联。自动操作账号可能违反 Twitch 的条款或活动规则。

## 目录

- [对比](#comparison)
- [下载](#download)
- [Docker](#docker)
- [工作原理](#how-it-works)
- [常见问题](#faq)
- [相关项目](#related-projects)
- [安全](#security)
- [从源码构建](#build)
- [许可证](#license)

## <a name="comparison"></a>对比

[Twitch Drops & Points Miner](https://github.com/SunkwiBOT/twitch-drops-points-miner) 与 [Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)（频道点数）和 [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner)（Drops）的对比如下。已于 2026 年 11 月对照它们的代码和 README 核实。

| | 本项目 | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Drops 活动：查找、挖取并领取 | ✅ | ❌ ¹ | ✅ |
| 频道点数：宝箱、突袭、时刻、预测 | ✅ | ✅ | ❌ |
| Spam Points：将某个频道的自定义奖励连续兑换多次 | ✅ | ❌ | ❌ |
| 同一账号同时挖取 Drops 和频道点数 | ✅ | 部分 ¹ | ❌ |
| 一个应用管理多个账号 | ✅ | ❌ ² | ❌ |
| 一次修改多个账号的设置 | ✅ | ❌ | ❌ |
| 图形界面 | ✅ 桌面版和网页版 ³ | ❌ ⁴ | ✅ 桌面版 |
| 适用于家用服务器的网页仪表盘（Docker），带密码，可通过 HTTPS 访问 | ✅ | ❌ ⁴ | ❌ |
| 会话在磁盘上加密，可选应用密码 | ✅ | ❌ | ❌ |
| 界面支持 29 种语言 | ✅ | ❌ | 部分 ⁵ |
| 在应用内完成签名更新（桌面版） | ✅ | ❌ | ❌ |
| 每个新账号都有设置向导 | ✅ | ❌ ⁴ | ❌ |
| 在界面中添加和编辑主播，可逐个或批量操作 | ✅ | ❌ ⁴ | — |
| 通过搜索、列表、另一个账号或关注的频道导入主播 | ✅ | 部分 ⁶ | — |
| 带图片的 Discord、Telegram 和 Webhook 通知 | ✅ | 部分 ⁷ | ❌ |
| 所有 Drops 活动的日历，支持搜索 | ✅ | ❌ | 部分 ⁸ |
| 按活动设置挖取顺序、排除词、忽略活动或奖励 | ✅ | ❌ | ❌ ⁹ |
| 挖取活动的 Drops 时优先观看你列表中的主播 | ✅ | 部分 ¹⁰ | ❌ |
| 界面中可筛选的事件记录，以及每个请求的列表 | ✅ | ❌ ⁴ | ❌ |
| 约 30 MB 内存（Docker 服务器，4 个账号） | ✅ | — ¹¹ | — ¹¹ |

¹ 它只观看自己列表中的主播：会推进并领取这些直播中的 Drops，但不会挑选活动。\
² 每个账号一个进程。\
³ 可以作为桌面应用运行在你的 PC 上，也可以通过 Docker 运行在服务器（VPS、NAS、Raspberry Pi）上，或两者兼用；那台机器上不需要图形环境，也不需要浏览器：仪表盘可以在任何浏览器中打开。\
⁴ 通过 Python 文件配置；它的网页显示点数图表和日志文件。\
⁵ 21 种语言。\
⁶ 它可以使用该账号关注的频道。\
⁷ 只有文字，没有图片。它还支持 Matrix、Pushover 和 Gotify，本项目不支持。\
⁸ 带筛选条件的活动列表，没有日历，也没有搜索。\
⁹ 它有优先游戏和排除游戏，按游戏名称选择。\
¹⁰ 默认情况下，它会先观看列表中直播带有 Drops 的主播（它的 `DROPS` 优先级），但从不观看其他频道。\
¹¹ 未测量。

## <a name="download"></a>下载

从[最新版本](https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest)下载适合你系统的文件。

| 系统 | 安装包 |
|---|---|
| **Windows** · x64 | 安装程序（`setup.exe`）或[便携版可执行文件](https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest/download/TwitchDropsPointsMiner-portable.exe) |
| **macOS** 10.15 或更高版本 · Intel 和 Apple Silicon | 磁盘映像（`.dmg`） |
| **Linux** · x86-64 | `.AppImage`、`.deb`（Debian、Ubuntu）或 `.rpm`（Fedora、openSUSE） |

macOS 版本未经公证。首次运行时，请右键点击应用并选择**打开**。

更新均经过签名。应用启动时会检查新版本，之后每小时检查一次（可在**设置 › 关于**中更改检查频率）。它从不自行安装更新：你可以在弹出的提示中或在**设置 › 关于**中一键安装。

数据保存在用户数据文件夹中：

- Linux：`~/.local/share/twitch-drops-points-miner`
- Windows：`%APPDATA%\twitch-drops-points-miner`
- macOS：`~/Library/Application Support/twitch-drops-points-miner`

## <a name="docker"></a>Docker

服务器在浏览器中提供同一个仪表盘。镜像为 `ghcr.io/sunkwibot/twitch-drops-points-miner`，支持 `amd64` 和 `arm64`。有新版本发布时，仪表盘会提示你：先运行 `docker compose pull`，再运行 `docker compose up -d`。

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-drops-points-miner:latest
    pull_policy: always   # 每次启动时拉取最新镜像
    restart: unless-stopped
    ports: ["8080:8080"]   # 位于 HTTPS 反向代理之后时：删除此行，或改为 "127.0.0.1:8080:8080"
    environment:
      MINER_PASSWORD: "change-me"   # 必填，至少 12 个字符
      # MINER_PUBLIC_URL: "https://miner.example.com"   # 仅在 HTTPS 反向代理之后使用
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

打开 <http://localhost:8080>（或这台机器在你网络中的地址），然后输入密码。

### 变量

| 变量 | 默认值 | 用途 |
|---|---|---|
| `MINER_PASSWORD` | 无 | 仪表盘密码，必填：没有它，服务器会拒绝启动。至少 12 个字符（更短的密码会在启动时显示警告）。示例值 `change-me` 会被拒绝。 |
| `MINER_PASSWORD_FILE` | 无 | 从文件读取 `MINER_PASSWORD`，例如 Docker secret。两者只能设置其一。 |
| `MINER_PUBLIC_URL` | 无 | 仅在 HTTPS 反向代理（Caddy、nginx、Traefik、Cloudflare Tunnel……）之后使用：仪表盘的 `https://` 地址，不带路径，例如 `https://miner.example.com`。代理必须转发 `X-Forwarded-Proto`、`X-Forwarded-For` 和 websocket（常见的代理默认都会这样做），并且容器的端口不能被直接访问。这样一来，仪表盘会拒绝一切不是经由代理通过 HTTPS 发来的请求，并在某个地址输错 5 次密码后将其封锁 30 秒。代理必须运行在同一台机器或其局域网中：来自公网地址的连接会被拒绝。 |
| `MINER_TRUSTED_HOPS` | `1` | 仅配合 `MINER_PUBLIC_URL` 使用：会向 `X-Forwarded-For` 添加条目的代理数量，取值 1 到 10。两层代理前后相连时（例如 Cloudflare 位于 Traefik 之前）设为 `2`。 |
| `MINER_VAULT_PASSPHRASE` | 无 | 仅用于从设置了应用密码的桌面应用复制过来的数据文件夹：在启动时将其解锁。从零搭建的服务器永远不需要它。 |
| `MINER_VAULT_PASSPHRASE_FILE` | 无 | 从文件读取 `MINER_VAULT_PASSPHRASE`。两者只能设置其一。 |
| `DATA_DIR` | 镜像中为 `/data`，否则为工作目录下的 `data` | 数据库、密钥文件和日志。 |
| `LOG_DIR` | `$DATA_DIR/logs` | 日志文件：每天一个，保留最近 14 个。 |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | 监听地址：一个 IP 地址和一个 1 到 65535 之间的端口。 |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | 日志级别。 |

服务器只在启动时读取并检查一次这些变量。无效的值会让服务器停止并给出提示，而不是静默回退到默认值。

## <a name="how-it-works"></a>工作原理

每个账号每 20 秒选定一次它的两个频道：先为队列中排在最前的活动选出最合适的直播频道，再安排点数列表中的主播。

Twitch 对每个账号同时只计入**一个 Drops 活动**和**两个频道的点数**。自 2026 年 10 月起，只有关联了游戏账号才能领取掉落。未关联的活动仍会累积进度，仪表盘会显示关联链接。

活动列表来自 [Twitch Drops API](https://drops.sunkwi.com) 项目（每 10 分钟刷新一次），因为 Twitch 对 TV 客户端隐藏了它。

## <a name="faq"></a>常见问题

### 风险与 Twitch

**我的账号会被封禁吗？**\
没有人能保证不会。本项目与 Twitch 无关联，自动操作账号可能违反 Twitch 的条款或活动规则。挖掘器每个账号最多观看两个频道，这是 Twitch 的上限，而且不下载任何视频。用不用由你自己判断。

**真的会观看视频吗？**\
不会，不下载任何视频。对于 Drops，挖掘器每 15 秒以最低画质读取一次直播的播放列表，并且只请求每个新片段的头部。对于点数，它每分钟发送一次 Twitch 的“已观看一分钟”信号。每个正在观看的频道都会显示已请求的片段数。

**为什么最多两个频道？**\
这是 Twitch 的限制：每个账号同时只计入两个正在观看的频道，并且一次只计入一个 Drops 活动。在 **Drops 和点数**模式下，一个频道用于活动，另一个用于你的主播。**账号设置**中的“一个频道”选项更低调。

### Drops

**某个活动没有被挖取，或者某个掉落没有进度。**\
打开该账号的 **Drops** 页面。在“挖取顺序”中，每个等待中的活动都会说明原因：“没有直播中的频道”“等待中（一次一个活动）”或“暂无可用掉落”。没有出现在这个列表中的活动已被排除，原因可能是“挖取哪些活动”设置、**排除**标签页中的某条规则，或者它来不及完成。需要订阅的奖励永远不会被挖取，而接在另一个掉落之后的掉落会等前一个完成。在**仅点数**模式下，挖掘器不会选择任何活动。在活动详情中点击“优先挖取”可以把它排到最前面；被忽略的活动可以在**排除**标签页中通过“恢复挖取”重新加入。

**需要关联游戏账号吗？**\
要领取的话，需要：自 2026 年 10 月起，Twitch 只允许在游戏账号已关联时领取掉落。默认情况下，未关联的活动仍会累积进度，领取则会等到关联之后。“未关联”徽章会标出这种情况，活动详情中的“关联游戏账号”会打开关联页面。如果只想挖取已关联的游戏，请选择“仅已关联账号的游戏”。

### 安装与数据

**桌面应用还是 Docker？**\
桌面应用在你的 PC 开机期间挖取，提供托盘图标、可选密码和一键更新。Docker 在一台始终开机的机器（VPS、NAS、Raspberry Pi）上日夜挖取；你在浏览器中打开它，由 `MINER_PASSWORD` 保护。两者的界面相同。

**我的数据在哪里？**\
在数据文件夹中：Linux 上是 `~/.local/share/twitch-drops-points-miner`，Windows 上是 `%APPDATA%\twitch-drops-points-miner`，macOS 上是 `~/Library/Application Support/twitch-drops-points-miner`，Docker 中则是 `/data` 卷。在桌面版中，**设置 › 桌面**可以一键打开它。其中包含 `miner.db`（账号、设置、记录）、`master.key`（未设置应用密码时的会话密钥）和日志。

**如何更新？**\
桌面应用启动时会检查新版本，之后每小时检查一次（可在**设置 › 关于**中更改）。点击“更新”徽章，然后点击“安装并重启”。使用 Docker 时，仪表盘会提示你：运行 `docker compose pull`，然后运行 `docker compose up -d`。

**我忘记了应用密码。**\
密码无法找回：你的 Twitch 会话正是用它加密的。退出应用，删除数据文件夹，然后重新添加账号；它们的设置需要重新配置。使用 Docker 时，仪表盘密码在 `MINER_PASSWORD` 中修改，然后运行 `docker compose up -d`。

### 点数与预测

**每小时能获得多少点数？**\
这取决于 Twitch 和各个频道。挖掘器能获得的，就是 Twitch 给观众的那些：观看时长点数、奖励宝箱（大约每 15 分钟一个）和观看连胜奖励。订阅会按等级让点数乘以 ×1.2 到 ×2。首页显示获得的点数，“记录”则列出每一笔收入。

**下注有风险吗？**\
有，点数可能会输掉。预测默认关闭，需要逐个主播开启。你可以设置策略、下注时机、下注额（默认为余额的 5%，最多 50,000 点）、最低余额，还可以设置一个条件，例如赔率至少为 1.5。请控制下注额。

**某位主播没有被观看。**\
只有已启用且正在直播的主播才会被观看，而且只能使用空闲的频道：在 **Drops 和点数**模式下，其中一个频道用于当前活动；在**仅 Drops** 模式下，不会观看任何主播；已暂停的账号什么都不观看。频道名额按以下顺序分配，并以徽章形式显示在每个正在观看的频道上：“连胜奖励”“连胜有风险”“已订阅”，然后是“列表顺序”。把该主播在列表中往上移，或者关闭“观看连胜优先”和“已订阅频道优先”。

## <a name="related-projects"></a>相关项目

同一作者的其他项目：

- [**Twitch Drops API**](https://drops.sunkwi.com)：免费的公开 API，收录所有进行中的 Twitch Drops 和奖励活动，每分钟更新一次。本应用的活动目录就来自这里。
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com)：一个 Discord 机器人，在你的服务器中播报 Twitch 直播和 Drops 活动。
- [**Kick Alerts 🔔**](https://kick.sunkwi.com)：一个 Discord 机器人，在你的服务器中播报 Kick 直播和 Drops 活动。

## <a name="security"></a>安全

- 令牌使用 AES-256-GCM 加密封存。密钥要么是数据库旁边随机生成的 `master.key`（默认，无需任何操作），要么通过 Argon2id 从应用密码派生。
- HTTP 仪表盘使用恒定时间的密码校验，以及 HttpOnly、SameSite=Strict 的 Cookie（位于 HTTPS 之后时还带有 `Secure` 和 `__Host-` 前缀）。服务器只保存每个会话的 SHA-256。会话在 30 天后或退出登录时结束。服务器在读取任何请求体之前先检查会话，并将请求体限制在 256 KiB 以内。
- 同一 IP 地址连续输错 5 次密码后（登录表单或 bearer），该地址需等待 30 秒。服务器会直接返回 `429` 和 `Retry-After`，不会查看密码。每次密码错误还会延迟半秒才响应。
- 在你的网络之外访问时，请把仪表盘放在 HTTPS 反向代理之后，并设置 `MINER_PUBLIC_URL`（参见 [Docker](#docker)）。
- 挖掘器只与 `twitch.tv`、`ttvnw.net`、`jtvnw.net`、`twitchcdn.net`、`twitch-drops-api.sunkwi.com` 以及你配置的通知目标通信。应用（桌面版和 Docker 版）会在 `github.com` 上检查新版本，桌面应用会从项目的 GitHub releases 下载新版本。新版本都经过签名：只会安装界面上显示的那个版本。

## <a name="build"></a>从源码构建

**先决条件：** Rust、Node.js，以及适用于你系统的 [Tauri 先决条件](https://v2.tauri.app/start/prerequisites/)。

```bash
npm ci
npm run build          # 仪表盘
cargo build --release -p miner-server   # 服务器：target/release/twitch-miner
npm run bundle:linux   # 桌面安装包（需要 Tauri 的 Linux 依赖）
```

代码检查：

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier、oxlint（类型感知）和 svelte-check
```

界面的 TypeScript 类型由 [ts-rs](https://github.com/Aleph-Alpha/ts-rs) 从 Rust 类型生成到 `ui/src/lib/bindings` 中。修改界面会用到的类型后，请运行 `npm run bindings`。

## <a name="license"></a>许可证

[MIT](../../LICENSE)。灵感来自 [DevilXD 的 TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner)、[Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2) 及其分支。
