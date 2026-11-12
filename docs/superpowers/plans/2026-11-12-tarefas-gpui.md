# Tarefas locais — plano de implementação Rust + GPUI

> **Para o executor:** usar `superpowers:executing-plans` e implementar as tarefas
> em sequência, nesta sessão, com o modelo Sol escolhido pelo usuário. Não exigir
> aulas/exercícios ou nova escolha de método. Delegação de implementação não foi
> solicitada. O codereview posterior segue seu próprio roteamento.

**Goal:** entregar a aplicação desktop de tarefas locais funcionando no Ubuntu,
com CRUD, filtros e autosave, para o usuário estudar depois.

**Architecture:** um binário, um contexto de domínio e SQLite local. GPUI Component
fornece controles; regras independem da UI; operações SQLite são serializadas no
background executor e a UI só confirma snapshots após commit.

**Tech Stack:** Rust 1.97.0/edition 2024; gpui 0.2.2; gpui-component e assets 0.5.1;
rusqlite 0.42.0 bundled; thiserror 2.0.21; tempfile 3.27.0 para testes.

**Spec:** [desenho completo](../specs/2026-11-12-tarefas-gpui-design.md).
**Diretório:** `/home/joruge/repos/rust_gpui`.
**Estado:** `in_progress`; incremento 1 comprovado; demais acompanhados em docs/stack.md.

## Restrições globais

- Aplicação primeiro; aprendizado por leitura do código depois. Não iniciar uma aula.
- Nesta conversa somente documentação. Executar após o `/clear` e pedido do usuário ao Sol.
- Ubuntu 24.04 x86_64/X11 é a plataforma inicial escolhida pelo planejador.
- Comentários, docs, commits e UI em pt-BR; identificadores/chaves/estados em inglês.
- Operações: criar, editar, concluir/reabrir, excluir com confirmação; filtros
  Todas/Pendentes/Concluídas; sem prazos, projetos, prioridades, contas ou servidor.
- Título: trim, 1–200 valores Unicode escalares, sem controles; duplicatas permitidas.
- Ordem crescente de TaskId; identidade estável. Tema escuro, janela 900×640,
  mínimo 640×480; lista virtualizada e controles acessíveis por teclado.
- `RUST_GPUI_DATA_DIR` absoluto tem precedência; senão XDG absoluto; senão HOME.
  Banco `tasks.sqlite3`; lock `app.lock`; nunca usar dados reais nos testes.
- Uma instância por diretório; SQLite schema 1, transações e commit antes de
  atualizar snapshot. Erros visíveis, rascunhos preservados, nada de reset do banco.
- Versões exatas na spec e Cargo.lock versionado; sem upgrade amplo ou refatoração adjacente.
- Primeiro resolver falha do incremento atual; não carregar defeito para o próximo.
- Sem push, PR, publicação, merge ou instalador nesta execução.

## Contexto verificado e limites

A pasta tem a referência original, AGENTS, CONTEXT, KAIZEN_LOG e estes documentos.
Ainda não há Git, Cargo.toml ou código. Rust/Cargo 1.97.0, rustfmt e clippy existem.
Sessão `DISPLAY=:1`, `XDG_SESSION_TYPE=x11`; Vulkan lista GPU Intel UHD 770 e llvmpipe.
`xdotool`, `wmctrl` e ImageMagick `import` existem, mas não foram exercitados no app.

Ausências verificadas por PATH/dpkg-query: clang, cmake, pkg-config e pacotes
de desenvolvimento fontconfig, FreeType, Wayland, X11/XCB, xkbcommon, OpenSSL.
`build-essential` e `cc` existem. Revalidar antes de instalar apenas o necessário.

Os arquivos publicados de `gpui-component` e assets 0.5.1 declaram `gpui 0.2.2`.
README da crate exige `gpui_component::init(cx)` e `Root::new(view, window, cx)`.
`InputState::new(window,cx)`, `.value()`, `.set_value(value,window,cx)` e eventos
`Change`/`PressEnter { secondary }` foram conferidos no fonte publicado.
docs.rs de component 0.5.1 não ficou acessível nesta inspeção; usar o fonte em
`~/.cargo/registry/src/` depois do fetch. Não usar exemplos de main/GPUI Kit 0.9.x.

