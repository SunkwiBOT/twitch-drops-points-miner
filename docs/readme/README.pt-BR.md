<p align="center">
  <img src="../../ui/public/app-icon.svg" width="112" height="112" alt="Logo do Twitch Drops &amp; Points Miner">
</p>

<h1 align="center">Twitch Drops &amp; Points Miner</h1>

<p align="center">
  <strong>Minera Drops da Twitch e pontos do canal em várias contas, sem navegador e sem nenhuma aba da Twitch aberta.</strong>
</p>

<p align="center">
  Um app para desktop no Linux, Windows e macOS, ou uma imagem Docker com o mesmo painel web, para rodar em um servidor caseiro ou em um Raspberry Pi. Com 4 contas, o servidor usa cerca de 30 MB de memória e quase nada de CPU.
</p>

<p align="center">
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest"><img src="https://img.shields.io/github/v/release/SunkwiBOT/twitch-drops-points-miner?label=release&color=9146FF" alt="Última versão"></a>
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/releases"><img src="https://img.shields.io/github/downloads/SunkwiBOT/twitch-drops-points-miner/total?label=downloads&color=9146FF" alt="Downloads"></a>
  <a href="#download"><img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-9146FF" alt="Plataformas: Linux, Windows, macOS"></a>
  <a href="https://github.com/users/SunkwiBOT/packages/container/package/twitch-drops-points-miner"><img src="https://img.shields.io/badge/docker-amd64%20%7C%20arm64-9146FF?logo=docker&logoColor=white" alt="Imagem Docker para amd64 e arm64"></a>
  <a href="../../LICENSE"><img src="https://img.shields.io/badge/license-MIT-9146FF" alt="Licença MIT"></a>
  <img src="https://komarev.com/ghpvc/?username=SunkwiBOT-twitch-drops-points-miner&label=views&color=9146FF&style=flat" alt="Visualizações">
</p>

<p align="center">
  <a href="#download"><strong>Baixar</strong></a> •
  <a href="#docker">Docker</a> •
  <a href="https://drops.sunkwi.com">Twitch Drops API</a> •
  <a href="https://twitch.sunkwi.com">Twitch Alerts 🔔</a> •
  <a href="https://kick.sunkwi.com">Kick Alerts 🔔</a> •
  <a href="https://github.com/SunkwiBOT/twitch-drops-points-miner/issues/new/choose">Reportar um bug</a>
</p>

<p align="center">
  <a href="../../README.md">English</a> ·
  <a href="README.fr.md">Français</a> ·
  <a href="README.de.md">Deutsch</a> ·
  <a href="README.es-ES.md">Español&nbsp;(España)</a> ·
  <a href="README.es-419.md">Español&nbsp;(Latinoamérica)</a> ·
  <a href="README.pt-PT.md">Português&nbsp;(Portugal)</a> ·
  <strong>Português&nbsp;(Brasil)</strong> ·
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
  <img src="../media/pt-BR/presentation.avif" alt="Apresentação em vídeo do Twitch Drops &amp; Points Miner">
</p>

## <a name="features"></a>Recursos

O que o diferencia de outros mineradores. “Ver em ação”, abaixo de um item, mostra o
recurso funcionando.

### Contas e streamers

- **Várias contas em um só app.** Cada conta minera por conta própria, no seu próprio
  modo: **Drops e pontos** (um canal para a melhor campanha, o outro para os seus
  streamers), **Só Drops** ou **Só pontos**.

  <details open><summary>Ver em ação</summary>
  <img src="../media/pt-BR/accounts-mining.avif" alt="Várias contas minerando ao mesmo tempo, na página Início">
  </details>

- **Um assistente para cada conta nova.** Depois do login, comece pelas configurações
  padrão ou pelas de outra conta. Escolha o modo, as campanhas e os streamers e, no fim,
  “Começar a minerar” ou “Salvar e começar depois”.

  <details open><summary>Ver em ação</summary>
  <img src="../media/pt-BR/account-setup.avif" alt="O assistente de configuração de uma conta nova, passo a passo">
  </details>

