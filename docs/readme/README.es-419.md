<p align="center">
  <img src="../../ui/public/app-icon.svg" width="112" height="112" alt="Logo de Twitch Miner">
</p>

<h1 align="center">Twitch Miner</h1>

<p align="center">
  <strong>Mina Drops de Twitch y puntos del canal en varias cuentas, sin navegador ni pestañas de Twitch abiertas.</strong>
</p>

<p align="center">
  Una app de escritorio para Linux, Windows y macOS, o una imagen de Docker con el mismo panel web, para correr en un servidor casero o en una Raspberry Pi. Con 4 cuentas, el servidor usa unos 30 MB de memoria y casi nada de CPU.
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-miner?label=release&color=9146FF" alt="Versión más reciente"></a>
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-miner/total?label=downloads&color=9146FF" alt="Descargas"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="Plataformas: Linux, Windows, macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="Imagen de Docker para amd64 y arm64"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="Licencia MIT"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="Visitas">
</p>

<p align="center">
  <a href="#download"><strong>Descargar</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-miner/issues/new/choose">Reportar un error</a>
</p>

<p align="center">
  <a href="../../README.md">English</a> ·
  <a href="README.fr.md">Français</a> ·
  <a href="README.de.md">Deutsch</a> ·
  <a href="README.es-ES.md">Español&nbsp;(España)</a> ·
  <strong>Español&nbsp;(Latinoamérica)</strong> ·
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
  <img src="../media/es-419/presentation.avif" alt="Presentación en video de Twitch Miner">
</p>

## <a name="features"></a>Funciones

Lo que lo diferencia de otros mineros. “Verlo en acción”, debajo de un punto, lo muestra
funcionando.

### Cuentas y streamers

- **Varias cuentas en una sola app.** Cada cuenta mina de forma independiente, en su
  propio modo: **Drops y puntos** (un canal para la mejor campaña y el otro para tus
  streamers), **Solo Drops** o **Solo puntos**.

  <details open><summary>Verlo en acción</summary>
  <img src="../media/es-419/accounts-mining.avif" alt="Varias cuentas minando al mismo tiempo, en la página Inicio">
  </details>

- **Un asistente para cada cuenta nueva.** Después de iniciar sesión, parte de la
  configuración predeterminada o de la de otra cuenta. Elige el modo, las campañas y los
  streamers y, al final, “Empezar a minar” o “Guardar y empezar más tarde”.

  <details open><summary>Verlo en acción</summary>
  <img src="../media/es-419/account-setup.avif" alt="El asistente de configuración de una cuenta nueva, paso a paso">
  </details>

- **Un cambio aplicado a otras cuentas con un clic.** Después de cambiar algo en una
  cuenta, el mensaje de confirmación ofrece “Aplicar a otras cuentas”. El cambio se repite
  en las cuentas que marques, que conservan el resto de su configuración.

  <details open><summary>Verlo en acción</summary>
  <img src="../media/es-419/apply-to-accounts.avif" alt="Un cambio repetido en otras cuentas con “Aplicar a otras cuentas”">
  </details>

- **Agregar un streamer.** Arranca de inmediato con las opciones predeterminadas: las de
  la app o las que elegiste para esta cuenta. Ajústalas en el momento si hace falta:
  cofres de bonificación, raids, momentos, racha de visualización, chat, predicciones.

  <details open><summary>Verlo en acción</summary>
  <img src="../media/es-419/streamer-add.avif" alt="Un streamer agregado con la búsqueda de Twitch y, después, sus opciones">
  </details>

- **Editar un streamer.** “Editar” abre sus opciones: cofres de bonificación, raids,
  momentos, racha de visualización, chat, predicciones… Un interruptor lo activa o lo
  desactiva sin quitarlo de la lista.

  <details open><summary>Verlo en acción</summary>
  <img src="../media/es-419/streamer-edit.avif" alt="Las opciones de un streamer, abiertas desde su fila">
  </details>

