<p align="center">
  <img src="../../ui/public/app-icon.svg" width="112" height="112" alt="Logo Twitch Miner">
</p>

<h1 align="center">Twitch Miner</h1>

<p align="center">
  <strong>Menambang Twitch Drops dan poin channel di beberapa akun, tanpa browser dan tanpa tab Twitch yang terbuka.</strong>
</p>

<p align="center">
  Aplikasi desktop untuk Linux, Windows, dan macOS, atau image Docker dengan dasbor web yang sama, untuk dijalankan di server rumahan atau di Raspberry Pi. Dengan 4 akun, server memakai sekitar 30 MB memori dan hampir tidak memakai CPU.
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-miner?label=release&color=9146FF" alt="Rilis terbaru"></a>
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-miner/total?label=downloads&color=9146FF" alt="Unduhan"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="Platform: Linux, Windows, macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="Image Docker untuk amd64 dan arm64"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="Lisensi MIT"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="Tayangan">
</p>

<p align="center">
  <a href="#download"><strong>Unduh</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-miner/issues/new/choose">Laporkan bug</a>
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
  <a href="README.ko.md">한국어</a> ·
  <a href="README.th.md">ไทย</a> ·
  <a href="README.vi.md">Tiếng&nbsp;Việt</a> ·
  <strong>Bahasa&nbsp;Indonesia</strong>
</p>

<p align="center">
  <img src="../media/id/presentation.avif" alt="Video pengenalan Twitch Miner">
</p>

## <a name="features"></a>Fitur

Hal-hal yang membedakannya dari penambang lain. “Lihat langsung” di bawah sebuah butir memperlihatkan fitur itu saat bekerja.

### Akun dan streamer

- **Banyak akun dalam satu aplikasi.** Setiap akun menambang sendiri-sendiri, dengan modenya masing-masing: **Drops dan poin** (satu channel untuk kampanye terbaik, satu lagi untuk streamer Anda), **Hanya Drops**, atau **Hanya poin**.

  <details open><summary>Lihat langsung</summary>
  <img src="../media/id/accounts-mining.avif" alt="Beberapa akun menambang bersamaan, di halaman Beranda">
  </details>

- **Panduan penyiapan untuk setiap akun baru.** Setelah masuk, mulailah dari pengaturan default atau dari pengaturan akun lain. Pilih mode, kampanye, dan streamer, lalu “Mulai menambang” atau “Simpan dan mulai nanti”.

  <details open><summary>Lihat langsung</summary>
  <img src="../media/id/account-setup.avif" alt="Panduan penyiapan akun baru, langkah demi langkah">
  </details>

- **Satu perubahan diterapkan ke akun lain dengan sekali klik.** Setelah Anda mengubah sesuatu di sebuah akun, pesan konfirmasi menawarkan “Terapkan ke akun lain”. Perubahan itu dilakukan lagi di akun yang Anda centang, dan pengaturan lain di akun-akun itu tetap seperti semula.

  <details open><summary>Lihat langsung</summary>
  <img src="../media/id/apply-to-accounts.avif" alt="Perubahan yang dilakukan lagi di akun lain dengan “Terapkan ke akun lain”">
  </details>

- **Tambah streamer.** Streamer langsung berjalan dengan opsi default: milik aplikasi, atau yang Anda pilih untuk akun ini. Sesuaikan saat itu juga jika perlu: peti bonus, raid, momen, streak tonton, chat, prediksi.

  <details open><summary>Lihat langsung</summary>
  <img src="../media/id/streamer-add.avif" alt="Menambahkan streamer lewat pencarian Twitch, lalu opsinya">
  </details>

- **Edit streamer.** “Edit” membuka opsinya: peti bonus, raid, momen, streak tonton, chat, prediksi… Sebuah sakelar menyalakan atau mematikannya tanpa menghapusnya dari daftar.

  <details open><summary>Lihat langsung</summary>
  <img src="../media/id/streamer-edit.avif" alt="Opsi sebuah streamer, dibuka dari barisnya">
  </details>

