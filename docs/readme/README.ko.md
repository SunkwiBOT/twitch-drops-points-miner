<p align="center">
  <img src="../../ui/public/app-icon.svg" width="112" height="112" alt="Twitch Miner 로고">
</p>

<h1 align="center">Twitch Miner</h1>

<p align="center">
  <strong>브라우저도, 열어 둔 Twitch 탭도 없이 여러 계정에서 Twitch Drops와 채널 포인트를 마이닝해요.</strong>
</p>

<p align="center">
  Linux, Windows, macOS용 데스크톱 앱이에요. 같은 웹 대시보드를 담은 Docker 이미지도 있어서 홈 서버나 Raspberry Pi에서 돌릴 수 있어요. 계정 4개일 때 서버는 메모리를 약 30 MB 쓰고 CPU는 거의 쓰지 않아요.
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-miner?label=release&color=9146FF" alt="최신 릴리스"></a>
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-miner/total?label=downloads&color=9146FF" alt="다운로드 수"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="플랫폼: Linux, Windows, macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="amd64 및 arm64용 Docker 이미지"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="MIT 라이선스"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="조회수">
</p>

<p align="center">
  <a href="#download"><strong>다운로드</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-miner/issues/new/choose">버그 신고</a>
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
  <a href="README.ja.md">日本語</a> ·
  <strong>한국어</strong> ·
  <a href="README.th.md">ไทย</a> ·
  <a href="README.vi.md">Tiếng&nbsp;Việt</a> ·
  <a href="README.id.md">Bahasa&nbsp;Indonesia</a>
</p>

<p align="center">
  <img src="../media/ko/presentation.avif" alt="Twitch Miner 소개 영상">
</p>

## <a name="features"></a>기능

다른 마이닝 도구와 다른 점을 모았어요. 항목 아래의 ‘작동 모습 보기’를 펼치면 실제로 동작하는 모습을 볼 수 있어요.

### 계정과 스트리머

- **앱 하나에 여러 계정.** 계정마다 각자의 모드로 따로 마이닝해요. **Drops와 포인트**(한 채널은 가장 좋은 캠페인, 다른 채널은 내 스트리머), **Drops만**, **포인트만** 중에서 고를 수 있어요.

  <details open><summary>작동 모습 보기</summary>
  <img src="../media/ko/accounts-mining.avif" alt="홈에서 여러 계정이 동시에 마이닝하는 모습">
  </details>

- **새 계정마다 설정 마법사.** 로그인한 뒤 기본 설정에서 시작하거나 다른 계정의 설정을 가져와요. 모드, 캠페인, 스트리머를 고른 다음 ‘마이닝 시작’이나 ‘저장하고 나중에 시작’을 누르세요.

  <details open><summary>작동 모습 보기</summary>
  <img src="../media/ko/account-setup.avif" alt="새 계정의 설정 마법사를 단계별로 진행하는 모습">
  </details>

- **변경 하나를 클릭 한 번으로 다른 계정에도.** 한 계정에서 무언가를 바꾸면 확인 메시지에 ‘다른 계정에 적용’이 나타나요. 체크한 계정에도 같은 변경이 그대로 적용되고, 나머지 설정은 그대로 유지돼요.

  <details open><summary>작동 모습 보기</summary>
  <img src="../media/ko/apply-to-accounts.avif" alt="‘다른 계정에 적용’으로 같은 변경을 다른 계정에도 적용하는 모습">
  </details>

- **스트리머 추가.** 추가한 스트리머는 기본 옵션으로 바로 마이닝을 시작해요. 기본 옵션은 앱의 기본값이거나, 이 계정에 맞게 골라 둔 옵션이에요. 필요하면 그 자리에서 조정할 수 있어요: 보너스 상자, 레이드, 모먼트, 시청 스트릭, 채팅, 예측.

  <details open><summary>작동 모습 보기</summary>
  <img src="../media/ko/streamer-add.avif" alt="Twitch 검색으로 스트리머를 추가하고 옵션을 여는 모습">
  </details>

- **스트리머 편집.** ‘편집’을 누르면 그 스트리머의 옵션이 열려요: 보너스 상자, 레이드, 모먼트, 시청 스트릭, 채팅, 예측… 스위치 하나로 목록에서 빼지 않고도 켜고 끌 수 있어요.

  <details open><summary>작동 모습 보기</summary>
  <img src="../media/ko/streamer-edit.avif" alt="스트리머 행에서 연 옵션">
  </details>