- **Editar varios streamers a la vez.** Márcalos y haz clic en “Editar los N streamers
  seleccionados”. Solo se aplican las opciones que cambies. Una opción que es distinta
  entre ellos aparece como “Mixto”, y su interruptor se convierte en dos botones, “Sí” y
  “No”. Después, la app te propone convertirlas en las opciones predeterminadas para
  streamers nuevos.

  <details open><summary>Verlo en acción</summary>
  <img src="../media/es-419/streamers-group-edit.avif" alt="Edición de varios streamers, con opciones “Mixto”">
  </details>

- **Importar streamers.** Pega una lista (hasta 500 nombres) o reutiliza la de otra cuenta
  o los canales que sigue la cuenta. La lista se verifica para agregar solo canales que
  existen.

  <details open><summary>Verlo en acción</summary>
  <img src="../media/es-419/streamers-import.avif" alt="Importación de streamers: una lista pegada, otra cuenta, los canales seguidos">
  </details>

- **Spam Points.** Canjea una recompensa personalizada de un canal varias veces seguidas.
  Elige un canal de tu lista de Puntos, o cualquier canal que encuentres con la búsqueda de
  Twitch: sus recompensas aparecen con el saldo de la cuenta, actualizado en tiempo real.
  Ajusta la cantidad (hasta 25, sin pasarte del saldo) y haz clic en “Canjear”. El
  resultado aparece cuando termina la ráfaga, y cada ráfaga queda en su propia línea de la
  Bitácora.

  <details open><summary>Verlo en acción</summary>
  <img src="../media/es-419/spam-points.avif" alt="Spam Points: un canal elegido, 10 × una recompensa con mensaje canjeada, los canjes llegan al chat de Twitch del canal y, después, la línea de la Bitácora">
  </details>

### Drops