Ainda não se verificaram build, desempenho, criação da janela nem funcionalidades.

## Estrutura prevista e contratos

| Arquivo | Responsabilidade |
|---|---|
| Cargo.toml, Cargo.lock, rust-toolchain.toml, .gitignore | Build reproduzível e exclusão de artefatos/dados |
| src/main.rs | Inicializar GPUI/assets/componentes, abrir Root e encerrar aplicação |
| src/lib.rs | Exportar módulos do projeto para binário e testes |
| src/domain.rs | TaskId, TaskTitle, TaskStatus, Task, TaskFilter e regras puras |
| src/error.rs | AppError e conversão de erros com mensagens seguras |
| src/config.rs | Resolver diretório por argumentos e ambiente |
| src/storage.rs | Lock, schema, validação, transações e mapeamento SQLite |
| src/state.rs | Estado de apresentação e transições testáveis, sem GPUI |
| src/ui/mod.rs | Tela, foco, input, filtros, diálogos, tarefas async e fechamento |
| src/ui/task_row.rs | Renderização de linha e ações ligadas a TaskId |
| tests/domain.rs, tests/config.rs, tests/persistence.rs, tests/state.rs | Contratos observáveis; detalhes privados de storage testados no próprio módulo |
| README.md, docs/stack.md, docs/qa/ | Iniciar/operar, versões efetivas e QA vivo |

Contratos de domínio em `src/domain.rs` (campos internos privados, getters públicos):

```rust
TaskId::new(value: i64) -> Result<TaskId, DomainError>
TaskId::get(self) -> i64
TaskTitle::parse(raw: &str) -> Result<TaskTitle, DomainError>
TaskTitle::as_str(&self) -> &str
Task::restore(id: TaskId, title: TaskTitle, status: TaskStatus) -> Task
Task::rename(&mut self, title: TaskTitle)
Task::set_status(&mut self, status: TaskStatus)
Task::id(&self) -> TaskId
Task::title(&self) -> &TaskTitle
Task::status(&self) -> TaskStatus
TaskFilter::matches(self, task: &Task) -> bool
```

`DomainError`: título vazio, longo, com controle; ID não positivo. `TaskStatus`
e `TaskFilter` seguem a spec. Tipos de valor/enums implementam comparação e Clone;
TaskId/TaskStatus/TaskFilter podem ser Copy. Nada de referências a GPUI ou SQLite.

`TaskCommand`, em storage: `Create(String)`, `Rename(TaskId, String)`,
`SetStatus(TaskId, TaskStatus)`, `Delete(TaskId)`; Clone para retry explícito.

```rust
resolve_data_dir(override_dir: Option<&Path>, xdg: Option<&Path>,
                 home: Option<&Path>) -> Result<PathBuf, AppError>
TaskRepository::open(data_dir: &Path) -> Result<TaskRepository, AppError>
TaskRepository::list(&self) -> Result<Vec<Task>, AppError>
TaskRepository::apply(&mut self, command: TaskCommand) -> Result<Vec<Task>, AppError>
```

`TaskRepository` mantém Connection e File do lock; `apply` valida strings pelo
domínio. `AppError` distingue validação, diretório inválido, I/O, banco inválido,
versão incompatível, `AlreadyRunning`, `NotFound` e falha SQLite. Guardar a causa
para diagnóstico sem colocar títulos nos logs.

`TaskScreenState` em state contém snapshot confirmado, filtro, draft, modo
criação/edição, confirmação de exclusão, `busy`, comando pendente, erro visível
e intenção de fechar. `draft` é um String; o adapter GPUI sincroniza InputState.

```rust
TaskScreenState::new(tasks: Vec<Task>) -> TaskScreenState
TaskScreenState::begin(&mut self, command: TaskCommand) -> Option<TaskCommand>
TaskScreenState::finish(&mut self, result: Result<Vec<Task>, AppError>)
TaskScreenState::visible_tasks(&self) -> Vec<&Task>
TaskScreenState::pending_count(&self) -> usize
```

