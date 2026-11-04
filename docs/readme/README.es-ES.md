<p align="center">
  <img src="../../ui/public/app-icon.svg" width="112" height="112" alt="Logotipo de Twitch Drops &amp; Points Miner">
</p>

<h1 align="center">Twitch Drops &amp; Points Miner</h1>

<p align="center">
  <strong>Mina Drops de Twitch y puntos del canal en varias cuentas, sin navegador y sin ninguna pestaña de Twitch abierta.</strong>
</p>

<p align="center">
  Una aplicación de escritorio para Linux, Windows y macOS, o una imagen de Docker con el mismo panel web, para ejecutarla en un servidor doméstico o en una Raspberry Pi. Con 4 cuentas, el servidor usa unos 30 MB de memoria y casi nada de CPU.
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-drops-points-miner?label=release&color=9146FF" alt="Última versión"></a>
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-drops-points-miner/total?label=downloads&color=9146FF" alt="Descargas"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="Plataformas: Linux, Windows, macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-drops-points-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="Imagen de Docker para amd64 y arm64"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="Licencia MIT"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="Visitas">
</p>

<p align="center">
  <a href="#download"><strong>Descargar</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/issues/new/choose">Informar de un error</a>
</p>

<p align="center">
  <a href="../../README.md">English</a> ·
  <a href="README.fr.md">Français</a> ·
  <a href="README.de.md">Deutsch</a> ·
  <strong>Español&nbsp;(España)</strong> ·
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
  <a href="README.id.md">Bahasa&nbsp;Indonesia</a>
</p>

<p align="center">
  <img src="../media/en/presentation.webp" alt="Presentación en vídeo de Twitch Drops &amp; Points Miner">
</p>

## <a name="features"></a>Funciones

Lo que lo diferencia de otros mineros. «Verlo en acción», debajo de un punto, lo muestra
funcionando.

### Cuentas y streamers

- **Varias cuentas en una sola app.** Cada cuenta mina de forma independiente, en su
  propio modo: **Drops y puntos** (un canal para la mejor campaña y el otro para tus
  streamers), **Solo Drops** o **Solo puntos**.

  <details><summary>Verlo en acción</summary>
  <img src="../media/en/accounts-mining.webp" alt="Varias cuentas minando a la vez, en la página Inicio">
  </details>

- **Un asistente para cada cuenta nueva.** Tras iniciar sesión, parte de los ajustes por
  defecto o de los de otra cuenta. Elige el modo, las campañas y los streamers y, al
  final, «Empezar a minar» o «Guardar y empezar más tarde».

  <details><summary>Verlo en acción</summary>
  <img src="../media/en/account-setup.webp" alt="El asistente de configuración de una cuenta nueva, paso a paso">
  </details>

- **Un cambio aplicado a otras cuentas con un clic.** Después de cambiar algo en una
  cuenta, el mensaje de confirmación ofrece «Aplicar a otras cuentas». El cambio se repite
  en las cuentas que marques, que conservan el resto de sus ajustes.

  <details><summary>Verlo en acción</summary>
  <img src="../media/en/apply-to-accounts.webp" alt="Un cambio repetido en otras cuentas con «Aplicar a otras cuentas»">
  </details>

- **Añadir un streamer.** Empieza al momento con las opciones por defecto: las de la app
  o las que hayas elegido para esta cuenta. Ajústalas en el acto si hace falta: cofres de
  bonificación, raids, momentos, racha de visualización, chat, predicciones.

  <details><summary>Verlo en acción</summary>
  <img src="../media/en/streamer-add.webp" alt="Un streamer añadido mediante la búsqueda de Twitch y, después, sus opciones">
  </details>

- **Editar un streamer.** «Editar» abre sus opciones: cofres de bonificación, raids,
  momentos, racha de visualización, chat, predicciones… Un interruptor lo activa o lo
  desactiva sin quitarlo de la lista.

  <details><summary>Verlo en acción</summary>
  <img src="../media/en/streamer-edit.webp" alt="Las opciones de un streamer, abiertas desde su fila">
  </details>

