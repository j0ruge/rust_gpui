# Stack e evidências de execução

Aplicação implementada e aberta para uso local. QA funcional encerrado: sete
cenários aprovados e uma medição pendente. AC08 permanece parcial pelo limite
de 1 segundo não cronometrado de forma confiável. A revisão posterior e as
correções estão em [codereview](reviews/2026-11-12-codereview.md).

## Código e Git

- Base develop: `802b129198e3e843b0a05f53ff8b77b7797fe628`.
- Branch: `feat/tarefas-gpui`; sem remoto, push, PR ou merge.
- Bootstrap: `19de704`; domínio/config: `b920071`; SQLite: `40f2480`;
  estado: `98f1898`; interface e correções de QA: `ef5f8ef04e5d0015ec23f49aa5c3f1f81f40c7a2`.
- Revisão futura: comparar develop com HEAD da feature, incluindo docs do commit
  de entrega. Não usar main como base nem confundir este checkpoint com nota A.

## Abrir

```bash
cd /home/joruge/repos/rust_gpui
cargo run --locked --release
```

Depois do build, `target/release/rust_gpui` abre diretamente. Lançamento final
pelo comando acima, sem RUST_GPUI_DATA_DIR de QA: PID 1653298, janela X11
0x05400001, visível. Captura `docs/qa/evidence/2026-11-12-full/final-application-running.png`.
SHA256 do arquivo e `/proc/1653298/exe`:
`25de9f5f56850fb0135df9b45e3d970c86bdeda508d7c158f9e46cc2fb569869`.
PID é evidência da sessão, não identificador durável para scripts futuros.

## Versões e ambiente

Rust/rustc 1.97.0 (2d8144b78), Cargo 1.97.0 (c980f4866), edition 2024;
rustfmt 1.9.0 e clippy 0.1.97. Pin nomeado 1.97.0 instalado e comandos normais
validados. Cargo.lock versionado; dependências diretas exatas:

| Dependência | Versão/configuração |
|---|---|
| gpui | 0.2.2, features padrão |
| gpui-component / assets | 0.5.1, sem features opcionais de webview/linguagens |
| rusqlite | 0.42.0, bundled |
| thiserror | 2.0.21 |
| tempfile | 3.27.0, somente testes |

`cargo tree --locked -i gpui` confirmou uma única 0.2.2 compartilhada. A resolução
transitiva usa libc 0.2.189 por incompatibilidade real descrita abaixo.

Ubuntu 24.04.5 LTS x86_64, X11 DISPLAY=:1, dois monitores 1920×1080. Pacotes do
`scripts/install-system-deps.sh` confirmados installed por dpkg-query após
instalação no terminal do usuário: build-essential, pkg-config, clang, cmake,
fontconfig/FreeType, Wayland, X11/XCB, xkbcommon, OpenSSL e zstd de desenvolvimento.
Não foi executado o setup inteiro do Zed nem instalados pacotes adicionais por suposição.

No processo de QA 1644448, VK_LOADER_DEBUG=driver registrou em vkCreateDevice
`Using "Intel(R) UHD Graphics 770 (ADL-S GT1)" with driver: "libvulkan_intel.so"`.
Extrato em `docs/qa/evidence/2026-11-12-full/gpu-selection.txt`; log seeded-volume.log
no workspace ignorado. Não se inferiu dispositivo apenas de vulkaninfo.

## Dados e desenho implementado

Um binário, domínio puro, estado de apresentação e SQLite. UI reutiliza InputState;
render não toca banco nem recria formulário. Carga/escritas são jobs de background
com Arc<Mutex<TaskRepository>> e retorno por WeakEntity. Uma escrita de cada vez;
snapshot só muda após commit. Busy bloqueia formulário/novas escritas, mantendo
filtro/rolagem; fechamento aguarda resultado ou decisão após falha.

Diretório: RUST_GPUI_DATA_DIR absoluto; senão XDG_DATA_HOME absoluto + rust-gpui;
senão HOME absoluto + .local/share/rust-gpui. tasks.sqlite3 e app.lock, fora do repo
por padrão. Lock do SO mantido vivo; arquivo não removido para liberar instância.
Schema 1, quick_check, SQL da tabela normalizado, validação de registros,
journal_mode DELETE, synchronous FULL e timeout 250ms. Banco inválido preservado,
sem reset, backup automático ou migração destrutiva. README descreve recuperação.

## Gate técnico da entrega ef5f8ef

| Comando | Resultado | Tempo desta execução |
|---|---|---|
| `cargo fmt --check` | exit 0 | 0.09s |
| `cargo clippy --locked --all-targets -- -D warnings` | exit 0 | 0.32s |
| `cargo test --locked` | exit 0 | 1.47s |
| `cargo build --locked --release` | exit 0 | 0.27s |

Saída completa: `.superpowers/sdd/2026-11-12-tarefas-gpui/gate-final.log`; resumo gate-final.json. Comandos padrão do plano executados após as últimas mudanças de código.


