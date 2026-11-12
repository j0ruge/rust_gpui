# Contexto — experimento Rust + GPUI

Data: 2026-11-12. Estado: `implemented_with_qa_limit`. Este documento preserva decisões e execução.
Plano: [tarefas GPUI](docs/superpowers/plans/2026-11-12-tarefas-gpui.md).
Retomada: [EXECUTAR_COM_SOL.md](EXECUTAR_COM_SOL.md).
Execução autorizada pelo usuário em 2026-11-12 após leitura do handoff.

## Glossário

| Termo | Significado | Decisão |
|---|---|---|
| Tarefa / task | Item pessoal de trabalho registrado localmente | Aplicação escolhida pelo usuário |
| Pendente / pending | Tarefa ainda não concluída | Rótulo confirmado; token definido na spec |
| Concluída / completed | Tarefa marcada como terminada | Rótulo confirmado; token definido na spec |
| Reabrir / reopen | Voltar uma tarefa concluída para pendente | Ação confirmada |
| Filtro / filter | Visualização Todas, Pendentes ou Concluídas | Confirmado; não altera as tarefas |
| Salvamento automático / autosave | Persistir cada ação confirmada no computador | SQLite; rascunho não confirmado não é salvo |
| GPUI Entity | Entidade de estado do framework | Não equivale automaticamente a uma entidade de domínio DDD |
| QA nativo | Percorrer jornadas na janela desktop real | Meio de verificação proposto |

## Fluxo e decisões

1. Ler a referência, reunir evidências e entrevistar com brainstorming e
   grill-with-docs, uma pergunta de cada vez.
2. Definir abordagem e desenho; validar DDD e Kaizen; fechar plano autocontido.
3. Usuário faz `/clear` e escolhe modelo mais barato antes da execução.
4. Implementar por incrementos e verificar cada entrega.
5. Planejar QA com `qa-report`, executar com `qa-execution`, registrar evidência.
6. Após novo `/clear`, iniciar o goal solicitado de revisão e correção até
   todas as categorias do codereview serem A. Goal iniciado em 2026-11-12;
   achados e evidências em `docs/reviews/2026-11-12-codereview.md`.

Respostas confirmadas pelo usuário nesta sessão:

- Lista de tarefas local, em vez do contador do documento.
- Salvar automaticamente no computador, sem conta ou servidor.
- Criar, editar, concluir/reabrir e excluir; filtros Todas/Pendentes/Concluídas.
- Deixar prazos, prioridades e projetos para depois.
- Criar `AGENTS.md` e duas skills separadas: Rust e GPUI.
- Objetivo principal: aprender Rust para uso no dia a dia, aproveitando LLM para
  reduzir a curva de aprendizado. A aplicação de tarefas é o laboratório inicial.
- Experiência declarada com Rust: praticamente zero.

Direção final explícita do usuário: construir a aplicação primeiro para vê-la
rodando; planejar agora e executar com Sol após `/clear`. A proposta anterior
de intercalar aulas e exercícios não foi adotada. Incrementos continuam pequenos
por engenharia, com primeira entrega sendo uma janela real.

O usuário também pediu registrar todo aprendizado reutilizável nas skills
respectivas. Rust/Cargo/build em `$rust`; GPUI/componentes/QA nativo em `$gpui`;
decisões próprias deste produto permanecem nestes documentos.

Artefatos desse pedido criados: `AGENTS.md`, este `CONTEXT.md`,
`KAIZEN_LOG.md`, `/home/joruge/.codex/skills/rust/SKILL.md` e
`/home/joruge/.codex/skills/gpui/SKILL.md`. As duas skills passaram no validador
estrutural; isso não valida o futuro aplicativo.

## Evidências observadas no planejamento

- A pasta inicialmente continha apenas `Rust e o Editor Zed.md`, sem manifesto,
  código, README ou instruções locais. `git status --short --branch` retornou
  `fatal: not a git repository (or any of the parent directories): .git`.