- **여러 스트리머를 한 번에 편집.** 체크한 다음 ‘선택한 스트리머 N명 편집’을 누르세요. 바꾼 옵션만 적용돼요. 스트리머마다 값이 다른 옵션은 ‘혼합’으로 표시되고, 스위치가 ‘예’와 ‘아니요’ 두 버튼으로 바뀌어요. 그다음 앱이 이 값을 새 스트리머의 기본 옵션으로 삼을지 물어봐요.

  <details open><summary>작동 모습 보기</summary>
  <img src="../media/ko/streamers-group-edit.avif" alt="‘혼합’ 옵션이 있는 여러 스트리머 편집">
  </details>

- **스트리머 가져오기.** 목록을 붙여 넣거나(최대 500명), 다른 계정의 목록이나 이 계정이 팔로우하는 채널을 그대로 가져와요. 목록을 확인해서 실제로 있는 채널만 추가해요.

  <details open><summary>작동 모습 보기</summary>
  <img src="../media/ko/streamers-import.avif" alt="스트리머 가져오기: 붙여 넣은 목록, 다른 계정, 팔로우한 채널">
  </details>

- **Spam Points.** 채널의 사용자 지정 보상을 여러 번 연속으로 교환해요. 포인트 목록에서 채널을 고르거나 Twitch 검색으로 찾은 아무 채널이나 고르면, 그 채널의 보상이 계정 잔액과 함께 표시돼요. 잔액은 실시간으로 다시 읽어요. 수량을 정하고(최대 25개, 잔액 범위 안에서) ‘교환’을 누르세요. 연속 교환이 끝나면 결과가 표시되고, 연속 교환마다 ‘일지’에 한 줄씩 남아요.

  <details open><summary>작동 모습 보기</summary>
  <img src="../media/ko/spam-points.avif" alt="Spam Points: 채널을 고르고 메시지를 넣는 보상을 10번 교환하자 교환 내역이 채널의 Twitch 채팅에 올라오고, 일지에 남은 줄을 보는 모습">
  </details>

### Drops

