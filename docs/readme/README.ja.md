<p align="center">
  <img src="../../ui/public/app-icon.svg" width="112" height="112" alt="Twitch Miner のロゴ">
</p>

<h1 align="center">Twitch Miner</h1>

<p align="center">
  <strong>ブラウザも Twitch のタブも開かずに、複数のアカウントで Twitch Drops とチャンネルポイントをマイニングします。</strong>
</p>

<p align="center">
  Linux、Windows、macOS 向けのデスクトップアプリです。同じ Web ダッシュボードを備えた Docker イメージもあり、自宅サーバーや Raspberry Pi で動かせます。4 アカウントの場合、サーバーのメモリ使用量は約 30 MB で、CPU はほとんど使いません。
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-miner?label=release&color=9146FF" alt="最新リリース"></a>
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-miner/total?label=downloads&color=9146FF" alt="ダウンロード数"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="対応プラットフォーム：Linux、Windows、macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="amd64 と arm64 向けの Docker イメージ"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="MIT ライセンス"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="閲覧数">
</p>

<p align="center">
  <a href="#download"><strong>ダウンロード</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-miner/issues/new/choose">バグを報告</a>
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
  <a href="README.zh-TW.md">繁體中文</a> ·
  <strong>日本語</strong> ·
  <a href="README.ko.md">한국어</a> ·
  <a href="README.th.md">ไทย</a> ·
  <a href="README.vi.md">Tiếng&nbsp;Việt</a> ·
  <a href="README.id.md">Bahasa&nbsp;Indonesia</a>
</p>

<p align="center">
  <img src="../media/ja/presentation.avif" alt="Twitch Miner の紹介動画">
</p>

## <a name="features"></a>機能

ほかのマイナーとの違いをまとめました。各項目の下にある「実際の動作を見る」を開くと、動いている様子を確認できます。

### アカウントと配信者

- **1 つのアプリで複数のアカウント。** 各アカウントが、それぞれのモードで個別にマイニングします。**Drops とポイント**（1 チャンネルで最適なキャンペーン、もう 1 チャンネルで自分の配信者）、**Drops のみ**、**ポイントのみ**から選べます。

  <details open><summary>実際の動作を見る</summary>
  <img src="../media/ja/accounts-mining.avif" alt="ホームで複数のアカウントが同時にマイニングしている様子">
  </details>

- **新しいアカウントごとにセットアップウィザード。** ログインしたら、既定の設定から始めるか、別のアカウントの設定を引き継ぎます。モード、キャンペーン、配信者を選び、「マイニングを開始」または「保存してあとで開始」を押します。

  <details open><summary>実際の動作を見る</summary>
  <img src="../media/ja/account-setup.avif" alt="新しいアカウントのセットアップウィザードを順に進める様子">
  </details>

- **1 つの変更をワンクリックでほかのアカウントにも。** あるアカウントで設定を変えると、確認メッセージに「ほかのアカウントに適用」が表示されます。チェックしたアカウントにも同じ変更が適用され、それ以外の設定はそのまま残ります。

  <details open><summary>実際の動作を見る</summary>
  <img src="../media/ja/apply-to-accounts.avif" alt="「ほかのアカウントに適用」で同じ変更をほかのアカウントにも反映する様子">
  </details>

- **配信者を追加。** 追加した配信者は、既定のオプションですぐにマイニングを始めます。既定のオプションとは、アプリの既定値か、このアカウント用に選んだものです。必要ならその場で調整できます：ボーナスチェスト、レイド、モーメント、視聴ストリーク、チャット、予想。

  <details open><summary>実際の動作を見る</summary>
  <img src="../media/ja/streamer-add.avif" alt="Twitch の検索で配信者を追加し、そのオプションを開く様子">
  </details>

