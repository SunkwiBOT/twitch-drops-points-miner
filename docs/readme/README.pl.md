<p align="center">
  <img src="../../ui/public/app-icon.svg" width="112" height="112" alt="Logo Twitch Miner">
</p>

<h1 align="center">Twitch Miner</h1>

<p align="center">
  <strong>Kopie Twitch Drops i punkty kanału na wielu kontach, bez przeglądarki i bez otwartej karty Twitcha.</strong>
</p>

<p align="center">
  Aplikacja desktopowa na Linuksa, Windows i macOS albo obraz Dockera z tym samym panelem WWW, do uruchomienia na domowym serwerze lub na Raspberry Pi. Przy 4 kontach serwer zużywa około 30 MB pamięci i prawie nie obciąża procesora.
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-miner?label=release&color=9146FF" alt="Najnowsze wydanie"></a>
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-miner/total?label=downloads&color=9146FF" alt="Pobrania"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="Platformy: Linux, Windows, macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="Obraz Dockera dla amd64 i arm64"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="Licencja MIT"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="Wyświetlenia">
</p>

<p align="center">
  <a href="#download"><strong>Pobierz</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-miner/issues/new/choose">Zgłoś błąd</a>
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
  <strong>Polski</strong> ·
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
  <img src="../media/pl/presentation.avif" alt="Prezentacja wideo Twitch Miner">
</p>

## <a name="features"></a>Funkcje

Co wyróżnia go na tle innych minerów. „Zobacz w akcji” pod każdym punktem pokazuje, jak to
działa.

### Konta i streamerzy

- **Wiele kont w jednej aplikacji.** Każde konto kopie samodzielnie, we własnym trybie:
  **Drops i punkty** (jeden kanał dla najlepszej kampanii, drugi dla Twoich streamerów),
  **Tylko Drops** lub **Tylko punkty**.

  <details open><summary>Zobacz w akcji</summary>
  <img src="../media/pl/accounts-mining.avif" alt="Kilka kont kopiących jednocześnie na stronie Start">
  </details>

- **Kreator dla każdego nowego konta.** Po zalogowaniu zacznij od ustawień domyślnych albo
  od ustawień innego konta. Wybierz tryb, kampanie i streamerów, a potem „Zacznij kopać”
  lub „Zapisz i uruchom później”.

  <details open><summary>Zobacz w akcji</summary>
  <img src="../media/pl/account-setup.avif" alt="Kreator konfiguracji nowego konta, krok po kroku">
  </details>

- **Jedna zmiana przeniesiona na inne konta jednym kliknięciem.** Gdy zmienisz coś na
  koncie, komunikat potwierdzenia proponuje „Zastosuj do innych kont”. Ta sama zmiana trafia
  na zaznaczone konta, a reszta ich ustawień pozostaje bez zmian.

  <details open><summary>Zobacz w akcji</summary>
  <img src="../media/pl/apply-to-accounts.avif" alt="Zmiana powtórzona na innych kontach przez „Zastosuj do innych kont”">
  </details>

- **Dodawanie streamera.** Od razu rusza z opcjami domyślnymi: aplikacji albo tymi
  wybranymi przez Ciebie dla tego konta. W razie potrzeby zmień je na miejscu: skrzynki
  bonusowe, rajdy, momenty, seria oglądania, czat, prognozy.

  <details open><summary>Zobacz w akcji</summary>
  <img src="../media/pl/streamer-add.avif" alt="Dodawanie streamera przez wyszukiwarkę Twitcha, a potem jego opcje">
  </details>

- **Edycja streamera.** „Edytuj” otwiera jego opcje: skrzynki bonusowe, rajdy, momenty,
  seria oglądania, czat, prognozy… Przełącznik włącza go lub wyłącza bez usuwania z listy.

  <details open><summary>Zobacz w akcji</summary>
  <img src="../media/pl/streamer-edit.avif" alt="Opcje streamera otwarte z jego wiersza">
  </details>