- **Edit beberapa streamer sekaligus.** Centang mereka, lalu “Edit N streamer yang dipilih”. Hanya opsi yang Anda ubah yang diterapkan. Opsi yang berbeda di antara mereka tampil sebagai “Campuran”, dan sakelarnya berubah menjadi dua tombol, “Ya” dan “Tidak”. Setelah itu aplikasi menawarkan untuk menjadikannya opsi default bagi streamer baru.

  <details open><summary>Lihat langsung</summary>
  <img src="../media/id/streamers-group-edit.avif" alt="Mengedit beberapa streamer, dengan opsi “Campuran”">
  </details>

- **Impor streamer.** Tempel daftar (hingga 500 nama), atau pakai lagi daftar akun lain atau channel yang diikuti akun tersebut. Daftar itu diperiksa agar hanya channel yang benar-benar ada yang ditambahkan.

  <details open><summary>Lihat langsung</summary>
  <img src="../media/id/streamers-import.avif" alt="Mengimpor streamer: daftar yang ditempel, akun lain, channel yang diikuti">
  </details>

- **Spam Points.** Tukarkan hadiah kustom sebuah channel beberapa kali berturut-turut. Pilih channel dari daftar Poin Anda, atau channel mana pun yang ditemukan lewat pencarian Twitch: hadiahnya tampil beserta saldo akun, yang dibaca ulang secara langsung. Atur jumlahnya (hingga 25, sesuai saldo), lalu “Tukar”. Hasilnya tampil saat rentetan selesai, dan setiap rentetan punya barisnya sendiri di Jurnal.

  <details open><summary>Lihat langsung</summary>
  <img src="../media/id/spam-points.avif" alt="Spam Points: sebuah channel dipilih, 10 × sebuah hadiah dengan pesan ditukar, penukarannya muncul di obrolan Twitch channel, lalu barisnya di Jurnal">
  </details>

### Drops