- **Uma alteração aplicada a outras contas com um clique.** Depois de mudar algo em uma
  conta, a mensagem de confirmação oferece “Aplicar a outras contas”. A alteração é
  repetida nas contas que você marcar, e elas mantêm o resto das suas configurações.

  <details open><summary>Ver em ação</summary>
  <img src="../media/pt-BR/apply-to-accounts.avif" alt="Uma alteração repetida em outras contas com “Aplicar a outras contas”">
  </details>

- **Adicionar um streamer.** Ele começa na hora com as opções padrão: as do app ou as que
  você escolheu para esta conta. Ajuste-as na hora, se precisar: baús de bônus, raids,
  momentos, sequência, chat, previsões.

  <details open><summary>Ver em ação</summary>
  <img src="../media/pt-BR/streamer-add.avif" alt="Um streamer adicionado pela busca da Twitch e, depois, as opções dele">
  </details>

- **Editar um streamer.** “Editar” abre as opções dele: baús de bônus, raids, momentos,
  sequência, chat, previsões… Um interruptor o ativa ou desativa sem tirá-lo da lista.

  <details open><summary>Ver em ação</summary>
  <img src="../media/pt-BR/streamer-edit.avif" alt="As opções de um streamer, abertas a partir da linha dele">
  </details>

- **Editar vários streamers de uma vez.** Marque-os e clique em “Editar os N streamers
  selecionados”. Só as opções que você mudar são aplicadas. Uma opção que é diferente
  entre eles aparece como “Misto”, e o interruptor dela vira dois botões, “Sim” e “Não”.
  Depois, o app oferece transformá-las nas opções padrão para novos streamers.

  <details open><summary>Ver em ação</summary>
  <img src="../media/pt-BR/streamers-group-edit.avif" alt="Edição de vários streamers, com opções “Misto”">
  </details>

- **Importar streamers.** Cole uma lista (até 500 nomes), ou reaproveite a de outra conta
  ou os canais que a conta segue. A lista é verificada para adicionar só canais que
  existem.

  <details open><summary>Ver em ação</summary>
  <img src="../media/pt-BR/streamers-import.avif" alt="Importação de streamers: uma lista colada, outra conta, os canais seguidos">
  </details>

- **Spam Points.** Resgate uma recompensa personalizada de um canal várias vezes seguidas.
  Escolha um canal da sua lista de Pontos, ou qualquer canal encontrado pela busca da
  Twitch: as recompensas dele aparecem com o saldo da conta, lido em tempo real. Defina a
  quantidade (até 25, dentro do saldo) e clique em “Resgatar”. O resultado aparece no fim
  da rajada, e cada rajada ganha a sua linha no Diário.

  <details open><summary>Ver em ação</summary>
  <img src="../media/pt-BR/spam-points.avif" alt="Spam Points: um canal escolhido, 10 × uma recompensa com mensagem resgatada, os resgates chegam ao chat da Twitch do canal e, depois, a linha do Diário">
  </details>

### Drops