- **진행 중인 모든 Drops 캠페인.** 캠페인마다 상세 정보가 있어요: 보상, 게임 계정 연동, ‘먼저 마이닝’. 목록은 [Twitch Drops API](https://drops.sunkwi.com) 프로젝트에서 가져오고, 10분마다 새로 고쳐져요.

  <details open><summary>작동 모습 보기</summary>
  <img src="../media/ko/drops-catalogue.avif" alt="달력 형태의 카탈로그, 검색, 캠페인 상세 정보">
  </details>

- **내가 정하는 마이닝 순서.** 캠페인을 원하는 순서로 끌어다 놓으세요. 새 캠페인은 알아서 자리를 잡아요. 우선 게임이 먼저, 그다음은 가장 빨리 끝나거나 남은 시간(분)이 가장 적은 캠페인 순이에요. 게임, 단어, 캠페인, 개별 보상을 제외할 수 있고, 기한 안에 끝낼 수 없는 캠페인은 건너뛸 수 있어요. 캠페인 상세 정보에서 ‘먼저 마이닝’을 누르면 맨 위로 올라가요.

  <details open><summary>작동 모습 보기</summary>
  <img src="../media/ko/drops-order.avif" alt="마이닝 순서, 우선 게임, 제외 항목">
  </details>

- **인벤토리와 ‘모두 받기’.** ‘인벤토리’ 페이지는 보상을 ‘받을 수 있음’, ‘진행 중’, ‘수령함’으로 나눠 보여 줘요. 하나씩 받거나, 한 번에 모두 받거나, ‘드롭 자동 수령’을 켤 수 있어요.

  <details open><summary>작동 모습 보기</summary>
  <img src="../media/ko/drops-inventory.avif" alt="인벤토리와 ‘모두 받기’ 버튼">
  </details>

- **‘Drops에서 내 스트리머 우선’.** 현재 캠페인에서, 내 목록에 있는 스트리머의 방송이 그 캠페인에 포함되면 다른 채널보다 먼저 시청해요. 같은 채널에서 드롭도 진행되고 그 채널의 채널 포인트도 쌓여요. 기본으로 켜져 있어요.

  <details open><summary>작동 모습 보기</summary>
  <img src="../media/ko/drops-streamers-first.avif" alt="목록의 스트리머를 Drops와 그 채널의 포인트를 위해 시청하는 모습">
  </details>

### 기록

- **모든 일과 모든 요청을 기록.** ‘일지’에는 모든 이벤트가 남아요: 드롭, 얻고 쓴 포인트, 상자, 베팅, 레이드, 방송 시작… 계정, 탭(Drops, 포인트, 기타), 유형별로 필터링할 수 있어요. **기술 로그 › 요청**에는 Twitch와 다른 서비스로 보낸 모든 요청이 결과와 걸린 시간과 함께 표시돼요. 로그와 요청에는 토큰이나 비밀 정보가 전혀 없어요. ‘버그 리포트용으로 복사’로 복사해서 [GitHub](https://github.com/SunkwiBOT/twitch-miner/issues)나 [Discord](https://discord.gg/support-bots-891685895781113886)의 버그 리포트에 바로 붙여 넣으면 돼요.

  <details open><summary>작동 모습 보기</summary>
  <img src="../media/ko/journal-requests.avif" alt="필터링한 일지와 기술 로그 › 요청">
  </details>

### 데스크톱이든 서버든

- **어디서나 같은 인터페이스.** 데스크톱 앱(Linux, Windows, macOS)과 Docker 이미지는 같은 대시보드를 보여 줘요. Docker는 VPS, NAS, Raspberry Pi에서 돌아가고, 그 기기에는 화면도 브라우저도 필요 없어요. 대시보드는 어떤 브라우저에서든 열려요.
  휴대폰에서는 대시보드가 화면에 맞춰지고, HTTPS로 열면 앱처럼 설치할 수 있어요.
- **가벼움.** 계정 4개에 메모리 약 30 MB, CPU는 거의 쓰지 않고, Docker 이미지는 약 12 MB예요.

### 보안과 업데이트

- **세션을 안전하게.** Twitch 비밀번호는 필요 없어요. 내 브라우저에서 `twitch.tv/activate`로 로그인해요. 세션은 디스크에 암호화되어 저장되고, [Twitch 설정](https://www.twitch.tv/settings/connections)에서 해지할 수 있어요. 데스크톱에서는 선택 사항인 앱 비밀번호로 앱을 잠글 수 있고, 비밀번호를 입력하기 전에는 아무것도 마이닝하지 않아요. Docker에서는 대시보드에 별도의 비밀번호가 있고, 몇 분 동안 활동이 없으면 스스로 잠기며, 프록시 뒤에서 HTTPS로 동작해요. 비밀번호를 연속 5번 틀리면 그 IP 주소는 30초 동안 차단돼요.

  <details open><summary>작동 모습 보기</summary>
  <img src="../media/ko/security-lock.avif" alt="자물쇠: 잠긴 앱을 비밀번호로 잠금 해제하는 모습">
  </details>

- **서명된 업데이트, 자동 설치는 없어요.** 상단 바에 버전 번호가 붙은 ‘업데이트’ 배지가 나타나요. ‘설치 후 다시 시작’을 누르면 먼저 확인을 요청하고, 그다음 앱이 그 버전을 설치하고 다시 시작해요. 앱 비밀번호를 설정했다면 잠긴 상태로 다시 시작하니, 비밀번호를 다시 입력하면 마이닝이 이어져요. 프로젝트가 서명한 버전만 설치돼요. Docker 버전도 실행할 명령어와 함께 알려 줘요.

  <details open><summary>작동 모습 보기</summary>
  <img src="../media/ko/updates.avif" alt="‘업데이트’ 배지, 확인, 그리고 설치">
  </details>

### 그 밖에

- 다른 마이닝 도구처럼 지원하는 기능: 보너스 상자, 레이드, 모먼트, 커뮤니티 목표, 예측(전략, 베팅 시점, 조건), 시청 스트릭과 구독 채널 우선, 채팅 접속(끌 수 있음), 새 스트리머의 기본 옵션, 이미지가 포함된 Discord, Telegram, 웹훅 알림, 전체 또는 계정별 프록시.
- **곧 지원: 배지 채굴.** 시청으로 얻는 Twitch 배지를 Drops처럼 채굴해요.
- 29개 언어, 다크·라이트 테마, 데스크톱 알림, 트레이 아이콘, 시스템 시작 시 실행.

  <details open><summary>작동 모습 보기</summary>
  <img src="../media/ko/basics.avif" alt="언어, 라이트·다크 테마, 데스크톱 알림, 트레이 아이콘">
  </details>

> [!WARNING]
> 이 프로젝트는 Twitch와 제휴 관계가 없어요. 계정을 자동화하면 Twitch 약관이나 캠페인 규칙을 위반할 수 있어요.

## 목차

- [비교](#comparison)
- [다운로드](#download)
- [Docker](#docker)
- [작동 방식](#how-it-works)
- [자주 묻는 질문](#faq)
- [관련 프로젝트](#related-projects)
- [보안](#security)
- [소스에서 빌드](#build)
- [라이선스](#license)

## <a name="comparison"></a>비교

[Twitch Miner](https://github.com/SunkwiBOT/twitch-miner)를 [Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)(채널 포인트), [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner)(Drops)와 비교하면 다음과 같아요. 2026년 11월에 각 프로젝트의 코드와 README로 확인했어요.

| | 이 프로젝트 | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Drops 캠페인: 찾기, 마이닝, 수령 | ✅ | ❌ ¹ | ✅ |
| 채널 포인트: 상자, 레이드, 모먼트, 예측 | ✅ | ✅ | ❌ |
| Spam Points: 채널의 사용자 지정 보상을 여러 번 연속으로 교환 | ✅ | ❌ | ❌ |
| 한 계정에서 Drops와 채널 포인트를 동시에 | ✅ | 일부 ¹ | ❌ |
| 앱 하나에서 여러 계정 | ✅ | ❌ ² | ❌ |
| 여러 계정의 설정을 한 번에 변경 | ✅ | ❌ | ❌ |
| 그래픽 인터페이스 | ✅ 데스크톱과 웹 ³ | ❌ ⁴ | ✅ 데스크톱 |
| 홈 서버용 웹 대시보드(Docker), 비밀번호 보호, HTTPS로 접속 가능 | ✅ | ❌ ⁴ | ❌ |
| 디스크에 암호화된 세션, 선택형 앱 비밀번호 | ✅ | ❌ | ❌ |
| 29개 언어 인터페이스 | ✅ | ❌ | 일부 ⁵ |
| 앱 안에서 받는 서명된 업데이트(데스크톱) | ✅ | ❌ | ❌ |
| 새 계정마다 설정 마법사 | ✅ | ❌ ⁴ | ❌ |
| 인터페이스에서 스트리머 추가·편집(하나씩 또는 여러 명 한 번에) | ✅ | ❌ ⁴ | — |
| 검색, 목록, 다른 계정, 팔로우한 채널에서 스트리머 가져오기 | ✅ | 일부 ⁶ | — |
| 이미지가 포함된 Discord, Telegram, 웹훅 알림 | ✅ | 일부 ⁷ | ❌ |
| 모든 Drops 캠페인 달력과 검색 | ✅ | ❌ | 일부 ⁸ |
| 캠페인별 마이닝 순서, 제외 단어, 무시한 캠페인이나 보상 | ✅ | ❌ | ❌ ⁹ |
| 캠페인 Drops를 위해 내 목록의 스트리머를 먼저 시청 | ✅ | 일부 ¹⁰ | ❌ |
| 인터페이스 안에서 필터링할 수 있는 이벤트 일지와 모든 요청 목록 | ✅ | ❌ ⁴ | ❌ |
| 메모리 약 30 MB(Docker 서버, 계정 4개) | ✅ | — ¹¹ | — ¹¹ |

¹ 자기 목록의 스트리머만 시청해요. 그 방송의 Drops를 진행하고 수령하지만, 캠페인을 고르지는 않아요.\
² 계정마다 프로세스 하나.\
³ PC에서 데스크톱 앱으로, 그리고/또는 Docker로 서버(VPS, NAS, Raspberry Pi)에서 실행할 수 있어요. 그 기기에는 그래픽 환경도 브라우저도 필요 없고, 대시보드는 어떤 브라우저에서든 열려요.\
⁴ Python 파일로 설정해요. 웹 페이지는 포인트 차트와 로그 파일을 보여 줘요.\
⁵ 21개 언어.\
⁶ 계정이 팔로우하는 채널을 대상으로 삼을 수 있어요.\
⁷ 이미지 없이 텍스트만 보내요. Matrix, Pushover, Gotify도 지원하는데, 이 프로젝트에는 없어요.\
⁸ 필터가 있는 캠페인 목록으로, 달력이나 검색은 없어요.\
⁹ 우선 게임과 제외 게임이 있고, 게임 이름으로 골라요.\
¹⁰ 기본적으로 목록의 스트리머 중 방송에 Drops가 걸린 스트리머를 먼저 시청하지만(`DROPS` 우선순위), 다른 채널은 절대 시청하지 않아요.\
¹¹ 측정하지 않았어요.

## <a name="download"></a>다운로드

[최신 릴리스](https://github.com/SunkwiBOT/twitch-miner/releases/latest)에서 내 시스템에 맞는 파일을 내려받으세요.

| 시스템 | 패키지 |
|---|---|
| **Windows** · x64 | 설치 프로그램(`setup.exe`) 또는 [포터블 실행 파일](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/TwitchMiner-portable.exe) |
| **macOS** 10.15 이상 · Intel 및 Apple Silicon | 디스크 이미지(`.dmg`) |
| **Linux** · x86-64 | `.AppImage`, `.deb`(Debian, Ubuntu) 또는 `.rpm`(Fedora, openSUSE) |

macOS 빌드는 공증을 받지 않았어요. 처음 실행할 때는 앱을 마우스 오른쪽 버튼으로 클릭하고 **열기**를 선택하세요.

업데이트는 서명되어 있어요. 앱은 시작할 때 새 버전이 있는지 확인하고, 그 뒤로는 한 시간마다 확인해요(확인 주기는 **설정 › 정보**에서 바꿀 수 있어요). 업데이트를 스스로 설치하지는 않아요. 화면에 뜨는 메시지나 **설정 › 정보**에서 클릭 한 번으로 설치하면 돼요.

데이터는 사용자 데이터 폴더에 저장돼요:

- Linux: `~/.local/share/twitch-miner`
- Windows: `%APPDATA%\twitch-miner`
- macOS: `~/Library/Application Support/twitch-miner`

## <a name="docker"></a>Docker

서버는 같은 대시보드를 브라우저에서 보여 줘요. 이미지는 `ghcr.io/sunkwibot/twitch-miner`이고, `amd64`와 `arm64`를 지원해요. 새 버전이 나오면 대시보드가 알려 줘요. `docker compose pull`을 실행한 다음 `docker compose up -d`를 실행하세요.

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-miner:latest
    pull_policy: always   # 시작할 때마다 최신 이미지 받기
    restart: unless-stopped
    ports: ["8080:8080"]   # HTTPS 리버스 프록시 뒤에 둘 때: 이 줄을 지우거나 "127.0.0.1:8080:8080"으로 변경
    environment:
      MINER_PASSWORD: "change-me"   # 필수, 12자 이상
      # MINER_PUBLIC_URL: "https://miner.example.com"   # HTTPS 리버스 프록시 뒤에 둘 때만
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

<http://localhost:8080>(또는 내 네트워크에서 이 기기의 주소)를 열고 비밀번호를 입력하세요.

### 변수

| 변수 | 기본값 | 용도 |
|---|---|---|
| `MINER_PASSWORD` | 없음 | 대시보드 비밀번호. 필수예요. 없으면 서버가 시작을 거부해요. 12자 이상이어야 해요(더 짧으면 시작할 때 경고가 표시돼요). 예시 값 `change-me`는 거부돼요. |
| `MINER_PASSWORD_FILE` | 없음 | `MINER_PASSWORD`를 파일에서 읽어요(예: Docker secret). 둘 중 하나만 설정하세요. |
| `MINER_PUBLIC_URL` | 없음 | HTTPS 리버스 프록시(Caddy, nginx, Traefik, Cloudflare Tunnel 등) 뒤에 둘 때만 써요. 대시보드의 `https://` 주소로, 경로 없이 `https://miner.example.com`처럼 써요. 프록시는 `X-Forwarded-Proto`, `X-Forwarded-For`, 웹소켓을 전달해야 하고(흔히 쓰는 프록시는 기본으로 그렇게 해요), 컨테이너의 포트에 직접 접속할 수 없어야 해요. 그러면 대시보드는 프록시를 거쳐 HTTPS로 들어오지 않은 접속을 모두 거부하고, 비밀번호를 5번 틀린 주소를 30초 동안 차단해요. 프록시는 같은 기기나 그 로컬 네트워크에서 실행해야 해요. 공인 주소에서 온 연결은 거부돼요. |
| `MINER_TRUSTED_HOPS` | `1` | `MINER_PUBLIC_URL`을 쓸 때만: `X-Forwarded-For`에 항목을 추가하는 프록시 수로, 1~10이에요. 프록시 두 개가 앞뒤로 놓여 있으면(예: Traefik 앞에 Cloudflare) `2`예요. |
| `MINER_PROXY_NETWORKS` | 없음 | `MINER_PUBLIC_URL`과 함께일 때만: 이 기기와 그 사설 로컬 네트워크 외에 프록시가 접속할 수 있는 다른 네트워크를 쉼표로 구분해 지정해요. 단일 주소나 `2001:db8:1::/64` 같은 네트워크로, IPv6 주소가 글로벌인 로컬 네트워크에 써요. |
| `MINER_VAULT_PASSPHRASE` | 없음 | 앱 비밀번호를 설정한 데스크톱 앱에서 복사한 데이터 폴더에만 필요해요. 시작할 때 잠금을 해제해요. 처음부터 새로 만든 서버에는 필요 없어요. |
| `MINER_VAULT_PASSPHRASE_FILE` | 없음 | `MINER_VAULT_PASSPHRASE`를 파일에서 읽어요. 둘 중 하나만 설정하세요. |
| `DATA_DIR` | 이미지에서는 `/data`, 그 외에는 작업 디렉터리의 `data` | 데이터베이스, 키 파일, 로그. |
| `LOG_DIR` | `$DATA_DIR/logs` | 로그 파일: 하루에 하나씩, 최근 14개 보관. |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | 수신 주소: IP 주소와 1~65535 사이의 포트. |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | 로그 레벨. |

서버는 시작할 때 변수를 한 번 읽고 검사해요. 값이 잘못되면 조용히 기본값으로 넘어가지 않고, 메시지를 남기며 서버가 멈춰요.

## <a name="how-it-works"></a>작동 방식

각 계정은 20초마다 두 채널을 골라요. 먼저 대기열 맨 위 캠페인에 가장 알맞은 라이브 채널을, 그다음 포인트 목록의 스트리머를 골라요.

Twitch는 계정마다 동시에 **Drops 캠페인 하나**와 **포인트용 채널 두 개**만 인정해요. 2026년 10월부터는 게임 계정이 연동되어 있어야 드롭을 수령할 수 있어요. 연동되지 않은 캠페인도 진행도는 쌓이고, 대시보드에 연동 링크가 표시돼요.

캠페인 목록은 [Twitch Drops API](https://drops.sunkwi.com) 프로젝트에서 가져와요(10분마다 갱신). Twitch가 TV 클라이언트에는 이 목록을 숨기기 때문이에요.

## <a name="faq"></a>자주 묻는 질문

### 위험과 Twitch

**계정이 정지될 수 있나요?**\
그렇지 않다고 장담할 수 있는 사람은 없어요. 이 프로젝트는 Twitch와 제휴 관계가 없고, 계정을 자동화하면 Twitch 약관이나 캠페인 규칙을 위반할 수 있어요. 마이닝 도구는 계정당 최대 두 채널(Twitch의 한도)만 시청하고, 영상은 전혀 내려받지 않아요. 사용 여부는 스스로 판단하세요.

**영상을 정말로 시청하나요?**\
아니요, 영상은 내려받지 않아요. Drops의 경우, 마이닝 도구가 15초마다 방송의 재생 목록을 가장 낮은 화질로 읽고, 새 세그먼트마다 헤더만 요청해요. 포인트의 경우, Twitch의 ‘1분 시청’ 신호를 1분마다 보내요. 시청 중인 채널마다 요청한 세그먼트 수가 표시돼요.

**왜 최대 두 채널인가요?**\
Twitch의 한도라서 그래요. Twitch는 계정마다 동시에 시청하는 채널 두 개까지만, 그리고 Drops 캠페인은 한 번에 하나만 인정해요. **Drops와 포인트** 모드에서는 한 채널이 캠페인을, 다른 채널이 내 스트리머를 맡아요. **계정 설정**의 ‘채널 하나’ 설정을 쓰면 더 조용하게 동작해요.

### Drops

**캠페인이 마이닝되지 않거나 드롭이 진행되지 않아요.**\
그 계정의 **Drops** 페이지를 여세요. ‘마이닝 순서’에서 대기 중인 캠페인마다 이유가 표시돼요: ‘라이브 채널 없음’, ‘대기 중 (한 번에 한 캠페인)’, ‘아직 받을 수 있는 드롭 없음’. 이 목록에 없는 캠페인은 제외된 거예요. ‘어떤 캠페인’ 설정, **제외** 탭의 규칙, 또는 기한 안에 끝낼 수 없다는 이유 때문이에요. 구독이 필요한 보상은 마이닝하지 않고, 다른 드롭에 이어지는 드롭은 앞의 드롭이 끝날 때까지 기다려요. **포인트만** 모드에서는 마이닝 도구가 캠페인을 고르지 않아요. 캠페인 상세 정보에서 ‘먼저 마이닝’을 누르면 맨 위로 올라가고, 무시한 캠페인은 **제외** 탭의 ‘다시 마이닝’으로 되돌릴 수 있어요.

**게임 계정을 연동해야 하나요?**\
수령하려면 그래요. 2026년 10월부터 Twitch는 게임 계정이 연동되어 있을 때만 드롭을 수령할 수 있게 해요. 기본적으로 연동되지 않은 캠페인도 진행도는 쌓이고, 수령은 연동할 때까지 기다려요. ‘연동 안 됨’ 배지로 알 수 있고, 캠페인 상세 정보의 ‘게임 계정 연동’을 누르면 연동 페이지가 열려요. 연동된 게임만 마이닝하려면 ‘계정이 연동된 게임만’을 고르세요.

### 설치와 데이터

**데스크톱 앱과 Docker 중 무엇이 좋을까요?**\
데스크톱 앱은 PC가 켜져 있는 동안 마이닝하고, 트레이 아이콘, 선택형 비밀번호, 클릭 한 번 업데이트를 갖추고 있어요. Docker는 항상 켜져 있는 기기(VPS, NAS, Raspberry Pi)에서 밤낮없이 마이닝하고, 브라우저로 열며 `MINER_PASSWORD`로 보호해요. 인터페이스는 같아요.

**데이터는 어디에 있나요?**\
데이터 폴더에 있어요. Linux는 `~/.local/share/twitch-miner`, Windows는 `%APPDATA%\twitch-miner`, macOS는 `~/Library/Application Support/twitch-miner`, Docker는 `/data` 볼륨이에요. 데스크톱에서는 **설정 › 데스크톱**에서 클릭 한 번으로 열 수 있어요. 안에는 `miner.db`(계정, 설정, 일지), `master.key`(앱 비밀번호를 설정하지 않았을 때의 세션 키), 로그가 있어요.

**업데이트는 어떻게 하나요?**\
데스크톱 앱은 시작할 때 새 버전이 있는지 확인하고, 그 뒤로는 한 시간마다 확인해요(**설정 › 정보**에서 바꿀 수 있어요). ‘업데이트’ 배지를 누른 다음 ‘설치 후 다시 시작’을 누르세요. Docker에서는 대시보드가 알려 줘요. `docker compose pull`을 실행한 다음 `docker compose up -d`를 실행하세요.

**앱 비밀번호를 잊어버렸어요.**\
복구할 수 없어요. Twitch 세션을 암호화하는 것이 바로 이 비밀번호이기 때문이에요. 앱을 종료하고 데이터 폴더를 삭제한 뒤 계정을 다시 추가하세요. 계정 설정도 다시 해야 해요. Docker에서는 대시보드 비밀번호를 `MINER_PASSWORD`에서 바꾼 다음 `docker compose up -d`를 실행하세요.

### 포인트와 예측

**한 시간에 포인트를 얼마나 얻나요?**\
Twitch와 채널마다 달라요. 마이닝 도구는 Twitch가 시청자에게 주는 만큼 얻어요: 시청 시간 포인트, 보너스 상자(약 15분마다 하나), 시청 스트릭 보너스. 구독하면 티어에 따라 포인트가 ×1.2에서 ×2까지 늘어나요. 얻은 포인트는 홈에, 하나하나의 획득 내역은 일지에 표시돼요.

**베팅은 위험한가요?**\
네, 포인트를 잃을 수 있어요. 예측은 기본으로 꺼져 있고, 스트리머마다 따로 설정해요. 전략, 베팅 시점, 베팅액(기본값은 잔액의 5%, 최대 50,000포인트), 최소 잔액을 정하고, 원하면 배당이 1.5 이상일 때처럼 조건을 걸 수 있어요. 베팅액은 작게 유지하세요.

**스트리머가 시청되지 않아요.**\
켜져 있고 라이브 중인 스트리머만, 그것도 비어 있는 채널이 있을 때만 시청해요. **Drops와 포인트** 모드에서는 두 채널 중 하나가 현재 캠페인을 맡고, **Drops만** 모드에서는 스트리머를 시청하지 않으며, 일시정지된 계정은 아무것도 시청하지 않아요. 채널 자리는 다음 순서로 배정되고, 시청 중인 채널마다 배지로 표시돼요: ‘스트릭 보너스’, ‘스트릭 위험’, ‘구독 중’, 그다음 ‘목록 순서’. 그 스트리머를 목록에서 위로 올리거나, ‘시청 스트릭 우선’과 ‘구독 채널 우선’을 끄세요.

## <a name="related-projects"></a>관련 프로젝트

같은 제작자의 다른 프로젝트:

- [**Twitch Drops API**](https://drops.sunkwi.com): 진행 중인 모든 Twitch Drops와 보상 캠페인을 담은 무료 공개 API로, 매분 갱신돼요. 이 앱은 캠페인 카탈로그를 여기서 읽어요.
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com): 내 서버에 Twitch 라이브와 Drops 캠페인을 알려 주는 Discord 봇이에요.
- [**Kick Alerts 🔔**](https://kick.sunkwi.com): 내 서버에 Kick 라이브와 Drops 캠페인을 알려 주는 Discord 봇이에요.

## <a name="security"></a>보안

- 토큰은 AES-256-GCM으로 봉인돼요. 키는 데이터베이스 옆에 있는 무작위 `master.key`(기본값, 따로 할 일 없음)이거나, 앱 비밀번호에서 Argon2id로 유도한 키예요.
- HTTP 대시보드는 상수 시간 비밀번호 비교와 HttpOnly, SameSite=Strict 쿠키(HTTPS 뒤에서는 `Secure`, `__Host-` 접두사 포함)를 사용해요. 서버는 각 세션의 SHA-256만 저장해요. 세션은 30일이 지나거나 로그아웃하면 끝나요. 서버는 요청 본문을 읽기 전에 세션을 확인하고, 본문 크기를 256 KiB로 제한해요.
- 한 IP 주소에서 비밀번호를 연속 5번 틀리면(로그인 폼 또는 bearer) 그 주소는 30초 동안 기다려야 해요. 서버는 비밀번호를 확인하지 않고 `Retry-After`와 함께 `429`를 응답해요. 비밀번호가 틀릴 때마다 응답에 0.5초가 걸리기도 해요.
- 내 네트워크 밖에서는 `MINER_PUBLIC_URL`을 설정하고 대시보드를 HTTPS 리버스 프록시 뒤에 두세요([Docker](#docker) 참고).
- 마이닝 도구는 `twitch.tv`, `ttvnw.net`, `jtvnw.net`, `twitchcdn.net`, `drops.sunkwi.com`, 그리고 직접 설정한 알림 대상하고만 통신해요. 앱(데스크톱과 Docker)은 `github.com`에서 새 버전을 확인하고, 데스크톱 앱은 프로젝트의 GitHub releases에서 새 버전을 내려받아요. 새 버전은 서명되어 있어서, 화면에 표시된 버전만 설치돼요.

## <a name="build"></a>소스에서 빌드

**사전 요구 사항:** Rust, Node.js, 그리고 내 시스템에 맞는 [Tauri 사전 요구 사항](https://v2.tauri.app/start/prerequisites/).

```bash
npm ci
npm run build          # 대시보드
cargo build --release -p miner-server   # 서버: target/release/twitch-miner
npm run bundle:linux   # 데스크톱 번들(Tauri Linux 의존성 필요)
```

코드 검사:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier, oxlint(타입 인식), svelte-check
```

인터페이스의 TypeScript 타입은 [ts-rs](https://github.com/Aleph-Alpha/ts-rs)로 Rust 타입에서 `ui/src/lib/bindings`에 생성돼요. 인터페이스가 쓰는 타입을 바꾼 뒤에는 `npm run bindings`를 실행하세요.

## <a name="license"></a>라이선스

[MIT](../../LICENSE). [DevilXD의 TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner), [Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)와 그 포크들에서 영감을 받았어요.