- Host observado: Ubuntu 24.04.5 LTS, sessão `x11`, `DISPLAY=:1`.
- `rustc --version`: 1.97.0; `cargo --version`: 1.97.0.
- `rustup show active-toolchain`: `stable-x86_64-unknown-linux-gnu`.
- `cargo fmt --version`: rustfmt 1.9.0; `cargo clippy --version`: clippy 0.1.97.
- `vulkaninfo --summary` detectou Intel UHD Graphics 770 e llvmpipe. Isso é
  inventário, não prova de execução do GPUI nem medição de desempenho.
- `xdotool`, `wmctrl` e ImageMagick `import` existem; controle e captura da
  janela da futura aplicação ainda não foram testados.
- Documentação consultada em docs.rs expõe GPUI 0.2.2; versão da aplicação
  agora fixada para o primeiro build. O exemplo oficial usa dois argumentos na closure de
  `open_window`, diferentemente do snippet fornecido. Nenhum dos exemplos foi
  compilado nesta sessão.
- Inspeção posterior dos pacotes publicados: component/assets 0.5.1 declaram
  dependência GPUI 0.2.2; APIs de input/Root conferidas no fonte. Compatibilidade
  completa e build ainda não executados.
- clang, cmake, pkg-config e vários headers de desenvolvimento estão ausentes;
  inventário e comandos de preparação estão no plano. Nenhum pacote foi instalado.

## Mapa de decisões do grill

As respostas do usuário estão identificadas. As demais decisões foram fechadas
pelo planejador após o pedido para concluir o plano; não são respostas atribuídas
ao usuário. Não há pergunta de produto bloqueando o handoff. Incertezas de build
e ambiente serão verificadas na execução, sem fingir que já passaram.

| ID | Questão | Situação / proposta |
|---|---|---|
| Q01 | Qual aplicação experimentar? | Resolvida: tarefas locais |
| Q02 | Preservar tarefas entre sessões? | Resolvida: autosave local |
| Q03 | Quais operações entram no MVP? | Resolvida: CRUD, concluir/reabrir e filtros |
| Q04 | Quais sistemas precisam funcionar? | Planejador: Ubuntu 24.04 x86_64/X11; outros sistemas fora do MVP |
| Q05 | Qual abordagem de UI e armazenamento? | Planejador: GPUI + component 0.5.1 + SQLite; alternativas na spec |
| Q06 | Como excluir sem perder uma tarefa por engano? | Planejador: confirmação com Cancelar em foco; sem undo |
| Q07 | Quais regras do título, ordem e edição? | Planejador: trim, 1–200 escalares Unicode, sem controles, duplicatas permitidas, ID crescente |
| Q08 | Como agir se leitura ou gravação falhar? | Planejador: erro visível, preservar rascunho/banco, retry explícito; snapshot só após commit |
| Q09 | Como lidar com duas instâncias abertas? | Planejador: uma por diretório, lock do SO mantido vivo |
| Q10 | Qual aparência e interação por teclado? | Planejador: tema escuro, pt-BR, tela única, foco/atalhos conforme spec |
| Q11 | Qual volume e expectativa de desempenho? | Planejador: 1.000 tarefas, ação/filtro até 1s medido em release; sem promessa de FPS |
| Q12 | Como executar e distribuir? | Planejador: Cargo + binário local; sem instalador/publicação remota |
| Q13 | Quais versões/dependências e requisitos nativos? | Versões e comandos fixados no plano; instalação/build pendentes |
| Q14 | Como testar a janela real? | QA nativo X11 com evidência; limitações instrumentais explícitas |
| Q15 | Como retomar após clear e escolher modelo? | Usuário escolheu Sol; instruções em EXECUTAR_COM_SOL.md; troca pelo usuário |
| Q16 | Qual baseline Git para revisão? | Inicializar develop, baseline docs, feature branch; registrar SHAs durante execução |
| Q17 | Qual experiência prévia com Rust? | Resolvida: praticamente zero |
| Q18 | Como dividir escrita e revisão entre usuário e LLM? | Usuário: LLM constrói primeiro; aprendizado depois com app funcionando |

## Checklist de aderência DDD e Kaizen

No planejamento, os critérios abaixo orientaram o desenho DDD/Kaizen. A execução
e o QA posteriores estão registrados em “Entrega local”; AC08 permanece parcial
pela medição de até 1s pendente. A revisão atual está em `docs/reviews/2026-11-12-codereview.md`.