- **配信者を編集。** 「編集」でその配信者のオプションが開きます：ボーナスチェスト、レイド、モーメント、視聴ストリーク、チャット、予想など。スイッチ 1 つで、リストから外さずにオン・オフを切り替えられます。

  <details open><summary>実際の動作を見る</summary>
  <img src="../media/ja/streamer-edit.avif" alt="配信者の行から開いたオプション">
  </details>

- **複数の配信者をまとめて編集。** チェックを入れてから「選択した N 人の配信者を編集」を押します。適用されるのは変更したオプションだけです。配信者によって値が異なるオプションは「混在」と表示され、スイッチが「はい」と「いいえ」の 2 つのボタンに変わります。そのあと、アプリがこれを新しい配信者の既定オプションにするかどうかを尋ねます。

  <details open><summary>実際の動作を見る</summary>
  <img src="../media/ja/streamers-group-edit.avif" alt="「混在」のオプションを含む、複数の配信者の一括編集">
  </details>

- **配信者をインポート。** リストを貼り付ける（最大 500 件）か、別のアカウントのリストやそのアカウントがフォローしているチャンネルを使います。リストは確認され、実在するチャンネルだけが追加されます。

  <details open><summary>実際の動作を見る</summary>
  <img src="../media/ja/streamers-import.avif" alt="配信者のインポート：貼り付けたリスト、別のアカウント、フォロー中のチャンネル">
  </details>

- **Spam Points。** チャンネルのカスタム報酬を続けて何度も交換します。ポイントリストのチャンネルか、Twitch の検索で見つけた任意のチャンネルを選ぶと、その報酬がアカウントの残高とともに表示されます。残高はリアルタイムで読み直されます。数量を設定し（最大 25 件、残高の範囲内）、「交換」を押します。結果は連続交換の終わりに表示され、連続交換ごとに「履歴」に 1 行が記録されます。

  <details open><summary>実際の動作を見る</summary>
  <img src="../media/ja/spam-points.avif" alt="Spam Points：チャンネルを選んでメッセージ付きの報酬を 10 回交換し、交換がチャンネルの Twitch チャットに流れたあと、履歴に記録された行を確認する様子">
  </details>

### Drops