- **Semua kampanye Drops yang sedang berjalan.** Setiap kampanye punya detailnya sendiri: hadiah, penautan akun game, “Tambang lebih dulu”. Daftarnya berasal dari proyek [Twitch Drops API](https://drops.sunkwi.com) dan diperbarui setiap 10 menit.

  <details open><summary>Lihat langsung</summary>
  <img src="../media/id/drops-catalogue.avif" alt="Katalog dalam tampilan kalender, pencarian, dan detail sebuah kampanye">
  </details>

- **Urutan penambangan Anda.** Seret kampanye ke urutan yang Anda mau. Kampanye baru menemukan tempatnya sendiri: game prioritas Anda dulu, lalu yang paling cepat berakhir atau yang sisa menitnya paling sedikit. Kecualikan game, kata, satu kampanye, atau satu hadiah saja, dan lewati kampanye yang tidak bisa selesai tepat waktu. “Tambang lebih dulu”, di detail sebuah kampanye, menaruhnya di urutan teratas.

  <details open><summary>Lihat langsung</summary>
  <img src="../media/id/drops-order.avif" alt="Urutan penambangan, game prioritas, dan pengecualian">
  </details>

- **Inventaris dan “Klaim semua”.** Halaman Inventaris memilah hadiah menjadi “Siap diklaim”, “Sedang berjalan”, dan “Sudah diklaim”. Klaim satu per satu, semuanya sekaligus, atau nyalakan “Klaim drop otomatis”.

  <details open><summary>Lihat langsung</summary>
  <img src="../media/id/drops-inventory.avif" alt="Inventaris dan tombol “Klaim semua”">
  </details>

- **“Streamer saya lebih dulu untuk Drops”.** Untuk kampanye saat ini, streamer dari daftar Anda yang stream-nya dihitung untuk kampanye itu didahulukan dari channel lain. Channel yang sama memajukan drop sekaligus memberi Anda poin channel-nya. Aktif secara default.

  <details open><summary>Lihat langsung</summary>
  <img src="../media/id/drops-streamers-first.avif" alt="Streamer dari daftar, ditonton untuk Drops dan untuk poinnya">
  </details>

### Pemantauan

- **Jurnal untuk semuanya, dan setiap permintaan.** Jurnal menyimpan setiap peristiwa: drop, poin yang didapat dan dibelanjakan, peti, taruhan, raid, stream yang mulai live… Saring berdasarkan akun, tab (Drops, Poin, Lainnya), dan jenis. **Log teknis › Permintaan** mencantumkan setiap permintaan yang dikirim ke Twitch dan ke layanan lain, beserta hasil dan durasinya. Log dan permintaan tidak memuat token atau rahasia apa pun: “Salin untuk laporan bug” menyalinnya, siap ditempel ke laporan bug di [GitHub](https://github.com/SunkwiBOT/twitch-miner/issues) atau [Discord](https://discord.gg/support-bots-891685895781113886).

  <details open><summary>Lihat langsung</summary>
  <img src="../media/id/journal-requests.avif" alt="Jurnal yang disaring, lalu Log teknis › Permintaan">
  </details>

### Di desktop atau di server

- **Antarmuka yang sama di mana saja.** Aplikasi desktop (Linux, Windows, macOS) dan image Docker menampilkan dasbor yang sama. Docker berjalan di VPS, NAS, atau Raspberry Pi, tanpa layar atau browser di mesin itu: dasbor bisa dibuka di browser mana pun.
  Di ponsel, dasbor menyesuaikan diri dengan layar, dan lewat HTTPS bisa dipasang seperti aplikasi.
- **Ringan.** Sekitar 30 MB memori untuk 4 akun, hampir tanpa CPU, dan image Docker sekitar 12 MB.

### Keamanan dan pembaruan

- **Sesi Anda terlindungi.** Tanpa kata sandi Twitch: Anda masuk di `twitch.tv/activate`, dari browser Anda. Sesi dienkripsi di disk, dan Anda bisa mencabutnya di [pengaturan Twitch](https://www.twitch.tv/settings/connections) Anda. Di desktop, kata sandi aplikasi opsional mengunci aplikasi: tidak ada yang ditambang sampai kata sandi diketik. Dengan Docker, dasbor punya kata sandinya sendiri, terkunci sendiri setelah beberapa menit tanpa aktivitas, dan berjalan lewat HTTPS di belakang proxy. Lima kata sandi salah berturut-turut memblokir alamat IP selama 30 detik.

  <details open><summary>Lihat langsung</summary>
  <img src="../media/id/security-lock.avif" alt="Gembok: aplikasi terkunci, lalu dibuka dengan kata sandinya">
  </details>

- **Pembaruan bertanda tangan, tidak pernah otomatis.** Lencana “Pembaruan” beserta nomor versinya muncul di bilah atas. “Pasang dan mulai ulang” meminta konfirmasi dulu, lalu aplikasi memasang versi itu dan memulai ulang. Jika ada kata sandi aplikasi, aplikasi dimulai ulang dalam keadaan terkunci: ketik lagi kata sandinya untuk melanjutkan penambangan. Hanya versi yang ditandatangani proyek yang dipasang. Versi Docker juga memberi tahu, lengkap dengan perintah yang perlu dijalankan.

  <details open><summary>Lihat langsung</summary>
  <img src="../media/id/updates.avif" alt="Lencana “Pembaruan”, konfirmasi, lalu pemasangan">
  </details>

### Selain itu

- Seperti penambang lain: peti bonus, raid, momen, target komunitas, prediksi (strategi, kapan bertaruh, syarat), streak tonton dan channel langganan lebih dulu, kehadiran di chat (bisa dimatikan), opsi default untuk streamer baru, notifikasi Discord, Telegram, dan webhook dengan gambar, proxy global atau per akun.
- **Segera hadir: menambang lencana.** Lencana Twitch yang didapat dengan menonton, ditambang seperti Drops.
- 29 bahasa, tema gelap dan terang, notifikasi desktop, ikon tray, jalan saat sistem dimulai.

  <details open><summary>Lihat langsung</summary>
  <img src="../media/id/basics.avif" alt="Bahasa, tema terang dan gelap, notifikasi desktop, dan ikon tray">
  </details>

> [!WARNING]
> Proyek ini tidak berafiliasi dengan Twitch. Mengotomatiskan akun bisa melanggar ketentuan Twitch atau aturan kampanye.

## Daftar isi

- [Perbandingan](#comparison)
- [Unduh](#download)
- [Docker](#docker)
- [Cara kerja](#how-it-works)
- [Pertanyaan umum](#faq)
- [Proyek terkait](#related-projects)
- [Keamanan](#security)
- [Build dari sumber](#build)
- [Lisensi](#license)

## <a name="comparison"></a>Perbandingan

Posisi [Twitch Miner](https://github.com/SunkwiBOT/twitch-miner) dibandingkan dengan [Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2) (poin channel) dan [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner) (Drops). Dicek berdasarkan kode dan README mereka pada November 2026.

| | Proyek ini | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Kampanye Drops: menemukan, menambang, dan mengklaimnya | ✅ | ❌ ¹ | ✅ |
| Poin channel: peti, raid, momen, prediksi | ✅ | ✅ | ❌ |
| Spam Points: hadiah kustom sebuah channel ditukarkan beberapa kali berturut-turut | ✅ | ❌ | ❌ |
| Drops dan poin channel sekaligus di satu akun | ✅ | sebagian ¹ | ❌ |
| Banyak akun dalam satu aplikasi | ✅ | ❌ ² | ❌ |
| Ubah pengaturan beberapa akun sekaligus | ✅ | ❌ | ❌ |
| Antarmuka grafis | ✅ desktop dan web ³ | ❌ ⁴ | ✅ desktop |
| Dasbor web untuk server rumahan (Docker), dengan kata sandi, bisa diakses lewat HTTPS | ✅ | ❌ ⁴ | ❌ |
| Sesi dienkripsi di disk, kata sandi aplikasi opsional | ✅ | ❌ | ❌ |
| Antarmuka dalam 29 bahasa | ✅ | ❌ | sebagian ⁵ |
| Pembaruan bertanda tangan dari dalam aplikasi (desktop) | ✅ | ❌ | ❌ |
| Panduan penyiapan untuk setiap akun baru | ✅ | ❌ ⁴ | ❌ |
| Streamer ditambahkan dan diedit di antarmuka, satu per satu atau beberapa sekaligus | ✅ | ❌ ⁴ | — |
| Streamer diimpor lewat pencarian, daftar, akun lain, atau channel yang diikuti | ✅ | sebagian ⁶ | — |
| Notifikasi Discord, Telegram, dan webhook dengan gambar | ✅ | sebagian ⁷ | ❌ |
| Kalender semua kampanye Drops, dengan pencarian | ✅ | ❌ | sebagian ⁸ |
| Urutan penambangan per kampanye, kata yang dikecualikan, kampanye atau hadiah yang diabaikan | ✅ | ❌ | ❌ ⁹ |
| Streamer di daftar Anda ditonton lebih dulu untuk Drops sebuah kampanye | ✅ | sebagian ¹⁰ | ❌ |
| Jurnal peristiwa yang bisa disaring dan daftar setiap permintaan, di antarmuka | ✅ | ❌ ⁴ | ❌ |
| Sekitar 30 MB memori (server Docker, 4 akun) | ✅ | — ¹¹ | — ¹¹ |

¹ Ia hanya menonton streamer di daftarnya: ia memajukan dan mengklaim Drops dari stream mereka, tanpa memilih kampanye.\
² Satu proses per akun.\
³ Di PC Anda sebagai aplikasi desktop, dan/atau di server (VPS, NAS, Raspberry Pi) dengan Docker, tanpa lingkungan grafis dan tanpa browser di mesin itu: dasbor bisa dibuka di browser mana pun.\
⁴ Dikonfigurasi lewat file Python; halaman webnya menampilkan grafik poin dan file log.\
⁵ 21 bahasa.\
⁶ Ia bisa mengikuti channel yang diikuti akun tersebut.\
⁷ Hanya teks, tanpa gambar. Ia juga punya Matrix, Pushover, dan Gotify, yang tidak dimiliki proyek ini.\
⁸ Daftar kampanye dengan filter, tanpa kalender atau pencarian.\
⁹ Ia punya game prioritas dan game yang dikecualikan, dipilih berdasarkan nama game.\
¹⁰ Secara default, ia lebih dulu menonton streamer di daftarnya yang stream-nya membawa Drops (prioritas `DROPS`-nya), tetapi tidak pernah channel lain.\
¹¹ Tidak diukur.

## <a name="download"></a>Unduh

Unduh file untuk sistem Anda dari [rilis terbaru](https://github.com/SunkwiBOT/twitch-miner/releases/latest).

| Sistem | Paket |
|---|---|
| **Windows** · x64 | Penginstal (`setup.exe`) atau [executable portabel](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/TwitchMiner-portable.exe) |
| **macOS** 10.15 atau lebih baru · Intel dan Apple Silicon | Image disk (`.dmg`) |
| **Linux** · x86-64 | `.AppImage`, `.deb` (Debian, Ubuntu), atau `.rpm` (Fedora, openSUSE) |

Build macOS tidak dinotarisasi. Saat pertama kali, klik kanan aplikasi lalu pilih **Buka**.

Pembaruan ditandatangani. Aplikasi memeriksa versi baru saat dimulai, lalu setiap jam (Anda bisa mengubah frekuensinya di **Pengaturan › Tentang**). Aplikasi tidak pernah memasangnya sendiri: Anda memasangnya dengan sekali klik, dari pesan yang muncul atau dari **Pengaturan › Tentang**.

Data disimpan di folder data pengguna:

- Linux: `~/.local/share/twitch-miner`
- Windows: `%APPDATA%\twitch-miner`
- macOS: `~/Library/Application Support/twitch-miner`

## <a name="docker"></a>Docker

Server menampilkan dasbor yang sama di browser. Image-nya adalah `ghcr.io/sunkwibot/twitch-miner`, untuk `amd64` dan `arm64`. Saat versi baru keluar, dasbor memberitahukannya: `docker compose pull`, lalu `docker compose up -d`.

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-miner:latest
    pull_policy: always   # ambil image terbaru setiap kali mulai
    restart: unless-stopped
    ports: ["8080:8080"]   # di belakang reverse proxy HTTPS: hapus, atau "127.0.0.1:8080:8080"
    environment:
      MINER_PASSWORD: "change-me"   # wajib, 12 karakter atau lebih
      # MINER_PUBLIC_URL: "https://miner.example.com"   # hanya di belakang reverse proxy HTTPS
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

Buka <http://localhost:8080> (atau alamat mesin itu di jaringan Anda) lalu masukkan kata sandi.

### Variabel

| Variabel | Default | Fungsi |
|---|---|---|
| `MINER_PASSWORD` | tidak ada | Kata sandi dasbor, wajib: tanpanya, server menolak untuk mulai. 12 karakter atau lebih (yang lebih pendek memunculkan peringatan saat mulai). Nilai contoh `change-me` ditolak. |
| `MINER_PASSWORD_FILE` | tidak ada | Membaca `MINER_PASSWORD` dari file, misalnya Docker secret. Pilih salah satu, jangan keduanya. |
| `MINER_PUBLIC_URL` | tidak ada | Hanya di belakang reverse proxy HTTPS (Caddy, nginx, Traefik, Cloudflare Tunnel…): alamat `https://` dasbor, tanpa path, seperti `https://miner.example.com`. Proxy harus meneruskan `X-Forwarded-Proto`, `X-Forwarded-For`, dan websocket, yang sudah dilakukan proxy umum secara default, dan port container tidak boleh bisa dijangkau langsung. Dasbor lalu menolak semua akses yang tidak datang lewat proxy melalui HTTPS, dan mengunci sebuah alamat selama 30 detik setelah 5 kata sandi salah. Proxy harus berjalan di mesin yang sama atau jaringan lokalnya: koneksi dari alamat publik ditolak. |
| `MINER_TRUSTED_HOPS` | `1` | Hanya bersama `MINER_PUBLIC_URL`: berapa banyak proxy yang menambahkan entri ke `X-Forwarded-For`, dari 1 sampai 10. `2` untuk dua proxy yang berlapis, seperti Cloudflare di depan Traefik. |
| `MINER_PROXY_NETWORKS` | tidak ada | Hanya dengan `MINER_PUBLIC_URL`: jaringan lain tempat proxy boleh terhubung, selain mesin ini dan jaringan lokal privatnya, dipisahkan koma: satu alamat atau jaringan seperti `2001:db8:1::/64`, untuk jaringan lokal yang alamat IPv6-nya global. |
| `MINER_VAULT_PASSPHRASE` | tidak ada | Hanya untuk folder data yang disalin dari aplikasi desktop yang memakai kata sandi aplikasi: membuka kuncinya saat mulai. Server yang disiapkan dari nol tidak pernah membutuhkannya. |
| `MINER_VAULT_PASSPHRASE_FILE` | tidak ada | Membaca `MINER_VAULT_PASSPHRASE` dari file. Pilih salah satu, jangan keduanya. |
| `DATA_DIR` | `/data` di image, selain itu `data` di direktori kerja | Database, file kunci, dan log. |
| `LOG_DIR` | `$DATA_DIR/logs` | File log: satu per hari, 14 file terakhir disimpan. |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | Alamat listen: alamat IP dan port dari 1 sampai 65535. |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | Level log. |

Server membaca dan memeriksa variabel sekali, saat mulai. Nilai yang tidak valid menghentikan server dengan pesan, alih-alih diam-diam beralih ke nilai cadangan.

## <a name="how-it-works"></a>Cara kerja

Setiap 20 detik, tiap akun memilih dua channel-nya: channel live terbaik untuk kampanye teratas di antreannya, lalu streamer di daftar poinnya.

Twitch menghitung **satu kampanye Drops** dan **dua channel untuk poin** sekaligus per akun. Sejak Oktober 2026, drop hanya bisa diklaim jika akun game sudah tertaut. Kampanye yang belum tertaut tetap berjalan, dan dasbor menampilkan tautan untuk menautkannya.

Daftar kampanye berasal dari proyek [Twitch Drops API](https://drops.sunkwi.com) (diperbarui setiap 10 menit), karena Twitch menyembunyikannya dari klien TV.

## <a name="faq"></a>Pertanyaan umum

### Risiko dan Twitch

**Apakah akun saya bisa diblokir?**\
Tidak ada yang bisa menjamin tidak. Proyek ini tidak berafiliasi dengan Twitch, dan mengotomatiskan akun bisa melanggar ketentuannya atau aturan kampanye. Penambang menonton paling banyak dua channel per akun, batas dari Twitch, tanpa mengunduh video apa pun. Keputusan ada di tangan Anda.

**Apakah videonya benar-benar ditonton?**\
Tidak, tidak ada video yang diunduh. Untuk Drops, setiap 15 detik penambang membaca playlist stream pada kualitas terendah dan hanya meminta header dari setiap segmen baru. Untuk poin, ia mengirim sinyal “menit ditonton” milik Twitch setiap menit. Setiap channel yang ditonton menampilkan berapa segmen yang sudah diminta.

**Kenapa paling banyak dua channel?**\
Itu batas dari Twitch: Twitch hanya menghitung dua channel yang ditonton bersamaan per akun, dan hanya satu kampanye Drops dalam satu waktu. Di mode **Drops dan poin**, satu channel melayani kampanye dan yang lain melayani streamer Anda. Pengaturan “Satu channel”, di **Pengaturan akun**, lebih tidak mencolok.

### Drops

**Sebuah kampanye tidak ditambang, atau sebuah drop tidak maju.**\
Buka halaman **Drops** akun tersebut. Di “Urutan penambangan”, setiap kampanye yang menunggu menyebutkan alasannya: “Tidak ada channel yang live”, “Menunggu (satu kampanye sekaligus)”, atau “Belum ada drop tersedia”. Kampanye yang tidak muncul di daftar itu sudah disisihkan: oleh pengaturan “Kampanye mana”, oleh aturan di tab **Pengecualian**, atau karena tidak bisa selesai tepat waktu. Hadiah yang membutuhkan langganan tidak pernah ditambang, dan drop yang menyusul drop lain menunggu sampai drop pertama selesai. Di mode **Hanya poin**, penambang tidak memilih kampanye apa pun. “Tambang lebih dulu”, di detail sebuah kampanye, menaruhnya di urutan teratas; kampanye yang diabaikan bisa kembali lewat “Tambang lagi”, di tab **Pengecualian**.

**Apakah saya perlu menautkan akun game?**\
Untuk mengklaim, ya: sejak Oktober 2026, Twitch hanya mengizinkan klaim drop jika akun game sudah tertaut. Secara default, kampanye yang belum tertaut tetap berjalan dan klaimnya menunggu penautan. Lencana “Belum tertaut” menandainya, dan “Tautkan akun game”, di detail kampanye, membuka halaman penautan. Untuk menambang game yang tertaut saja, pilih “Hanya game yang akunnya tertaut”.

### Instalasi dan data

**Aplikasi desktop atau Docker?**\
Aplikasi desktop menambang selama PC Anda menyala, dengan ikon tray, kata sandi opsional, dan pembaruan sekali klik. Docker menambang siang dan malam di mesin yang selalu menyala (VPS, NAS, Raspberry Pi); Anda membukanya di browser, dilindungi oleh `MINER_PASSWORD`. Antarmukanya sama.

**Di mana data saya?**\
Di folder data: `~/.local/share/twitch-miner` di Linux, `%APPDATA%\twitch-miner` di Windows, `~/Library/Application Support/twitch-miner` di macOS, dan volume `/data` dengan Docker. Di desktop, **Pengaturan › Desktop** membukanya dengan sekali klik. Isinya `miner.db` (akun, pengaturan, jurnal), `master.key` (kunci sesi, jika kata sandi aplikasi tidak diatur), dan log.

**Bagaimana cara memperbarui?**\
Aplikasi desktop memeriksa versi baru saat dimulai, lalu setiap jam (Anda bisa mengubahnya di **Pengaturan › Tentang**). Klik lencana “Pembaruan”, lalu “Pasang dan mulai ulang”. Dengan Docker, dasbor memberi tahu Anda: jalankan `docker compose pull`, lalu `docker compose up -d`.

**Saya lupa kata sandi aplikasi.**\
Kata sandi itu tidak bisa dipulihkan: justru kata sandi itulah yang mengenkripsi sesi Twitch Anda. Tutup aplikasi, hapus folder data, lalu tambahkan lagi akun Anda; pengaturannya harus dibuat ulang. Dengan Docker, kata sandi dasbor diubah di `MINER_PASSWORD`, lalu `docker compose up -d`.

### Poin dan prediksi

**Berapa poin per jam?**\
Tergantung Twitch dan tiap channel. Penambang mendapat apa yang Twitch berikan kepada penonton: poin waktu tonton, peti bonus (sekitar satu setiap 15 menit), dan bonus streak tonton. Langganan melipatgandakan poin, dari ×1,2 sampai ×2 tergantung tingkatannya. Halaman Beranda menampilkan poin yang didapat, dan Jurnal setiap perolehannya.

**Apakah taruhan berisiko?**\
Ya, poin bisa hilang. Prediksi nonaktif secara default, per streamer. Anda mengatur strateginya, kapan bertaruh, besar taruhan (5% dari saldo secara default, paling banyak 50.000 poin), saldo minimum, dan, jika mau, sebuah syarat, misalnya odds minimal 1,5. Jaga taruhan tetap kecil.

**Sebuah streamer tidak ditonton.**\
Hanya streamer yang dinyalakan dan sedang live yang ditonton, dan hanya di channel yang kosong: di mode **Drops dan poin**, salah satu dari dua channel melayani kampanye saat ini; di mode **Hanya Drops**, tidak ada streamer yang ditonton; akun yang dijeda tidak menonton apa pun. Slot diisi dengan urutan ini, yang tampil sebagai lencana di setiap channel yang ditonton: “Bonus streak”, “Streak terancam”, “Berlangganan”, lalu “Urutan daftar”. Naikkan streamer itu di daftar, atau matikan “Streak tonton lebih dulu” dan “Channel langganan lebih dulu”.

## <a name="related-projects"></a>Proyek terkait

Proyek lain dari pembuat yang sama:

- [**Twitch Drops API**](https://drops.sunkwi.com): API publik gratis untuk semua kampanye Twitch Drops dan kampanye hadiah yang sedang berjalan, diperbarui setiap menit. Aplikasi ini membaca katalog kampanyenya dari sana.
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com): bot Discord yang mengumumkan live Twitch dan kampanye Drops di server Anda.
- [**Kick Alerts 🔔**](https://kick.sunkwi.com): bot Discord yang mengumumkan live Kick dan kampanye Drops di server Anda.

## <a name="security"></a>Keamanan

- Token disegel dengan AES-256-GCM. Kuncinya berupa `master.key` acak di samping database (default, tidak perlu melakukan apa pun) atau diturunkan dari kata sandi aplikasi dengan Argon2id.
- Dasbor HTTP memeriksa kata sandi dalam waktu konstan dan memakai cookie HttpOnly, SameSite=Strict (`Secure`, dengan awalan `__Host-`, di belakang HTTPS). Server hanya menyimpan SHA-256 dari setiap sesi. Sesi diakhiri setelah 30 hari atau saat keluar. Server memeriksa sesi sebelum membaca body request apa pun dan membatasi body hingga 256 KiB.
- Setelah 5 kata sandi salah berturut-turut dari satu alamat IP (formulir masuk atau bearer), alamat itu harus menunggu 30 detik. Server menjawab `429` dengan `Retry-After` tanpa melihat kata sandinya. Setiap kata sandi salah juga butuh setengah detik untuk dijawab.
- Di luar jaringan Anda, taruh dasbor di belakang reverse proxy HTTPS dengan `MINER_PUBLIC_URL` (lihat [Docker](#docker)).
- Penambang hanya berkomunikasi dengan `twitch.tv`, `ttvnw.net`, `jtvnw.net`, `twitchcdn.net`, `drops.sunkwi.com`, dan tujuan notifikasi yang Anda atur. Aplikasi (desktop dan Docker) memeriksa versi baru di `github.com`, dan aplikasi desktop mengunduhnya dari GitHub releases proyek. Versi itu ditandatangani: hanya versi yang ditampilkan yang dipasang.

## <a name="build"></a>Build dari sumber

**Prasyarat:** Rust, Node.js, dan [prasyarat Tauri](https://v2.tauri.app/start/prerequisites/) untuk sistem Anda.

```bash
npm ci
npm run build          # dasbor
cargo build --release -p miner-server   # server: target/release/twitch-miner
npm run bundle:linux   # bundle desktop (butuh dependensi Linux untuk Tauri)
```

Pemeriksaan kode:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier, oxlint (sadar tipe), dan svelte-check
```

Tipe TypeScript antarmuka dibuat dari tipe Rust dengan [ts-rs](https://github.com/Aleph-Alpha/ts-rs) ke dalam `ui/src/lib/bindings`. Setelah mengubah tipe yang dipakai antarmuka, jalankan `npm run bindings`.

## <a name="license"></a>Lisensi

[MIT](../../LICENSE). Terinspirasi dari [TwitchDropsMiner karya DevilXD](https://github.com/DevilXD/TwitchDropsMiner), [Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2), dan fork-fork mereka.