Suíte: 6 domínio + 4 config + 9 persistência + 10 estado + 1 rollback interno =
30 testes. Diretórios temporários, SQLite real, sem alterar HOME/XDG globais.
Inclui rollback por trigger, reabertura, duplicatas por identidade, IDs não
reutilizados, bytes inválidos preservados, schema/versão, lock/drop, NotFound,
reentrada, erro preservando draft/snapshot, filtro em busy e decisão de fechar.
Widgets GPUI são verificados separadamente nas jornadas, não por DOM.

Aviso upstream de futura incompatibilidade: proc-macro-error2 2.0.1 reexporta
proc_macro privado (rust-lang/rust#127909). Não impede a toolchain fixada; Clippy
passou para os targets do projeto. Nenhum upgrade amplo foi feito para ocultar o aviso.

## QA e aceite

Relatório: [2026-11-12-full](qa/reports/2026-11-12-full.md). Quatro jornadas,
quatro personas, oito cenários; 5 Pass + 2 Fixed + 1 Blocked (needs human verify),
zero Pending. Duas falhas corrigidas e verificadas em ef5f8ef: largura do título
longo e feedback Salvo apagado por eco do input. Nenhum bug ficou aberto.

| AC | Resultado | Evidência |
|---|---|---|
| AC01 | aprovado | cargo run release, janela final visível e hash do processo |
| AC02–04 | aprovados | CRUD, filtros, cancelamento, exclusão e reabertura na janela |
| AC05 | aprovado | vazio/controle/201 recusados, 200/duplicatas/Unicode aceitos; testes e UI |
| AC06 | aprovado no escopo registrado | falha real de escrita/retry e cancelamento nativos; busy/rollback em testes |
| AC07 | aprovado | path/DB/versão/schema/segunda instância visíveis; preservação também testada |
| AC08 | parcial | teclado, foco, 640×480 e 1.000 itens exercitados; limite de 1s pendente |
| AC09 | aprovado | fmt/clippy/test/build no gate final |
| AC10 | aprovado | README, árvore QA e evidências do binário identificado |
| AC11 | aprovado no escopo registrado | referências Rust/GPUI atualizadas e validadas estruturalmente |

A captura contínua FFmpeg não observou o frame de fim, embora a captura posterior
da própria janela mostrasse o resultado correto. Instrumento inconclusivo; não
se afirmou latência >1s ou aprovação. Protocolo humano no relatório; reproduzir
com `bash scripts/qa-volume.sh`, que sempre cria diretório novo de 1.000 tarefas
artificiais. Script executado e janela conferida; fechamento exit 0.
Leitor de tela, contraste formal, escala de SO e outros sistemas não certificados.
Não há promessa de FPS/consumo de memória.

## Ajustes e decisões da execução

- Sem worktree adicional: pasta inicialmente sem Git e plano explícito. A branch
  isola o trabalho; não oferece um segundo checkout independente.
- O parser task-start esperava “Task N” e recusou “Tarefa N”. Briefs foram lidos
  no plano e ledger/testes registrados manualmente. Custo: controle manual do progresso.
- xattr 0.2.3, via GPUI → gpui_http_client → zed-async-tar, falhou com libc
  0.2.191 por ENOATTR ausente. Ajuste localizado do lock para 0.2.189 permitiu
  build. [Fonte upstream](https://github.com/rust-lang/libc/issues/1356).
  Custo: reavaliar esse pin ao atualizar a cadeia, sem tratá-lo como universal.
- Quit síncrono em on_window_closed causou RefCell already borrowed no X11.
  Backtrace mostrou reentrada em Platform::quit/flush_effects. App::defer ainda
  falhou; App::spawn em outro ciclo encerrou com exit 0. Replay nativo antes/depois.
- Instalações rustup concorrentes disputaram clippy.partial. Repetição sequencial
  concluiu. Fetch/build provisório usou stable com o mesmo rustc/commit 1.97.0;
  depois o pin nomeado e cargo run normal foram conferidos.
- Revisão independente/goal adiada por instrução expressa do usuário, mantendo
  branch local. Custo: a aplicação ainda aguarda essa revisão; não recebeu notas A.

## Aprendizados e retomada

Rust: /home/joruge/.codex/skills/rust/references/build-compatibility.md.
GPUI: /home/joruge/.codex/skills/gpui/references/state-and-x11.md. Entradas SKILL.md
ligadas às referências; quick_validate.py retornou Skill is valid! para ambas.
Exemplos vêm de falhas e replays reais desta execução; não houve avaliação
independente de comportamento de agentes para essas adições de referência.

Logs técnicos permanecem ignorados em `.superpowers/sdd/2026-11-12-tarefas-gpui/`;
PNGs e extratos em `docs/qa/evidence/2026-11-12-full/`. Fontes QA são versionadas;
DB, target, evidências pesadas, CSV gerado e credenciais não entram no Git.
O goal de codereview foi iniciado após a entrega, com base develop. As correções
recusam objetos extras no esquema e preservam literais SQL; a seleção de teclado
só age sobre tarefas visíveis. A suíte passou a 33 testes, com fmt/clippy/build
release aprovados e replay nativo dos filtros, diálogos e reabertura. Evidências
e achados originais no relatório de codereview; consulte ali o resultado da
revisão do último delta. Não repetir scaffolding ou apagar banco.
