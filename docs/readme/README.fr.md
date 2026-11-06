<p align="center">
  <img src="../../ui/public/app-icon.svg" width="112" height="112" alt="Logo de Twitch Miner">
</p>

<h1 align="center">Twitch Miner</h1>

<p align="center">
  <strong>Mine les Drops Twitch et les points de chaîne sur plusieurs comptes, sans navigateur ni onglet Twitch ouvert.</strong>
</p>

<p align="center">
  Une application de bureau pour Linux, Windows et macOS, ou une image Docker avec le même tableau de bord web, à lancer sur un serveur à la maison ou sur un Raspberry Pi. Avec 4 comptes, le serveur utilise environ 30 Mo de mémoire et presque pas de processeur.
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-miner?label=release&color=9146FF" alt="Dernière version"></a>
  <a href="https://github.com/SunkwiBOT/twitch-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-miner/total?label=downloads&color=9146FF" alt="Téléchargements"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="Plateformes : Linux, Windows, macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="Image Docker pour amd64 et arm64"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="Licence MIT"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="Vues">
</p>

<p align="center">
  <a href="#download"><strong>Télécharger</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-miner/issues/new/choose">Signaler un bug</a>
</p>

<p align="center">
  <a href="../../README.md">English</a> ·
  <strong>Français</strong> ·
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
  <a href="README.id.md">Bahasa&nbsp;Indonesia</a>
</p>

<p align="center">
  <img src="../media/fr/presentation.avif" alt="Présentation en vidéo de Twitch Miner">
</p>

## <a name="features"></a>Fonctionnalités

Ce qui le distingue des autres mineurs. « Voir en image » montre chaque point en action.

### Comptes et streamers

- **Plusieurs comptes dans une seule application.** Chaque compte mine de son côté, dans
  son mode : **Drops et points** (une chaîne pour la meilleure campagne, l'autre pour vos
  streamers), **Drops uniquement** ou **Points uniquement**.

  <details open><summary>Voir en image</summary>
  <img src="../media/fr/accounts-mining.avif" alt="Plusieurs comptes qui minent en même temps, sur l'accueil">
  </details>

- **Un assistant pour chaque nouveau compte.** Après la connexion, partez des réglages par
  défaut ou de ceux d'un autre compte. Choisissez le mode, les campagnes et les streamers,
  puis « Commencer le minage » ou « Enregistrer et démarrer plus tard ».

  <details open><summary>Voir en image</summary>
  <img src="../media/fr/account-setup.avif" alt="L'assistant de configuration d'un nouveau compte, étape par étape">
  </details>

- **Une modification appliquée à d'autres comptes en un clic.** Après un changement sur un
  compte, le message de confirmation propose « Appliquer à d'autres comptes ». Le
  changement est refait sur les comptes cochés, qui gardent le reste de leurs réglages.

  <details open><summary>Voir en image</summary>
  <img src="../media/fr/apply-to-accounts.avif" alt="Un changement refait sur d'autres comptes avec « Appliquer à d'autres comptes »">
  </details>

- **Ajouter un streamer.** Il démarre automatiquement avec les options par défaut : celles
  de l'application, ou celles que vous avez choisies pour ce compte. Ajustez-les tout de
  suite si besoin : coffres bonus, raids, moments, série de visionnage, chat, prédictions.

  <details open><summary>Voir en image</summary>
  <img src="../media/fr/streamer-add.avif" alt="Ajout d'un streamer par la recherche Twitch, puis ses options">
  </details>

- **Modifier un streamer.** « Modifier » ouvre ses options : coffres bonus, raids, moments,
  série de visionnage, chat, prédictions… Un interrupteur l'active ou le désactive sans le
  retirer de la liste.

  <details open><summary>Voir en image</summary>
  <img src="../media/fr/streamer-edit.avif" alt="Les options d'un streamer, ouvertes depuis sa ligne">
  </details>

- **Modifier plusieurs streamers d'un coup.** Cochez-les, puis « Modifier les N streamers
  sélectionnés ». Seules les options que vous changez s'appliquent. Une option qui diffère
  entre eux s'affiche « Mixte », et son interrupteur devient deux boutons, « Oui » et
  « Non ». L'application propose ensuite d'en faire les options par défaut des nouveaux
  streamers.

  <details open><summary>Voir en image</summary>
  <img src="../media/fr/streamers-group-edit.avif" alt="Modification de plusieurs streamers, avec des options « Mixte »">
  </details>