- **Todas las campañas de Drops activas.** Cada campaña tiene su ficha: recompensas,
  vinculación de la cuenta del juego, “Minar primero”. La lista viene del proyecto
  [Twitch Drops API](https://drops.sunkwi.com) y se actualiza cada 10 minutos.

  <details open><summary>Verlo en acción</summary>
  <img src="../media/es-419/drops-catalogue.avif" alt="El Catálogo en vista de calendario, la búsqueda y la ficha de una campaña">
  </details>

- **Tu orden de minado.** Arrastra las campañas al orden que quieras. Las nuevas encuentran
  su lugar solas: primero tus juegos prioritarios y después las que terminan antes o a las
  que les quedan menos minutos. Excluye juegos, palabras, una campaña o una sola
  recompensa, y omite las campañas que no alcanzan a terminar a tiempo. “Minar primero”,
  en la ficha de una campaña, la pone al principio.

  <details open><summary>Verlo en acción</summary>
  <img src="../media/es-419/drops-order.avif" alt="El orden de minado, los juegos prioritarios y las exclusiones">
  </details>

- **Inventario y “Reclamar todo”.** La página Inventario clasifica las recompensas en
  “Por reclamar”, “En curso” y “Reclamadas”. Reclámalas una por una, todas a la vez, o
  activa “Reclamar drops automáticamente”.

  <details open><summary>Verlo en acción</summary>
  <img src="../media/es-419/drops-inventory.avif" alt="El inventario y el botón “Reclamar todo”">
  </details>

- **“Mis streamers primero para los Drops”.** En la campaña actual, un streamer de tu
  lista cuya transmisión cuenta para esa campaña pasa antes que los demás canales. El
  mismo canal hace avanzar el drop y te da sus puntos del canal. Activado de forma
  predeterminada.

  <details open><summary>Verlo en acción</summary>
  <img src="../media/es-419/drops-streamers-first.avif" alt="Un streamer de la lista, mirado para los Drops y por sus puntos">
  </details>

### Seguimiento

- **Una bitácora de todo, y cada solicitud.** La Bitácora guarda cada evento: drops,
  puntos ganados y gastados, cofres, apuestas, raids, transmisiones que comienzan…
  Fíltrala por cuenta, por pestaña (Drops, Puntos, Otros) y por tipo.
  **Registros técnicos › Solicitudes** muestra cada solicitud enviada a Twitch y a otros
  servicios, con su resultado y su duración. Los registros y las solicitudes no contienen
  ningún token ni secreto: “Copiar para un reporte de error” los copia, listos para pegar
  en un reporte de error en
  [GitHub](https://github.com/SunkwiBOT/twitch-miner/issues) o
  [Discord](https://discord.gg/support-bots-891685895781113886).

  <details open><summary>Verlo en acción</summary>
  <img src="../media/es-419/journal-requests.avif" alt="La Bitácora filtrada y, después, Registros técnicos › Solicitudes">
  </details>

### En el escritorio o en un servidor

- **La misma interfaz en todos lados.** La app de escritorio (Linux, Windows, macOS) y la
  imagen de Docker muestran el mismo panel. Docker corre en un VPS, un NAS o una
  Raspberry Pi, sin pantalla ni navegador en esa máquina: el panel se abre en cualquier
  navegador.
- **Ligero.** Unos 30 MB de memoria para 4 cuentas, casi nada de CPU y una imagen de
  Docker de unos 12 MB.

### Seguridad y actualizaciones

- **Tus sesiones, protegidas.** Sin contraseña de Twitch: inicias sesión en
  `twitch.tv/activate`, desde tu navegador. Las sesiones se guardan cifradas en el disco
  y puedes revocarlas en tu
  [configuración de Twitch](https://www.twitch.tv/settings/connections). En el
  escritorio, una contraseña de la app opcional la bloquea: no se mina nada hasta que se
  ingresa. Con Docker, el panel tiene su propia contraseña, se bloquea solo después de
  unos minutos sin actividad y funciona con HTTPS detrás de un proxy. Cinco contraseñas
  incorrectas seguidas bloquean la dirección IP durante 30 segundos.

  <details open><summary>Verlo en acción</summary>
  <img src="../media/es-419/security-lock.avif" alt="El candado: la app bloqueada y, después, desbloqueada con su contraseña">
  </details>

- **Actualizaciones firmadas, nunca automáticas.** En la barra superior aparece una
  insignia “Actualización” con el número de versión. “Instalar y reiniciar” primero te
  pide confirmación; luego la app instala la versión y se reinicia. Si tiene contraseña de
  la app, se reinicia bloqueada: vuelve a ingresarla para seguir minando. Solo se instala
  una versión firmada por el proyecto. La versión de Docker también te avisa, con los
  comandos que tienes que ejecutar.

  <details open><summary>Verlo en acción</summary>
  <img src="../media/es-419/updates.avif" alt="La insignia “Actualización”, la confirmación y, después, la instalación">
  </details>

### Y además

- Como otros mineros: cofres de bonificación, raids, momentos, metas de la comunidad,
  predicciones (estrategias, momento de la apuesta, condiciones), rachas de visualización
  y canales suscritos primero, presencia en el chat (se puede desactivar), opciones
  predeterminadas para streamers nuevos, notificaciones de Discord, Telegram y webhook con
  imágenes, proxy global o por cuenta.
- 29 idiomas, temas oscuro y claro, notificaciones de escritorio, ícono en la bandeja,
  inicio con el sistema.

  <details open><summary>Verlo en acción</summary>
  <img src="../media/es-419/basics.avif" alt="Idiomas, temas claro y oscuro, una notificación de escritorio y el ícono de la bandeja">
  </details>

> [!WARNING]
> Este proyecto no está afiliado a Twitch. Automatizar una cuenta puede violar los
> términos de Twitch o las reglas de una campaña.

## Índice

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

Cómo se compara [Twitch Miner](https://github.com/SunkwiBOT/twitch-miner)
con [Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
(puntos del canal) y [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner) (Drops).
Verificado en su código y sus README en noviembre de 2026.

| | Este proyecto | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Campañas de Drops: las encuentra, las mina y las reclama | ✅ | ❌ ¹ | ✅ |
| Puntos del canal: cofres, raids, momentos, predicciones | ✅ | ✅ | ❌ |
| Spam Points: una recompensa personalizada de un canal canjeada varias veces seguidas | ✅ | ❌ | ❌ |
| Drops y puntos del canal al mismo tiempo en una cuenta | ✅ | en parte ¹ | ❌ |
| Varias cuentas en una sola app | ✅ | ❌ ² | ❌ |
| Cambiar la configuración de varias cuentas a la vez | ✅ | ❌ | ❌ |
| Interfaz gráfica | ✅ escritorio y web ³ | ❌ ⁴ | ✅ escritorio |
| Panel web para un servidor casero (Docker), con contraseña y accesible por HTTPS | ✅ | ❌ ⁴ | ❌ |
| Sesiones cifradas en el disco, contraseña de la app opcional | ✅ | ❌ | ❌ |
| Interfaz en 29 idiomas | ✅ | ❌ | en parte ⁵ |
| Actualizaciones firmadas desde la propia app (escritorio) | ✅ | ❌ | ❌ |
| Asistente de configuración para cada cuenta nueva | ✅ | ❌ ⁴ | ❌ |
| Streamers agregados y editados en la interfaz, uno por uno o varios a la vez | ✅ | ❌ ⁴ | — |
| Streamers importados con una búsqueda, una lista, otra cuenta o los canales seguidos | ✅ | en parte ⁶ | — |
| Notificaciones de Discord, Telegram y webhook con imágenes | ✅ | en parte ⁷ | ❌ |
| Calendario de todas las campañas de Drops, con búsqueda | ✅ | ❌ | en parte ⁸ |
| Orden de minado por campaña, palabras excluidas, campaña o recompensa ignorada | ✅ | ❌ | ❌ ⁹ |
| Los streamers de tu lista, mirados primero para los Drops de una campaña | ✅ | en parte ¹⁰ | ❌ |
| Bitácora de eventos filtrable y lista de cada solicitud, en la interfaz | ✅ | ❌ ⁴ | ❌ |
| Unos 30 MB de memoria (servidor Docker, 4 cuentas) | ✅ | — ¹¹ | — ¹¹ |

¹ Solo mira a los streamers de su lista: hace avanzar los Drops de sus transmisiones y los
reclama, sin elegir campaña.\
² Un proceso por cuenta.\
³ En tu computadora como app de escritorio, y/o en un servidor (VPS, NAS, Raspberry Pi) con
Docker, sin entorno gráfico ni navegador en esa máquina: el panel se abre en cualquier
navegador.\
⁴ Se configura en un archivo de Python; su página web muestra gráficas de puntos y el
archivo de registro.\
⁵ 21 idiomas.\
⁶ Puede seguir los canales que sigue la cuenta.\
⁷ Solo texto, sin imágenes. También tiene Matrix, Pushover y Gotify, que este proyecto no
tiene.\
⁸ Una lista de campañas con filtros, sin calendario ni búsqueda.\
⁹ Tiene juegos prioritarios y juegos excluidos, elegidos por el nombre del juego.\
¹⁰ De forma predeterminada, primero mira a los streamers de su lista cuya transmisión
tiene Drops (su prioridad `DROPS`), pero nunca otro canal.\
¹¹ Sin medir.

## <a name="download"></a>Descarga

Descarga el archivo para tu sistema desde la
[versión más reciente](https://github.com/SunkwiBOT/twitch-miner/releases/latest).

| Sistema | Paquete |
|---|---|
| **Windows** · x64 | Instalador (`setup.exe`) o [ejecutable portátil](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/TwitchMiner-portable.exe) |
| **macOS** 10.15 o posterior · Intel y Apple Silicon | Imagen de disco (`.dmg`) |
| **Linux** · x86-64 | `.AppImage`, `.deb` (Debian, Ubuntu) o `.rpm` (Fedora, openSUSE) |

La compilación para macOS no está notarizada. La primera vez, haz clic derecho en la app y
elige **Abrir**.

Las actualizaciones están firmadas. La app busca una nueva versión cuando se inicia y
después cada hora (puedes cambiar la frecuencia en **Configuración › Acerca de**). Nunca
instala una por sí sola: tú la instalas con un clic, desde el mensaje que aparece o desde
**Configuración › Acerca de**.

Los datos se guardan en la carpeta de datos del usuario:

- Linux: `~/.local/share/twitch-miner`
- Windows: `%APPDATA%\twitch-miner`
- macOS: `~/Library/Application Support/twitch-miner`

## <a name="docker"></a>Docker

El servidor muestra el mismo panel en un navegador. La imagen es
`ghcr.io/sunkwibot/twitch-miner`, para `amd64` y `arm64`. Cuando sale una
nueva versión, el panel te avisa: `docker compose pull` y después
`docker compose up -d`.

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-miner:latest
    pull_policy: always   # descarga la imagen más nueva en cada inicio
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

Abre <http://localhost:8080> (o la dirección de la máquina en tu red) e ingresa la
contraseña.

### Variables

| Variable | Predeterminado | Uso |
|---|---|---|
| `MINER_PASSWORD` | ninguno | Contraseña del panel, obligatoria: sin ella, el servidor se niega a iniciar. De 12 caracteres o más (si es más corta, aparece una advertencia al iniciar). El valor de ejemplo `change-me` se rechaza. |
| `MINER_PASSWORD_FILE` | ninguno | Lee `MINER_PASSWORD` desde un archivo, por ejemplo un secreto de Docker. Una u otra, no ambas. |
| `MINER_PUBLIC_URL` | ninguno | Solo detrás de un proxy inverso HTTPS (Caddy, nginx, Traefik, Cloudflare Tunnel…): la dirección `https://` del panel, sin ruta, como `https://miner.example.com`. El proxy debe reenviar `X-Forwarded-Proto`, `X-Forwarded-For` y los websockets, algo que los proxies comunes hacen de forma predeterminada, y el puerto del contenedor no debe ser accesible directamente. Así, el panel rechaza todo lo que no llegue por el proxy con HTTPS y bloquea una dirección durante 30 segundos después de 5 contraseñas incorrectas. El proxy debe correr en la misma máquina o en su red local: una conexión desde una dirección pública se rechaza. |
| `MINER_TRUSTED_HOPS` | `1` | Solo con `MINER_PUBLIC_URL`: cuántos proxies agregan una entrada a `X-Forwarded-For`, de 1 a 10. `2` con dos proxies, uno detrás del otro, como Cloudflare delante de Traefik. |
| `MINER_PROXY_NETWORKS` | ninguno | Solo con `MINER_PUBLIC_URL`: otras redes desde las que el proxy puede conectarse, además de esta máquina y su red local privada, separadas por comas: una sola dirección o una red como `2001:db8:1::/64`, para una red local con direcciones IPv6 globales. |
| `MINER_VAULT_PASSPHRASE` | ninguno | Solo para una carpeta de datos copiada de la app de escritorio con una contraseña de la app: la desbloquea al iniciar. Un servidor creado desde cero nunca la necesita. |
| `MINER_VAULT_PASSPHRASE_FILE` | ninguno | Lee `MINER_VAULT_PASSPHRASE` desde un archivo. Una u otra, no ambas. |
| `DATA_DIR` | `/data` en la imagen; si no, `data` en el directorio de trabajo | Base de datos, archivo de clave y registros. |
| `LOG_DIR` | `$DATA_DIR/logs` | Archivos de registro: uno por día, y se guardan los últimos 14. |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | Dirección de escucha: una dirección IP y un puerto de 1 a 65535. |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | Nivel de registro. |

El servidor lee y valida las variables una sola vez, al iniciar. Un valor inválido detiene
el servidor con un mensaje, en lugar de usar en silencio un valor de respaldo.

## <a name="how-it-works"></a>Cómo funciona

Cada 20 segundos, cada cuenta elige sus dos canales: el mejor canal en vivo para la
primera campaña de su cola y, después, los streamers de su lista de puntos.

Twitch acredita **una campaña de Drops** y **dos canales para puntos** a la vez por
cuenta. Desde octubre de 2026, un drop solo se puede reclamar cuando la cuenta del juego
está vinculada. Las campañas sin vincular siguen avanzando, y el panel muestra el enlace
para vincularla.

La lista de campañas viene del proyecto [Twitch Drops API](https://drops.sunkwi.com) (se
actualiza cada 10 minutos), porque Twitch la oculta al cliente de TV.

## <a name="faq"></a>Preguntas frecuentes

### Riesgos y Twitch

**¿Me pueden banear la cuenta?**\
Nadie puede garantizar que no. Este proyecto no está afiliado a Twitch, y automatizar una
cuenta puede violar sus términos o las reglas de una campaña. El minero mira como máximo
dos canales por cuenta, el límite de Twitch, sin descargar ningún video. Tú decides.

**¿De verdad se mira el video?**\
No, no se descarga ningún video. Para los Drops, el minero lee cada 15 segundos la lista
de reproducción de la transmisión en la calidad más baja y solo pide el encabezado de cada
segmento nuevo. Para los puntos, envía cada minuto la señal de “minuto visto” de Twitch.
Cada canal mirado muestra cuántos segmentos se pidieron.

**¿Por qué dos canales como máximo?**\
Es el límite de Twitch: solo acredita dos canales mirados al mismo tiempo por cuenta, y
una sola campaña de Drops a la vez. En el modo **Drops y puntos**, un canal sirve a la
campaña y el otro a tus streamers. La opción “Un canal”, en
**Configuración de la cuenta**, es más discreta.

### Drops

**Una campaña no se mina, o un drop no avanza.**\
Abre la página **Drops** de la cuenta. En “Orden de minado”, cada campaña en espera indica
por qué: “Ningún canal en vivo”, “En espera (una campaña a la vez)” o “Todavía no hay drops
disponibles”. Una campaña que no aparece en esa lista quedó descartada: por la opción
“Qué campañas”, por una regla de la pestaña **Exclusiones** o porque no alcanza a terminar
a tiempo. Una recompensa que requiere una suscripción nunca se mina, y un drop que va
después de otro espera a que el primero termine. En el modo **Solo puntos**, el minero no
elige ninguna campaña. “Minar primero”, en la ficha de una campaña, la pone al principio;
una campaña ignorada puede volver con “Volver a minar”, en la pestaña **Exclusiones**.

**¿Tengo que vincular la cuenta del juego?**\
Para reclamar, sí: desde octubre de 2026, Twitch solo permite reclamar un drop cuando la
cuenta del juego está vinculada. De forma predeterminada, una campaña sin vincular sigue
avanzando y el reclamo espera a la vinculación. La insignia “No vinculada” lo indica, y
“Vincular la cuenta del juego”, en la ficha de la campaña, abre la página de vinculación.
Para minar solo los juegos vinculados, elige “Solo juegos con la cuenta vinculada”.

### Instalación y datos

**¿App de escritorio o Docker?**\
La app de escritorio mina mientras tu computadora está encendida, con su ícono en la
bandeja, una contraseña opcional y actualizaciones con un clic. Docker mina día y noche en
una máquina siempre encendida (VPS, NAS, Raspberry Pi); lo abres en un navegador,
protegido por `MINER_PASSWORD`. La interfaz es la misma.

**¿Dónde están mis datos?**\
En la carpeta de datos: `~/.local/share/twitch-miner` en Linux,
`%APPDATA%\twitch-miner` en Windows,
`~/Library/Application Support/twitch-miner` en macOS, y el volumen `/data`
con Docker. En el escritorio, **Configuración › Escritorio** la abre con un clic. Contiene
`miner.db` (cuentas, configuración, bitácora), `master.key` (la clave de las sesiones,
cuando no hay contraseña de la app) y los registros.

**¿Cómo actualizo?**\
La app de escritorio busca una nueva versión cuando se inicia y después cada hora (puedes
cambiarlo en **Configuración › Acerca de**). Haz clic en la insignia “Actualización” y
después en “Instalar y reiniciar”. Con Docker, el panel te avisa: ejecuta
`docker compose pull` y después `docker compose up -d`.

**Olvidé la contraseña de la app.**\
No se puede recuperar: es lo que cifra tus sesiones de Twitch. Cierra la app, borra la
carpeta de datos y vuelve a agregar tus cuentas; vas a tener que configurarlas de nuevo.
Con Docker, la contraseña del panel se cambia en `MINER_PASSWORD` y después
`docker compose up -d`.

### Puntos y predicciones

**¿Cuántos puntos por hora?**\
Depende de Twitch y de cada canal. El minero gana lo que Twitch le da a un espectador: los
puntos por tiempo de visualización, los cofres de bonificación (más o menos uno cada 15
minutos) y el bono de racha de visualización. Una suscripción multiplica los puntos, de
×1.2 a ×2 según el nivel. La página Inicio muestra los puntos ganados, y la Bitácora, cada
ganancia.

**¿Las apuestas son riesgosas?**\
Sí, se pueden perder puntos. Las predicciones están desactivadas de forma predeterminada,
streamer por streamer. Tú eliges la estrategia, cuándo apostar, la apuesta (5% del saldo
de forma predeterminada, 50,000 puntos como máximo), un saldo mínimo y, si quieres, una
condición, como una cuota de al menos 1.5. Mantén una apuesta pequeña.

**No se está mirando a un streamer.**\
Solo se mira a los streamers activados y en vivo, y solo en un canal libre: en el modo
**Drops y puntos**, uno de los dos sirve a la campaña actual; en el modo **Solo Drops**, no
se mira a ningún streamer; una cuenta en pausa no mira nada. Los lugares se asignan en
este orden, que aparece como insignia en cada canal mirado: “Bono de racha”, “Racha en
riesgo”, “Suscrito” y, por último, “Orden de la lista”. Sube al streamer en la lista, o
desactiva “Rachas de visualización primero” y “Canales suscritos primero”.

## <a name="related-projects"></a>Proyectos relacionados

Otros proyectos del mismo autor:

- [**Twitch Drops API**](https://drops.sunkwi.com): una API pública y gratuita con todas
  las campañas de Drops y de recompensas de Twitch activas, actualizada cada minuto. Esta
  app toma de ahí su catálogo de campañas.
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com): un bot de Discord que anuncia en tu
  servidor las transmisiones en vivo de Twitch y las campañas de Drops.
- [**Kick Alerts 🔔**](https://kick.sunkwi.com): un bot de Discord que anuncia en tu
  servidor las transmisiones en vivo de Kick y las campañas de Drops.

## <a name="security"></a>Seguridad

- Los tokens se sellan con AES-256-GCM. La clave es un archivo `master.key` aleatorio
  junto a la base de datos (predeterminado, sin hacer nada) o se deriva de la contraseña
  de la app con Argon2id.
- El panel HTTP verifica la contraseña en tiempo constante y usa cookies HttpOnly y
  SameSite=Strict (`Secure`, con el prefijo `__Host-`, detrás de HTTPS). El servidor
  solo guarda un SHA-256 de cada sesión. Termina una sesión después de 30 días o al
  cerrar sesión. Verifica la sesión antes de leer cualquier cuerpo de solicitud y limita
  los cuerpos a 256 KiB.
- Después de 5 contraseñas incorrectas seguidas desde una misma dirección IP (formulario
  de inicio de sesión o bearer), esa dirección debe esperar 30 segundos. El servidor
  responde `429` con `Retry-After` sin mirar la contraseña. Además, cada contraseña
  incorrecta tarda medio segundo en recibir respuesta.
- Fuera de tu red, coloca el panel detrás de un proxy inverso HTTPS con
  `MINER_PUBLIC_URL` (consulta [Docker](#docker)).
- El minero solo se comunica con `twitch.tv`, `ttvnw.net`, `jtvnw.net`, `twitchcdn.net`,
  `drops.sunkwi.com` y los destinos de notificación que configures. La app
  (escritorio y Docker) busca nuevas versiones en `github.com`, y la app de escritorio las
  descarga de las releases del proyecto en GitHub. Están firmadas: solo se instala la
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

Verificaciones del código:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier, oxlint (con tipos) y svelte-check
```

Los tipos TypeScript de la interfaz se generan a partir de los tipos de Rust con
[ts-rs](https://github.com/Aleph-Alpha/ts-rs) en `ui/src/lib/bindings`. Después de cambiar
un tipo que ve la interfaz, ejecuta `npm run bindings`.

## <a name="license"></a>Licencia

[MIT](../../LICENSE). Inspirado en
[TwitchDropsMiner de DevilXD](https://github.com/DevilXD/TwitchDropsMiner),
[Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
y sus forks.
