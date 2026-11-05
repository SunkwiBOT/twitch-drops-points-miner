<p align="center">
  <img src="../../ui/public/app-icon.svg" width="112" height="112" alt="Twitch Miner logosu">
</p>

<h1 align="center">Twitch Miner</h1>

<p align="center">
  <strong>Birden fazla hesapta, tarayıcı ve açık bir Twitch sekmesi olmadan Twitch Drops ve kanal puanları kazar.</strong>
</p>

<p align="center">
  Linux, Windows ve macOS için bir masaüstü uygulaması ya da ev sunucusunda veya Raspberry Pi üzerinde çalıştırmak için aynı web paneline sahip bir Docker imajı. 4 hesapla sunucu yaklaşık 30 MB bellek kullanır ve işlemciyi neredeyse hiç yormaz.
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-miner?label=release&color=9146FF" alt="Son sürüm"></a>
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-miner/total?label=downloads&color=9146FF" alt="İndirmeler"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="Platformlar: Linux, Windows, macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="amd64 ve arm64 için Docker imajı"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="MIT lisansı"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="Görüntülenme">
</p>

<p align="center">
  <a href="#download"><strong>İndir</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-miner/issues/new/choose">Hata bildir</a>
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
  <strong>Türkçe</strong> ·
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
  <img src="../media/tr/presentation.avif" alt="Twitch Miner'ın video tanıtımı">
</p>

## <a name="features"></a>Özellikler

Onu diğer madencilerden ayıran şeyler. Bir maddenin altındaki “Çalışırken görün”, o özelliği iş başında gösterir.

### Hesaplar ve yayıncılar

- **Tek uygulamada birden fazla hesap.** Her hesap kendi başına, kendi modunda kazar: **Drops ve puan** (bir kanal en iyi kampanya için, diğeri yayıncılarınız için), **Yalnızca Drops** veya **Yalnızca puan**.

  <details open><summary>Çalışırken görün</summary>
  <img src="../media/tr/accounts-mining.avif" alt="Ana sayfada aynı anda madencilik yapan birkaç hesap">
  </details>

- **Her yeni hesap için bir sihirbaz.** Giriş yaptıktan sonra varsayılan ayarlardan ya da başka bir hesabın ayarlarından başlayın. Modu, kampanyaları ve yayıncıları seçin, ardından “Kazmaya başla” veya “Kaydet ve sonra başlat” düğmesine basın.

  <details open><summary>Çalışırken görün</summary>
  <img src="../media/tr/account-setup.avif" alt="Yeni bir hesabın kurulum sihirbazı, adım adım">
  </details>

- **Tek tıkla başka hesaplara da uygulanan bir değişiklik.** Bir hesapta bir şeyi değiştirdiğinizde onay mesajı “Diğer hesaplara uygula” seçeneğini sunar. Değişiklik, işaretlediğiniz hesaplarda da yapılır; bu hesapların diğer ayarları olduğu gibi kalır.

  <details open><summary>Çalışırken görün</summary>
  <img src="../media/tr/apply-to-accounts.avif" alt="“Diğer hesaplara uygula” ile başka hesaplarda da yapılan bir değişiklik">
  </details>

- **Yayıncı ekleme.** Yayıncı varsayılan seçeneklerle hemen başlar: uygulamanın seçenekleriyle ya da bu hesap için seçtiklerinizle. Gerekirse onları hemen orada ayarlayın: bonus sandıkları, baskınlar, anlar, izleme serisi, sohbet, tahminler.

  <details open><summary>Çalışırken görün</summary>
  <img src="../media/tr/streamer-add.avif" alt="Twitch aramasıyla bir yayıncı ekleme, ardından seçenekleri">
  </details>

- **Yayıncı düzenleme.** “Düzenle” yayıncının seçeneklerini açar: bonus sandıkları, baskınlar, anlar, izleme serisi, sohbet, tahminler… Bir anahtar, yayıncıyı listeden çıkarmadan açar veya kapatır.

  <details open><summary>Çalışırken görün</summary>
  <img src="../media/tr/streamer-edit.avif" alt="Bir yayıncının kendi satırından açılan seçenekleri">
  </details>