`begin` recusa segunda escrita se busy. `finish(Err)` libera busy, preserva
draft/modo/comando/snapshot e expõe erro; `finish(Ok)` aplica snapshot e limpa
o formulário ou confirmação somente relativos ao comando concluído. Filtro
permanece como escolhido durante a operação. UI mantém um `Arc<Mutex<TaskRepository>>`;
lock e SQLite são acessados apenas em jobs de background. Não segurar mutex
durante `await`, renderização ou atualização de Entity.

## Foco de revisão

| Risco | Proteção e teste proprietário |
|---|---|
| Escrita falha mas formulário some ou lista mente | state: erro mantém rascunho/snapshot; persistence: rollback real |
| Duplo clique/fechamento com operação pendente | state: busy recusa reentrada; UI nativa: fechamento espera resultado |
| Unicode, duplicatas ou mudança de filtro editam item errado | domain + persistence por ID; QA edita duplicata e cancela |
| Banco corrompido/mais novo vira lista vazia | persistence: falha explícita e arquivo preservado; QA tela de erro |
| Segunda instância ou lock órfão impede/estraga uso | persistence: lock vivo bloqueia, drop/crash permite reabrir; QA dois processos |

## Tarefa 1 — Primeira janela real e ambiente reproduzível

**Arquivos:** manifesto/lock/toolchain/gitignore, src/main.rs, src/lib.rs,
src/ui/mod.rs, README e docs/stack.md.
**Interfaces:** produz janela GPUI com Root/assets; sem depender do CRUD.

- [x] Releia AGENTS, CONTEXT e spec. Revalide se ainda não há Git/projeto; preserve
  qualquer alteração posterior. Não sobrescrever arquivos existentes com init.
- [x] Se continuar sem Git, `git init -b develop`. Criar .gitignore para `/target/`,
  `.env`, `.env.*` (permitir `.env.example` se surgir sem valores), `*.sqlite3*`,
  `app.lock`, `docs/qa/evidence/`, `docs/qa/state.csv`. Não criar `.env` desnecessário.
- [x] Criar commit de baseline com docs e gitignore, mensagem em português;
  gravar o SHA em docs/stack.md no commit seguinte. Criar `feat/tarefas-gpui`
  a partir desse develop. Se já existir Git, respeitar estado e base reais.
- [x] Verificar pacotes com `dpkg-query` e PATH. Na execução autorizada, instalar
  os ausentes desta base inicial, ajustando por evidência do build:

```bash
sudo apt-get update
sudo apt-get install --no-install-recommends \
  build-essential pkg-config clang cmake libfontconfig-dev libfreetype-dev \
  libwayland-dev libx11-xcb-dev libxkbcommon-dev libxkbcommon-x11-dev \
  libxcb1-dev libssl-dev libzstd-dev
```

  Não executar o script inteiro de instalação do Zed. ALSA, libgit2 ou outro
  pacote só entram se o grafo/build desta aplicação exigir. Registrar inclusão
  e motivo. Se sudo não estiver disponível ou exigir credencial humana, informar
  o comando pendente; não contornar nem marcar ambiente como pronto.
- [x] Iniciar pacote no diretório atual: `cargo init --bin --name rust_gpui --vcs none .`.
  Fixar Rust 1.97.0 em rust-toolchain.toml, edition 2024, dependências exatas da
  spec e componentes rustfmt/clippy. Não adicionar todos os features.
- [x] Gerar lock com `cargo generate-lockfile`; `cargo fetch --locked`; inspecionar
  `cargo tree -i gpui` e confirmar que UI/componentes usam uma versão compatível.
- [x] Implementar bootstrap com `Application::new().with_assets(...)`, init dos
  componentes e Root. Abrir janela da spec mostrando título e texto “Preparando
  tarefas…”. Tratar erro de abertura; sem unwrap de erro operacional.
- [x] `cargo build --locked --release` e `cargo run --locked --release`: confirmar
  janela visível e fechamento sem processo órfão. Capturar somente a janela.
  Essa é a primeira entrega verificável, não aceite de CRUD.
- [x] Registrar comandos, versão, dependências reais e evidência; commit pequeno.

## Tarefa 2 — Regras das tarefas e caminho dos dados