### DDD

- [x] Identificar atividade do usuário: gerenciar tarefas pessoais locais.
- [x] Iniciar glossário e distinguir entidades GPUI de entidades do domínio.
- [x] Fechar significado dos estados, comandos e invariantes de tarefa na spec.
- [x] Definir uma fronteira de domínio simples para tarefas locais; separar
  domínio, apresentação e armazenamento apenas onde houver responsabilidade.
- [x] Definir cenários de aceite com termos iguais aos usados na interface.
- [x] Conferir linguagem contra implementação e rótulos reais quando existirem.
- [x] Avaliar necessidade real de repositório/abstrações; eventos, CQRS e
  microserviços não são requisitos desta missão.

Base conceitual consultada: skill `ddd` 0.6.0, referências
`strategic-design.md` (Ubiquitous Language), `refactoring-and-insights.md`
§4.1 e `architecture-styles.md`; Evans, DDD parte IV; Vernon, IDDD cap.4.
Essas fontes orientam a análise, não comprovam aderência do aplicativo futuro.

### Kaizen

- [x] Gemba inicial: ler referência e inspecionar pasta, Rust e ambiente gráfico.
- [x] Reduzir escopo: aplicativo local, sem prazos, prioridades ou projetos.
- [x] Fechar problema em uma frase com métrica de sucesso observável (AC01–AC11).
- [x] Escrever incrementos pequenos com Plan/Do/Check/Act e reversão.
- [x] Prever primeira entrega executável e testes proporcionais aos riscos.
- [x] Definir prevenção de perda de dados e plano de falhas deliberadas em área isolada.
- [x] Executar e comprovar as prevenções e falhas planejadas.
- [x] Verificar cada incremento no artefato real; corrigir falhas antes de avançar.
- [x] Registrar lições e padronização no `KAIZEN_LOG.md` após cada ciclo concluído.

### Aceite e revisão futuros

- [x] Plano autocontido e handoff prontos antes de implementar.
- [x] Usuário retoma após clear e solicita execução ao Sol.
- [x] `qa-report`: jornadas, personas, cenários e charters em `docs/qa/`.
- [x] `qa-execution`: interações nativas, reabertura, evidências e falhas registradas.
- [x] Codereview configurado para Rust (`.rs`, testes, Cargo e docs), base `develop`.
- [x] Categorias de revisão avaliadas após correções; veredito do último HEAD na
  conversa e histórico em `docs/reviews/2026-11-12-codereview.md`. AC08 parcial
  não recebe Pass de desempenho.

## Fontes e skills localizadas