- **Edycja wielu streamerów naraz.** Zaznacz ich, a potem
  „Edytuj N zaznaczonych streamerów”. Stosowane są tylko opcje, które zmienisz. Opcja,
  która się między nimi różni, pokazuje „Mieszane”, a jej przełącznik zamienia się w dwa
  przyciski: „Tak” i „Nie”. Potem aplikacja proponuje, by uczynić je opcjami domyślnymi
  dla nowych streamerów.

  <details open><summary>Zobacz w akcji</summary>
  <img src="../media/pl/streamers-group-edit.avif" alt="Edycja kilku streamerów z opcjami „Mieszane”">
  </details>

- **Import streamerów.** Wklej listę (do 500 nazw) albo weź listę innego konta lub kanały,
  które obserwuje konto. Lista jest sprawdzana, więc dodawane są tylko istniejące kanały.

  <details open><summary>Zobacz w akcji</summary>
  <img src="../media/pl/streamers-import.avif" alt="Import streamerów: wklejona lista, inne konto, obserwowane kanały">
  </details>

- **Spam Points.** Odbierz niestandardową nagrodę kanału kilka razy z rzędu. Wybierz kanał
  ze swojej listy Punktów albo dowolny kanał znaleziony wyszukiwarką Twitcha: jego nagrody
  pojawiają się razem z saldem konta, odczytywanym na bieżąco. Ustaw liczbę (do 25,
  w granicach salda), a potem „Odbierz”. Wynik pojawia się po zakończeniu serii, a każda
  seria ma swój wiersz w Dzienniku.

  <details open><summary>Zobacz w akcji</summary>
  <img src="../media/pl/spam-points.avif" alt="Spam Points: wybrany kanał, 10 × odebrana nagroda z wiadomością, odbiory pojawiają się na czacie kanału na Twitchu, a potem wiersz w Dzienniku">
  </details>

### Drops