- **Todas as campanhas de Drops ativas.** Cada campanha tem a sua ficha: recompensas,
  vinculação da conta do jogo, “Minerar primeiro”. A lista vem do projeto
  [Twitch Drops API](https://drops.sunkwi.com) e é atualizada a cada 10 minutos.

  <details open><summary>Ver em ação</summary>
  <img src="../media/pt-BR/drops-catalogue.avif" alt="O Catálogo em formato de calendário, a busca e a ficha de uma campanha">
  </details>

- **A sua ordem de mineração.** Arraste as campanhas na ordem que quiser. As novas
  encontram o próprio lugar: primeiro os seus jogos prioritários, depois as que terminam
  antes ou as que têm menos minutos restantes. Exclua jogos, palavras, uma campanha ou uma
  única recompensa, e pule as campanhas que não dá tempo de concluir. “Minerar primeiro”,
  na ficha de uma campanha, coloca-a no topo.

  <details open><summary>Ver em ação</summary>
  <img src="../media/pt-BR/drops-order.avif" alt="A ordem de mineração, os jogos prioritários e as exclusões">
  </details>

- **Inventário e “Resgatar tudo”.** A página Inventário separa as recompensas em
  “Para resgatar”, “Em andamento” e “Resgatadas”. Resgate uma por uma, todas de uma vez,
  ou ative “Resgatar drops automaticamente”.

  <details open><summary>Ver em ação</summary>
  <img src="../media/pt-BR/drops-inventory.avif" alt="O inventário e o botão “Resgatar tudo”">
  </details>

- **“Meus streamers primeiro para os Drops”.** Para a campanha atual, um streamer da sua
  lista cuja live conta para essa campanha passa na frente dos outros canais. O mesmo
  canal faz o drop avançar e rende os pontos do canal dele. Ativado por padrão.

  <details open><summary>Ver em ação</summary>
  <img src="../media/pt-BR/drops-streamers-first.avif" alt="Um streamer da lista, assistido para os Drops e pelos pontos dele">
  </details>

### Acompanhamento

- **Um diário de tudo, e cada requisição.** O Diário guarda cada evento: drops, pontos
  ganhos e gastos, baús, apostas, raids, lives que começam… Filtre por conta, por aba
  (Drops, Pontos, Outros) e por tipo. **Registros técnicos › Requisições** lista cada
  requisição enviada à Twitch e a outros serviços, com o resultado e o tempo que levou.
  Os logs e as requisições não contêm nenhum token nem segredo: “Copiar para um relatório
  de bug” copia tudo, pronto para colar em um relatório de bug no
  [GitHub](https://github.com/SunkwiBOT/twitch-drops-points-miner/issues) ou no
  [Discord](https://discord.gg/support-bots-891685895781113886).

  <details open><summary>Ver em ação</summary>
  <img src="../media/pt-BR/journal-requests.avif" alt="O Diário filtrado e, depois, Registros técnicos › Requisições">
  </details>

### No desktop ou em um servidor

- **A mesma interface em todo lugar.** O app para desktop (Linux, Windows, macOS) e a
  imagem Docker mostram o mesmo painel. O Docker roda em um VPS, um NAS ou um
  Raspberry Pi, sem tela nem navegador nessa máquina: o painel abre em qualquer navegador.
- **Leve.** Cerca de 30 MB de memória para 4 contas, quase nada de CPU e uma imagem Docker
  de cerca de 12 MB.

### Segurança e atualizações

- **Suas sessões, protegidas.** Nada de senha da Twitch: você faz login em
  `twitch.tv/activate`, pelo seu navegador. As sessões ficam criptografadas no disco, e
  você pode revogá-las nas suas
  [configurações da Twitch](https://www.twitch.tv/settings/connections). No desktop, uma
  senha do app opcional o bloqueia: nada é minerado até ela ser digitada. Com o Docker, o
  painel tem a própria senha, se bloqueia sozinho depois de alguns minutos sem atividade
  e roda em HTTPS atrás de um proxy. Cinco senhas erradas seguidas bloqueiam o endereço IP
  por 30 segundos.

  <details open><summary>Ver em ação</summary>
  <img src="../media/pt-BR/security-lock.avif" alt="O cadeado: o app bloqueado e, depois, desbloqueado com a senha dele">
  </details>

- **Atualizações assinadas, nunca automáticas.** Um selo “Atualização” com o número da
  versão aparece na barra superior. “Instalar e reiniciar” primeiro pede confirmação;
  depois, o app instala a versão e reinicia. Com uma senha do app, ele reinicia
  bloqueado: digite a senha de novo para retomar a mineração. Só uma versão assinada pelo
  projeto é instalada. A versão Docker também avisa, com os comandos a rodar.

  <details open><summary>Ver em ação</summary>
  <img src="../media/pt-BR/updates.avif" alt="O selo “Atualização”, a confirmação e, depois, a instalação">
  </details>

### E também

- Como outros mineradores: baús de bônus, raids, momentos, metas da comunidade, previsões
  (estratégias, momento da aposta, condições), sequências e canais com inscrição primeiro,
  presença no chat (pode ser desativada), opções padrão para novos streamers, notificações
  por Discord, Telegram e webhook com imagens, proxy global ou por conta.
- 29 idiomas, temas escuro e claro, notificações da área de trabalho, ícone na bandeja,
  inicialização com o sistema.

  <details open><summary>Ver em ação</summary>
  <img src="../media/pt-BR/basics.avif" alt="Idiomas, temas claro e escuro, uma notificação da área de trabalho e o ícone na bandeja">
  </details>

> [!WARNING]
> Este projeto não tem vínculo com a Twitch. Automatizar uma conta pode violar os termos
> da Twitch ou as regras de uma campanha.

## Sumário

- [Comparação](#comparison)
- [Download](#download)
- [Docker](#docker)
- [Como funciona](#how-it-works)
- [Perguntas frequentes](#faq)
- [Projetos relacionados](#related-projects)
- [Segurança](#security)
- [Compilar a partir do código-fonte](#build)
- [Licença](#license)

## <a name="comparison"></a>Comparação

Como o [Twitch Drops & Points Miner](https://github.com/SunkwiBOT/twitch-drops-points-miner)
se compara ao [Twitch Channel Points Miner v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
(pontos do canal) e ao [TwitchDropsMiner](https://github.com/DevilXD/TwitchDropsMiner) (Drops).
Verificado no código e nos READMEs deles em novembro de 2026.

| | Este projeto | Channel Points Miner v2 | TwitchDropsMiner |
|---|:---:|:---:|:---:|
| Campanhas de Drops: encontra, minera e resgata | ✅ | ❌ ¹ | ✅ |
| Pontos do canal: baús, raids, momentos, previsões | ✅ | ✅ | ❌ |
| Spam Points: uma recompensa personalizada de um canal resgatada várias vezes seguidas | ✅ | ❌ | ❌ |
| Drops e pontos do canal ao mesmo tempo em uma conta | ✅ | em parte ¹ | ❌ |
| Várias contas em um só app | ✅ | ❌ ² | ❌ |
| Alterar as configurações de várias contas de uma vez | ✅ | ❌ | ❌ |
| Interface gráfica | ✅ desktop e web ³ | ❌ ⁴ | ✅ desktop |
| Painel web para um servidor caseiro (Docker), com senha e acessível por HTTPS | ✅ | ❌ ⁴ | ❌ |
| Sessões criptografadas no disco, senha do app opcional | ✅ | ❌ | ❌ |
| Interface em 29 idiomas | ✅ | ❌ | em parte ⁵ |
| Atualizações assinadas direto no app (desktop) | ✅ | ❌ | ❌ |
| Assistente de configuração para cada conta nova | ✅ | ❌ ⁴ | ❌ |
| Streamers adicionados e editados na interface, um por um ou vários de uma vez | ✅ | ❌ ⁴ | — |
| Streamers importados por busca, lista, outra conta ou canais seguidos | ✅ | em parte ⁶ | — |
| Notificações por Discord, Telegram e webhook com imagens | ✅ | em parte ⁷ | ❌ |
| Calendário de todas as campanhas de Drops, com busca | ✅ | ❌ | em parte ⁸ |
| Ordem de mineração por campanha, palavras excluídas, campanha ou recompensa ignorada | ✅ | ❌ | ❌ ⁹ |
| Os streamers da sua lista assistidos primeiro para os Drops de uma campanha | ✅ | em parte ¹⁰ | ❌ |
| Diário de eventos filtrável e lista de cada requisição, na interface | ✅ | ❌ ⁴ | ❌ |
| Cerca de 30 MB de memória (servidor Docker, 4 contas) | ✅ | — ¹¹ | — ¹¹ |

¹ Ele só assiste aos streamers da lista dele: faz avançar os Drops das lives deles e os
resgata, sem escolher campanha.\
² Um processo por conta.\
³ No seu PC como app para desktop, e/ou em um servidor (VPS, NAS, Raspberry Pi) com
Docker, sem ambiente gráfico nem navegador nessa máquina: o painel abre em qualquer
navegador.\
⁴ Configurado em um arquivo Python; a página web dele mostra gráficos de pontos e o
arquivo de log.\
⁵ 21 idiomas.\
⁶ Pode seguir os canais que a conta segue.\
⁷ Só texto, sem imagens. Também tem Matrix, Pushover e Gotify, que este projeto não tem.\
⁸ Uma lista de campanhas com filtros, sem calendário nem busca.\
⁹ Tem jogos prioritários e jogos excluídos, escolhidos pelo nome do jogo.\
¹⁰ Por padrão, assiste primeiro aos streamers da lista dele cuja live tem Drops (a
prioridade `DROPS` dele), mas nunca a outro canal.\
¹¹ Não medido.

## <a name="download"></a>Download

Baixe o arquivo do seu sistema na
[versão mais recente](https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest).

| Sistema | Pacote |
|---|---|
| **Windows** · x64 | Instalador (`setup.exe`) ou [executável portátil](https://github.com/SunkwiBOT/twitch-drops-points-miner/releases/latest/download/TwitchDropsPointsMiner-portable.exe) |
| **macOS** 10.15 ou mais recente · Intel e Apple Silicon | Imagem de disco (`.dmg`) |
| **Linux** · x86-64 | `.AppImage`, `.deb` (Debian, Ubuntu) ou `.rpm` (Fedora, openSUSE) |

O build para macOS não é notarizado. Na primeira vez, clique com o botão direito no app e
escolha **Abrir**.

As atualizações são assinadas. O app verifica se há uma nova versão ao iniciar e depois a
cada hora (você pode mudar a frequência em **Configurações › Sobre**). Ele nunca instala
uma sozinho: você instala com um clique, pela mensagem que aparece ou em
**Configurações › Sobre**.

Os dados ficam na pasta de dados do usuário:

- Linux: `~/.local/share/twitch-drops-points-miner`
- Windows: `%APPDATA%\twitch-drops-points-miner`
- macOS: `~/Library/Application Support/twitch-drops-points-miner`

## <a name="docker"></a>Docker

O servidor mostra o mesmo painel em um navegador. A imagem é
`ghcr.io/sunkwibot/twitch-drops-points-miner`, para `amd64` e `arm64`. Quando sai uma
versão nova, o painel avisa: `docker compose pull` e depois `docker compose up -d`.

```yaml
services:
  miner:
    image: ghcr.io/sunkwibot/twitch-drops-points-miner:latest
    pull_policy: always   # baixa a imagem mais nova a cada inicialização
    restart: unless-stopped
    ports: ["8080:8080"]   # atrás de um proxy reverso HTTPS: remova, ou "127.0.0.1:8080:8080"
    environment:
      MINER_PASSWORD: "change-me"   # obrigatória, 12 caracteres ou mais
      # MINER_PUBLIC_URL: "https://miner.example.com"   # só atrás de um proxy reverso HTTPS
    volumes:
      - miner-data:/data

volumes:
  miner-data:
```

```bash
docker compose up -d
```

Abra <http://localhost:8080> (ou o endereço da máquina na sua rede) e digite a senha.

### Variáveis

| Variável | Padrão | Função |
|---|---|---|
| `MINER_PASSWORD` | nenhum | Senha do painel, obrigatória: sem ela, o servidor se recusa a iniciar. Com 12 caracteres ou mais (uma mais curta mostra um aviso ao iniciar). O valor de exemplo `change-me` é recusado. |
| `MINER_PASSWORD_FILE` | nenhum | Lê `MINER_PASSWORD` de um arquivo, por exemplo um secret do Docker. Uma ou outra, não as duas. |
| `MINER_PUBLIC_URL` | nenhum | Só atrás de um proxy reverso HTTPS (Caddy, nginx, Traefik, Cloudflare Tunnel…): o endereço `https://` do painel, sem caminho, como `https://miner.example.com`. O proxy precisa repassar `X-Forwarded-Proto`, `X-Forwarded-For` e os websockets, o que os proxies comuns fazem por padrão, e a porta do contêiner não pode ficar acessível diretamente. Assim, o painel recusa tudo o que não passe pelo proxy em HTTPS e bloqueia um endereço por 30 segundos depois de 5 senhas erradas. O proxy precisa rodar na mesma máquina ou na rede local dela: uma conexão vinda de um endereço público é recusada. |
| `MINER_TRUSTED_HOPS` | `1` | Só com `MINER_PUBLIC_URL`: quantos proxies adicionam uma entrada a `X-Forwarded-For`, de 1 a 10. `2` com dois proxies, um atrás do outro, como a Cloudflare na frente do Traefik. |
| `MINER_PROXY_NETWORKS` | nenhum | Só com `MINER_PUBLIC_URL`: outras redes de onde o proxy pode se conectar, além desta máquina e da rede local privada dela, separadas por vírgulas: um único endereço ou uma rede como `2001:db8:1::/64`, para uma rede local com endereços IPv6 globais. |
| `MINER_VAULT_PASSPHRASE` | nenhum | Só para uma pasta de dados copiada do app para desktop com uma senha do app: desbloqueia a pasta ao iniciar. Um servidor criado do zero nunca precisa dela. |
| `MINER_VAULT_PASSPHRASE_FILE` | nenhum | Lê `MINER_VAULT_PASSPHRASE` de um arquivo. Uma ou outra, não as duas. |
| `DATA_DIR` | `/data` na imagem; fora dela, `data` no diretório de trabalho | Banco de dados, arquivo de chave e logs. |
| `LOG_DIR` | `$DATA_DIR/logs` | Arquivos de log: um por dia, mantendo os últimos 14. |
| `HOST` / `PORT` | `0.0.0.0` / `8080` | Endereço de escuta: um endereço IP e uma porta de 1 a 65535. |
| `RUST_LOG` | `info,hyper=warn,reqwest=warn` | Nível de log. |

O servidor lê e valida as variáveis uma única vez, ao iniciar. Um valor inválido
interrompe o servidor com uma mensagem, em vez de cair silenciosamente em um valor
alternativo.

## <a name="how-it-works"></a>Como funciona

A cada 20 segundos, cada conta escolhe seus dois canais: o melhor canal ao vivo para a
primeira campanha da sua fila e, depois, os streamers da sua lista de pontos.

A Twitch credita **uma campanha de Drops** e **dois canais para pontos** ao mesmo tempo
por conta. Desde outubro de 2026, um drop só pode ser resgatado quando a conta do jogo
está vinculada. Campanhas não vinculadas continuam progredindo, e o painel mostra o link
para vincular a conta.

A lista de campanhas vem do projeto [Twitch Drops API](https://drops.sunkwi.com)
(atualizada a cada 10 minutos), porque a Twitch a esconde do cliente de TV.

## <a name="faq"></a>Perguntas frequentes

### Riscos e Twitch

**Minha conta pode ser banida?**\
Ninguém pode garantir que não. Este projeto não tem vínculo com a Twitch, e automatizar
uma conta pode violar os termos dela ou as regras de uma campanha. O minerador assiste a
no máximo dois canais por conta, o limite da Twitch, sem baixar nenhum vídeo. A decisão é
sua.

**O vídeo é mesmo assistido?**\
Não, nenhum vídeo é baixado. Para os Drops, a cada 15 segundos o minerador lê a playlist
da live na qualidade mais baixa e só pede o cabeçalho de cada segmento novo. Para os
pontos, ele envia a cada minuto o sinal de “minuto assistido” da Twitch. Cada canal
assistido mostra quantos segmentos foram pedidos.

**Por que no máximo dois canais?**\
É o limite da Twitch: ela só credita dois canais assistidos ao mesmo tempo por conta, e
uma única campanha de Drops por vez. No modo **Drops e pontos**, um canal serve à campanha
e o outro aos seus streamers. A opção “Um canal”, em **Configurações da conta**, é mais
discreta.

### Drops

**Uma campanha não está sendo minerada, ou um drop não avança.**\
Abra a página **Drops** da conta. Em “Ordem de mineração”, cada campanha em espera diz o
motivo: “Nenhum canal ao vivo”, “Aguardando (uma campanha por vez)” ou “Nenhum drop
disponível ainda”. Uma campanha que não aparece nessa lista foi deixada de fora: pela
opção “Quais campanhas”, por uma regra da aba **Exclusões**, ou porque não dá tempo de
concluí-la. Uma recompensa que exige inscrição nunca é minerada, e um drop que vem depois
de outro espera o primeiro terminar. No modo **Só pontos**, o minerador não escolhe
nenhuma campanha. “Minerar primeiro”, na ficha de uma campanha, coloca-a no topo; uma
campanha ignorada pode voltar com “Minerar novamente”, na aba **Exclusões**.

**Preciso vincular a conta do jogo?**\
Para resgatar, sim: desde outubro de 2026, a Twitch só deixa resgatar um drop quando a
conta do jogo está vinculada. Por padrão, uma campanha não vinculada continua progredindo
e o resgate espera a vinculação. O selo “Não vinculada” indica isso, e “Vincular a conta
do jogo”, na ficha da campanha, abre a página de vinculação. Para minerar só jogos
vinculados, escolha “Só jogos com a conta vinculada”.

### Instalação e dados

**App para desktop ou Docker?**\
O app para desktop minera enquanto o seu PC está ligado, com o ícone na bandeja, uma senha
opcional e atualizações com um clique. O Docker minera dia e noite em uma máquina sempre
ligada (VPS, NAS, Raspberry Pi); você o abre em um navegador, protegido por
`MINER_PASSWORD`. A interface é a mesma.

**Onde ficam os meus dados?**\
Na pasta de dados: `~/.local/share/twitch-drops-points-miner` no Linux,
`%APPDATA%\twitch-drops-points-miner` no Windows,
`~/Library/Application Support/twitch-drops-points-miner` no macOS, e o volume `/data` no
Docker. No desktop, **Configurações › Área de trabalho** abre a pasta com um clique. Ela
contém `miner.db` (contas, configurações, diário), `master.key` (a chave das sessões,
quando nenhuma senha do app está definida) e os logs.

**Como atualizar?**\
O app para desktop verifica se há uma nova versão ao iniciar e depois a cada hora (dá
para mudar isso em **Configurações › Sobre**). Clique no selo “Atualização” e depois em
“Instalar e reiniciar”. No Docker, o painel avisa: rode `docker compose pull` e depois
`docker compose up -d`.

**Esqueci a senha do app.**\
Ela não pode ser recuperada: é ela que criptografa as suas sessões da Twitch. Feche o app,
apague a pasta de dados e adicione as suas contas de novo; as configurações delas terão
de ser refeitas. No Docker, a senha do painel é trocada em `MINER_PASSWORD`, seguida de
`docker compose up -d`.

### Pontos e previsões

**Quantos pontos por hora?**\
Depende da Twitch e de cada canal. O minerador ganha o que a Twitch dá a um espectador:
os pontos por tempo assistido, os baús de bônus (cerca de um a cada 15 minutos) e o bônus
de sequência. Uma inscrição multiplica os pontos, de ×1,2 a ×2 conforme o nível. A página
Início mostra os pontos ganhos, e o Diário, cada ganho.

**As apostas são arriscadas?**\
Sim, dá para perder pontos. As previsões vêm desativadas por padrão, streamer por
streamer. Você define a estratégia, o momento da aposta, o valor apostado (5% do saldo por
padrão, no máximo 50.000 pontos), um saldo mínimo e, se quiser, uma condição, como uma odd
de pelo menos 1,5. Mantenha a aposta pequena.

**Um streamer não está sendo assistido.**\
Só são assistidos os streamers ativados e ao vivo, e só em um canal livre: no modo
**Drops e pontos**, um dos dois serve à campanha atual; no modo **Só Drops**, nenhum
streamer é assistido; uma conta pausada não assiste a nada. As vagas seguem esta ordem,
mostrada em um selo em cada canal assistido: “Bônus de sequência”, “Sequência em risco”,
“Inscrito” e, por último, “Ordem da lista”. Suba o streamer na lista, ou desative
“Sequências primeiro” e “Canais com inscrição primeiro”.

## <a name="related-projects"></a>Projetos relacionados

Outros projetos do mesmo autor:

- [**Twitch Drops API**](https://drops.sunkwi.com): uma API pública e gratuita com todas
  as campanhas de Drops e de recompensas da Twitch ativas, atualizada a cada minuto. Este
  app busca nela o seu catálogo de campanhas.
- [**Twitch Alerts 🔔**](https://twitch.sunkwi.com): um bot do Discord que anuncia no seu
  servidor as lives da Twitch e as campanhas de Drops.
- [**Kick Alerts 🔔**](https://kick.sunkwi.com): um bot do Discord que anuncia no seu
  servidor as lives da Kick e as campanhas de Drops.

## <a name="security"></a>Segurança

- Os tokens são selados com AES-256-GCM. A chave é um arquivo `master.key` aleatório ao
  lado do banco de dados (padrão, nada a fazer) ou é derivada da senha do app com
  Argon2id.
- O painel HTTP verifica a senha em tempo constante e usa cookies HttpOnly e
  SameSite=Strict (`Secure`, com o prefixo `__Host-`, atrás de HTTPS). O servidor guarda
  só um SHA-256 de cada sessão. Ele encerra uma sessão depois de 30 dias ou no logout.
  Ele verifica a sessão antes de ler qualquer corpo de requisição e limita os corpos a
  256 KiB.
- Depois de 5 senhas erradas seguidas de um mesmo endereço IP (formulário de login ou
  bearer), esse endereço precisa esperar 30 segundos. O servidor responde `429` com
  `Retry-After` sem olhar a senha. Cada senha errada também leva meio segundo para
  receber resposta.
- Fora da sua rede, coloque o painel atrás de um proxy reverso HTTPS com
  `MINER_PUBLIC_URL` (veja [Docker](#docker)).
- O minerador só se comunica com `twitch.tv`, `ttvnw.net`, `jtvnw.net`, `twitchcdn.net`,
  `twitch-drops-api.sunkwi.com` e os destinos de notificação que você configurar. O app
  (desktop e Docker) verifica novas versões no `github.com`, e o app para desktop as
  baixa das releases do projeto no GitHub. Elas são assinadas: só a versão mostrada é
  instalada.

## <a name="build"></a>Compilar a partir do código-fonte

**Pré-requisitos:** Rust, Node.js e os
[pré-requisitos do Tauri](https://v2.tauri.app/start/prerequisites/) para o seu sistema.

```bash
npm ci
npm run build          # painel
cargo build --release -p miner-server   # servidor: target/release/twitch-miner
npm run bundle:linux   # pacotes para desktop (precisa das dependências do Tauri para Linux)
```

Verificações de código:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
npm run verify         # prettier, oxlint (com tipos) e svelte-check
```

Os tipos TypeScript da interface são gerados a partir dos tipos Rust com o
[ts-rs](https://github.com/Aleph-Alpha/ts-rs) em `ui/src/lib/bindings`. Depois de
alterar um tipo que a interface vê, rode `npm run bindings`.

## <a name="license"></a>Licença

[MIT](../../LICENSE). Inspirado no
[TwitchDropsMiner de DevilXD](https://github.com/DevilXD/TwitchDropsMiner), no
[Twitch-Channel-Points-Miner-v2](https://github.com/rdavydov/Twitch-Channel-Points-Miner-v2)
e nos forks deles.