- **Importer des streamers.** Collez une liste (jusqu'à 500 noms), reprenez celle d'un autre
  compte ou les chaînes suivies par le compte. La liste est vérifiée pour n'ajouter que des
  chaînes qui existent.

  <details open><summary>Voir en image</summary>
  <img src="../media/fr/streamers-import.avif" alt="Import de streamers : liste collée, autre compte, chaînes suivies">
  </details>

- **Spam Points.** Échangez une récompense personnalisée d'une chaîne plusieurs fois
  d'affilée. Choisissez une chaîne de votre liste Points, ou n'importe quelle chaîne
  trouvée par la recherche Twitch : ses récompenses s'affichent avec le solde du compte,
  relu en direct. Réglez la quantité (jusqu'à 25, dans la limite du solde), puis
  « Échanger ». Le résultat s'affiche à la fin de la rafale, et chaque rafale a sa
  ligne dans le Journal.

  <details open><summary>Voir en image</summary>
  <img src="../media/fr/spam-points.avif" alt="Spam Points : une chaîne choisie, 10 × une récompense avec message échangée, les échanges arrivent dans le chat Twitch de la chaîne, puis la ligne du Journal">
  </details>

### Drops

- **Toutes les campagnes Drops en cours.** Chaque campagne a sa fiche : récompenses,
  liaison du compte de jeu, « Miner en priorité ». La liste vient du projet
  [Twitch Drops API](https://drops.sunkwi.com) et se rafraîchit toutes les 10 minutes.

  <details open><summary>Voir en image</summary>
  <img src="../media/fr/drops-catalogue.avif" alt="Le Catalogue en calendrier, la recherche et la fiche d'une campagne">
  </details>

- **Votre ordre de minage.** Glissez les campagnes dans l'ordre voulu. Les nouvelles se
  placent seules : vos jeux prioritaires d'abord, puis la fin la plus proche ou le moins de
  minutes restantes. Excluez des jeux, des mots, une campagne ou une seule récompense, et
  passez les campagnes qui ne peuvent pas finir à temps. « Miner en priorité », dans la
  fiche d'une campagne, la met en tête.

  <details open><summary>Voir en image</summary>
  <img src="../media/fr/drops-order.avif" alt="L'ordre de minage, les jeux prioritaires et les exclusions">
  </details>

- **Inventaire et « Tout réclamer ».** La page Inventaire range les récompenses en
  « À réclamer », « En cours » et « Réclamés ». Réclamez-les une par une, toutes d'un coup,
  ou activez « Réclamer les drops automatiquement ».

  <details open><summary>Voir en image</summary>
  <img src="../media/fr/drops-inventory.avif" alt="L'inventaire et le bouton « Tout réclamer »">
  </details>

- **« Mes streamers d'abord pour les Drops ».** Pour la campagne en cours, un streamer de
  votre liste dont le live compte pour cette campagne passe avant les autres chaînes. La
  même chaîne fait progresser le drop et vous rapporte ses points de chaîne. Activé par
  défaut.

  <details open><summary>Voir en image</summary>
  <img src="../media/fr/drops-streamers-first.avif" alt="Un streamer de la liste regardé pour les Drops et pour ses points">
  </details>

### Suivi

- **Un journal de tout, et chaque requête.** Le Journal garde chaque événement : drops,
  points gagnés et dépensés, coffres, paris, raids, lives… Filtrez-le par compte, par
  onglet (Drops, Points, Autres) et par type. **Logs techniques › Requêtes** liste chaque
  requête envoyée à Twitch et aux autres services, avec son résultat et sa durée. Logs et
  requêtes ne contiennent aucun jeton ni secret : « Copier pour un rapport de bug » les
  copie, prêts à coller dans un rapport de bug sur
  [GitHub](https://github.com/SunkwiBOT/twitch-miner/issues) ou
  [Discord](https://discord.gg/support-bots-891685895781113886).

  <details open><summary>Voir en image</summary>
  <img src="../media/fr/journal-requests.avif" alt="Le Journal filtré, puis Logs techniques › Requêtes">
  </details>

### Sur le bureau ou sur un serveur

- **La même interface partout.** L'application de bureau (Linux, Windows, macOS) et l'image
  Docker affichent le même tableau de bord. Docker tourne sur un VPS, un NAS ou un
  Raspberry Pi, sans écran ni navigateur sur cette machine : le tableau de bord s'ouvre
  dans n'importe quel navigateur.
  Sur un téléphone, le tableau de bord s'adapte à l'écran et, en HTTPS, s'installe comme une application.
- **Léger.** Environ 30 Mo de mémoire pour 4 comptes, presque pas de processeur, et une
  image Docker d'environ 12 Mo.

### Sécurité et mises à jour

- **Vos sessions protégées.** Pas de mot de passe Twitch : vous vous connectez sur
  `twitch.tv/activate`, depuis votre navigateur. Les sessions sont chiffrées sur le disque,
  et révocables dans vos [paramètres Twitch](https://www.twitch.tv/settings/connections).
  Sur le bureau, un mot de passe de l'application, en option, la verrouille : rien n'est
  miné avant sa saisie. Avec Docker, le tableau de bord a son mot de passe, se verrouille
  après quelques minutes sans activité et passe en HTTPS derrière un proxy. Cinq mauvais
  mots de passe de suite bloquent l'adresse IP pendant 30 secondes.

  <details open><summary>Voir en image</summary>
  <img src="../media/fr/security-lock.avif" alt="Le cadenas : l'application verrouillée, puis déverrouillée avec son mot de passe">
  </details>

- **Des mises à jour signées, jamais automatiques.** Un badge « Mise à jour » et son numéro
  de version apparaissent dans la barre du haut. « Installer et redémarrer » demande
  d'abord une confirmation, puis l'application installe la version et redémarre. Avec un
  mot de passe de l'application, elle redémarre verrouillée : retapez-le pour reprendre le
  minage. Seule une version signée par le projet est installée. La version Docker prévient
  aussi, avec les commandes à lancer.

  <details open><summary>Voir en image</summary>
  <img src="../media/fr/updates.avif" alt="Le badge « Mise à jour », la confirmation, puis l'installation">
  </details>

### Et aussi

- Comme d'autres mineurs : coffres bonus, raids, moments, objectifs communautaires,
  prédictions (stratégies, moment du pari, conditions), séries de visionnage et priorité
  aux chaînes abonnées, présence dans le chat (désactivable), options par défaut des
  nouveaux streamers, notifications Discord, Telegram et webhook avec images, proxy global
  ou par compte.
- **Bientôt : le minage de badges.** Les badges Twitch obtenus en regardant, minés comme les Drops.
- 29 langues, thèmes sombre et clair, notifications de bureau, icône dans la zone de
  notification, lancement au démarrage du système.

  <details open><summary>Voir en image</summary>
  <img src="../media/fr/basics.avif" alt="Langues, thème clair et sombre, notification de bureau et icône de la zone de notification">
  </details>

> [!WARNING]
> Ce projet n'est pas affilié à Twitch. Automatiser un compte peut enfreindre les
> conditions de Twitch ou les règles d'une campagne.

## Sommaire

- [Comparaison](#comparison)
- [Télécharger](#download)
- [Docker](#docker)
- [Fonctionnement](#how-it-works)
- [Questions fréquentes](#faq)
- [Projets liés](#related-projects)
- [Sécurité](#security)
- [Compiler depuis les sources](#build)
- [Licence](#license)

## <a name="comparison"></a>Comparaison

Où se situe [Twitch Miner](https://github.com/SunkwiBOT/twitch-miner)
par rapport à [Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
(points de chaîne) et à [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner) (Drops).
Vérifié dans leur code et leur README en novembre 2026.

| | Ce projet | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Campagnes de Drops : il les trouve, les mine et les réclame | ✅ | ❌ ¹ | ✅ |
| Points de chaîne : coffres, raids, moments, prédictions | ✅ | ✅ | ❌ |
| Spam Points : une récompense personnalisée d'une chaîne échangée plusieurs fois d'affilée | ✅ | ❌ | ❌ |
| Drops et points de chaîne en même temps sur un seul compte | ✅ | en partie ¹ | ❌ |
| Plusieurs comptes dans une seule application | ✅ | ❌ ² | ❌ |
| Modifier les réglages de plusieurs comptes d'un coup | ✅ | ❌ | ❌ |
| Interface graphique | ✅ bureau et web ³ | ❌ ⁴ | ✅ bureau |
| Tableau de bord web pour un serveur à la maison (Docker), avec mot de passe, accessible en HTTPS | ✅ | ❌ ⁴ | ❌ |
| Sessions chiffrées sur le disque, mot de passe de l'application en option | ✅ | ❌ | ❌ |
| Interface en 29 langues | ✅ | ❌ | en partie ⁵ |
| Mises à jour signées depuis l'application (bureau) | ✅ | ❌ | ❌ |
| Assistant de configuration pour chaque nouveau compte | ✅ | ❌ ⁴ | ❌ |
| Streamers ajoutés et modifiés dans l'interface, un par un ou plusieurs d'un coup | ✅ | ❌ ⁴ | — |
| Streamers importés par recherche, liste, autre compte ou chaînes suivies | ✅ | en partie ⁶ | — |
| Notifications Discord, Telegram et webhook avec images | ✅ | en partie ⁷ | ❌ |
| Calendrier de toutes les campagnes de Drops, avec recherche | ✅ | ❌ | en partie ⁸ |
| Ordre de minage par campagne, mots exclus, campagne ou récompense ignorée | ✅ | ❌ | ❌ ⁹ |
| Vos streamers listés regardés en premier pour les Drops d'une campagne | ✅ | en partie ¹⁰ | ❌ |
| Journal des événements filtrable et liste de chaque requête, dans l'interface | ✅ | ❌ ⁴ | ❌ |
| Environ 30 Mo de mémoire (serveur Docker, 4 comptes) | ✅ | — ¹¹ | — ¹¹ |

¹ Il ne regarde que les streamers de sa liste : il fait progresser et réclame les Drops de
leurs streams, sans choisir de campagne.\
² Un processus par compte.\
³ Sur votre PC comme application de bureau, et/ou sur un serveur (VPS, NAS, Raspberry Pi)
avec Docker, sans interface graphique ni navigateur sur cette machine : le tableau de bord
s'ouvre dans n'importe quel navigateur.\
⁴ Configuré dans un fichier Python ; sa page web montre des graphiques de points et le
fichier de logs.\
⁵ 21 langues.\
⁶ Il peut suivre les chaînes que le compte suit.\
⁷ Du texte seul, sans images. Il a aussi Matrix, Pushover et Gotify, que ce projet n'a pas.\
⁸ Une liste des campagnes avec des filtres, sans calendrier ni recherche.\
⁹ Il a des jeux prioritaires et des jeux exclus, choisis par nom de jeu.\
¹⁰ Par défaut, il regarde d'abord les streamers de sa liste dont le stream porte des Drops
(sa priorité `DROPS`), mais jamais une autre chaîne.\
¹¹ Non mesuré.

## <a name="download"></a>Télécharger

Téléchargez le fichier pour votre système depuis la
[dernière version](https://github.com/SunkwiBOT/twitch-miner/releases/latest).

| Système | Paquet |
|---|---|
| **Windows** · x64 | Installateur (`setup.exe`) ou [exécutable portable](https://github.com/SunkwiBOT/twitch-miner/releases/latest/download/TwitchMiner-portable.exe) |
| **macOS** 10.15 ou plus récent · Intel et Apple Silicon | Image disque (`.dmg`) |
| **Linux** · x86-64 | `.AppImage`, `.deb` (Debian, Ubuntu) ou `.rpm` (Fedora, openSUSE) |

La version macOS n'est pas notariée. La première fois, faites un clic droit sur
l'application et choisissez **Ouvrir**.

Les mises à jour sont signées. L'application cherche une nouvelle version au démarrage,
puis toutes les heures (vous pouvez changer la fréquence dans **Paramètres › À propos**).
Elle n'en installe jamais une toute seule : vous l'installez en un clic, depuis le message
qui s'affiche ou depuis **Paramètres › À propos**.

Les données sont dans le dossier de données de l'utilisateur :

- Linux : `~/.local/share/twitch-miner`
- Windows : `%APPDATA%\twitch-miner`
- macOS : `~/Library/Application Support/twitch-miner`

## <a name="docker"></a>Docker

Le serveur affiche le même tableau de bord dans un navigateur. L'image est
`ghcr.io/sunkwibot/twitch-miner`, pour `amd64` et `arm64`. Quand une
nouvelle version sort, le tableau de bord le dit : `docker compose pull`, puis
`docker compose up -d`.

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-miner:latest
    pull_policy: always   # prend l'image la plus récente à chaque démarrage
    restart: unless-stopped
    ports: ["8080:8080"]   # derrière un reverse proxy HTTPS : à retirer, ou "127.0.0.1:8080:8080"
    environment:
      MINER_PASSWORD: "change-me"   # obligatoire, 12 caractères ou plus
      # MINER_PUBLIC_URL: "https://miner.example.com"   # seulement derrière un reverse proxy HTTPS
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

Ouvrez <http://localhost:8080> (ou l'adresse de la machine sur votre réseau) et entrez le
mot de passe.

### Variables

| Variable | Par défaut | Rôle |
|---|---|---|
| `MINER_PASSWORD` | aucun | Mot de passe du tableau de bord, obligatoire : le serveur refuse de démarrer sans lui. De 12 caractères ou plus (plus court, un avertissement s'affiche au démarrage). La valeur d'exemple `change-me` est refusée. |
| `MINER_PASSWORD_FILE` | aucun | Lit `MINER_PASSWORD` depuis un fichier, par exemple un secret Docker. L'un ou l'autre, pas les deux. |
| `MINER_PUBLIC_URL` | aucun | Seulement derrière un reverse proxy HTTPS (Caddy, nginx, Traefik, Cloudflare Tunnel…) : l'adresse `https://` du tableau de bord, sans chemin, comme `https://miner.example.com`. Le proxy doit transmettre `X-Forwarded-Proto`, `X-Forwarded-For` et les websockets, ce que font les proxys courants par défaut, et le port du conteneur ne doit pas être joignable directement. Le tableau de bord refuse alors tout ce qui ne passe pas par le proxy en HTTPS, et bloque une adresse pendant 30 secondes après 5 mauvais mots de passe. Le proxy doit tourner sur la même machine ou son réseau local : une connexion depuis une adresse publique est refusée. |
| `MINER_TRUSTED_HOPS` | `1` | Avec `MINER_PUBLIC_URL` seulement : le nombre de proxys qui ajoutent une entrée à `X-Forwarded-For`, de 1 à 10. `2` avec deux proxys l'un derrière l'autre, comme Cloudflare devant Traefik. |
| `MINER_PROXY_NETWORKS` | aucun | Seulement avec `MINER_PUBLIC_URL` : d'autres réseaux depuis lesquels le proxy peut se connecter, en plus de cette machine et de son réseau local privé, séparés par des virgules : une seule adresse ou un réseau comme `2001:db8:1::/64`, pour un réseau local dont les adresses IPv6 sont globales. |
| `MINER_VAULT_PASSPHRASE` | aucun | Seulement pour un dossier de données copié depuis l'application de bureau avec un mot de passe de l'application : le déverrouille au démarrage. Un serveur créé de zéro n'en a jamais besoin. |
| `MINER_VAULT_PASSPHRASE_FILE` | aucun | Lit `MINER_VAULT_PASSPHRASE` depuis un fichier. L'un ou l'autre, pas les deux. |
| `DATA_DIR` | `/data` dans l'image, sinon `data` dans le dossier de travail | Base de données, fichier de clé et logs. |
| `LOG_DIR` | `$DATA_DIR/logs` | Fichiers de logs : un par jour, les 14 derniers gardés. |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | Adresse d'écoute : une adresse IP et un port de 1 à 65535. |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | Niveau des logs. |

Le serveur lit et vérifie les variables une seule fois, au démarrage. Une valeur invalide
arrête le serveur avec un message, au lieu d'un repli silencieux.

## <a name="how-it-works"></a>Fonctionnement

Toutes les 20 secondes, chaque compte choisit ses deux chaînes : la meilleure chaîne en
direct pour la première campagne de sa file, puis les streamers de sa liste de points.

Twitch crédite **une campagne de Drops** et **deux chaînes pour les points** à la fois par
compte. Depuis octobre 2026, un drop ne peut être réclamé que si le compte du jeu est lié.
Les campagnes non liées progressent quand même, et le tableau de bord affiche le lien.

La liste des campagnes vient du projet [Twitch Drops API](https://drops.sunkwi.com)
(actualisée toutes les 10 minutes), parce que Twitch la cache au client TV.

## <a name="faq"></a>Questions fréquentes

### Risques et Twitch

**Mon compte risque-t-il d'être banni ?**\
Personne ne peut le garantir. Ce projet n'est pas affilié à Twitch, et automatiser un
compte peut enfreindre ses conditions ou les règles d'une campagne. Le mineur regarde au
plus deux chaînes par compte, la limite de Twitch, sans télécharger de vidéo. À vous de
juger.

**La vidéo est-elle vraiment regardée ?**\
Non, aucune vidéo n'est téléchargée. Pour les Drops, le mineur lit toutes les 15 secondes
la liste de lecture du stream, en plus basse qualité, et demande seulement l'en-tête de
chaque nouveau segment. Pour les points, il envoie chaque minute le signal « minute
regardée » de Twitch. Chaque chaîne regardée affiche le nombre de segments demandés.

**Pourquoi deux chaînes au plus ?**\
C'est la limite de Twitch : il ne crédite que deux chaînes regardées en même temps par
compte, et une seule campagne de Drops à la fois. En mode **Drops et points**, une chaîne
sert la campagne et l'autre vos streamers. Le réglage « Une chaîne », dans **Paramètres du
compte**, est plus discret.

### Drops

**Une campagne n'est pas minée, ou un drop ne progresse pas.**\
Ouvrez la page **Drops** du compte. Dans « Ordre de minage », chaque campagne en attente dit
pourquoi : « Aucune chaîne en direct », « En attente (une campagne à la fois) » ou « Aucun
drop disponible pour l'instant ». Une campagne qui n'apparaît pas dans cette liste
a été écartée : par le réglage « Quelles campagnes », par une règle de l'onglet **Exclusions**, ou
parce qu'elle ne peut pas finir à temps. Une récompense qui demande un abonnement n'est jamais minée, et un drop qui en suit
un autre attend que le premier soit fini. En mode **Points uniquement**, le mineur ne
choisit aucune campagne. « Miner en priorité », dans la fiche d'une campagne, la met en
tête ; une campagne ignorée peut revenir avec « Miner à nouveau », dans l'onglet **Exclusions**.

**Faut-il lier le compte du jeu ?**\
Pour réclamer, oui : depuis octobre 2026, Twitch ne laisse réclamer un drop que si le
compte du jeu est lié. Par défaut, une campagne non liée progresse quand même et la
réclamation attend la liaison. Le badge « Non lié » le signale, et « Lier le compte de
jeu », dans la fiche de la campagne, ouvre la page de liaison. Pour ne miner que les jeux
liés, choisissez « Uniquement les jeux dont le compte est lié ».

### Installation et données

**Application de bureau ou Docker ?**\
L'application de bureau mine tant que votre PC est allumé, avec son icône dans la zone de
notification, un mot de passe en option et les mises à jour en un clic. Docker mine jour et
nuit sur une machine toujours allumée (VPS, NAS, Raspberry Pi) ; vous l'ouvrez dans un
navigateur, protégé par `MINER_PASSWORD`. L'interface est la même.

**Où sont mes données ?**\
Dans le dossier des données : `~/.local/share/twitch-miner` sous Linux,
`%APPDATA%\twitch-miner` sous Windows,
`~/Library/Application Support/twitch-miner` sous macOS, et le volume `/data`
avec Docker. Sur le bureau, **Paramètres › Bureau** l'ouvre en un clic. On y trouve
`miner.db` (comptes, réglages, journal), `master.key` (la clé des sessions, quand aucun mot
de passe de l'application n'est défini) et les logs.

**Comment mettre à jour ?**\
L'application de bureau cherche une nouvelle version au démarrage, puis toutes les heures
(réglable dans **Paramètres › À propos**). Cliquez sur le badge « Mise à jour », puis sur
« Installer et redémarrer ». Avec Docker, le tableau de bord prévient : lancez
`docker compose pull`, puis `docker compose up -d`.

**J'ai oublié le mot de passe de l'application.**\
Il ne peut pas être récupéré : c'est lui qui chiffre vos sessions Twitch. Quittez
l'application, supprimez le dossier des données, puis ajoutez à nouveau vos comptes ; leurs
réglages sont à refaire. Avec Docker, le mot de passe du tableau de bord se change dans
`MINER_PASSWORD`, puis `docker compose up -d`.

### Points et prédictions

**Combien de points par heure ?**\
Cela dépend de Twitch et de chaque chaîne. Le mineur gagne ce que Twitch donne à un
spectateur : les points de visionnage, les coffres bonus (environ un toutes les 15 minutes)
et le bonus de série de visionnage. Un abonnement multiplie les points, de ×1,2 à ×2 selon
le palier. L'accueil montre les points gagnés, et le Journal chaque gain.

**Les paris sont-ils risqués ?**\
Oui, des points peuvent être perdus. Les prédictions sont désactivées par défaut, streamer
par streamer. Vous réglez la stratégie, le moment du pari, la mise (5 % du solde par
défaut, 50 000 points au plus), un solde minimum et, si vous voulez, une condition, comme
une cote d'au moins 1,5. Gardez une mise faible.

**Un streamer n'est pas regardé.**\
Seuls les streamers activés et en direct sont regardés, et seulement sur une chaîne libre :
en mode **Drops et points**, l'une sert la campagne en cours ; en mode **Drops uniquement**,
aucun streamer n'est regardé ; un compte en pause ne regarde rien. Les places vont dans cet
ordre, affiché en badge sur chaque chaîne regardée : « Bonus de série », « Série menacée »,
« Abonné », puis « Ordre de la liste ». Montez le streamer dans la liste, ou coupez
« Priorité aux séries de visionnage » et « Priorité aux chaînes abonnées ».

## <a name="related-projects"></a>Projets liés

Autres projets du même auteur :

- [**Twitch Drops API**](https://drops.sunkwi.com) : une API publique et gratuite de toutes
  les campagnes de Drops et de récompenses Twitch en cours, mise à jour chaque minute.
  Cette application y lit son catalogue de campagnes.
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com) : un bot Discord qui annonce les lives
  Twitch et les campagnes de Drops sur votre serveur.
- [**Kick Alerts 🔔**](https://kick.sunkwi.com) : un bot Discord qui annonce les lives Kick et
  les campagnes de Drops sur votre serveur.

## <a name="security"></a>Sécurité

- Les jetons sont scellés avec AES-256-GCM. La clé est soit un `master.key` aléatoire à
  côté de la base de données (par défaut, sans intervention), soit dérivée du mot de passe
  de l'application avec Argon2id.
- Le tableau de bord HTTP vérifie le mot de passe en temps constant et utilise des cookies
  HttpOnly, SameSite=Strict (`Secure`, avec le préfixe `__Host-`, derrière HTTPS). Le
  serveur ne garde qu'un SHA-256 de chaque session. Il termine une session après 30 jours
  ou à la déconnexion. Il vérifie la session avant de lire un corps de requête et limite
  les corps à 256 Kio.
- Après 5 mauvais mots de passe de suite depuis une adresse IP (formulaire de connexion ou
  bearer), cette adresse attend 30 secondes. Le serveur répond `429` avec `Retry-After`
  sans regarder le mot de passe. Chaque mauvais mot de passe met aussi une demi-seconde à
  répondre.
- Hors de votre réseau, placez le tableau de bord derrière un reverse proxy HTTPS avec
  `MINER_PUBLIC_URL` (voir [Docker](#docker)).
- Le mineur ne parle qu'à `twitch.tv`, `ttvnw.net`, `jtvnw.net`, `twitchcdn.net`,
  `drops.sunkwi.com` et aux cibles de notification que vous configurez.
  L'application (bureau et Docker) cherche les nouvelles versions sur `github.com`, et
  l'application de bureau les télécharge depuis les versions GitHub du projet. Elles sont
  signées : seule la version affichée est installée.

## <a name="build"></a>Compiler depuis les sources

**Prérequis :** Rust, Node.js et les
[prérequis de Tauri](https://v2.tauri.app/start/prerequisites/) pour votre système.

```bash
npm ci
npm run build          # tableau de bord
cargo build --release -p miner-server   # serveur : target/release/twitch-miner
npm run bundle:linux   # paquets de bureau (dépendances Linux de Tauri requises)
```

Vérifications du code :

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier, oxlint (avec les types) et svelte-check
```

Les types TypeScript de l'interface sont générés depuis les types Rust avec
[ts-rs](https://github.com/Aleph-Alpha/ts-rs) dans `ui/src/lib/bindings`. Après un
changement d'un type que l'interface voit, lancez `npm run bindings`.

## <a name="license"></a>Licence

[MIT](../../LICENSE). Inspiré de [TwitchDropsMiner de DevilXD](https://github.com/DevilXD/TwitchDropsMiner),
de [Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
et de leurs forks.