- **Birkaç yayıncıyı aynı anda düzenleme.** Yayıncıları işaretleyin, ardından “Seçili N yayıncıyı düzenle” düğmesine basın. Yalnızca değiştirdiğiniz seçenekler uygulanır. Aralarında farklı olan bir seçenek “Karışık” olarak görünür ve anahtarı “Evet” ve “Hayır” adlı iki düğmeye dönüşür. Uygulama ardından bunları yeni yayıncılar için varsayılan seçenekler yapmayı önerir.

  <details open><summary>Çalışırken görün</summary>
  <img src="../media/tr/streamers-group-edit.avif" alt="“Karışık” seçeneklerle birkaç yayıncıyı düzenleme">
  </details>

- **Yayıncıları içe aktarma.** Bir liste yapıştırın (en fazla 500 ad) ya da başka bir hesabın listesini veya hesabın takip ettiği kanalları kullanın. Liste denetlenir, böylece yalnızca gerçekten var olan kanallar eklenir.

  <details open><summary>Çalışırken görün</summary>
  <img src="../media/tr/streamers-import.avif" alt="Yayıncıları içe aktarma: yapıştırılan liste, başka bir hesap, takip edilen kanallar">
  </details>

- **Spam Points.** Bir kanalın özel ödülünü art arda birkaç kez kullanın. Puan listenizden bir kanal ya da Twitch aramasıyla bulunan herhangi bir kanal seçin: ödülleri, canlı olarak yeniden okunan hesap bakiyesiyle birlikte görünür. Adedi ayarlayın (bakiyenin yettiği kadar, en fazla 25), ardından “Kullan”. Sonuç seri bittiğinde görünür ve her serinin Günlük'te kendi satırı olur.

  <details open><summary>Çalışırken görün</summary>
  <img src="../media/tr/spam-points.avif" alt="Spam Points: seçilen bir kanal, 10 × kullanılan mesajlı bir ödül, kullanımlar kanalın Twitch sohbetine düşüyor, ardından Günlük satırı">
  </details>

### Drops