- **開催中のすべての Drops キャンペーン。** キャンペーンごとに詳細があります：報酬、ゲームアカウントの連携、「優先してマイニング」。一覧は [Twitch Drops API](https://drops.sunkwi.com) プロジェクトから取得し、10 分ごとに更新されます。

  <details open><summary>実際の動作を見る</summary>
  <img src="../media/ja/drops-catalogue.avif" alt="カレンダー表示のカタログ、検索、キャンペーンの詳細">
  </details>

- **自分で決めるマイニング順。** キャンペーンをドラッグして好きな順番に並べます。新しいキャンペーンは自動で適切な位置に入ります。まず優先ゲーム、次に終了が近いものや残り時間（分）が少ないものの順です。ゲーム、語句、キャンペーン、個別の報酬を除外でき、期限内に終わらないキャンペーンはスキップできます。キャンペーンの詳細で「優先してマイニング」を押すと、先頭に移動します。

  <details open><summary>実際の動作を見る</summary>
  <img src="../media/ja/drops-order.avif" alt="マイニング順、優先ゲーム、除外設定">
  </details>

- **インベントリと「すべて受け取る」。** 「インベントリ」ページでは、報酬が「受け取り可能」「進行中」「受け取り済み」に分かれています。1 つずつ受け取ることも、まとめて受け取ることも、「ドロップを自動で受け取る」をオンにすることもできます。

  <details open><summary>実際の動作を見る</summary>
  <img src="../media/ja/drops-inventory.avif" alt="インベントリと「すべて受け取る」ボタン">
  </details>

- **「Drops で自分の配信者を優先」。** 現在のキャンペーンについて、リストにいる配信者の配信がそのキャンペーンの対象なら、ほかのチャンネルより先に視聴します。同じチャンネルでドロップが進み、そのチャンネルのチャンネルポイントも貯まります。既定でオンです。

  <details open><summary>実際の動作を見る</summary>
  <img src="../media/ja/drops-streamers-first.avif" alt="リストの配信者を、Drops とそのチャンネルのポイントのために視聴している様子">
  </details>

### 記録

- **すべてを記録し、リクエストも 1 件残らず表示。** 「履歴」には、ドロップ、獲得・使用したポイント、チェスト、ベット、レイド、配信開始など、すべてのイベントが残ります。アカウント、タブ（Drops、ポイント、その他）、種類で絞り込めます。**技術ログ › リクエスト**には、Twitch やほかのサービスに送ったすべてのリクエストが、結果と所要時間つきで一覧表示されます。ログにもリクエストにもトークンや秘密情報は含まれません。「バグ報告用にコピー」でコピーすれば、そのまま [GitHub](https://github.com/SunkwiBOT/twitch-miner/issues) や [Discord](https://discord.gg/support-bots-891685895781113886) のバグ報告に貼り付けられます。

  <details open><summary>実際の動作を見る</summary>
  <img src="../media/ja/journal-requests.avif" alt="絞り込んだ履歴と、技術ログ › リクエスト">
  </details>

### デスクトップでもサーバーでも

- **どこでも同じインターフェース。** デスクトップアプリ（Linux、Windows、macOS）と Docker イメージは、同じダッシュボードを表示します。Docker は VPS、NAS、Raspberry Pi で動き、そのマシンに画面もブラウザも要りません。ダッシュボードはどのブラウザからでも開けます。スマートフォンではダッシュボードが画面に合わせて表示され、HTTPS ならアプリのようにインストールできます。
- **軽量。** 4 アカウントでメモリは約 30 MB、CPU はほとんど使わず、Docker イメージは約 12 MB です。

### セキュリティとアップデート

- **セッションを守る仕組み。** Twitch のパスワードは不要です。ログインは自分のブラウザで `twitch.tv/activate` から行います。セッションはディスク上で暗号化され、[Twitch の設定](https://www.twitch.tv/settings/connections)から取り消せます。デスクトップでは、任意のアプリのパスワードでアプリをロックでき、入力するまで何もマイニングしません。Docker では、ダッシュボードに専用のパスワードがあり、操作のないまま数分たつと自動でロックされ、プロキシの背後で HTTPS で動作します。パスワードを 5 回続けて間違えると、その IP アドレスは 30 秒間ブロックされます。

  <details open><summary>実際の動作を見る</summary>
  <img src="../media/ja/security-lock.avif" alt="南京錠：ロックされたアプリを、パスワードで解除する様子">
  </details>

- **署名付きのアップデート、自動では入りません。** 上部のバーに、バージョン番号つきの「アップデート」バッジが表示されます。「インストールして再起動」を押すとまず確認を求められ、そのあとアプリがそのバージョンをインストールして再起動します。アプリのパスワードを設定している場合は、ロックされた状態で再起動するので、もう一度入力するとマイニングが再開します。インストールされるのは、プロジェクトが署名したバージョンだけです。Docker 版も、実行するコマンドとあわせて知らせてくれます。

  <details open><summary>実際の動作を見る</summary>
  <img src="../media/ja/updates.avif" alt="「アップデート」バッジ、確認、そしてインストール">
  </details>

### そのほか

- ほかのマイナーと同じく対応しているもの：ボーナスチェスト、レイド、モーメント、コミュニティゴール、予想（戦略、賭けるタイミング、条件）、視聴ストリークとサブスク中のチャンネルの優先、チャット参加（オフにできます）、新しい配信者の既定オプション、画像付きの Discord・Telegram・Webhook 通知、全体またはアカウントごとのプロキシ。
- **近日対応：バッジのマイニング。** 視聴で手に入る Twitch のバッジを、Drops と同じようにマイニングします。
- 29 言語、ダークとライトのテーマ、デスクトップ通知、トレイアイコン、システム起動時の開始。

  <details open><summary>実際の動作を見る</summary>
  <img src="../media/ja/basics.avif" alt="言語、ライトとダークのテーマ、デスクトップ通知、トレイアイコン">
  </details>

> [!WARNING]
> このプロジェクトは Twitch とは無関係です。アカウントの自動化は、Twitch の規約やキャンペーンのルールに違反する可能性があります。

## 目次

- [比較](#comparison)
- [ダウンロード](#download)
- [Docker](#docker)
- [仕組み](#how-it-works)
- [よくある質問](#faq)
- [関連プロジェクト](#related-projects)
- [セキュリティ](#security)
- [ソースからビルド](#build)
- [ライセンス](#license)

## <a name="comparison"></a>比較

[Twitch Miner](https://github.com/SunkwiBOT/twitch-miner) と、[Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)（チャンネルポイント）および [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner)（Drops）との比較です。2026 年 11 月に、それぞれのコードと README で確認しました。

| | このプロジェクト | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Drops キャンペーン：探して、マイニングして、受け取る | ✅ | ❌ ¹ | ✅ |
| チャンネルポイント：チェスト、レイド、モーメント、予想 | ✅ | ✅ | ❌ |
| Spam Points：チャンネルのカスタム報酬を続けて何度も交換 | ✅ | ❌ | ❌ |
| 1 つのアカウントで Drops とチャンネルポイントを同時に | ✅ | 一部 ¹ | ❌ |
| 1 つのアプリで複数のアカウント | ✅ | ❌ ² | ❌ |
| 複数のアカウントの設定を一度に変更 | ✅ | ❌ | ❌ |
| グラフィカルインターフェース | ✅ デスクトップと Web ³ | ❌ ⁴ | ✅ デスクトップ |
| 自宅サーバー向けの Web ダッシュボード（Docker）、パスワード付き、HTTPS でアクセス可能 | ✅ | ❌ ⁴ | ❌ |
| ディスク上で暗号化されたセッション、任意のアプリのパスワード | ✅ | ❌ | ❌ |
| 29 言語に対応したインターフェース | ✅ | ❌ | 一部 ⁵ |
| アプリ内からの署名付きアップデート（デスクトップ） | ✅ | ❌ | ❌ |
| 新しいアカウントごとのセットアップウィザード | ✅ | ❌ ⁴ | ❌ |
| インターフェース上で配信者を追加・編集（1 人ずつでも、まとめてでも） | ✅ | ❌ ⁴ | — |
| 検索、リスト、別のアカウント、フォロー中のチャンネルから配信者をインポート | ✅ | 一部 ⁶ | — |
| 画像付きの Discord、Telegram、Webhook 通知 | ✅ | 一部 ⁷ | ❌ |
| すべての Drops キャンペーンのカレンダーと検索 | ✅ | ❌ | 一部 ⁸ |
| キャンペーン単位のマイニング順、除外する語、無視するキャンペーンや報酬 | ✅ | ❌ | ❌ ⁹ |
| キャンペーンの Drops のために、リストにある配信者を優先して視聴 | ✅ | 一部 ¹⁰ | ❌ |
| 絞り込める履歴と、すべてのリクエストの一覧をインターフェース上で表示 | ✅ | ❌ ⁴ | ❌ |
| メモリ約 30 MB（Docker サーバー、4 アカウント） | ✅ | — ¹¹ | — ¹¹ |

¹ リストにある配信者だけを視聴します。その配信の Drops を進めて受け取りますが、キャンペーンは選びません。\
² アカウントごとに 1 プロセス。\
³ PC 上のデスクトップアプリとして、または Docker でサーバー（VPS、NAS、Raspberry Pi）上で、あるいはその両方で動かせます。そのマシンにはグラフィカル環境もブラウザも不要で、ダッシュボードはどのブラウザでも開けます。\
⁴ Python ファイルで設定します。Web ページにはポイントのグラフとログファイルが表示されます。\
⁵ 21 言語。\
⁶ アカウントがフォローしているチャンネルを対象にできます。\
⁷ テキストのみで、画像はありません。Matrix、Pushover、Gotify にも対応していますが、このプロジェクトは未対応です。\
⁸ フィルター付きのキャンペーン一覧で、カレンダーや検索はありません。\
⁹ 優先ゲームと除外ゲームがあり、ゲーム名で指定します。\
¹⁰ 既定では、リストの配信者のうち Drops 対象の配信をしている人を先に視聴します（`DROPS` 優先度）が、ほかのチャンネルは視聴しません。\
¹¹ 未計測。

## <a name="download"></a>ダウンロード

[最新リリース](https://github.com/SunkwiBOT/twitch-miner/releases/latest)から、お使いのシステム用のファイルをダウンロードしてください。

| システム | パッケージ |
|---|---|
| **Windows** · x64 | インストーラー（`setup.exe`）または[ポータブル版の実行ファイル](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/TwitchMiner-portable.exe) |
| **macOS** 10.15 以降 · Intel と Apple Silicon | ディスクイメージ（`.dmg`） |
| **Linux** · x86-64 | `.AppImage`、`.deb`（Debian、Ubuntu）、`.rpm`（Fedora、openSUSE） |

macOS 版は公証を受けていません。初回は、アプリを右クリックして**開く**を選んでください。

アップデートには署名が付いています。アプリは起動時に新しいバージョンを確認し、その後は 1 時間ごとに確認します（間隔は**設定 › 情報**で変更できます）。自動でインストールすることはありません。表示されるメッセージか**設定 › 情報**から、ワンクリックでインストールします。

データはユーザーデータフォルダに保存されます：

- Linux：`~/.local/share/twitch-miner`
- Windows：`%APPDATA%\twitch-miner`
- macOS：`~/Library/Application Support/twitch-miner`

## <a name="docker"></a>Docker

サーバーは同じダッシュボードをブラウザに表示します。イメージは `ghcr.io/sunkwibot/twitch-miner` で、`amd64` と `arm64` に対応しています。新しいバージョンが出るとダッシュボードに表示されるので、`docker compose pull` を実行してから `docker compose up -d` を実行します。

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-miner:latest
    pull_policy: always   # 起動のたびに最新のイメージを取得する
    restart: unless-stopped
    ports: ["8080:8080"]   # HTTPS リバースプロキシの背後に置く場合：この行を削除するか "127.0.0.1:8080:8080" にする
    environment:
      MINER_PASSWORD: "change-me"   # 必須、12 文字以上
      # MINER_PUBLIC_URL: "https://miner.example.com"   # HTTPS リバースプロキシの背後に置く場合のみ
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

<http://localhost:8080>（またはネットワーク上のそのマシンのアドレス）を開き、パスワードを入力します。

### 変数

| 変数 | 既定値 | 用途 |
|---|---|---|
| `MINER_PASSWORD` | なし | ダッシュボードのパスワード。必須で、設定されていないとサーバーは起動を拒否します。12 文字以上にします（短い場合は起動時に警告が表示されます）。例の値 `change-me` は拒否されます。 |
| `MINER_PASSWORD_FILE` | なし | `MINER_PASSWORD` をファイルから読み込みます（Docker secret など）。どちらか一方だけを設定してください。 |
| `MINER_PUBLIC_URL` | なし | HTTPS リバースプロキシ（Caddy、nginx、Traefik、Cloudflare Tunnel など）の背後に置く場合のみ：ダッシュボードの `https://` アドレス。パスは含めず、`https://miner.example.com` のように指定します。プロキシは `X-Forwarded-Proto`、`X-Forwarded-For`、websocket を転送する必要があり（一般的なプロキシは既定でそうします）、コンテナーのポートに直接アクセスできてはいけません。これにより、ダッシュボードはプロキシを経由した HTTPS 以外のアクセスをすべて拒否し、パスワードを 5 回間違えたアドレスを 30 秒間ブロックします。プロキシは同じマシンかそのローカルネットワーク上で動かす必要があり、パブリックアドレスからの接続は拒否されます。 |
| `MINER_TRUSTED_HOPS` | `1` | `MINER_PUBLIC_URL` を使う場合のみ：`X-Forwarded-For` にエントリを追加するプロキシの数（1〜10）。プロキシを 2 段に重ねる場合（Traefik の前に Cloudflare など）は `2` です。 |
| `MINER_PROXY_NETWORKS` | なし | `MINER_PUBLIC_URL` と併用する場合のみ：このマシンとそのプライベートなローカルネットワーク以外に、プロキシの接続を許可するネットワーク。カンマ区切りで、単一のアドレスか `2001:db8:1::/64` のようなネットワークを指定します（IPv6 アドレスがグローバルなローカルネットワーク向け）。 |
| `MINER_VAULT_PASSPHRASE` | なし | アプリのパスワードを設定したデスクトップアプリからコピーしたデータフォルダ専用です。起動時にロックを解除します。一から構築したサーバーでは必要ありません。 |
| `MINER_VAULT_PASSPHRASE_FILE` | なし | `MINER_VAULT_PASSPHRASE` をファイルから読み込みます。どちらか一方だけを設定してください。 |
| `DATA_DIR` | イメージ内では `/data`、それ以外は作業ディレクトリの `data` | データベース、鍵ファイル、ログ。 |
| `LOG_DIR` | `$DATA_DIR/logs` | ログファイル：1 日 1 ファイル、直近 14 個を保持。 |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | 待ち受けアドレス：IP アドレスと、1〜65535 のポート。 |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | ログレベル。 |

サーバーは起動時に一度だけ変数を読み込んで検証します。不正な値があると、黙って既定値に戻すのではなく、メッセージを出してサーバーを停止します。

## <a name="how-it-works"></a>仕組み

各アカウントは 20 秒ごとに 2 つのチャンネルを選びます。まずキューの先頭にあるキャンペーンに最適なライブチャンネル、次にポイントリストの配信者です。

Twitch がカウントするのは、1 アカウントにつき同時に**Drops キャンペーン 1 つ**と**ポイント用のチャンネル 2 つ**までです。2026 年 10 月以降、ドロップを受け取れるのは、ゲームアカウントを連携している場合だけです。未連携のキャンペーンでも進捗は貯まり、ダッシュボードに連携用のリンクが表示されます。

キャンペーン一覧は [Twitch Drops API](https://drops.sunkwi.com) プロジェクトから取得します（10 分ごとに更新）。Twitch が TV クライアントには一覧を見せないためです。

## <a name="faq"></a>よくある質問

### リスクと Twitch

**アカウントが BAN される可能性はありますか？**\
絶対にないとは誰にも言えません。このプロジェクトは Twitch とは無関係で、アカウントの自動化は Twitch の規約やキャンペーンのルールに違反する可能性があります。マイナーが視聴するのは 1 アカウントにつき最大 2 チャンネル（Twitch の上限）で、動画は一切ダウンロードしません。使うかどうかはご自身で判断してください。

**動画は本当に視聴されているのですか？**\
いいえ、動画はダウンロードしません。Drops では、マイナーが 15 秒ごとに配信のプレイリストを最低画質で読み込み、新しいセグメントごとにヘッダーだけをリクエストします。ポイントでは、Twitch の「1 分視聴」シグナルを毎分送ります。視聴中の各チャンネルに、リクエストしたセグメント数が表示されます。

**なぜ最大 2 チャンネルなのですか？**\
Twitch の上限だからです。Twitch がカウントするのは、1 アカウントにつき同時に視聴している 2 チャンネルまでで、Drops キャンペーンは一度に 1 つだけです。**Drops とポイント**モードでは、1 チャンネルがキャンペーン用、もう 1 チャンネルが自分の配信者用になります。**アカウント設定**の「1 チャンネル」にすると、より控えめに動きます。

### Drops

**キャンペーンがマイニングされない、またはドロップが進まない。**\
そのアカウントの **Drops** ページを開いてください。「マイニング順」では、待機中の各キャンペーンに理由が表示されます：「ライブ中のチャンネルなし」「待機中（一度に 1 キャンペーン）」「利用できるドロップはまだありません」。この一覧にないキャンペーンは対象から外れています。原因は「対象のキャンペーン」の設定、**除外**タブのルール、または期限内に終わらないことのいずれかです。サブスクが必要な報酬はマイニングされません。また、別のドロップに続くドロップは、前のドロップが終わるまで待ちます。**ポイントのみ**モードでは、マイナーはキャンペーンを選びません。キャンペーンの詳細で「優先してマイニング」を押すと先頭に移動し、無視したキャンペーンは**除外**タブの「再びマイニング」で戻せます。

**ゲームアカウントの連携は必要ですか？**\
受け取るには必要です。2026 年 10 月以降、Twitch ではゲームアカウントを連携していないとドロップを受け取れません。既定では、未連携のキャンペーンも進捗は貯まり、受け取りは連携するまで待ちます。「未連携」バッジでわかるようになっていて、キャンペーンの詳細にある「ゲームアカウントを連携」で連携ページが開きます。連携済みのゲームだけをマイニングするには、「アカウント連携済みのゲームのみ」を選んでください。

### インストールとデータ

**デスクトップアプリと Docker、どちらがいいですか？**\
デスクトップアプリは PC が起動している間マイニングし、トレイアイコン、任意のパスワード、ワンクリックのアップデートを備えています。Docker は常時稼働のマシン（VPS、NAS、Raspberry Pi）で昼夜を問わずマイニングし、ブラウザで開いて `MINER_PASSWORD` で保護します。インターフェースは同じです。

**データはどこにありますか？**\
データフォルダにあります。Linux では `~/.local/share/twitch-miner`、Windows では `%APPDATA%\twitch-miner`、macOS では `~/Library/Application Support/twitch-miner`、Docker では `/data` ボリュームです。デスクトップでは、**設定 › デスクトップ**からワンクリックで開けます。中には `miner.db`（アカウント、設定、履歴）、`master.key`（アプリのパスワードを設定していない場合のセッション鍵）、ログがあります。

**アップデートの方法は？**\
デスクトップアプリは起動時に新しいバージョンを確認し、その後は 1 時間ごとに確認します（**設定 › 情報**で変更できます）。「アップデート」バッジを押し、次に「インストールして再起動」を押します。Docker では、ダッシュボードが知らせてくれるので、`docker compose pull` を実行してから `docker compose up -d` を実行します。

**アプリのパスワードを忘れました。**\
パスワードは復元できません。Twitch のセッションを暗号化しているのがこのパスワードだからです。アプリを終了してデータフォルダを削除し、アカウントを追加し直してください。各アカウントの設定もやり直しになります。Docker では、ダッシュボードのパスワードを `MINER_PASSWORD` で変更してから `docker compose up -d` を実行します。

### ポイントと予想

**1 時間に何ポイント貯まりますか？**\
Twitch とチャンネルによって異なります。マイナーが獲得するのは、Twitch が視聴者に与えるものと同じです：視聴時間のポイント、ボーナスチェスト（約 15 分ごとに 1 つ）、視聴ストリークのボーナス。サブスクをしていると、ティアに応じてポイントが ×1.2〜×2 になります。獲得したポイントは「ホーム」に、1 回ごとの獲得は「履歴」に表示されます。

**ベットにリスクはありますか？**\
はい、ポイントを失うことがあります。予想は既定でオフで、配信者ごとに設定します。戦略、賭けるタイミング、賭け額（既定は残高の 5%、最大 50,000 ポイント）、最低残高を設定でき、必要なら「オッズ 1.5 以上」のような条件も付けられます。賭け額は小さくしておきましょう。

**配信者が視聴されない。**\
視聴されるのは、オンになっていてライブ中の配信者だけで、しかも空いているチャンネルがある場合に限られます。**Drops とポイント**モードでは 2 つのうち 1 つが現在のキャンペーン用です。**Drops のみ**モードでは配信者は視聴されず、一時停止中のアカウントは何も視聴しません。チャンネルの枠は次の順で割り当てられ、視聴中の各チャンネルにバッジで表示されます：「ストリークボーナス」「ストリークが危険」「サブスク中」、そして「リスト順」。その配信者をリストの上のほうに移すか、「視聴ストリークを優先」と「サブスク中のチャンネルを優先」をオフにしてください。

## <a name="related-projects"></a>関連プロジェクト

同じ作者によるほかのプロジェクト：

- [**Twitch Drops API**](https://drops.sunkwi.com)：開催中のすべての Twitch Drops と報酬キャンペーンをまとめた無料の公開 API で、毎分更新されます。このアプリのキャンペーンカタログは、ここから読み込んでいます。
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com)：Twitch の配信開始と Drops キャンペーンを、あなたのサーバーで告知する Discord ボットです。
- [**Kick Alerts 🔔**](https://kick.sunkwi.com)：Kick の配信開始と Drops キャンペーンを、あなたのサーバーで告知する Discord ボットです。

## <a name="security"></a>セキュリティ

- トークンは AES-256-GCM で封印されます。鍵は、データベースの隣に置かれるランダムな `master.key`（既定、何もする必要はありません）か、アプリのパスワードから Argon2id で導出したもののどちらかです。
- HTTP ダッシュボードは、定数時間でのパスワード照合と、HttpOnly・SameSite=Strict の Cookie（HTTPS の背後では `Secure` 付き、`__Host-` プレフィックス付き）を使います。サーバーに保存されるのは各セッションの SHA-256 だけです。セッションは 30 日後またはログアウト時に終了します。リクエストボディを読む前にセッションを確認し、ボディは 256 KiB までに制限します。
- 同じ IP アドレスから 5 回続けてパスワードを間違えると（ログインフォームまたは bearer）、そのアドレスは 30 秒待たされます。サーバーはパスワードを確認せずに `429` と `Retry-After` を返します。また、パスワードを間違えるたびに、応答に 0.5 秒かかります。
- 自分のネットワークの外から使う場合は、`MINER_PUBLIC_URL` を設定して、ダッシュボードを HTTPS リバースプロキシの背後に置いてください（[Docker](#docker) を参照）。
- マイナーが通信するのは、`twitch.tv`、`ttvnw.net`、`jtvnw.net`、`twitchcdn.net`、`drops.sunkwi.com` と、設定した通知先だけです。アプリ（デスクトップ版と Docker 版）は `github.com` で新しいバージョンを確認し、デスクトップアプリはプロジェクトの GitHub releases からダウンロードします。新しいバージョンには署名があり、表示されたバージョンだけがインストールされます。

## <a name="build"></a>ソースからビルド

**前提条件：** Rust、Node.js、およびお使いのシステム向けの [Tauri の前提条件](https://v2.tauri.app/start/prerequisites/)。

```bash
npm ci
npm run build          # ダッシュボード
cargo build --release -p miner-server   # サーバー：target/release/twitch-miner
npm run bundle:linux   # デスクトップ用バンドル（Tauri の Linux 依存パッケージが必要）
```

コードチェック：

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier、oxlint（型情報を利用）、svelte-check
```

インターフェースの TypeScript 型は、[ts-rs](https://github.com/Aleph-Alpha/ts-rs) によって Rust の型から `ui/src/lib/bindings` に生成されます。インターフェースが参照する型を変更したら、`npm run bindings` を実行してください。

## <a name="license"></a>ライセンス

[MIT](../../LICENSE)。[DevilXD の TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner)、[Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)、およびそれらのフォークに着想を得ています。
