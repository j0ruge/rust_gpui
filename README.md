# Tarefas

[Repositório público](https://github.com/j0ruge/rust_gpui) ·
[PR de revisão](https://github.com/j0ruge/rust_gpui/pull/1) · base `develop`.

Aplicação desktop de tarefas locais em Rust + GPUI para Ubuntu 24.04 x86_64/X11.
Crie, edite, conclua/reabra e exclua tarefas. Filtros Todas/Pendentes/Concluídas.
Sem conta, servidor, prazos, prioridades ou projetos.

## Compilar e abrir

Na sessão gráfica do computador:

```bash
cd /home/joruge/repos/rust_gpui
cargo run --locked --release
```

O primeiro build baixa as dependências e pode demorar. Rust 1.97.0 com rustfmt e
clippy é selecionado por `rust-toolchain.toml`. Para preparar os pacotes nativos:

```bash
sudo bash /home/joruge/repos/rust_gpui/scripts/install-system-deps.sh
```

Depois de compilar, abrir diretamente dispensa o Cargo:

```bash
/home/joruge/repos/rust_gpui/target/release/rust_gpui
```

## Uso e teclado

- Digite o título e pressione Enter ou Adicionar. Na edição, use Enter/Salvar;
  Esc/Cancelar abandona a alteração. Títulos duplicados são independentes.
- Marque o checkbox para concluir/reabrir. Excluir abre confirmação; o foco
  inicial fica em Cancelar. Tab/Shift+Tab percorrem controles; Space/Enter ativam.
- Ctrl+N foca nova tarefa; Ctrl+1/2/3 selecionam filtros; Ctrl+Q fecha.
- Na lista focada, ↑/↓ e Home/End selecionam e rolam. Enter edita, Space alterna
  conclusão e Delete pede exclusão. Setas dentro do campo continuam editando texto.

Títulos aceitam acentos e emojis, de 1 a 200 escalares Unicode, sem caracteres
de controle nem separadores de linha/parágrafo U+2028/U+2029. Espaços nas
extremidades são removidos; espaços internos permanecem.
A ordem é a de criação e não muda ao editar/concluir. Concluídas têm checkbox,
rótulo e texto riscado. O contador de pendentes considera todos os filtros.

**Autosave salva cada ação confirmada, não cada tecla.** Rascunho não confirmado
pode ser descartado ao fechar. Durante gravação, os campos e novas escritas ficam
bloqueados, enquanto filtros e rolagem continuam disponíveis. Fechamento durante
escrita aguarda o resultado; falha permite repetir ou sair sem salvar essa ação.

## Onde os dados ficam

Precedência do diretório:

1. `RUST_GPUI_DATA_DIR`, obrigatório absoluto quando fornecido.
2. `$XDG_DATA_HOME/rust-gpui`, se XDG for absoluto e não vazio.
3. `$HOME/.local/share/rust-gpui`, com HOME absoluto e não vazio.

O diretório contém `tasks.sqlite3` e `app.lock`. Uma instância por diretório;
o lock do sistema operacional é liberado no fechamento ou crash. **Não apague
app.lock para tentar liberar outra instância.** Diretórios distintos funcionam
independentemente. Dados ficam fora do repositório por padrão.

Para experimentar sem tocar seus dados habituais:

```bash
task_data_dir=$(mktemp -d /tmp/rust-gpui-experimento.XXXXXX)
RUST_GPUI_DATA_DIR="$task_data_dir" cargo run --locked --release
```

Reutilize o mesmo caminho para conferir a persistência ao reabrir. A variável
é local ao comando; nenhum HOME/XDG global precisa ser alterado.

## Falhas e recuperação

Erros aparecem na janela com caminho e ação de repetir quando aplicável. Um erro
de escrita mantém o rascunho e a última lista confirmada. Corrija permissões ou
disponibilidade do diretório antes de tentar novamente. Se outra instância já
estiver usando os dados, volte à janela existente ou feche-a normalmente.
Se o diretório não puder ser resolvido, corrija as variáveis do ambiente e
reabra o aplicativo. Título inválido pede correção no formulário e não inicia
uma gravação pendente.

Banco inválido ou de versão incompatível é recusado e preservado. Não há reset,
migração destrutiva, exportação nem backup automático. Antes de uma recuperação
externa, feche o aplicativo e faça uma cópia do arquivo original; não substitua
seu banco por um arquivo vazio. Logs técnicos não incluem títulos das tarefas.

## Verificação e estudo posterior

```bash
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --locked --release
bash scripts/qa-headless.sh
```

Cada teste usa armazenamento temporário isolado. Verificações e limitações reais
ficam em [docs/stack.md](docs/stack.md) e nas [jornadas de QA](docs/qa/README.md).
Formatação/testes não substituem uso da janela GPUI. Achados e evidências das
correções posteriores estão no [codereview](docs/reviews/2026-11-12-codereview.md).
As [correções do parecer do Claude](docs/reviews/2026-11-12-claude-pr1.md)
acrescentam regressões de foco, seleção, validação e recuperação.

O QA automático usa uma janela nativa em Xvfb exclusivo, com dados artificiais
e Vulkan por software; não envia teclado/mouse ao desktop em uso. Requer
Xvfb/xauth, Openbox, xdotool, wmctrl, ImageMagick, Tesseract, Python 3/Pillow
e lavapipe. Procedimento, seleção de casos e limites em [QA](docs/qa/README.md).

As regras estão em `src/domain.rs`; persistência em `src/storage.rs`; transições
em `src/state.rs`; widgets e jobs assíncronos em `src/ui/`. O
[CONTEXT.md](CONTEXT.md) mantém o glossário e as decisões do produto.


Para reproduzir manualmente a jornada visível de volume em dados artificiais, execute
`bash scripts/qa-volume.sh` depois do build. Ele sempre cria outro diretório;
a temporização de até 1s está pendente de verificação conforme o relatório QA.