- **Süren tüm Drops kampanyaları.** Her kampanyanın kendi ayrıntıları vardır: ödüller, oyun hesabı bağlama, “Önce kaz”. Liste [Twitch Drops API](https://drops.sunkwi.com) projesinden gelir ve 10 dakikada bir yenilenir.

  <details open><summary>Çalışırken görün</summary>
  <img src="../media/tr/drops-catalogue.avif" alt="Takvim görünümündeki Katalog, arama ve bir kampanyanın ayrıntıları">
  </details>

- **Kendi kazma sıranız.** Kampanyaları istediğiniz sıraya sürükleyin. Yeni kampanyalar yerlerini kendileri bulur: önce öncelikli oyunlarınız, ardından en erken bitenler ya da en az dakikası kalanlar. Oyunları, kelimeleri, bir kampanyayı veya tek bir ödülü hariç tutun ve zamanında bitemeyecek kampanyaları atlayın. Bir kampanyanın ayrıntılarındaki “Önce kaz”, onu en başa taşır.

  <details open><summary>Çalışırken görün</summary>
  <img src="../media/tr/drops-order.avif" alt="Kazma sırası, öncelikli oyunlar ve hariç tutmalar">
  </details>

- **Envanter ve “Tümünü al”.** Envanter sayfası ödülleri “Alınacaklar”, “Devam edenler” ve “Alınanlar” olarak ayırır. Ödülleri tek tek ya da hepsini birden alın veya “Drops'u otomatik al” seçeneğini açın.

  <details open><summary>Çalışırken görün</summary>
  <img src="../media/tr/drops-inventory.avif" alt="Envanter ve “Tümünü al” düğmesi">
  </details>

- **“Drops için önce benim yayıncılarım”.** Geçerli kampanyada, listenizdeki yayıncılardan yayını o kampanyaya sayılan biri diğer kanallardan önce gelir. Aynı kanal hem drop'u ilerletir hem de size kanal puanlarını kazandırır. Varsayılan olarak açıktır.

  <details open><summary>Çalışırken görün</summary>
  <img src="../media/tr/drops-streamers-first.avif" alt="Listeden bir yayıncı, hem Drops hem de puanları için izleniyor">
  </details>

### Takip

- **Her şeyin günlüğü ve her istek.** Günlük her olayı tutar: drop'lar, kazanılan ve harcanan puanlar, sandıklar, bahisler, baskınlar, başlayan yayınlar… Onu hesaba, sekmeye (Drops, Puan, Diğer) ve türe göre filtreleyin. **Teknik kayıtlar › İstekler**, Twitch'e ve diğer hizmetlere gönderilen her isteği sonucu ve süresiyle birlikte listeler. Kayıtlar ve istekler hiçbir token veya gizli bilgi içermez: “Hata raporu için kopyala” onları kopyalar; [GitHub](https://github.com/SunkwiBOT/twitch-miner/issues) veya [Discord](https://discord.gg/support-bots-891685895781113886) üzerinde bir hata raporuna yapıştırmaya hazırdır.

  <details open><summary>Çalışırken görün</summary>
  <img src="../media/tr/journal-requests.avif" alt="Filtrelenmiş Günlük, ardından Teknik kayıtlar › İstekler">
  </details>

### Masaüstünde veya bir sunucuda

- **Her yerde aynı arayüz.** Masaüstü uygulaması (Linux, Windows, macOS) ve Docker imajı aynı paneli gösterir. Docker bir VPS, NAS veya Raspberry Pi üzerinde, o makinede ekran ya da tarayıcı olmadan çalışır: panel herhangi bir tarayıcıda açılır.
- **Hafif.** 4 hesap için yaklaşık 30 MB bellek, neredeyse hiç işlemci kullanımı ve yaklaşık 12 MB'lık bir Docker imajı.

### Güvenlik ve güncellemeler

- **Oturumlarınız korunur.** Twitch şifresi yok: tarayıcınızdan `twitch.tv/activate` adresinde giriş yaparsınız. Oturumlar diskte şifrelenir ve onları [Twitch ayarlarınızdan](https://www.twitch.tv/settings/connections) iptal edebilirsiniz. Masaüstünde isteğe bağlı bir uygulama şifresi uygulamayı kilitler: şifre yazılana kadar hiçbir şey kazılmaz. Docker'da panelin kendi şifresi vardır; panel birkaç dakika işlem yapılmayınca kendini kilitler ve bir proxy arkasında HTTPS üzerinden çalışır. Art arda beş yanlış şifre, IP adresini 30 saniyeliğine engeller.

  <details open><summary>Çalışırken görün</summary>
  <img src="../media/tr/security-lock.avif" alt="Asma kilit: kilitli uygulama, ardından şifresiyle açılması">
  </details>

- **İmzalı güncellemeler, asla otomatik değil.** Üst çubukta sürüm numarasıyla birlikte bir “Güncelleme” rozeti belirir. “Yükle ve yeniden başlat” önce onay ister, ardından uygulama sürümü yükler ve yeniden başlar. Uygulama şifresi varsa uygulama kilitli olarak yeniden başlar: kazmaya devam etmek için şifreyi yeniden yazın. Yalnızca proje tarafından imzalanmış bir sürüm yüklenir. Docker sürümü de çalıştırılacak komutlarla birlikte haber verir.

  <details open><summary>Çalışırken görün</summary>
  <img src="../media/tr/updates.avif" alt="“Güncelleme” rozeti, onay, ardından yükleme">
  </details>

### Ve ayrıca

- Diğer madencilerde olduğu gibi: bonus sandıkları, baskınlar, anlar, topluluk hedefleri, tahminler (stratejiler, bahis zamanı, koşullar), önce izleme serileri ve abone olunan kanallar, sohbette bulunma (kapatılabilir), yeni yayıncılar için varsayılan seçenekler, resimli Discord, Telegram ve webhook bildirimleri, genel veya hesap başına proxy.
- **Yakında: rozet madenciliği.** İzleyerek kazanılan Twitch rozetleri, Drops gibi kazılır.
- 29 dil, koyu ve açık tema, masaüstü bildirimleri, sistem tepsisi simgesi, sistemle birlikte başlatma.

  <details open><summary>Çalışırken görün</summary>
  <img src="../media/tr/basics.avif" alt="Diller, açık ve koyu tema, bir masaüstü bildirimi ve sistem tepsisi simgesi">
  </details>

> [!WARNING]
> Bu proje Twitch ile bağlantılı değildir. Bir hesabı otomatikleştirmek Twitch'in koşullarını veya bir kampanyanın kurallarını ihlal edebilir.

## İçindekiler

- [Karşılaştırma](#comparison)
- [İndirme](#download)
- [Docker](#docker)
- [Nasıl çalışır](#how-it-works)
- [Sık sorulan sorular](#faq)
- [İlgili projeler](#related-projects)
- [Güvenlik](#security)
- [Kaynaktan derleme](#build)
- [Lisans](#license)

## <a name="comparison"></a>Karşılaştırma

[Twitch Miner](https://github.com/SunkwiBOT/twitch-miner) ile [Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2) (kanal puanları) ve [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner) (Drops) arasındaki karşılaştırma. Kasım 2026'da bu projelerin kodu ve README dosyaları üzerinden doğrulandı.

| | Bu proje | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Drops kampanyaları: bulur, kazar ve alır | ✅ | ❌ ¹ | ✅ |
| Kanal puanları: sandıklar, baskınlar, anlar, tahminler | ✅ | ✅ | ❌ |
| Spam Points: bir kanalın özel ödülü art arda birkaç kez kullanılır | ✅ | ❌ | ❌ |
| Tek hesapta aynı anda Drops ve kanal puanları | ✅ | kısmen ¹ | ❌ |
| Tek uygulamada birden fazla hesap | ✅ | ❌ ² | ❌ |
| Birden fazla hesabın ayarlarını aynı anda değiştirme | ✅ | ❌ | ❌ |
| Grafik arayüz | ✅ masaüstü ve web ³ | ❌ ⁴ | ✅ masaüstü |
| Ev sunucusu için web paneli (Docker), şifreli ve HTTPS üzerinden erişilebilir | ✅ | ❌ ⁴ | ❌ |
| Diskte şifrelenmiş oturumlar, isteğe bağlı uygulama şifresi | ✅ | ❌ | ❌ |
| 29 dilde arayüz | ✅ | ❌ | kısmen ⁵ |
| Uygulama içinden imzalı güncellemeler (masaüstü) | ✅ | ❌ | ❌ |
| Her yeni hesap için kurulum sihirbazı | ✅ | ❌ ⁴ | ❌ |
| Arayüzden tek tek veya toplu olarak eklenen ve düzenlenen yayıncılar | ✅ | ❌ ⁴ | — |
| Aramayla, bir listeden, başka bir hesaptan veya takip edilen kanallardan içe aktarılan yayıncılar | ✅ | kısmen ⁶ | — |
| Resimli Discord, Telegram ve webhook bildirimleri | ✅ | kısmen ⁷ | ❌ |
| Tüm Drops kampanyalarının aranabilir takvimi | ✅ | ❌ | kısmen ⁸ |
| Kampanya bazında kazma sırası, hariç tutulan kelimeler, yok sayılan kampanya veya ödül | ✅ | ❌ | ❌ ⁹ |
| Bir kampanyanın Drops'ları için önce listenizdeki yayıncılar izlenir | ✅ | kısmen ¹⁰ | ❌ |
| Arayüzde filtrelenebilir olay günlüğü ve her isteğin listesi | ✅ | ❌ ⁴ | ❌ |
| Yaklaşık 30 MB bellek (Docker sunucusu, 4 hesap) | ✅ | — ¹¹ | — ¹¹ |

¹ Yalnızca listesindeki yayıncıları izler: kampanya seçmeden, onların yayınlarındaki Drops'ları ilerletir ve alır.\
² Hesap başına bir süreç.\
³ Bilgisayarınızda masaüstü uygulaması olarak ve/veya Docker ile bir sunucuda (VPS, NAS, Raspberry Pi); o makinede grafik ortam ve tarayıcı gerekmez: panel herhangi bir tarayıcıda açılır.\
⁴ Bir Python dosyasında yapılandırılır; web sayfası puan grafiklerini ve kayıt dosyasını gösterir.\
⁵ 21 dil.\
⁶ Hesabın takip ettiği kanalları takip edebilir.\
⁷ Yalnızca metin, resim yok. Ayrıca Matrix, Pushover ve Gotify desteği var; bunlar bu projede yok.\
⁸ Filtreli bir kampanya listesi; takvim veya arama yok.\
⁹ Oyun adına göre seçilen öncelikli oyunları ve hariç tutulan oyunları vardır.\
¹⁰ Varsayılan olarak önce listesindeki yayıncılardan yayınında Drops olanları izler (`DROPS` önceliği), ama hiçbir zaman başka bir kanalı izlemez.\
¹¹ Ölçülmedi.

## <a name="download"></a>İndirme

Sisteminize uygun dosyayı [son sürümden](https://github.com/SunkwiBOT/twitch-miner/releases/latest) indirin.

| Sistem | Paket |
|---|---|
| **Windows** · x64 | Yükleyici (`setup.exe`) veya [taşınabilir çalıştırılabilir dosya](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/TwitchMiner-portable.exe) |
| **macOS** 10.15 veya üstü · Intel ve Apple Silicon | Disk görüntüsü (`.dmg`) |
| **Linux** · x86-64 | `.AppImage`, `.deb` (Debian, Ubuntu) veya `.rpm` (Fedora, openSUSE) |

macOS derlemesi notarize edilmemiştir. İlk seferde uygulamaya sağ tıklayıp **Aç** komutunu seçin.

Güncellemeler imzalıdır. Uygulama açılırken, ardından saatte bir yeni bir sürüm olup olmadığını denetler (sıklığı **Ayarlar › Hakkında** bölümünden değiştirebilirsiniz). Hiçbir sürümü kendiliğinden yüklemez: güncellemeyi, beliren mesajdan ya da **Ayarlar › Hakkında** bölümünden tek tıkla siz yüklersiniz.

Veriler kullanıcı veri klasöründe bulunur:

- Linux: `~/.local/share/twitch-miner`
- Windows: `%APPDATA%\twitch-miner`
- macOS: `~/Library/Application Support/twitch-miner`

## <a name="docker"></a>Docker

Sunucu aynı paneli bir tarayıcıda gösterir. İmajın adı `ghcr.io/sunkwibot/twitch-miner`; `amd64` ve `arm64` için sunulur. Yeni bir sürüm çıktığında panel bunu söyler: `docker compose pull`, ardından `docker compose up -d`.

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-miner:latest
    pull_policy: always   # her başlangıçta en yeni imajı çeker
    restart: unless-stopped
    ports: ["8080:8080"]   # HTTPS ters proxy arkasında: kaldırın ya da "127.0.0.1:8080:8080" yapın
    environment:
      MINER_PASSWORD: "change-me"   # zorunlu, 12 karakter veya daha uzun
      # MINER_PUBLIC_URL: "https://miner.example.com"   # yalnızca HTTPS ters proxy arkasında
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

<http://localhost:8080> adresini (veya makinenin ağınızdaki adresini) açın ve şifreyi girin.

### Değişkenler

| Değişken | Varsayılan | Amaç |
|---|---|---|
| `MINER_PASSWORD` | yok | Panel şifresi, zorunludur: sunucu onsuz başlamayı reddeder. 12 karakter veya daha uzun olmalıdır (daha kısa bir şifre başlangıçta bir uyarı gösterir). Örnek değer `change-me` reddedilir. |
| `MINER_PASSWORD_FILE` | yok | `MINER_PASSWORD` değerini bir dosyadan, örneğin bir Docker secret'tan okur. İkisinden yalnızca biri, ikisi birden değil. |
| `MINER_PUBLIC_URL` | yok | Yalnızca bir HTTPS ters proxy (Caddy, nginx, Traefik, Cloudflare Tunnel…) arkasında: panelin `https://` adresi, yol olmadan; örneğin `https://miner.example.com`. Proxy `X-Forwarded-Proto`, `X-Forwarded-For` başlıklarını ve websocket'leri iletmelidir (yaygın proxy'ler bunu varsayılan olarak yapar) ve container'ın portuna doğrudan ulaşılamamalıdır. Panel bu durumda proxy üzerinden HTTPS ile gelmeyen her şeyi reddeder ve 5 yanlış şifreden sonra bir adresi 30 saniyeliğine kilitler. Proxy aynı makinede veya onun yerel ağında çalışmalıdır: genel bir adresten gelen bağlantı reddedilir. |
| `MINER_TRUSTED_HOPS` | `1` | Yalnızca `MINER_PUBLIC_URL` ile: `X-Forwarded-For` başlığına girdi ekleyen proxy sayısı, 1 ile 10 arasında. Art arda iki proxy varsa, örneğin Traefik'in önünde Cloudflare, `2`. |
| `MINER_PROXY_NETWORKS` | yok | Yalnızca `MINER_PUBLIC_URL` ile: bu makine ve onun özel yerel ağı dışında proxy'nin bağlanabileceği diğer ağlar, virgülle ayrılmış: tek bir adres ya da `2001:db8:1::/64` gibi bir ağ; IPv6 adresleri genel olan bir yerel ağ için. |
| `MINER_VAULT_PASSPHRASE` | yok | Yalnızca uygulama şifresi olan masaüstü uygulamasından kopyalanmış bir veri klasörü için: başlangıçta klasörün kilidini açar. Sıfırdan kurulan bir sunucunun buna hiç ihtiyacı olmaz. |
| `MINER_VAULT_PASSPHRASE_FILE` | yok | `MINER_VAULT_PASSPHRASE` değerini bir dosyadan okur. İkisinden yalnızca biri, ikisi birden değil. |
| `DATA_DIR` | imajda `/data`, aksi hâlde çalışma dizinindeki `data` | Veritabanı, anahtar dosyası ve kayıtlar. |
| `LOG_DIR` | `$DATA_DIR/logs` | Kayıt dosyaları: günde bir tane, son 14 tanesi saklanır. |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | Dinleme adresi: bir IP adresi ve 1 ile 65535 arasında bir port. |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | Kayıt düzeyi. |

Sunucu değişkenleri başlangıçta bir kez okur ve denetler. Geçersiz bir değer, sessizce varsayılana dönmek yerine sunucuyu bir mesajla durdurur.

## <a name="how-it-works"></a>Nasıl çalışır

Her hesap 20 saniyede bir iki kanalını seçer: kuyruğunun başındaki kampanya için en iyi canlı kanal, ardından puan listesindeki yayıncılar.

Twitch, hesap başına aynı anda **bir Drops kampanyası** ve **puanlar için iki kanal** sayar. Ekim 2026'dan beri bir drop yalnızca oyun hesabı bağlıysa alınabilir. Bağlı olmayan kampanyalar yine de ilerler ve panel bağlantıyı gösterir.

Kampanya listesi [Twitch Drops API](https://drops.sunkwi.com) projesinden gelir (10 dakikada bir yenilenir), çünkü Twitch onu TV istemcisinden gizler.

## <a name="faq"></a>Sık sorulan sorular

### Riskler ve Twitch

**Hesabım yasaklanabilir mi?**\
Kimse yasaklanmayacağını garanti edemez. Bu proje Twitch ile bağlantılı değildir ve bir hesabı otomatikleştirmek Twitch'in koşullarını veya bir kampanyanın kurallarını ihlal edebilir. Madenci hesap başına en fazla iki kanal izler, bu da Twitch'in sınırıdır, ve hiçbir video indirmez. Karar sizin.

**Video gerçekten izleniyor mu?**\
Hayır, hiçbir video indirilmez. Drops için madenci 15 saniyede bir yayının oynatma listesini en düşük kalitede okur ve yalnızca her yeni segmentin başlığını ister. Puanlar için her dakika Twitch'in “izlenen dakika” sinyalini gönderir. İzlenen her kanal kaç segment istendiğini gösterir.

**Neden en fazla iki kanal?**\
Twitch'in sınırı bu: hesap başına yalnızca aynı anda izlenen iki kanalı ve tek seferde yalnızca bir Drops kampanyasını sayar. **Drops ve puan** modunda bir kanal kampanyaya, diğeri yayıncılarınıza hizmet eder. **Hesap ayarları** içindeki “Tek kanal” ayarı daha az dikkat çeker.

### Drops

**Bir kampanya kazılmıyor veya bir drop ilerlemiyor.**\
Hesabın **Drops** sayfasını açın. “Kazma sırası” bölümünde bekleyen her kampanya nedenini söyler: “Canlı kanal yok”, “Bekliyor (tek seferde bir kampanya)” veya “Henüz drop yok”. Bu listede görünmeyen bir kampanya dışarıda bırakılmıştır: “Hangi kampanyalar” ayarı ya da **Hariç tutmalar** sekmesindeki bir kural yüzünden veya zamanında bitemeyeceği için. Abonelik gerektiren bir ödül hiçbir zaman kazılmaz ve başka bir drop'un ardından gelen bir drop, ilki bitene kadar bekler. **Yalnızca puan** modunda madenci hiçbir kampanya seçmez. Bir kampanyanın ayrıntılarındaki “Önce kaz” onu en başa taşır; yok sayılan bir kampanya, **Hariç tutmalar** sekmesindeki “Yeniden kaz” ile geri gelebilir.

**Oyun hesabını bağlamam gerekiyor mu?**\
Ödülü almak için evet: Ekim 2026'dan beri Twitch, bir drop yalnızca oyun hesabı bağlıysa alınmasına izin veriyor. Varsayılan olarak bağlı olmayan bir kampanya yine de ilerler ve alma işlemi bağlantıyı bekler. “Bağlı değil” rozeti bunu belirtir; kampanyanın ayrıntılarındaki “Oyun hesabını bağla” da bağlama sayfasını açar. Yalnızca bağlı oyunları kazmak için “Yalnızca hesabı bağlı oyunlar” seçeneğini seçin.

### Kurulum ve veriler

**Masaüstü uygulaması mı, Docker mı?**\
Masaüstü uygulaması bilgisayarınız açıkken kazar; sistem tepsisi simgesi, isteğe bağlı şifresi ve tek tıkla güncellemeleri vardır. Docker, sürekli açık duran bir makinede (VPS, NAS, Raspberry Pi) gece gündüz kazar; onu bir tarayıcıda, `MINER_PASSWORD` ile korunarak açarsınız. Arayüz aynıdır.

**Verilerim nerede?**\
Veri klasöründe: Linux'ta `~/.local/share/twitch-miner`, Windows'ta `%APPDATA%\twitch-miner`, macOS'ta `~/Library/Application Support/twitch-miner`, Docker'da ise `/data` birimi. Masaüstünde **Ayarlar › Masaüstü** onu tek tıkla açar. İçinde `miner.db` (hesaplar, ayarlar, günlük), `master.key` (uygulama şifresi ayarlanmamışsa oturum anahtarı) ve kayıtlar bulunur.

**Nasıl güncellerim?**\
Masaüstü uygulaması açılışta, ardından saatte bir yeni sürüm olup olmadığını denetler (bunu **Ayarlar › Hakkında** bölümünden değiştirebilirsiniz). “Güncelleme” rozetine, ardından “Yükle ve yeniden başlat” düğmesine tıklayın. Docker'da panel size haber verir: `docker compose pull`, ardından `docker compose up -d` komutunu çalıştırın.

**Uygulama şifresini unuttum.**\
Kurtarılamaz: Twitch oturumlarınızı şifreleyen odur. Uygulamadan çıkın, veri klasörünü silin, ardından hesaplarınızı yeniden ekleyin; ayarlarını da baştan yapmanız gerekir. Docker'da panel şifresi `MINER_PASSWORD` içinde değiştirilir, ardından `docker compose up -d` çalıştırılır.

### Puanlar ve tahminler

**Saatte kaç puan?**\
Twitch'e ve her kanala bağlıdır. Madenci, Twitch'in bir izleyiciye verdiğini kazanır: izleme süresi puanları, bonus sandıkları (yaklaşık 15 dakikada bir) ve izleme serisi bonusu. Abonelik, seviyesine göre puanları ×1,2 ile ×2 arasında katlar. Ana sayfa kazanılan puanları, Günlük ise her kazancı gösterir.

**Bahisler riskli mi?**\
Evet, puan kaybedebilirsiniz. Tahminler varsayılan olarak her yayıncı için ayrı ayrı kapalıdır. Stratejiyi, bahis zamanını, bahis miktarını (varsayılan olarak bakiyenin %5'i, en fazla 50.000 puan), bir asgari bakiyeyi ve isterseniz en az 1,5 oran gibi bir koşulu siz belirlersiniz. Bahis miktarını düşük tutun.

**Bir yayıncı izlenmiyor.**\
Yalnızca açık ve canlı yayında olan yayıncılar, o da yalnızca boş bir kanalda izlenir: **Drops ve puan** modunda iki kanaldan biri geçerli kampanyaya hizmet eder; **Yalnızca Drops** modunda hiçbir yayıncı izlenmez; duraklatılmış bir hesap hiçbir şey izlemez. Yerler şu sırayla dağıtılır ve izlenen her kanalda bir rozet olarak gösterilir: “Seri bonusu”, “Seri tehlikede”, “Abone”, ardından “Liste sırası”. Yayıncıyı listede yukarı taşıyın ya da “Önce izleme serileri” ve “Önce abone olunan kanallar” seçeneklerini kapatın.

## <a name="related-projects"></a>İlgili projeler

Aynı geliştiricinin diğer projeleri:

- [**Twitch Drops API**](https://drops.sunkwi.com): süren tüm Twitch Drops ve ödül kampanyalarını sunan, dakikada bir güncellenen ücretsiz ve herkese açık bir API. Bu uygulama kampanya kataloğunu buradan okur.
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com): Twitch canlı yayınlarını ve Drops kampanyalarını sunucunuzda duyuran bir Discord botu.
- [**Kick Alerts 🔔**](https://kick.sunkwi.com): Kick canlı yayınlarını ve Drops kampanyalarını sunucunuzda duyuran bir Discord botu.

## <a name="security"></a>Güvenlik

- Token'lar AES-256-GCM ile mühürlenir. Anahtar ya veritabanının yanındaki rastgele bir `master.key` dosyasıdır (varsayılan, yapmanız gereken bir şey yok) ya da uygulama şifresinden Argon2id ile türetilir.
- HTTP paneli şifreyi sabit sürede denetler ve HttpOnly, SameSite=Strict çerezleri kullanır (HTTPS arkasında `__Host-` önekiyle birlikte `Secure`). Sunucu her oturumun yalnızca SHA-256 özetini tutar. Bir oturumu 30 gün sonra veya çıkışta sonlandırır. Herhangi bir istek gövdesini okumadan önce oturumu denetler ve gövdeleri 256 KiB ile sınırlar.
- Bir IP adresinden art arda 5 yanlış şifre girilirse (giriş formu veya bearer), o adres 30 saniye bekler. Sunucu şifreye bakmadan `Retry-After` ile `429` yanıtı verir. Her yanlış şifrenin yanıtı ayrıca yarım saniye sürer.
- Ağınızın dışında paneli `MINER_PUBLIC_URL` ile bir HTTPS ters proxy'nin arkasına koyun ([Docker](#docker) bölümüne bakın).
- Madenci yalnızca `twitch.tv`, `ttvnw.net`, `jtvnw.net`, `twitchcdn.net`, `drops.sunkwi.com` ve yapılandırdığınız bildirim hedefleriyle konuşur. Uygulama (masaüstü ve Docker) yeni sürümleri `github.com` üzerinde denetler; masaüstü uygulaması bunları projenin GitHub sürümlerinden indirir. Sürümler imzalıdır: yalnızca gösterilen sürüm yüklenir.

## <a name="build"></a>Kaynaktan derleme

**Ön koşullar:** Rust, Node.js ve sisteminiz için [Tauri ön koşulları](https://v2.tauri.app/start/prerequisites/).

```bash
npm ci
npm run build          # panel
cargo build --release -p miner-server   # sunucu: target/release/twitch-miner
npm run bundle:linux   # masaüstü paketleri (Tauri'nin Linux bağımlılıkları gerekir)
```

Kod denetimleri:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier, oxlint (tür bilgisiyle) ve svelte-check
```

Arayüzün TypeScript türleri, Rust türlerinden [ts-rs](https://github.com/Aleph-Alpha/ts-rs) ile `ui/src/lib/bindings` içine üretilir. Arayüzün gördüğü bir türü değiştirdikten sonra `npm run bindings` komutunu çalıştırın.

## <a name="license"></a>Lisans

[MIT](../../LICENSE). [DevilXD'nin TwitchDropsMiner'ından](https://github.com/DevilXD/TwitchDropsMiner), [Twitch-Channel-Points-Miner-v2'den](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2) ve bunların fork'larından ilham alınmıştır.