**Arquivos:** src/domain.rs, src/config.rs, src/error.rs, src/lib.rs;
tests/domain.rs, tests/config.rs.
**Interfaces:** produz os tipos/assinaturas de domínio e resolve_data_dir acima.

- [x] Escrever testes de título antes da implementação. Asserções centrais:

```rust
assert_eq!(TaskTitle::parse("  Comprar café ☕  ").unwrap().as_str(), "Comprar café ☕");
assert!(TaskTitle::parse("   ").is_err());
assert!(TaskTitle::parse("uma\noutra").is_err());
assert!(TaskTitle::parse(&"é".repeat(200)).is_ok());
assert!(TaskTitle::parse(&"é".repeat(201)).is_err());
```

  Acrescentar tab/controle, espaços internos, ID zero/negativo, rename mantém ID,
  set_status idempotente e filtros exatos. Testes em diretório sem dados do usuário.
- [x] Testar caminhos por argumentos: override absoluto prevalece; override
  relativo falha; XDG relativo/vazio é ignorado; fallback HOME absoluto/não vazio;
  HOME inválido sem alternativa e ausência total
  falha. Não mudar HOME/XDG globais da suíte.
- [x] Rodar `cargo test --locked --test domain --test config`; observar falha pela
  regra ausente, distinguindo-a de erros de ambiente/dependência. Implementar o
  mínimo e repetir até passar. Nenhum teste deve apenas espelhar getters.
- [x] Atualizar termos no CONTEXT, sem expandir escopo; commit com motivo.

## Tarefa 3 — SQLite, lock e operações duráveis

**Arquivos:** src/storage.rs, src/error.rs, src/lib.rs; tests/persistence.rs;
testes privados de falha dentro de src/storage.rs.
**Interfaces:** consome domínio/config; produz TaskCommand e TaskRepository.

- [x] Escrever teste com `tempfile::tempdir`: open → Create → Rename → SetStatus
  → drop → reopen → conferir ID/título/estado/ordem → Delete → reopen → ausente.
  Criar duas tarefas de mesmo título e comprovar alteração só no ID escolhido.
- [x] Testar schema incompatível, bytes que não são SQLite, registro inválido e
  banco com tabelas estranhas/schema zero. Recusar sem apagar/resetar; comparar
  bytes do fixture corrompido antes/depois. Novo banco vazio é inicializado uma vez.
- [x] Testar lock: segundo open no mesmo diretório falha; outro diretório passa;
  após drop pode reabrir mesmo com app.lock ainda existente. Não testar por PID
  escrito em arquivo nem remover arquivo para liberar.
- [x] Testar transação/rollback: após duas tarefas confirmadas, adicionar trigger
  SQLite temporário de teste que aborte UPDATE; tentativa de rename deve falhar
  e a leitura/reabertura preservar título/estado anteriores. Testar `NotFound`
  sem criar linha e erro de diretório que é arquivo. Testes privados podem acessar
  a Connection, sem adicionar API de debug ao produto.
- [x] Rodar `cargo test --locked --test persistence` e testes internos afetados;
  observar falhas, implementar schema/lock/transactions conforme spec e repetir.
  Manter File do lock vivo; usar SQL parametrizado e mapear registro via domínio.
- [x] Atualizar docs/stack com caminho e política de recuperação; commit.

## Tarefa 4 — Estado de UI resistente a falhas

**Arquivos:** src/state.rs, src/lib.rs; tests/state.rs.
**Interfaces:** consome Task/TaskCommand/AppError; produz TaskScreenState.

- [ ] Antes da implementação, testar begin/finish: busy começa ao aceitar ação;
  segunda begin retorna None; erro mantém snapshot/draft/edição/confirmação;
  sucesso troca snapshot, limpa somente edição da ação e libera busy.
- [ ] Testar filtro alterado durante busy permanece; contagem considera todas
  as pendentes; concluir no filtro Pendentes remove linha somente após sucesso.
  Estado de erro não se converte em lista vazia.
- [ ] Testar close_requested: sucesso permite concluir fechamento; falha exige
  decisão retry/sair sem salvar; cancelamento de edição não gera TaskCommand.