- **Editar varios streamers a la vez.** Márcalos y pulsa «Editar los N streamers
  seleccionados». Solo se aplican las opciones que cambies. Una opción que difiere entre
  ellos aparece como «Mixto», y su interruptor se convierte en dos botones, «Sí» y «No».
  Luego la app te propone convertirlas en las opciones por defecto para nuevos streamers.

  <details><summary>Verlo en acción</summary>
  <img src="../media/en/streamers-group-edit.webp" alt="Edición de varios streamers, con opciones «Mixto»">
  </details>

- **Importar streamers.** Pega una lista (hasta 500 nombres) o reutiliza la de otra cuenta
  o los canales que sigue la cuenta. La lista se comprueba para añadir solo canales que
  existen.

  <details><summary>Verlo en acción</summary>
  <img src="../media/en/streamers-import.webp" alt="Importación de streamers: una lista pegada, otra cuenta, los canales seguidos">
  </details>

### Drops

- **Todas las campañas de Drops en curso.** Cada campaña tiene su ficha: recompensas,
  vinculación de la cuenta del juego, «Minar primero». La lista viene del proyecto
  [Twitch Drops API](https://drops.sunkwi.com) y se actualiza cada 10 minutos.

  <details><summary>Verlo en acción</summary>
  <img src="../media/en/drops-catalogue.webp" alt="El Catálogo en forma de calendario, la búsqueda y la ficha de una campaña">
  </details>

- **Tu orden de minado.** Arrastra las campañas al orden que quieras. Las nuevas encuentran
  su sitio solas: primero tus juegos prioritarios y después las que terminan antes o a las
  que les quedan menos minutos. Excluye juegos, palabras, una campaña o una sola
  recompensa, y omite las campañas que no pueden terminar a tiempo. «Minar primero», en la
  ficha de una campaña, la pone en cabeza.

  <details><summary>Verlo en acción</summary>
  <img src="../media/en/drops-order.webp" alt="El orden de minado, los juegos prioritarios y las exclusiones">
  </details>

- **Inventario y «Reclamar todo».** La página Inventario clasifica las recompensas en
  «Por reclamar», «En curso» y «Reclamadas». Reclámalas una a una, todas a la vez, o
  activa «Reclamar drops automáticamente».

  <details><summary>Verlo en acción</summary>
  <img src="../media/en/drops-inventory.webp" alt="El inventario y el botón «Reclamar todo»">
  </details>

- **«Mis streamers primero para los Drops».** En la campaña actual, un streamer de tu
  lista cuyo directo cuenta para esa campaña pasa por delante de los demás canales. El
  mismo canal hace avanzar el drop y te da sus puntos del canal. Activado por defecto.

  <details><summary>Verlo en acción</summary>
  <img src="../media/en/drops-streamers-first.webp" alt="Un streamer de la lista, visto para los Drops y por sus puntos">
  </details>

### Seguimiento

- **Un diario de todo, y cada solicitud.** El Diario guarda cada evento: drops, puntos
  ganados y gastados, cofres, apuestas, raids, directos que empiezan… Fíltralo por cuenta,
  por pestaña (Drops, Puntos, Otros) y por tipo. **Registros técnicos › Solicitudes**
  muestra cada solicitud enviada a Twitch y a otros servicios, con su resultado y su
  duración. Los registros y las solicitudes no contienen ningún token ni secreto:
  «Copiar para un informe de error» los copia, listos para pegarlos en un informe de error
  en [GitHub](https://github.com/SunkwiBOT/twitch-drops-points-miner/issues) o
  [Discord](https://discord.gg/support-bots-891685895781113886).

  <details><summary>Verlo en acción</summary>
  <img src="../media/en/journal-requests.webp" alt="El Diario filtrado y, después, Registros técnicos › Solicitudes">
  </details>

### En el escritorio o en un servidor

- **La misma interfaz en todas partes.** La app de escritorio (Linux, Windows, macOS) y la
  imagen de Docker muestran el mismo panel. Docker funciona en un VPS, un NAS o una
  Raspberry Pi, sin pantalla ni navegador en esa máquina: el panel se abre en cualquier
  navegador.
- **Ligero.** Unos 30 MB de memoria para 4 cuentas, casi nada de CPU y una imagen de
  Docker de unos 12 MB.

### Seguridad y actualizaciones

- **Tus sesiones, protegidas.** Sin contraseña de Twitch: inicias sesión en
  `twitch.tv/activate`, desde tu navegador. Las sesiones se guardan cifradas en el disco
  y puedes revocarlas en tus [ajustes de Twitch](https://www.twitch.tv/settings/connections).
  En el escritorio, una contraseña de la app opcional la bloquea: no se mina nada hasta
  que se escribe. Con Docker, el panel tiene su propia contraseña, se bloquea solo tras
  unos minutos sin actividad y funciona por HTTPS detrás de un proxy. Cinco contraseñas
  incorrectas seguidas bloquean la dirección IP durante 30 segundos.

  <details><summary>Verlo en acción</summary>
  <img src="../media/en/security-lock.webp" alt="El candado: la app bloqueada y, después, desbloqueada con su contraseña">
  </details>

- **Actualizaciones firmadas, nunca automáticas.** En la barra superior aparece una
  insignia «Actualización» con el número de versión. «Instalar y reiniciar» primero te
  pide confirmación; luego la app instala la versión y se reinicia. Si tiene contraseña de
  la app, se reinicia bloqueada: vuelve a escribirla para seguir minando. Solo se instala
  una versión firmada por el proyecto. La versión de Docker también te avisa, con los
  comandos que hay que ejecutar.

  <details><summary>Verlo en acción</summary>
  <img src="../media/en/updates.webp" alt="La insignia «Actualización», la confirmación y, después, la instalación">
  </details>

### Y además

- Como otros mineros: cofres de bonificación, raids, momentos, metas de la comunidad,
  predicciones (estrategias, momento de la apuesta, condiciones), rachas de visualización
  y canales suscritos primero, presencia en el chat (desactivable), opciones por defecto
  para nuevos streamers, notificaciones de Discord, Telegram y webhook con imágenes, proxy
  global o por cuenta.
- 29 idiomas, temas oscuro y claro, notificaciones de escritorio, icono en la bandeja,
  inicio con el sistema.

  <details><summary>Verlo en acción</summary>
  <img src="../media/en/basics.webp" alt="Idiomas, temas claro y oscuro, una notificación de escritorio y el icono de la bandeja">
  </details>

> [!WARNING]
> Este proyecto no está afiliado a Twitch. Automatizar una cuenta puede incumplir las
> condiciones de Twitch o las reglas de una campaña.

## Contenido

- [Comparación](#comparison)
- [Descarga](#download)
- [Docker](#docker)
- [Cómo funciona](#how-it-works)
- [Preguntas frecuentes](#faq)
- [Proyectos relacionados](#related-projects)
- [Seguridad](#security)
- [Compilar desde el código fuente](#build)
- [Licencia](#license)

## <a name="comparison"></a>Comparación

Así queda [Twitch Drops & Points Miner](https://github.com/SunkwiBOT/twitch-drops-points-miner)
frente a [Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
(puntos del canal) y [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner) (Drops).
Comprobado en su código y sus README en noviembre de 2026.

| | Este proyecto | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Campañas de Drops: las encuentra, las mina y las reclama | ✅ | ❌ ¹ | ✅ |
| Puntos del canal: cofres, raids, momentos, predicciones | ✅ | ✅ | ❌ |
| Drops y puntos del canal a la vez en una misma cuenta | ✅ | en parte ¹ | ❌ |
| Varias cuentas en una sola app | ✅ | ❌ ² | ❌ |
| Cambiar los ajustes de varias cuentas a la vez | ✅ | ❌ | ❌ |
| Interfaz gráfica | ✅ escritorio y web ³ | ❌ ⁴ | ✅ escritorio |
| Panel web para un servidor doméstico (Docker), con contraseña y accesible por HTTPS | ✅ | ❌ ⁴ | ❌ |
| Sesiones cifradas en el disco, contraseña de la app opcional | ✅ | ❌ | ❌ |
| Interfaz en 29 idiomas | ✅ | ❌ | en parte ⁵ |
| Actualizaciones firmadas desde la propia app (escritorio) | ✅ | ❌ | ❌ |
| Asistente de configuración para cada cuenta nueva | ✅ | ❌ ⁴ | ❌ |
| Streamers añadidos y editados en la interfaz, de uno en uno o varios a la vez | ✅ | ❌ ⁴ | — |
| Streamers importados por búsqueda, una lista, otra cuenta o los canales seguidos | ✅ | en parte ⁶ | — |
| Notificaciones de Discord, Telegram y webhook con imágenes | ✅ | en parte ⁷ | ❌ |
| Calendario de todas las campañas de Drops, con búsqueda | ✅ | ❌ | en parte ⁸ |
| Orden de minado por campaña, palabras excluidas, campaña o recompensa ignorada | ✅ | ❌ | ❌ ⁹ |
| Los streamers de tu lista, vistos primero para los Drops de una campaña | ✅ | en parte ¹⁰ | ❌ |
| Diario de eventos filtrable y lista de cada solicitud, en la interfaz | ✅ | ❌ ⁴ | ❌ |
| Unos 30 MB de memoria (servidor Docker, 4 cuentas) | ✅ | — ¹¹ | — ¹¹ |

¹ Solo ve los streamers de su lista: hace avanzar los Drops de sus directos y los reclama,
sin elegir campaña.\
² Un proceso por cuenta.\
³ En tu PC como app de escritorio, y/o en un servidor (VPS, NAS, Raspberry Pi) con Docker,
sin entorno gráfico ni navegador en esa máquina: el panel se abre en cualquier navegador.\
⁴ Se configura en un archivo de Python; su página web muestra gráficos de puntos y el
archivo de registro.\
⁵ 21 idiomas.\
⁶ Puede seguir los canales que sigue la cuenta.\
⁷ Solo texto, sin imágenes. También tiene Matrix, Pushover y Gotify, que este proyecto no
tiene.\
⁸ Una lista de campañas con filtros, sin calendario ni búsqueda.\
⁹ Tiene juegos prioritarios y juegos excluidos, elegidos por el nombre del juego.\
¹⁰ Por defecto, primero ve los streamers de su lista cuyo directo tiene Drops (su
prioridad `DROPS`), pero nunca otro canal.\
¹¹ No medido.

## <a name="download"></a>Descarga

Descarga el archivo para tu sistema desde la
[última versión](https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest).

| Sistema | Paquete |
|---|---|
| **Windows** · x64 | Instalador (`setup.exe`) o [ejecutable portátil](https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest/download/TwitchDropsPointsMiner-portable.exe) |
| **macOS** 10.15 o posterior · Intel y Apple Silicon | Imagen de disco (`.dmg`) |
| **Linux** · x86-64 | `.AppImage`, `.deb` (Debian, Ubuntu) o `.rpm` (Fedora, openSUSE) |

La versión para macOS no está notarizada. La primera vez, haz clic derecho en la app y
elige **Abrir**.

Las actualizaciones están firmadas. La app busca una versión nueva al arrancar y luego cada
hora (puedes cambiar la frecuencia en **Ajustes › Acerca de**). Nunca instala una por su
cuenta: la instalas tú con un clic, desde el mensaje que aparece o desde
**Ajustes › Acerca de**.

Los datos se guardan en la carpeta de datos del usuario:

- Linux: `~/.local/share/twitch-drops-points-miner`
- Windows: `%APPDATA%\twitch-drops-points-miner`
- macOS: `~/Library/Application Support/twitch-drops-points-miner`

## <a name="docker"></a>Docker

El servidor muestra el mismo panel en un navegador. La imagen es
`ghcr.io/sunkwibot/twitch-drops-points-miner`, para `amd64` y `arm64`. Cuando sale una
versión nueva, el panel te lo indica: `docker compose pull` y luego
`docker compose up -d`.

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-drops-points-miner:latest
    pull_policy: always   # descarga la imagen más reciente en cada arranque
    restart: unless-stopped
    ports: ["8080:8080"]   # detrás de un proxy inverso HTTPS: quítalo, o "127.0.0.1:8080:8080"
    environment:
      MINER_PASSWORD: "change-me"   # obligatoria, 12 caracteres o más
      # MINER_PUBLIC_URL: "https://miner.example.com"   # solo detrás de un proxy inverso HTTPS
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

Abre <http://localhost:8080> (o la dirección de la máquina en tu red) e introduce la
contraseña.

### Variables

| Variable | Por defecto | Función |
|---|---|---|
| `MINER_PASSWORD` | ninguno | Contraseña del panel, obligatoria: sin ella, el servidor se niega a arrancar. De 12 caracteres o más (si es más corta, aparece una advertencia al arrancar). |
| `MINER_PASSWORD_FILE` | ninguno | Lee `MINER_PASSWORD` desde un archivo, por ejemplo un secreto de Docker. Una u otra, no las dos. |
| `MINER_PUBLIC_URL` | ninguno | Solo detrás de un proxy inverso HTTPS (Caddy, nginx, Traefik, Cloudflare Tunnel…): la dirección `https://` del panel, sin ruta, como `https://miner.example.com`. El proxy debe reenviar `X-Forwarded-Proto`, `X-Forwarded-For` y los websockets, algo que los proxies habituales hacen por defecto, y el puerto del contenedor no debe ser accesible directamente. Así, el panel rechaza todo lo que no llegue por el proxy en HTTPS y bloquea una dirección durante 30 segundos tras 5 contraseñas incorrectas. |
| `MINER_TRUSTED_HOPS` | `1` | Solo con `MINER_PUBLIC_URL`: cuántos proxies añaden una entrada a `X-Forwarded-For`, de 1 a 10. `2` con dos proxies, uno detrás de otro, como Cloudflare delante de Traefik. |
| `MINER_VAULT_PASSPHRASE` | ninguno | Solo para una carpeta de datos copiada de la app de escritorio con una contraseña de la app: la desbloquea al arrancar. Un servidor creado desde cero nunca la necesita. |
| `MINER_VAULT_PASSPHRASE_FILE` | ninguno | Lee `MINER_VAULT_PASSPHRASE` desde un archivo. Una u otra, no las dos. |
| `DATA_DIR` | `/data` en la imagen; si no, `data` en el directorio de trabajo | Base de datos, archivo de clave y registros. |
| `LOG_DIR` | `$DATA_DIR/logs` | Archivos de registro: uno al día, y se conservan los 14 últimos. |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | Dirección de escucha: una dirección IP y un puerto del 1 al 65535. |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | Nivel de registro. |

El servidor lee y comprueba las variables una sola vez, al arrancar. Un valor no válido
detiene el servidor con un mensaje, en lugar de recurrir en silencio a un valor
alternativo.

## <a name="how-it-works"></a>Cómo funciona

Cada 20 segundos, cada cuenta elige sus dos canales: el mejor canal en directo para la
primera campaña de su cola y, después, los streamers de su lista de puntos.

Twitch acredita **una campaña de Drops** y **dos canales para puntos** a la vez por
cuenta. Desde octubre de 2026, un drop solo se puede reclamar si la cuenta del juego está
vinculada. Las campañas sin vincular siguen avanzando, y el panel muestra el enlace para
vincularla.

La lista de campañas viene del proyecto [Twitch Drops API](https://drops.sunkwi.com)
(se actualiza cada 10 minutos), porque Twitch la oculta al cliente de TV.

## <a name="faq"></a>Preguntas frecuentes

### Riesgos y Twitch

**¿Me pueden banear la cuenta?**\
Nadie puede garantizar que no. Este proyecto no está afiliado a Twitch, y automatizar una
cuenta puede incumplir sus condiciones o las reglas de una campaña. El minero ve como
mucho dos canales por cuenta, el límite de Twitch, sin descargar ningún vídeo. Tú decides.

**¿De verdad se ve el vídeo?**\
No, no se descarga ningún vídeo. Para los Drops, el minero lee cada 15 segundos la lista de
reproducción del directo en la calidad más baja y solo pide la cabecera de cada segmento
nuevo. Para los puntos, envía cada minuto la señal de «minuto visto» de Twitch. Cada canal
visto muestra cuántos segmentos se han pedido.

**¿Por qué dos canales como máximo?**\
Es el límite de Twitch: solo acredita dos canales vistos a la vez por cuenta, y una sola
campaña de Drops a la vez. En el modo **Drops y puntos**, un canal sirve a la campaña y el
otro a tus streamers. El ajuste «Un canal», en **Ajustes de la cuenta**, es más discreto.

### Drops

**Una campaña no se mina, o un drop no avanza.**\
Abre la página **Drops** de la cuenta. En «Orden de minado», cada campaña en espera indica
por qué: «Ningún canal en directo», «En espera (una campaña a la vez)» o «Aún no hay drops
disponibles». Una campaña que no aparece en esa lista se ha descartado: por el ajuste
«Qué campañas», por una regla de la pestaña **Exclusiones** o porque no puede terminar a
tiempo. Una recompensa que requiere una suscripción nunca se mina, y un drop que va
después de otro espera a que el primero termine. En el modo **Solo puntos**, el minero no
elige ninguna campaña. «Minar primero», en la ficha de una campaña, la pone en cabeza; una
campaña ignorada puede volver con «Volver a minar», en la pestaña **Exclusiones**.

**¿Tengo que vincular la cuenta del juego?**\
Para reclamar, sí: desde octubre de 2026, Twitch solo deja reclamar un drop si la cuenta
del juego está vinculada. Por defecto, una campaña sin vincular sigue avanzando y la
reclamación espera a la vinculación. La insignia «No vinculada» lo indica, y «Vincular la
cuenta del juego», en la ficha de la campaña, abre la página de vinculación. Para minar
solo los juegos vinculados, elige «Solo juegos con la cuenta vinculada».

### Instalación y datos

**¿App de escritorio o Docker?**\
La app de escritorio mina mientras tu PC está encendido, con su icono en la bandeja, una
contraseña opcional y actualizaciones con un clic. Docker mina día y noche en una máquina
siempre encendida (VPS, NAS, Raspberry Pi); lo abres en un navegador, protegido por
`MINER_PASSWORD`. La interfaz es la misma.

**¿Dónde están mis datos?**\
En la carpeta de datos: `~/.local/share/twitch-drops-points-miner` en Linux,
`%APPDATA%\twitch-drops-points-miner` en Windows,
`~/Library/Application Support/twitch-drops-points-miner` en macOS, y el volumen `/data`
con Docker. En el escritorio, **Ajustes › Escritorio** la abre con un clic. Contiene
`miner.db` (cuentas, ajustes, diario), `master.key` (la clave de las sesiones, cuando no
hay contraseña de la app) y los registros.

**¿Cómo actualizo?**\
La app de escritorio busca una versión nueva al arrancar y luego cada hora (puedes
cambiarlo en **Ajustes › Acerca de**). Haz clic en la insignia «Actualización» y después
en «Instalar y reiniciar». Con Docker, el panel te avisa: ejecuta `docker compose pull` y
luego `docker compose up -d`.

**He olvidado la contraseña de la app.**\
No se puede recuperar: es lo que cifra tus sesiones de Twitch. Cierra la app, borra la
carpeta de datos y vuelve a añadir tus cuentas; tendrás que configurar de nuevo sus
ajustes. Con Docker, la contraseña del panel se cambia en `MINER_PASSWORD` y luego
`docker compose up -d`.

### Puntos y predicciones

**¿Cuántos puntos por hora?**\
Depende de Twitch y de cada canal. El minero gana lo que Twitch da a un espectador: los
puntos por tiempo de visualización, los cofres de bonificación (más o menos uno cada 15
minutos) y el bono de racha de visualización. Una suscripción multiplica los puntos, de
×1,2 a ×2 según el nivel. La página Inicio muestra los puntos ganados, y el Diario, cada
ganancia.

**¿Son arriesgadas las apuestas?**\
Sí, se pueden perder puntos. Las predicciones están desactivadas por defecto, streamer por
streamer. Tú eliges la estrategia, cuándo apostar, la apuesta (5 % del saldo por defecto,
50 000 puntos como máximo), un saldo mínimo y, si quieres, una condición, como una cuota
de al menos 1,5. Mantén una apuesta pequeña.

**No se está viendo un streamer.**\
Solo se ven los streamers activados y en directo, y solo en un canal libre: en el modo
**Drops y puntos**, uno de los dos sirve a la campaña actual; en el modo **Solo Drops**, no
se ve ningún streamer; una cuenta en pausa no ve nada. Los huecos se asignan en este
orden, que aparece como insignia en cada canal visto: «Bono de racha», «Racha en riesgo»,
«Suscrito» y, por último, «Orden de la lista». Sube el streamer en la lista, o desactiva
«Rachas de visualización primero» y «Canales suscritos primero».

## <a name="related-projects"></a>Proyectos relacionados

Otros proyectos del mismo autor:

- [**Twitch Drops API**](https://drops.sunkwi.com): una API pública y gratuita con todas
  las campañas de Drops y de recompensas de Twitch en curso, actualizada cada minuto. Esta
  app saca de ella su catálogo de campañas.
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com): un bot de Discord que anuncia en tu
  servidor los directos de Twitch y las campañas de Drops.
- [**Kick Alerts 🔔**](https://kick.sunkwi.com): un bot de Discord que anuncia en tu
  servidor los directos de Kick y las campañas de Drops.

## <a name="security"></a>Seguridad

- Los tokens se sellan con AES-256-GCM. La clave es o bien un archivo `master.key`
  aleatorio junto a la base de datos (por defecto, sin hacer nada), o bien una clave
  derivada de la contraseña de la app con Argon2id.
- El panel HTTP comprueba la contraseña en tiempo constante y usa cookies HttpOnly y
  SameSite=Strict (`Secure`, con el prefijo `__Host-`, detrás de HTTPS). El servidor
  solo guarda un SHA-256 de cada sesión. Termina una sesión a los 30 días o al cerrar
  sesión. Comprueba la sesión antes de leer cualquier cuerpo de petición y limita los
  cuerpos a 256 KiB.
- Tras 5 contraseñas incorrectas seguidas desde una misma dirección IP (formulario de
  inicio de sesión o bearer), esa dirección espera 30 segundos. El servidor responde
  `429` con `Retry-After` sin mirar la contraseña. Además, cada contraseña incorrecta
  tarda medio segundo en recibir respuesta.
- Fuera de tu red, pon el panel detrás de un proxy inverso HTTPS con
  `MINER_PUBLIC_URL` (consulta [Docker](#docker)).
- El minero solo se comunica con `twitch.tv`, `ttvnw.net`, `jtvnw.net`, `twitchcdn.net`,
  `twitch-drops-api.sunkwi.com` y los destinos de notificación que configures. La app
  (escritorio y Docker) busca versiones nuevas en `github.com`, y la app de escritorio las
  descarga de las releases de GitHub del proyecto. Están firmadas: solo se instala la
  versión que se muestra.

## <a name="build"></a>Compilar desde el código fuente

**Requisitos previos:** Rust, Node.js y los
[requisitos previos de Tauri](https://v2.tauri.app/start/prerequisites/) para tu sistema.

```bash
npm ci
npm run build          # panel
cargo build --release -p miner-server   # servidor: target/release/twitch-miner
npm run bundle:linux   # paquetes de escritorio (requiere las dependencias de Tauri para Linux)
```

Comprobaciones del código:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier, oxlint (con tipos) y svelte-check
```

Los tipos TypeScript de la interfaz se generan a partir de los tipos de Rust con
[ts-rs](https://github.com/Aleph-Alpha/ts-rs) en `ui/src/lib/bindings`. Tras cambiar un
tipo que ve la interfaz, ejecuta `npm run bindings`.

## <a name="license"></a>Licencia

[MIT](../../LICENSE). Inspirado en
[TwitchDropsMiner de DevilXD](https://github.com/DevilXD/TwitchDropsMiner),
[Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
y sus forks.
