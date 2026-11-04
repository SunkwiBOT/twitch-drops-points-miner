<p align="center">
  <img src="../../ui/public/app-icon.svg" width="112" height="112" alt="Logótipo do Twitch Drops &amp; Points Miner">
</p>

<h1 align="center">Twitch Drops &amp; Points Miner</h1>

<p align="center">
  <strong>Minera Drops da Twitch e pontos do canal em várias contas, sem navegador nem separador da Twitch aberto.</strong>
</p>

<p align="center">
  Uma app para computador com Linux, Windows e macOS, ou uma imagem Docker com o mesmo painel web, para executar num servidor doméstico ou num Raspberry Pi. Com 4 contas, o servidor usa cerca de 30 MB de memória e quase nada de CPU.
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-drops-points-miner?label=release&color=9146FF" alt="Versão mais recente"></a>
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-drops-points-miner/total?label=downloads&color=9146FF" alt="Transferências"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="Plataformas: Linux, Windows, macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-drops-points-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="Imagem Docker para amd64 e arm64"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="Licença MIT"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="Visualizações">
</p>

<p align="center">
  <a href="#download"><strong>Transferir</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/issues/new/choose">Reportar um erro</a>
</p>

<p align="center">
  <a href="../../README.md">English</a> ·
  <a href="README.fr.md">Français</a> ·
  <a href="README.de.md">Deutsch</a> ·
  <a href="README.es-ES.md">Español&nbsp;(España)</a> ·
  <a href="README.es-419.md">Español&nbsp;(Latinoamérica)</a> ·
  <strong>Português&nbsp;(Portugal)</strong> ·
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
  <img src="../media/en/presentation.webp" alt="Apresentação em vídeo do Twitch Drops &amp; Points Miner">
</p>

## <a name="features"></a>Funcionalidades

O que o distingue de outros mineradores. «Ver em ação», por baixo de um ponto, mostra-o a
funcionar.

### Contas e streamers

- **Várias contas numa só app.** Cada conta minera de forma independente, no seu próprio
  modo: **Drops e pontos** (um canal para a melhor campanha, o outro para os seus
  streamers), **Só Drops** ou **Só pontos**.

  <details><summary>Ver em ação</summary>
  <img src="../media/en/accounts-mining.webp" alt="Várias contas a minerar ao mesmo tempo, na página Início">
  </details>

- **Um assistente para cada nova conta.** Depois de iniciar sessão, parta das definições
  predefinidas ou das de outra conta. Escolha o modo, as campanhas e os streamers e, por
  fim, «Começar a minerar» ou «Guardar e começar mais tarde».

  <details><summary>Ver em ação</summary>
  <img src="../media/en/account-setup.webp" alt="O assistente de configuração de uma nova conta, passo a passo">
  </details>

- **Uma alteração aplicada a outras contas num clique.** Depois de alterar algo numa
  conta, a mensagem de confirmação propõe «Aplicar a outras contas». A alteração é
  repetida nas contas que assinalar, que mantêm o resto das suas definições.

  <details><summary>Ver em ação</summary>
  <img src="../media/en/apply-to-accounts.webp" alt="Uma alteração repetida noutras contas com «Aplicar a outras contas»">
  </details>

- **Adicionar um streamer.** Começa logo com as opções predefinidas: as da app ou as que
  escolheu para esta conta. Ajuste-as no momento, se for preciso: baús de bónus, raids,
  momentos, sequência de visualização, chat, previsões.

  <details><summary>Ver em ação</summary>
  <img src="../media/en/streamer-add.webp" alt="Um streamer adicionado através da pesquisa da Twitch e, depois, as suas opções">
  </details>

- **Editar um streamer.** «Editar» abre as suas opções: baús de bónus, raids, momentos,
  sequência de visualização, chat, previsões… Um interruptor ativa-o ou desativa-o sem o
  retirar da lista.

  <details><summary>Ver em ação</summary>
  <img src="../media/en/streamer-edit.webp" alt="As opções de um streamer, abertas a partir da sua linha">
  </details>