- [ ] `cargo test --locked --test state`: observar falha, implementar transições
  puras e repetir até passar. Esses testes não alegam validar widgets GPUI.
- [ ] Commit do incremento.

## Tarefa 5 — Tela completa integrada ao armazenamento

**Arquivos:** src/ui/mod.rs, src/ui/task_row.rs, src/main.rs; README.
**Interfaces:** consome TaskRepository e TaskScreenState; sem novas regras de domínio.

- [ ] Carregar repositório/lista fora da thread gráfica e mostrar loading até
  obter resultado. Em erro, tela com resumo, caminho, Tentar novamente e Sair.
  Para AlreadyRunning, só mensagem e Sair.
- [ ] Manter `Entity<InputState>` por formulário, criada fora de render; usar
  Input/Checkbox/Button de component 0.5.1 e assinaturas do fonte instalado.
  Guardar Subscriptions e tasks enquanto necessárias. Render não grava nem cria
  entidades novas a cada frame.
- [ ] Renderizar layout, filtros, contadores e estados vazios da spec. Usar lista
  virtualizada (`gpui::uniform_list`), linha fixa de 64px e ID por TaskId. Texto
  longo truncado na linha fica completo no input de edição.
- [ ] Ligar input/Enter/Adicionar e edição/cancelamento; confirmar exclusão com
  diálogo seguro. Atualizar rascunho em Change, persistir somente confirmação.
- [ ] Despachar um job de escrita por vez, usando begin antes de spawn; lock/repo
  dentro do job, resultado no contexto GPUI via handle fraco. Retorno nunca
  atualiza entidade já destruída. Refletir finish e mensagens da spec.
  Desabilitar inputs e ações dos diálogos durante busy, mantendo filtro/rolagem;
  testar que uma conclusão não apaga texto novo porque edição não foi admitida.
- [ ] Implementar fechamento adiado em busy; sucesso encerra, falha mostra retry
  ou sair sem salvar. Ctrl+Q e fechamento pelo gerenciador de janelas seguem a
  mesma política. Não depender apenas do callback final on_app_quit.
- [ ] Implementar foco visível, Tab/Shift+Tab, Ctrl+N, Ctrl+1/2/3 e Enter/Esc.
  Na lista focada, Up/Down selecionam TaskId e rolam até o item para que tarefas
  virtualizadas possam ser alcançadas por teclado. Não interceptar setas do input.
- [ ] Executar release e percorrer criar/editar/concluir/reabrir/excluir/filtros;
  fechar e reabrir o mesmo diretório. Comparar UI com persistência observada.
- [ ] Confirmar 640×480 e 900×640 sem ações inacessíveis. Atualizar README com
  execução, diretório, atalhos, rascunho vs ação confirmada e recuperação. Commit.

## Tarefa 6 — QA, documentação e entrega para uso

**Arquivos:** docs/qa seguindo skills locais, README, docs/stack, CONTEXT,
KAIZEN_LOG e skills pessoais rust/gpui para lições verificadas.
**Interfaces:** exercita o binário real e os AC01–AC11 da spec.

- [ ] Aplicar `qa-report` de `/home/joruge/.claude/skills/qa-report/SKILL.md` em
  `docs/qa/`; bootstrap dos templates e estado exatos da skill. Persona: usuário
  que administra tarefas locais em pt-BR. Jornadas: CRUD e filtros; reabertura;
  falhas de armazenamento/inicialização; uso por teclado e volume. Toda jornada
  tem fim observável, abandono e charter. Cenários começam `untested`.
- [ ] Aplicar `qa-execution` de `/home/joruge/.claude/skills/qa-execution/SKILL.md`.
  Usar janela nativa, nunca DOM/Playwright. Criar report antes da primeira sessão
  e atualizar após cada uma. Evidências em diretório ignorado, referenciadas pelo report.
- [ ] Criar diretório temporário com mktemp para QA e iniciar o binário com
  `RUST_GPUI_DATA_DIR` apontando para ele; nunca usar banco real nas sondas.
  Registrar SHA256 do binário e conferir contra `/proc/<pid>/exe` do processo
  observado, além de versões, perfil e DISPLAY. Capturar a janela por seu ID.