- Referência original: [Rust e o Editor Zed.md](Rust%20e%20o%20Editor%20Zed.md).
- [Exemplo oficial GPUI](https://gpui.rs/).
- [API GPUI 0.2.2](https://docs.rs/gpui/0.2.2/gpui/struct.App.html#method.open_window).
- [Build Zed Linux](https://zed.dev/docs/development/linux): referência auxiliar;
  dependências de Zed inteiro não são automaticamente requisitos deste app.
- `grill-with-docs`: `/home/joruge/.claude/skills/grill-with-docs/SKILL.md`.
- `ddd`: `/home/joruge/.claude/plugins/cache/chewiesoft-marketplace/ddd/0.6.0/skills/ddd/SKILL.md`.
- `kaizen-software`: `/home/joruge/.agents/skills/kaizen-software/SKILL.md` (1.3.5 lida).
- `qa-report`: `/home/joruge/.claude/skills/qa-report/SKILL.md`.
- `qa-execution`: `/home/joruge/.claude/skills/qa-execution/SKILL.md`.
- Codereview fornecido pelo usuário: 2.27.16. Há registro local de 2.27.37
  instalado no cache Claude. Resolver a versão aplicável na fase de revisão;
  não misturar contrato de uma versão com scripts de outra.


## Execução — incrementos 1 e 2

Janela real do bootstrap validada, incluindo fechamento exit 0. Regras puras e
resolução de diretório passaram em dez testes após RED comportamental (nove
falhas, uma recusa já atendida pelo contrato de erro). Rejeição de controles
ocorre antes de trim para não aceitar tab/quebra nas bordas. Títulos mantêm
espaços internos; limite usa escalares Unicode. Ver `docs/stack.md`.


## Entrega local — 2026-11-12

Código no checkpoint ef5f8ef da feature, base develop 802b129. A janela final
foi aberta com cargo run --locked --release, sem diretório de QA. Binário
SHA256 25de9f5f56850fb0135df9b45e3d970c86bdeda508d7c158f9e46cc2fb569869.
CRUD, filtros, autosave e recuperação implementados. Trinta testes; gate técnico,
QA e evidências em docs/stack.md e docs/qa/reports/2026-11-12-full.md.

Quatro jornadas percorridas: sete cenários aprovados, um blocked-verify pela
medição de até 1s inconclusiva. AC08 parcial; não é aprovação de desempenho.
Dois defeitos de UI encontrados e corrigidos, ambos verified. Naquela entrega, nenhum goal ou
codereview havia sido iniciado. A revisão posterior está em `docs/reviews/2026-11-12-codereview.md`.
Skills Rust/GPUI receberam referências verificadas; validação estrutural passou.

Vocabulário implementado: TaskId positivo e estável; TaskTitle validado;
TaskStatus Pending/Completed (pending/completed no banco); TaskFilter
All/Pending/Completed; draft é rascunho, snapshot é a última lista confirmada,
busy é escrita em andamento. A entidade GPUI continua distinta do domínio.


## Publicação e revisão externa — 2026-11-12

Por pedido explícito do usuário, repositório público criado em
https://github.com/j0ruge/rust_gpui. Origin publicado com develop como padrão e
feat/tarefas-gpui como branch de implementação. PR #1 contra develop, em rascunho:
https://github.com/j0ruge/rust_gpui/pull/1. Sem merge.

Revisão independente concluída pelo Claude Code 2.1.322 / Claude Opus 5.5
sobre o HEAD 83f47ca, contra develop. O Ultrareview remoto recusou o tamanho
do PR; o CLI local fez análise estática com Read/Glob/Grep, sem executar
comandos, testes, build ou janela, sem delegações e sem alterações no código.

Resultado publicado em
https://github.com/j0ruge/rust_gpui/pull/1#issuecomment-6835369884:
um achado médio e sete baixos. O principal envolve foco após desaparecer a
última tarefa visível; os cenários nativos indicados precisam de reprodução.
Achados não foram corrigidos nesta solicitação de revisão. O PR segue em
rascunho, sem merge; AC08 continua parcial. Relatório bruto local ignorado em
`.superpowers/review/2026-11-12/claude-review-result.json`.
Na publicação inicial, as alterações posteriores ao HEAD revisado apenas
registravam este resultado.

## Tratamento dos achados do Claude — 2026-11-12

Por pedido do usuário, os oito achados foram verificados e tratados, com
evidências em [correções do PR #1](docs/reviews/2026-11-12-claude-pr1.md).
O baseline e5ca7a8 tinha 33 testes; as correções passaram em 40 testes Rust e
15 jornadas nativas em Xvfb exclusivo. Controles de seleção e retry usam o
adaptador real; guards de busy/finish têm regressões de estado. Sabotagens em
cópias comprovaram os sensores sem alterar o fonte ou executável do produto.

Preferência explícita: QA automático headless para não disputar teclado/mouse
com o desktop em uso. `scripts/qa-headless.sh` implementa esse isolamento;
janela visível só quando solicitada. AC08 continua parcial. O merge foi
autorizado condicionalmente à resolução/verificação; gates finais, revisão do
delta e estado de integração devem ser consultados no relatório e no PR.

Na revisão corretiva de a3313e7, o Claude apontou um caso de retry inválido
dentro da decisão de saída, a identidade da cópia testada e duas observações
sobre evidências/sensores. O caso de saída foi reproduzido e corrigido no
adaptador GPUI; hash agora identifica a cópia executada. A suíte evoluiu para
41 testes Rust e 16 jornadas; histórico dos controles e drivers distinguido
do build final. O relatório mantém as observações e respectivas respostas.