- **Wszystkie trwające kampanie Drops.** Każda kampania ma własną kartę szczegółów: nagrody,
  połączenie konta gry, „Kop najpierw”. Lista pochodzi z projektu
  [Twitch Drops API](https://drops.sunkwi.com) i odświeża się co 10 minut.

  <details open><summary>Zobacz w akcji</summary>
  <img src="../media/pl/drops-catalogue.avif" alt="Katalog w widoku kalendarza, wyszukiwarka i szczegóły kampanii">
  </details>

- **Twoja kolejność kopania.** Przeciągnij kampanie w wybranej kolejności. Nowe same
  znajdują swoje miejsce: najpierw Twoje gry priorytetowe, potem najbliższy koniec albo
  najmniej pozostałych minut. Wykluczaj gry, słowa, kampanię albo pojedynczą nagrodę
  i pomijaj kampanie, których nie da się ukończyć na czas. „Kop najpierw” w szczegółach
  kampanii przenosi ją na samą górę.

  <details open><summary>Zobacz w akcji</summary>
  <img src="../media/pl/drops-order.avif" alt="Kolejność kopania, gry priorytetowe i wykluczenia">
  </details>

- **Ekwipunek i „Odbierz wszystkie”.** Strona Ekwipunek dzieli nagrody na „Do odebrania”,
  „W toku” i „Odebrane”. Odbieraj je pojedynczo, wszystkie naraz albo włącz
  „Odbieraj dropy automatycznie”.

  <details open><summary>Zobacz w akcji</summary>
  <img src="../media/pl/drops-inventory.avif" alt="Ekwipunek i przycisk „Odbierz wszystkie”">
  </details>

- **„Najpierw moi streamerzy dla Drops”.** W bieżącej kampanii streamer z Twojej listy,
  którego transmisja liczy się do tej kampanii, ma pierwszeństwo przed innymi kanałami.
  Jeden i ten sam kanał posuwa drop naprzód i przynosi Ci swoje punkty kanału. Domyślnie
  włączone.

  <details open><summary>Zobacz w akcji</summary>
  <img src="../media/pl/drops-streamers-first.avif" alt="Streamer z listy oglądany dla Drops i dla swoich punktów">
  </details>

### Monitorowanie

- **Dziennik wszystkiego i każde żądanie.** Dziennik zapisuje każde zdarzenie: dropy,
  zdobyte i wydane punkty, skrzynki, zakłady, rajdy, rozpoczęte transmisje… Filtruj go
  według konta, karty (Drops, Punkty, Inne) i typu. **Logi techniczne › Żądania** wyświetla
  każde żądanie wysłane do Twitcha i innych usług, z wynikiem i czasem trwania. Logi
  i żądania nie zawierają żadnego tokenu ani sekretu: „Kopiuj do zgłoszenia błędu” kopiuje
  je, gotowe do wklejenia w zgłoszeniu błędu na
  [GitHubie](https://github.com/SunkwiBOT/twitch-miner/issues) lub
  [Discordzie](https://discord.gg/support-bots-891685895781113886).

  <details open><summary>Zobacz w akcji</summary>
  <img src="../media/pl/journal-requests.avif" alt="Przefiltrowany Dziennik, a potem Logi techniczne › Żądania">
  </details>

### Na komputerze lub na serwerze

- **Ten sam interfejs wszędzie.** Aplikacja desktopowa (Linux, Windows, macOS) i obraz
  Dockera pokazują ten sam panel. Docker działa na VPS-ie, NAS-ie lub Raspberry Pi, bez
  ekranu i przeglądarki na tej maszynie: panel otwiera się w dowolnej przeglądarce.
- **Lekki.** Około 30 MB pamięci przy 4 kontach, prawie zerowe użycie procesora i obraz
  Dockera o rozmiarze około 12 MB.

### Bezpieczeństwo i aktualizacje

- **Twoje sesje pod ochroną.** Bez hasła do Twitcha: logujesz się na `twitch.tv/activate`
  we własnej przeglądarce. Sesje są zaszyfrowane na dysku i możesz je unieważnić
  w [ustawieniach Twitcha](https://www.twitch.tv/settings/connections). Na komputerze
  opcjonalne hasło aplikacji ją blokuje: nic nie jest kopane, dopóki go nie wpiszesz.
  W Dockerze panel ma własne hasło, sam się blokuje po kilku minutach bezczynności i działa
  przez HTTPS za proxy. Pięć błędnych haseł z rzędu blokuje adres IP na 30 sekund.

  <details open><summary>Zobacz w akcji</summary>
  <img src="../media/pl/security-lock.avif" alt="Kłódka: aplikacja zablokowana, a potem odblokowana hasłem">
  </details>

- **Podpisane aktualizacje, nigdy automatyczne.** Na górnym pasku pojawia się plakietka
  „Aktualizacja” z numerem wersji. „Zainstaluj i uruchom ponownie” najpierw prosi
  o potwierdzenie, a potem aplikacja instaluje wersję i uruchamia się ponownie. Z hasłem
  aplikacji uruchamia się zablokowana: wpisz je ponownie, aby wznowić kopanie. Instalowana
  jest tylko wersja podpisana przez projekt. Wersja Docker też o tym informuje i podaje
  polecenia do uruchomienia.

  <details open><summary>Zobacz w akcji</summary>
  <img src="../media/pl/updates.avif" alt="Plakietka „Aktualizacja”, potwierdzenie, a potem instalacja">
  </details>

### A także

- Tak jak w innych minerach: skrzynki bonusowe, rajdy, momenty, cele społeczności, prognozy
  (strategie, moment zakładu, warunki), pierwszeństwo dla serii oglądania i subskrybowanych
  kanałów, obecność na czacie (można ją wyłączyć), domyślne opcje dla nowych streamerów,
  powiadomienia Discord, Telegram i webhook z obrazkami, proxy globalne lub osobne dla
  każdego konta.
- **Wkrótce: kopanie odznak.** Odznaki Twitcha zdobywane za oglądanie, kopane tak jak Dropsy.
- 29 języków, ciemny i jasny motyw, powiadomienia na pulpicie, ikona w zasobniku,
  uruchamianie razem z systemem.

  <details open><summary>Zobacz w akcji</summary>
  <img src="../media/pl/basics.avif" alt="Języki, jasny i ciemny motyw, powiadomienie na pulpicie i ikona w zasobniku">
  </details>

> [!WARNING]
> Ten projekt nie jest powiązany z Twitchem. Automatyzacja konta może naruszać regulamin
> Twitcha lub zasady kampanii.

## Spis treści

- [Porównanie](#comparison)
- [Pobieranie](#download)
- [Docker](#docker)
- [Jak to działa](#how-it-works)
- [Najczęstsze pytania](#faq)
- [Powiązane projekty](#related-projects)
- [Bezpieczeństwo](#security)
- [Kompilacja ze źródeł](#build)
- [Licencja](#license)

## <a name="comparison"></a>Porównanie

Jak [Twitch Miner](https://github.com/SunkwiBOT/twitch-miner)
wypada na tle [Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
(punkty kanału) i [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner) (Drops).
Sprawdzone w ich kodzie i plikach README w listopadzie 2026 r.

| | Ten projekt | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Kampanie Drops: znajduje je, kopie i odbiera | ✅ | ❌ ¹ | ✅ |
| Punkty kanału: skrzynki, rajdy, momenty, prognozy | ✅ | ✅ | ❌ |
| Spam Points: niestandardowa nagroda kanału odebrana kilka razy z rzędu | ✅ | ❌ | ❌ |
| Drops i punkty kanału jednocześnie na jednym koncie | ✅ | częściowo ¹ | ❌ |
| Wiele kont w jednej aplikacji | ✅ | ❌ ² | ❌ |
| Zmiana ustawień wielu kont naraz | ✅ | ❌ | ❌ |
| Interfejs graficzny | ✅ desktopowy i webowy ³ | ❌ ⁴ | ✅ desktopowy |
| Panel WWW dla domowego serwera (Docker), chroniony hasłem, dostępny przez HTTPS | ✅ | ❌ ⁴ | ❌ |
| Sesje zaszyfrowane na dysku, opcjonalne hasło aplikacji | ✅ | ❌ | ❌ |
| Interfejs w 29 językach | ✅ | ❌ | częściowo ⁵ |
| Podpisane aktualizacje z poziomu aplikacji (desktop) | ✅ | ❌ | ❌ |
| Kreator konfiguracji dla każdego nowego konta | ✅ | ❌ ⁴ | ❌ |
| Streamerzy dodawani i edytowani w interfejsie, pojedynczo lub po kilku naraz | ✅ | ❌ ⁴ | — |
| Import streamerów przez wyszukiwanie, listę, inne konto lub obserwowane kanały | ✅ | częściowo ⁶ | — |
| Powiadomienia Discord, Telegram i webhook z obrazkami | ✅ | częściowo ⁷ | ❌ |
| Kalendarz wszystkich kampanii Drops, z wyszukiwarką | ✅ | ❌ | częściowo ⁸ |
| Kolejność kopania według kampanii, wykluczone słowa, ignorowana kampania lub nagroda | ✅ | ❌ | ❌ ⁹ |
| Streamerzy z Twojej listy oglądani w pierwszej kolejności dla Drops z kampanii | ✅ | częściowo ¹⁰ | ❌ |
| Filtrowany dziennik zdarzeń i lista wszystkich żądań, w interfejsie | ✅ | ❌ ⁴ | ❌ |
| Około 30 MB pamięci (serwer Docker, 4 konta) | ✅ | — ¹¹ | — ¹¹ |

¹ Ogląda tylko streamerów ze swojej listy: posuwa naprzód i odbiera Drops z ich transmisji,
nie wybierając kampanii.\
² Jeden proces na konto.\
³ Na Twoim komputerze jako aplikacja desktopowa i/lub na serwerze (VPS, NAS, Raspberry Pi)
z Dockerem, bez środowiska graficznego i bez przeglądarki na tej maszynie: panel otwiera
się w dowolnej przeglądarce.\
⁴ Konfiguracja w pliku Pythona; jego strona WWW pokazuje wykresy punktów i plik logu.\
⁵ 21 języków.\
⁶ Potrafi obserwować kanały, które obserwuje konto.\
⁷ Sam tekst, bez obrazków. Ma też Matrix, Pushover i Gotify, których ten projekt nie ma.\
⁸ Lista kampanii z filtrami, bez kalendarza i wyszukiwarki.\
⁹ Ma gry priorytetowe i wykluczone, wybierane po nazwie gry.\
¹⁰ Domyślnie najpierw ogląda streamerów ze swojej listy, których transmisja ma Drops (jego
priorytet `DROPS`), ale nigdy innego kanału.\
¹¹ Nie mierzono.

## <a name="download"></a>Pobieranie

Pobierz plik dla swojego systemu z
[najnowszego wydania](https://github.com/SunkwiBOT/twitch-miner/releases/latest).

| System | Pakiet |
|---|---|
| **Windows** · x64 | Instalator (`setup.exe`) lub [wersja przenośna](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/TwitchMiner-portable.exe) |
| **macOS** 10.15 lub nowszy · Intel i Apple Silicon | Obraz dysku (`.dmg`) |
| **Linux** · x86-64 | `.AppImage`, `.deb` (Debian, Ubuntu) lub `.rpm` (Fedora, openSUSE) |

Wersja na macOS nie jest notaryzowana. Za pierwszym razem kliknij aplikację prawym
przyciskiem myszy i wybierz **Otwórz**.

Aktualizacje są podpisane. Aplikacja sprawdza dostępność nowej wersji przy uruchomieniu,
a potem co godzinę (częstotliwość zmienisz w sekcji **Ustawienia › O programie**). Nigdy
nie instaluje jej sama: instalujesz ją jednym kliknięciem, z wyświetlonego komunikatu lub
z sekcji **Ustawienia › O programie**.

Dane są przechowywane w folderze danych użytkownika:

- Linux: `~/.local/share/twitch-miner`
- Windows: `%APPDATA%\twitch-miner`
- macOS: `~/Library/Application Support/twitch-miner`

## <a name="docker"></a>Docker

Serwer wyświetla ten sam panel w przeglądarce. Obraz to
`ghcr.io/sunkwibot/twitch-miner`, dla `amd64` i `arm64`. Gdy pojawi się
nowa wersja, panel o tym informuje: `docker compose pull`, a potem `docker compose up -d`.

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-miner:latest
    pull_policy: always   # pobieraj najnowszy obraz przy każdym starcie
    restart: unless-stopped
    ports: ["8080:8080"]   # za odwrotnym proxy HTTPS: usuń albo "127.0.0.1:8080:8080"
    environment:
      MINER_PASSWORD: "change-me"   # wymagane, co najmniej 12 znaków
      # MINER_PUBLIC_URL: "https://miner.example.com"   # tylko za odwrotnym proxy HTTPS
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

Otwórz <http://localhost:8080> (lub adres maszyny w Twojej sieci) i wpisz hasło.

### Zmienne

| Zmienna | Domyślnie | Przeznaczenie |
|---|---|---|
| `MINER_PASSWORD` | brak | Hasło panelu, wymagane: bez niego serwer odmawia uruchomienia. Co najmniej 12 znaków (krótsze wyświetla ostrzeżenie przy starcie). Przykładowa wartość `change-me` jest odrzucana. |
| `MINER_PASSWORD_FILE` | brak | Odczytuje `MINER_PASSWORD` z pliku, np. z sekretu Dockera. Ustaw jedno albo drugie, nie oba. |
| `MINER_PUBLIC_URL` | brak | Tylko za odwrotnym proxy HTTPS (Caddy, nginx, Traefik, Cloudflare Tunnel…): adres `https://` panelu, bez ścieżki, np. `https://miner.example.com`. Proxy musi przekazywać `X-Forwarded-Proto`, `X-Forwarded-For` i websockety, co popularne proxy robią domyślnie, a port kontenera nie może być dostępny bezpośrednio. Panel odrzuca wtedy wszystko, co nie przechodzi przez proxy po HTTPS, a po 5 błędnych hasłach blokuje adres na 30 sekund. Proxy musi działać na tym samym komputerze lub w jego sieci lokalnej: połączenie z adresu publicznego jest odrzucane. |
| `MINER_TRUSTED_HOPS` | `1` | Tylko z `MINER_PUBLIC_URL`: ile proxy dodaje wpis do `X-Forwarded-For`, od 1 do 10. `2` przy dwóch proxy, jednym za drugim, np. Cloudflare przed Traefikiem. |
| `MINER_PROXY_NETWORKS` | brak | Tylko z `MINER_PUBLIC_URL`: inne sieci, z których proxy może się łączyć, oprócz tego komputera i jego prywatnej sieci lokalnej, rozdzielone przecinkami: pojedynczy adres lub sieć jak `2001:db8:1::/64`, dla sieci lokalnej z globalnymi adresami IPv6. |
| `MINER_VAULT_PASSPHRASE` | brak | Tylko dla folderu danych skopiowanego z aplikacji desktopowej z ustawionym hasłem aplikacji: odblokowuje go przy starcie. Serwer postawiony od zera nigdy jej nie potrzebuje. |
| `MINER_VAULT_PASSPHRASE_FILE` | brak | Odczytuje `MINER_VAULT_PASSPHRASE` z pliku. Ustaw jedno albo drugie, nie oba. |
| `DATA_DIR` | `/data` w obrazie, w przeciwnym razie `data` w katalogu roboczym | Baza danych, plik klucza i logi. |
| `LOG_DIR` | `$DATA_DIR/logs` | Pliki logów: jeden dziennie, przechowywanych jest 14 ostatnich. |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | Adres nasłuchiwania: adres IP i port od 1 do 65535. |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | Poziom logowania. |

Serwer odczytuje i sprawdza zmienne raz, przy starcie. Nieprawidłowa wartość zatrzymuje
serwer z komunikatem, zamiast po cichu zastąpić ją wartością domyślną.

## <a name="how-it-works"></a>Jak to działa

Co 20 sekund każde konto wybiera swoje dwa kanały: najlepszy kanał na żywo dla pierwszej
kampanii w swojej kolejce, a potem streamerów ze swojej listy punktów.

Twitch zalicza na konto jednocześnie **jedną kampanię Drops** i **dwa kanały dla punktów**.
Od października 2026 r. drop można odebrać tylko wtedy, gdy konto gry jest połączone.
Niepołączone kampanie nadal robią postępy, a panel pokazuje link do połączenia.

Lista kampanii pochodzi z projektu [Twitch Drops API](https://drops.sunkwi.com) (odświeżana
co 10 minut), bo Twitch ukrywa ją przed klientem TV.

## <a name="faq"></a>Najczęstsze pytania

### Ryzyko i Twitch

**Czy moje konto może zostać zbanowane?**\
Nikt nie może zagwarantować, że nie. Ten projekt nie jest powiązany z Twitchem,
a automatyzacja konta może naruszać jego regulamin lub zasady kampanii. Miner ogląda
najwyżej dwa kanały na konto, czyli tyle, ile wynosi limit Twitcha, i nie pobiera żadnego
wideo. Decyzja należy do Ciebie.

**Czy wideo jest naprawdę oglądane?**\
Nie, żadne wideo nie jest pobierane. Dla Drops miner co 15 sekund odczytuje playlistę
transmisji w najniższej jakości i pobiera tylko nagłówek każdego nowego segmentu. Dla
punktów co minutę wysyła sygnał Twitcha „obejrzana minuta”. Przy każdym oglądanym kanale
widać, ilu segmentów zażądano.

**Dlaczego najwyżej dwa kanały?**\
To limit Twitcha: zalicza on jednocześnie tylko dwa oglądane kanały na konto i tylko jedną
kampanię Drops naraz. W trybie **Drops i punkty** jeden kanał obsługuje kampanię, a drugi
Twoich streamerów. Ustawienie „Jeden kanał” w sekcji **Ustawienia konta** jest
dyskretniejsze.

### Drops

**Kampania nie jest kopana albo drop nie robi postępów.**\
Otwórz stronę **Drops** danego konta. W sekcji „Kolejność kopania” każda oczekująca
kampania podaje powód: „Brak kanału na żywo”, „Oczekiwanie (jedna kampania naraz)” lub
„Brak dostępnego dropa”. Kampania, której nie ma na tej liście, została pominięta: przez
ustawienie „Które kampanie”, przez regułę na karcie **Wykluczenia** albo dlatego, że nie da
się jej ukończyć na czas. Nagroda wymagająca subskrypcji nigdy nie jest kopana, a drop,
który następuje po innym, czeka, aż pierwszy zostanie ukończony. W trybie **Tylko punkty**
miner nie wybiera żadnej kampanii. „Kop najpierw” w szczegółach kampanii przenosi ją na
górę; ignorowana kampania może wrócić dzięki „Kop ponownie” na karcie **Wykluczenia**.

**Czy muszę połączyć konto gry?**\
Żeby odebrać nagrodę, tak: od października 2026 r. Twitch pozwala odebrać drop tylko
wtedy, gdy konto gry jest połączone. Domyślnie niepołączona kampania i tak robi postępy,
a odbiór czeka na połączenie. Sygnalizuje to plakietka „Niepołączone”, a „Połącz konto gry”
w szczegółach kampanii otwiera stronę łączenia. Aby kopać tylko połączone gry, wybierz
„Tylko gry z połączonym kontem”.

### Instalacja i dane

**Aplikacja desktopowa czy Docker?**\
Aplikacja desktopowa kopie, dopóki komputer jest włączony, z ikoną w zasobniku,
opcjonalnym hasłem i aktualizacjami jednym kliknięciem. Docker kopie dzień i noc na
maszynie, która jest zawsze włączona (VPS, NAS, Raspberry Pi); otwierasz go
w przeglądarce, a chroni go `MINER_PASSWORD`. Interfejs jest ten sam.

**Gdzie są moje dane?**\
W folderze danych: `~/.local/share/twitch-miner` na Linuksie,
`%APPDATA%\twitch-miner` w Windows,
`~/Library/Application Support/twitch-miner` na macOS i wolumin `/data`
w Dockerze. Na komputerze **Ustawienia › Pulpit** otwiera go jednym kliknięciem. Zawiera
`miner.db` (konta, ustawienia, dziennik), `master.key` (klucz sesji, gdy nie ustawiono
hasła aplikacji) i logi.

**Jak zaktualizować?**\
Aplikacja desktopowa sprawdza dostępność nowej wersji przy uruchomieniu, a potem co godzinę
(zmienisz to w sekcji **Ustawienia › O programie**). Kliknij plakietkę „Aktualizacja”,
a potem „Zainstaluj i uruchom ponownie”. W Dockerze panel o tym informuje: uruchom
`docker compose pull`, a potem `docker compose up -d`.

**Nie pamiętam hasła aplikacji.**\
Nie da się go odzyskać: to ono szyfruje Twoje sesje Twitch. Zamknij aplikację, usuń folder
danych, a potem dodaj konta ponownie; ich ustawienia trzeba skonfigurować od nowa.
W Dockerze hasło panelu zmienia się w `MINER_PASSWORD`, a potem uruchamia
`docker compose up -d`.

### Punkty i prognozy

**Ile punktów na godzinę?**\
To zależy od Twitcha i od każdego kanału. Miner zdobywa to, co Twitch daje widzowi: punkty
za oglądanie, skrzynki bonusowe (mniej więcej jedna co 15 minut) i bonus za serię
oglądania. Subskrypcja mnoży punkty, od ×1,2 do ×2 w zależności od poziomu. Strona Start
pokazuje zdobyte punkty, a Dziennik każdy zysk.

**Czy zakłady są ryzykowne?**\
Tak, punkty można stracić. Prognozy są domyślnie wyłączone, dla każdego streamera osobno.
Ustawiasz strategię, moment zakładu, stawkę (domyślnie 5% salda, najwyżej 50 000 punktów),
minimalne saldo i, jeśli chcesz, warunek, np. kurs co najmniej 1,5. Trzymaj stawkę niską.

**Streamer nie jest oglądany.**\
Oglądani są tylko streamerzy włączeni i nadający na żywo, i tylko na wolnym kanale:
w trybie **Drops i punkty** jeden z dwóch obsługuje bieżącą kampanię; w trybie
**Tylko Drops** żaden streamer nie jest oglądany; wstrzymane konto nic nie ogląda. Miejsca
są przydzielane w tej kolejności, pokazanej jako plakietka na każdym oglądanym kanale:
„Bonus za serię”, „Seria zagrożona”, „Subskrybowany”, potem „Kolejność listy”. Przesuń
streamera wyżej na liście albo wyłącz „Najpierw serie oglądania” i
„Najpierw subskrybowane kanały”.

## <a name="related-projects"></a>Powiązane projekty

Inne projekty tego samego autora:

- [**Twitch Drops API**](https://drops.sunkwi.com): darmowe publiczne API wszystkich
  trwających kampanii Twitch Drops i kampanii z nagrodami, aktualizowane co minutę. Z niego
  ta aplikacja pobiera swój katalog kampanii.
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com): bot Discord, który ogłasza na Twoim
  serwerze transmisje na żywo z Twitcha i kampanie Drops.
- [**Kick Alerts 🔔**](https://kick.sunkwi.com): bot Discord, który ogłasza na Twoim serwerze
  transmisje na żywo z Kicka i kampanie Drops.

## <a name="security"></a>Bezpieczeństwo

- Tokeny są szyfrowane algorytmem AES-256-GCM. Kluczem jest albo losowy `master.key` obok
  bazy danych (domyślnie, bez udziału użytkownika), albo klucz wyprowadzony z hasła
  aplikacji za pomocą Argon2id.
- Panel HTTP sprawdza hasło w stałym czasie i używa ciasteczek HttpOnly, SameSite=Strict
  (`Secure`, z prefiksem `__Host-`, za HTTPS). Serwer przechowuje tylko skrót
  SHA-256 każdej sesji. Kończy sesję po 30 dniach lub przy wylogowaniu. Sprawdza sesję
  przed odczytaniem treści żądania i ogranicza treść do 256 KiB.
- Po 5 błędnych hasłach z rzędu z jednego adresu IP (formularz logowania lub bearer) ten
  adres czeka 30 sekund. Serwer odpowiada wtedy `429` z `Retry-After`, nie patrząc na
  hasło. Każde błędne hasło dostaje też odpowiedź dopiero po pół sekundy.
- Poza swoją siecią umieść panel za odwrotnym proxy HTTPS z `MINER_PUBLIC_URL`
  (zob. [Docker](#docker)).
- Miner łączy się tylko z `twitch.tv`, `ttvnw.net`, `jtvnw.net`, `twitchcdn.net`,
  `drops.sunkwi.com` oraz z ustawionymi przez Ciebie celami powiadomień.
  Aplikacja (desktopowa i Docker) sprawdza nowe wersje na `github.com`, a aplikacja
  desktopowa pobiera je z wydań projektu na GitHubie. Są podpisane: instalowana jest tylko
  wyświetlona wersja.

## <a name="build"></a>Kompilacja ze źródeł

**Wymagania:** Rust, Node.js oraz
[wymagania wstępne Tauri](https://v2.tauri.app/start/prerequisites/) dla Twojego systemu.

```bash
npm ci
npm run build          # panel
cargo build --release -p miner-server   # serwer: target/release/twitch-miner
npm run bundle:linux   # pakiety desktopowe (wymagają zależności Tauri dla Linuksa)
```

Kontrola kodu:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier, oxlint (z analizą typów) i svelte-check
```

Typy TypeScript interfejsu są generowane z typów Rusta za pomocą
[ts-rs](https://github.com/Aleph-Alpha/ts-rs) do `ui/src/lib/bindings`. Po zmianie typu
widocznego dla interfejsu uruchom `npm run bindings`.

## <a name="license"></a>Licencja

[MIT](../../LICENSE). Inspiracją były [TwitchDropsMiner autorstwa DevilXD](https://github.com/DevilXD/TwitchDropsMiner),
[Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
i ich forki.