- [ ] Percorrer AC02–08. Em QA de falhas, preparar fixtures isoladas antes de
  iniciar e verificar pelo caminho público da UI. Testes internos de trigger
  comprovam rollback; não são evidência de mensagem exibida. Provocar também
  falha real de gravação no diretório descartável e conferir rascunho na janela.
- [ ] Para 1.000 tarefas, semear SQLite de QA antes da abertura com IDs/estados
  válidos. Cronometrar filtro/ação no binário release por timestamps/vídeo com
  início e fim observáveis (até 1s); registrar método. Se não houver instrumento
  confiável, registrar pendência de medição, não inventar FPS ou aprovação.
- [ ] Corrigir defeitos dentro do escopo, retestar jornada afetada e teste que
  protege a regra. Bloqueio de ambiente/sessão fica explícito com passo humano
  restante; zero linhas Pending ao fechar relatório não implica todos Pass.
- [ ] Executar gate técnico final uma vez após as últimas mudanças:

```bash
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --locked --release
```

- [ ] Atualizar CONTEXT/KAIZEN_LOG com observado vs esperado e caminhos de
  evidência. Registrar aprendizados reutilizáveis em `$rust`/`$gpui`, cada qual
  em seu escopo; não transformar hipótese ou preferência deste MVP em regra geral.
  Validar skills editadas com o quick_validate da skill-creator.
- [ ] Commit final pequeno; garantir que dados, target e evidências pesadas não
  entraram. Gravar baseline, SHA revisável e comando de início em docs/stack.
  Entregar comando para abrir a aplicação, evidência visual e limitações reais.
  Explicação didática pode vir depois, conforme solicitado pelo usuário.

## PDCA, riscos e reversão

Plan: spec, contratos e ACs deste plano. Do: uma tarefa por vez. Check: testes
de cada tarefa e jornadas nativas; Act: docs/skills/log com evidência.

| Incremento | Check principal | Reversão |
|---|---|---|
| 1 | Janela real em release | Reverter commit específico, mantendo documento original; não remover pacotes de sistema automaticamente |
| 2 | Regras/caminhos testados | Reverter commit do domínio/config; sem dados reais envolvidos |
| 3 | Persistência, rollback e lock | Reverter código; preservar DB existente, nenhuma migration destrutiva |
| 4 | Transições e falhas | Reverter estado sem apagar dados |
| 5 | Jornadas na janela | Reverter integração; preservar banco e última versão funcional |
| 6 | ACs e evidências | Corrigir achados, registrar pendências; não maquiar resultados |

Não criar worktree adicional para pasta ainda sem Git. Na retomada, se outro
trabalho compartilhar branch, usar isolamento conforme estado real, sem descartar
mudanças. Não publicar nada como efeito colateral de finalizar uma branch.

## Revisão posterior após novo clear

Esta fase foi reservada pelo usuário para outra sessão; não iniciar goal agora.
Na sessão futura, criar goal explícito de revisar/corrigir até todas as categorias
serem A, mantendo escopo e evidência. Usar `codereview:codereview` do ambiente;
carregar seus arquivos da mesma versão. O contrato enviado originalmente era
2.27.16; existe instalação 2.27.37. Conferir roteamento antes de executar scripts.

Configuração de review: base develop/checkpoint registrado, `baseDir=src/`,
`fileExtensions=rs`, `frameworkPatterns=generic`, testes `tests/**/*.rs` e
blocos `#[cfg(test)]`, CONFIG Cargo.toml/rust-toolchain.toml/.cargo/*.toml.
Incluir docs escritos nesta missão; excluir target/DB/artefatos. Adaptar a
classificação inline Rust explicitamente, sem defaults TS/React.

O reviewer é somente leitura. Correções ocorrem fora da skill sob o goal do
usuário. Preservar resultados anteriores; após cada correção, revisar o delta
desde o SHA anterior e retestar áreas afetadas. Uma rodada limpa de delta não
apaga achado ainda aberto do relatório original. Encerrar só com zero achados
pendentes e todas as categorias A sustentadas por análise/testes aplicáveis.