- **Editar vários streamers de uma vez.** Assinale-os e clique em «Editar os N streamers
  selecionados». Só se aplicam as opções que alterar. Uma opção que difere entre eles
  aparece como «Misto», e o seu interruptor passa a dois botões, «Sim» e «Não». Depois, a
  app propõe torná-las as opções predefinidas para novos streamers.

  <details><summary>Ver em ação</summary>
  <img src="../media/en/streamers-group-edit.webp" alt="Edição de vários streamers, com opções «Misto»">
  </details>

- **Importar streamers.** Cole uma lista (até 500 nomes), ou reutilize a de outra conta ou
  os canais que a conta segue. A lista é verificada para só adicionar canais que existem.

  <details><summary>Ver em ação</summary>
  <img src="../media/en/streamers-import.webp" alt="Importação de streamers: uma lista colada, outra conta, os canais seguidos">
  </details>

### Drops

- **Todas as campanhas de Drops em curso.** Cada campanha tem a sua ficha: recompensas,
  associação da conta do jogo, «Minerar primeiro». A lista vem do projeto
  [Twitch Drops API](https://drops.sunkwi.com) e é atualizada a cada 10 minutos.

  <details><summary>Ver em ação</summary>
  <img src="../media/en/drops-catalogue.webp" alt="O Catálogo em calendário, a pesquisa e a ficha de uma campanha">
  </details>

- **A sua ordem de mineração.** Arraste as campanhas para a ordem que quiser. As novas
  encontram o seu lugar sozinhas: primeiro os seus jogos prioritários, depois as que
  terminam mais cedo ou as que têm menos minutos em falta. Exclua jogos, palavras, uma
  campanha ou uma única recompensa, e ignore as campanhas que não conseguem terminar a
  tempo. «Minerar primeiro», na ficha de uma campanha, coloca-a no topo.

  <details><summary>Ver em ação</summary>
  <img src="../media/en/drops-order.webp" alt="A ordem de mineração, os jogos prioritários e as exclusões">
  </details>

- **Inventário e «Reclamar tudo».** A página Inventário organiza as recompensas em
  «Por reclamar», «Em curso» e «Reclamadas». Reclame-as uma a uma, todas de uma vez, ou
  ative «Reclamar drops automaticamente».

  <details><summary>Ver em ação</summary>
  <img src="../media/en/drops-inventory.webp" alt="O inventário e o botão «Reclamar tudo»">
  </details>

- **«Os meus streamers primeiro para os Drops».** Para a campanha atual, um streamer da
  sua lista cujo direto conta para essa campanha passa à frente dos outros canais. O
  mesmo canal faz avançar o drop e dá-lhe os respetivos pontos do canal. Ativado por
  predefinição.

  <details><summary>Ver em ação</summary>
  <img src="../media/en/drops-streamers-first.webp" alt="Um streamer da lista, visto para os Drops e pelos seus pontos">
  </details>

### Acompanhamento

- **Um diário de tudo, e cada pedido.** O Diário guarda cada evento: drops, pontos ganhos
  e gastos, baús, apostas, raids, diretos que começam… Filtre-o por conta, por separador
  (Drops, Pontos, Outros) e por tipo. **Registos técnicos › Pedidos** lista cada pedido
  enviado à Twitch e a outros serviços, com o resultado e a duração. Os registos e os
  pedidos não contêm nenhum token nem segredo: «Copiar para um relatório de erro»
  copia-os, prontos a colar num relatório de erro no
  [GitHub](https://github.com/SunkwiBOT/twitch-drops-points-miner/issues) ou no
  [Discord](https://discord.gg/support-bots-891685895781113886).

  <details><summary>Ver em ação</summary>
  <img src="../media/en/journal-requests.webp" alt="O Diário filtrado e, depois, Registos técnicos › Pedidos">
  </details>

### No computador ou num servidor

- **A mesma interface em todo o lado.** A app para computador (Linux, Windows, macOS) e a
  imagem Docker mostram o mesmo painel. O Docker corre num VPS, num NAS ou num
  Raspberry Pi, sem ecrã nem navegador nessa máquina: o painel abre-se em qualquer
  navegador.
- **Leve.** Cerca de 30 MB de memória para 4 contas, quase nada de CPU e uma imagem Docker
  de cerca de 12 MB.

### Segurança e atualizações

- **As suas sessões, protegidas.** Sem palavra-passe da Twitch: inicia sessão em
  `twitch.tv/activate`, a partir do seu navegador. As sessões são cifradas no disco e pode
  revogá-las nas suas [definições da Twitch](https://www.twitch.tv/settings/connections).
  No computador, uma palavra-passe da app opcional bloqueia-a: nada é minerado até ser
  introduzida. Com o Docker, o painel tem a sua própria palavra-passe, bloqueia-se sozinho
  após alguns minutos sem atividade e funciona em HTTPS atrás de um proxy. Cinco
  palavras-passe erradas seguidas bloqueiam o endereço IP durante 30 segundos.

  <details><summary>Ver em ação</summary>
  <img src="../media/en/security-lock.webp" alt="O cadeado: a app bloqueada e, depois, desbloqueada com a sua palavra-passe">
  </details>

- **Atualizações assinadas, nunca automáticas.** Na barra superior aparece um emblema
  «Atualização» com o número da versão. «Instalar e reiniciar» pede primeiro uma
  confirmação; depois, a app instala a versão e reinicia. Com uma palavra-passe da app,
  reinicia bloqueada: introduza-a de novo para retomar a mineração. Só é instalada uma
  versão assinada pelo projeto. A versão Docker também avisa, com os comandos a executar.

  <details><summary>Ver em ação</summary>
  <img src="../media/en/updates.webp" alt="O emblema «Atualização», a confirmação e, depois, a instalação">
  </details>

### E ainda

- Como outros mineradores: baús de bónus, raids, momentos, metas da comunidade, previsões
  (estratégias, momento da aposta, condições), sequências de visualização e canais
  subscritos primeiro, presença no chat (pode ser desativada), opções predefinidas para
  novos streamers, notificações por Discord, Telegram e webhook com imagens, proxy global
  ou por conta.
- 29 idiomas, temas escuro e claro, notificações do ambiente de trabalho, ícone na área de
  notificação, arranque com o sistema.

  <details><summary>Ver em ação</summary>
  <img src="../media/en/basics.webp" alt="Idiomas, temas claro e escuro, uma notificação do ambiente de trabalho e o ícone na área de notificação">
  </details>

> [!WARNING]
> Este projeto não está afiliado à Twitch. Automatizar uma conta pode violar os termos da
> Twitch ou as regras de uma campanha.

## Índice

- [Comparação](#comparison)
- [Transferência](#download)
- [Docker](#docker)
- [Como funciona](#how-it-works)
- [Perguntas frequentes](#faq)
- [Projetos relacionados](#related-projects)
- [Segurança](#security)
- [Compilar a partir do código-fonte](#build)
- [Licença](#license)

## <a name="comparison"></a>Comparação

Como o [Twitch Drops & Points Miner](https://github.com/SunkwiBOT/twitch-drops-points-miner)
se compara com o [Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
(pontos do canal) e o [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner) (Drops).
Verificado no código e nos README de cada um em novembro de 2026.

| | Este projeto | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Campanhas de Drops: encontra-as, minera-as e reclama-as | ✅ | ❌ ¹ | ✅ |
| Pontos do canal: baús, raids, momentos, previsões | ✅ | ✅ | ❌ |
| Drops e pontos do canal em simultâneo numa só conta | ✅ | em parte ¹ | ❌ |
| Várias contas numa só app | ✅ | ❌ ² | ❌ |
| Alterar as definições de várias contas de uma vez | ✅ | ❌ | ❌ |
| Interface gráfica | ✅ computador e web ³ | ❌ ⁴ | ✅ computador |
| Painel web para um servidor doméstico (Docker), com palavra-passe e acessível por HTTPS | ✅ | ❌ ⁴ | ❌ |
| Sessões cifradas no disco, palavra-passe da app opcional | ✅ | ❌ | ❌ |
| Interface em 29 idiomas | ✅ | ❌ | em parte ⁵ |
| Atualizações assinadas a partir da própria app (computador) | ✅ | ❌ | ❌ |
| Assistente de configuração para cada nova conta | ✅ | ❌ ⁴ | ❌ |
| Streamers adicionados e editados na interface, um a um ou vários de uma vez | ✅ | ❌ ⁴ | — |
| Streamers importados por pesquisa, lista, outra conta ou canais seguidos | ✅ | em parte ⁶ | — |
| Notificações por Discord, Telegram e webhook com imagens | ✅ | em parte ⁷ | ❌ |
| Calendário de todas as campanhas de Drops, com pesquisa | ✅ | ❌ | em parte ⁸ |
| Ordem de mineração por campanha, palavras excluídas, campanha ou recompensa ignorada | ✅ | ❌ | ❌ ⁹ |
| Os streamers da sua lista vistos primeiro para os Drops de uma campanha | ✅ | em parte ¹⁰ | ❌ |
| Diário de eventos filtrável e lista de cada pedido, na interface | ✅ | ❌ ⁴ | ❌ |
| Cerca de 30 MB de memória (servidor Docker, 4 contas) | ✅ | — ¹¹ | — ¹¹ |

¹ Só vê os streamers da sua lista: faz avançar os Drops dos diretos deles e reclama-os,
sem escolher campanha.\
² Um processo por conta.\
³ No seu PC como app para computador, e/ou num servidor (VPS, NAS, Raspberry Pi) com
Docker, sem ambiente gráfico nem navegador nessa máquina: o painel abre-se em qualquer
navegador.\
⁴ Configurado num ficheiro Python; a sua página web mostra gráficos de pontos e o ficheiro
de registo.\
⁵ 21 idiomas.\
⁶ Pode seguir os canais que a conta segue.\
⁷ Só texto, sem imagens. Também tem Matrix, Pushover e Gotify, que este projeto não tem.\
⁸ Uma lista de campanhas com filtros, sem calendário nem pesquisa.\
⁹ Tem jogos prioritários e jogos excluídos, escolhidos pelo nome do jogo.\
¹⁰ Por predefinição, vê primeiro os streamers da sua lista cujo direto tem Drops (a sua
prioridade `DROPS`), mas nunca outro canal.\
¹¹ Não medido.

## <a name="download"></a>Transferência

Transfira o ficheiro para o seu sistema a partir da
[versão mais recente](https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest).

| Sistema | Pacote |
|---|---|
| **Windows** · x64 | Instalador (`setup.exe`) ou [executável portátil](https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest/download/TwitchDropsPointsMiner-portable.exe) |
| **macOS** 10.15 ou posterior · Intel e Apple Silicon | Imagem de disco (`.dmg`) |
| **Linux** · x86-64 | `.AppImage`, `.deb` (Debian, Ubuntu) ou `.rpm` (Fedora, openSUSE) |

A versão para macOS não está notarizada. Da primeira vez, clique com o botão direito na
app e escolha **Abrir**.

As atualizações são assinadas. A app verifica se há uma nova versão ao arrancar e, depois,
a cada hora (pode alterar a frequência em **Definições › Acerca de**). Nunca instala
nenhuma por iniciativa própria: a instalação faz-se com um clique, a partir da mensagem que
aparece ou de **Definições › Acerca de**.

Os dados ficam na pasta de dados do utilizador:

- Linux: `~/.local/share/twitch-drops-points-miner`
- Windows: `%APPDATA%\twitch-drops-points-miner`
- macOS: `~/Library/Application Support/twitch-drops-points-miner`

## <a name="docker"></a>Docker

O servidor mostra o mesmo painel num navegador. A imagem é
`ghcr.io/sunkwibot/twitch-drops-points-miner`, para `amd64` e `arm64`. Quando sai uma
nova versão, o painel indica-o: `docker compose pull` e depois `docker compose up -d`.

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-drops-points-miner:latest
    pull_policy: always   # obtém a imagem mais recente a cada arranque
    restart: unless-stopped
    ports: ["8080:8080"]   # atrás de um proxy inverso HTTPS: remova, ou "127.0.0.1:8080:8080"
    environment:
      MINER_PASSWORD: "change-me"   # obrigatória, 12 caracteres ou mais
      # MINER_PUBLIC_URL: "https://miner.example.com"   # só atrás de um proxy inverso HTTPS
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

Abra <http://localhost:8080> (ou o endereço da máquina na sua rede) e introduza a
palavra-passe.

### Variáveis

| Variável | Predefinição | Função |
|---|---|---|
| `MINER_PASSWORD` | nenhuma | Palavra-passe do painel, obrigatória: sem ela, o servidor recusa-se a arrancar. Com 12 caracteres ou mais (uma mais curta mostra um aviso ao arrancar). |
| `MINER_PASSWORD_FILE` | nenhuma | Lê `MINER_PASSWORD` a partir de um ficheiro, por exemplo um segredo do Docker. Uma ou outra, não as duas. |
| `MINER_PUBLIC_URL` | nenhuma | Só atrás de um proxy inverso HTTPS (Caddy, nginx, Traefik, Cloudflare Tunnel…): o endereço `https://` do painel, sem caminho, como `https://miner.example.com`. O proxy tem de reencaminhar `X-Forwarded-Proto`, `X-Forwarded-For` e os websockets, o que os proxies comuns fazem por predefinição, e a porta do contentor não pode estar acessível diretamente. O painel recusa então tudo o que não passe pelo proxy em HTTPS e bloqueia um endereço durante 30 segundos após 5 palavras-passe erradas. |
| `MINER_TRUSTED_HOPS` | `1` | Só com `MINER_PUBLIC_URL`: quantos proxies acrescentam uma entrada a `X-Forwarded-For`, de 1 a 10. `2` com dois proxies, um atrás do outro, como a Cloudflare à frente do Traefik. |
| `MINER_VAULT_PASSPHRASE` | nenhuma | Só para uma pasta de dados copiada da app para computador com uma palavra-passe da app: desbloqueia-a ao arrancar. Um servidor criado de raiz nunca precisa dela. |
| `MINER_VAULT_PASSPHRASE_FILE` | nenhuma | Lê `MINER_VAULT_PASSPHRASE` a partir de um ficheiro. Uma ou outra, não as duas. |
| `DATA_DIR` | `/data` na imagem; caso contrário, `data` no diretório de trabalho | Base de dados, ficheiro da chave e registos. |
| `LOG_DIR` | `$DATA_DIR/logs` | Ficheiros de registo: um por dia, e os últimos 14 são mantidos. |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | Endereço de escuta: um endereço IP e uma porta de 1 a 65535. |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | Nível de registo. |

O servidor lê e verifica as variáveis uma única vez, ao arrancar. Um valor inválido
interrompe o servidor com uma mensagem, em vez de recorrer silenciosamente a um valor
alternativo.

## <a name="how-it-works"></a>Como funciona

A cada 20 segundos, cada conta escolhe os seus dois canais: o melhor canal em direto para
a primeira campanha da sua fila e, depois, os streamers da sua lista de pontos.

A Twitch credita **uma campanha de Drops** e **dois canais para pontos** em simultâneo por
conta. Desde outubro de 2026, um drop só pode ser reclamado se a conta do jogo estiver
associada. As campanhas não associadas continuam a progredir, e o painel mostra a ligação
para a associar.

A lista de campanhas vem do projeto [Twitch Drops API](https://drops.sunkwi.com)
(atualizada a cada 10 minutos), porque a Twitch a esconde do cliente de TV.

## <a name="faq"></a>Perguntas frequentes

### Riscos e Twitch

**A minha conta pode ser banida?**\
Ninguém pode garantir que não. Este projeto não está afiliado à Twitch, e automatizar uma
conta pode violar os termos dela ou as regras de uma campanha. O minerador vê no máximo
dois canais por conta, o limite da Twitch, sem transferir nenhum vídeo. A decisão é sua.

**O vídeo é mesmo visto?**\
Não, nenhum vídeo é transferido. Para os Drops, o minerador lê a cada 15 segundos a lista
de reprodução do direto na qualidade mais baixa e só pede o cabeçalho de cada novo
segmento. Para os pontos, envia a cada minuto o sinal de «minuto visto» da Twitch. Cada
canal visto mostra quantos segmentos foram pedidos.

**Porquê dois canais no máximo?**\
É o limite da Twitch: só credita dois canais vistos em simultâneo por conta, e uma única
campanha de Drops de cada vez. No modo **Drops e pontos**, um canal serve a campanha e o
outro os seus streamers. A definição «Um canal», em **Definições da conta**, é mais
discreta.

### Drops

**Uma campanha não está a ser minerada, ou um drop não avança.**\
Abra a página **Drops** da conta. Em «Ordem de mineração», cada campanha em espera diz
porquê: «Nenhum canal em direto», «Em espera (uma campanha de cada vez)» ou «Ainda nenhum
drop disponível». Uma campanha que não aparece nessa lista foi posta de parte: pela
definição «Que campanhas», por uma regra do separador **Exclusões**, ou porque não
consegue terminar a tempo. Uma recompensa que exige uma subscrição nunca é minerada, e um
drop que vem depois de outro espera que o primeiro termine. No modo **Só pontos**, o
minerador não escolhe nenhuma campanha. «Minerar primeiro», na ficha de uma campanha,
coloca-a no topo; uma campanha ignorada pode voltar com «Voltar a minerar», no separador
**Exclusões**.

**Tenho de associar a conta do jogo?**\
Para reclamar, sim: desde outubro de 2026, a Twitch só deixa reclamar um drop se a conta
do jogo estiver associada. Por predefinição, uma campanha não associada continua a
progredir e a reclamação fica à espera da associação. O emblema «Não associada»
assinala-o, e «Associar a conta do jogo», na ficha da campanha, abre a página de
associação. Para minerar só jogos associados, escolha «Só jogos com a conta associada».

### Instalação e dados

**App para computador ou Docker?**\
A app para computador minera enquanto o seu PC está ligado, com o ícone na área de
notificação, uma palavra-passe opcional e atualizações num clique. O Docker minera dia e
noite numa máquina sempre ligada (VPS, NAS, Raspberry Pi); abre-o num navegador, protegido
por `MINER_PASSWORD`. A interface é a mesma.

**Onde estão os meus dados?**\
Na pasta de dados: `~/.local/share/twitch-drops-points-miner` no Linux,
`%APPDATA%\twitch-drops-points-miner` no Windows,
`~/Library/Application Support/twitch-drops-points-miner` no macOS, e o volume `/data` com
o Docker. No computador, **Definições › Ambiente de trabalho** abre-a num clique. Contém
`miner.db` (contas, definições, diário), `master.key` (a chave das sessões, quando não há
palavra-passe da app definida) e os registos.

**Como atualizo?**\
A app para computador verifica se há uma nova versão ao arrancar e, depois, a cada hora
(pode alterar isto em **Definições › Acerca de**). Clique no emblema «Atualização» e
depois em «Instalar e reiniciar». Com o Docker, o painel avisa: execute
`docker compose pull` e depois `docker compose up -d`.

**Esqueci-me da palavra-passe da app.**\
Não pode ser recuperada: é ela que cifra as suas sessões da Twitch. Feche a app, apague a
pasta de dados e volte a adicionar as suas contas; as definições delas têm de ser feitas
de novo. Com o Docker, a palavra-passe do painel muda-se em `MINER_PASSWORD`, seguida de
`docker compose up -d`.

### Pontos e previsões

**Quantos pontos por hora?**\
Depende da Twitch e de cada canal. O minerador ganha o que a Twitch dá a um espetador: os
pontos por tempo de visualização, os baús de bónus (cerca de um a cada 15 minutos) e o
bónus de sequência de visualização. Uma subscrição multiplica os pontos, de ×1,2 a ×2
consoante o nível. A página Início mostra os pontos ganhos, e o Diário cada ganho.

**As apostas são arriscadas?**\
Sim, pode perder pontos. As previsões estão desativadas por predefinição, streamer a
streamer. Define a estratégia, o momento da aposta, o valor apostado (5% do saldo por
predefinição, 50 000 pontos no máximo), um saldo mínimo e, se quiser, uma condição, como
uma cota de pelo menos 1,5. Mantenha a aposta pequena.

**Um streamer não está a ser visto.**\
Só são vistos os streamers ativados e em direto, e só num canal livre: no modo
**Drops e pontos**, um dos dois serve a campanha atual; no modo **Só Drops**, nenhum
streamer é visto; uma conta em pausa não vê nada. Os lugares são atribuídos por esta
ordem, mostrada num emblema em cada canal visto: «Bónus de sequência», «Sequência em
risco», «Subscrito» e, por fim, «Ordem da lista». Suba o streamer na lista, ou desative
«Sequências primeiro» e «Canais subscritos primeiro».

## <a name="related-projects"></a>Projetos relacionados

Outros projetos do mesmo autor:

- [**Twitch Drops API**](https://drops.sunkwi.com): uma API pública e gratuita com todas
  as campanhas de Drops e de recompensas da Twitch em curso, atualizada a cada minuto.
  Esta app obtém dela o seu catálogo de campanhas.
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com): um bot do Discord que anuncia no seu
  servidor os diretos da Twitch e as campanhas de Drops.
- [**Kick Alerts 🔔**](https://kick.sunkwi.com): um bot do Discord que anuncia no seu
  servidor os diretos da Kick e as campanhas de Drops.

## <a name="security"></a>Segurança

- Os tokens são selados com AES-256-GCM. A chave é um ficheiro `master.key` aleatório
  junto à base de dados (predefinição, nada a fazer) ou é derivada da palavra-passe da app
  com Argon2id.
- O painel HTTP verifica a palavra-passe em tempo constante e usa cookies HttpOnly e
  SameSite=Strict (`Secure`, com o prefixo `__Host-`, atrás de HTTPS). O servidor guarda
  apenas um SHA-256 de cada sessão. Encerra uma sessão ao fim de 30 dias ou quando se
  termina sessão. Verifica a sessão antes de ler qualquer corpo de pedido e limita os
  corpos a 256 KiB.
- Após 5 palavras-passe erradas seguidas a partir do mesmo endereço IP (formulário de
  início de sessão ou bearer), esse endereço tem de esperar 30 segundos. O servidor
  responde `429` com `Retry-After` sem olhar para a palavra-passe. Cada palavra-passe
  errada demora também meio segundo a obter resposta.
- Fora da sua rede, coloque o painel atrás de um proxy inverso HTTPS com
  `MINER_PUBLIC_URL` (consulte [Docker](#docker)).
- O minerador só comunica com `twitch.tv`, `ttvnw.net`, `jtvnw.net`, `twitchcdn.net`,
  `twitch-drops-api.sunkwi.com` e com os destinos de notificação que configurar. A app
  (computador e Docker) procura novas versões em `github.com`, e a app para computador
  transfere-as das releases do projeto no GitHub. São assinadas: só é instalada a versão
  mostrada.

## <a name="build"></a>Compilar a partir do código-fonte

**Pré-requisitos:** Rust, Node.js e os
[pré-requisitos do Tauri](https://v2.tauri.app/start/prerequisites/) para o seu sistema.

```bash
npm ci
npm run build          # painel
cargo build --release -p miner-server   # servidor: target/release/twitch-miner
npm run bundle:linux   # pacotes para computador (requer as dependências Linux do Tauri)
```

Verificações do código:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier, oxlint (com tipos) e svelte-check
```

Os tipos TypeScript da interface são gerados a partir dos tipos Rust com
[ts-rs](https://github.com/Aleph-Alpha/ts-rs) em `ui/src/lib/bindings`. Depois de alterar
um tipo que a interface vê, execute `npm run bindings`.

## <a name="license"></a>Licença

[MIT](../../LICENSE). Inspirado no
[TwitchDropsMiner de DevilXD](https://github.com/DevilXD/TwitchDropsMiner), no
[Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
e nos respetivos forks.
