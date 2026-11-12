# Tarefas locais — desenho para Rust e GPUI

Data: 2026-11-12. Estado: `ready_for_execution_handoff`.
Execução reservada ao Sol após o `/clear` do usuário. Não houve build nesta fase.

## Intenção e escopo

Entregar uma aplicação desktop que o usuário possa abrir e usar, depois estudar
seu código para aprender Rust. Ele declarou experiência praticamente zero e
escolheu explicitamente ver o aplicativo funcionando primeiro. Não intercalar
execução com aulas, exercícios ou novas rodadas de escolha de implementação.

Confirmado pelo usuário: criar, editar, concluir/reabrir e excluir tarefas;
filtros Todas/Pendentes/Concluídas; salvamento automático local; sem conta ou
servidor. Prazos, prioridades e projetos ficam fora do MVP.

Decisões abaixo são escolhas técnicas do planejador para fechar o escopo, não
respostas atribuídas ao usuário. Plataforma inicial: Ubuntu 24.04 x86_64, sessão
X11 da máquina atual. Windows/macOS, instalador, integração com sistemas JRC,
sincronização, telemetria e publicação remota ficam fora desta execução.

## Abordagens consideradas

| Alternativa | Benefício | Custo e decisão |
|---|---|---|
| GPUI puro + JSON | Poucas abstrações de UI e arquivo legível | Implementar entrada textual e gravação segura aumenta o trabalho; adiada |
| GPUI + componentes + SQLite | Controles existentes, transações e persistência por tarefa | Mais dependências, porém menor implementação de infraestrutura; escolhida |
| GPUI + componentes + JSON | UI pronta e persistência simples no caminho feliz | Exige escrita atômica e política própria para snapshots; não escolhida |

Usar um executável, um contexto de domínio (tarefas pessoais), módulos pequenos
para domínio, armazenamento e UI. Sem servidor, event bus, CQRS, containers ou
interfaces genéricas criadas apenas para testes. DDD é aplicado aos nomes e
invariantes; `Entity<T>` do GPUI não determina o agregado do negócio.

## Stack fixada para o primeiro build

| Item | Versão/configuração |
|---|---|
| Rust | `1.97.0`, edition `2024`, componentes rustfmt e clippy |
| Pacote/binário | `rust_gpui`, versão `0.1.0` |
| `gpui` | `=0.2.2`, features padrão |
| `gpui-component` | `=0.5.1`, sem features opcionais de webview ou linguagens |
| `gpui-component-assets` | `=0.5.1` |
| `rusqlite` | `=0.42.0`, feature `bundled` |
| `thiserror` | `=2.0.21` |
| `tempfile` | `=3.27.0`, somente dev-dependency |

Os manifests dos arquivos publicados de component/assets 0.5.1 declaram GPUI
0.2.2. Isso confirma o alinhamento declarado, não a compilação completa do grafo.
Fixar `Cargo.lock` no bootstrap. Não copiar a documentação atual de GPUI Kit
0.9.x para essa versão: ela usa outra família de dependências. Se um erro real
exigir ajuste, registrar causa e menor alteração em `docs/stack.md`, manter o
mesmo escopo e repetir o check afetado. Não trocar de framework silenciosamente.

## Regras e linguagem

- `TaskId`: inteiro positivo atribuído pelo SQLite; identidade estável, nunca
  índice da lista nem título. IDs excluídos não são reutilizados.
- `TaskTitle`: `trim()` nas extremidades; entre 1 e 200 valores Unicode escalares
  (`chars().count()`); rejeitar caracteres de controle, inclusive tab e quebra
  de linha; preservar espaços internos, acentos e emojis. Duplicatas permitidas.
- `TaskStatus`: `Pending`/`Completed`; no banco `pending`/`completed`; na UI
  “Pendente”/“Concluída”. Criar começa pendente. Concluir e reabrir são idempotentes.
- `TaskFilter`: `All`/`Pending`/`Completed`; alterar filtro não grava dados.
- Ordenação: ID crescente (mais antigas primeiro), mantida ao editar ou concluir.
- Editar altera apenas o título; mantém identidade, estado e posição. Cancelar
  mantém o valor persistido. Nunca usar título como chave de atualização.
- Excluir exige confirmação “Excluir tarefa?” com título e botões “Cancelar”
  e “Excluir”; foco inicial em Cancelar. Após confirmar, remoção é definitiva.
- Operar ID inexistente retorna `NotFound`; não inventar sucesso ou nova tarefa.
- Não limitar quantidade de tarefas por uma regra artificial; exercitar 1.000
  tarefas na verificação de interação e usar lista virtualizada com altura fixa.

## Interface e comportamento

Janela “Tarefas”, 900×640, mínimo 640×480, tema escuro, fonte do sistema. Uma
tela com título, campo “Nova tarefa”, botão “Adicionar”, três filtros, contagem
de pendentes e lista rolável. Cada linha contém checkbox, título e ações textuais
“Editar” e “Excluir”; concluídas possuem indicação visual além de cor.

Sem tarefas: “Nenhuma tarefa ainda. Adicione a primeira acima.” Filtro vazio:
“Nenhuma tarefa neste filtro.” Títulos longos podem ser truncados na linha;
edição deve expor o conteúdo completo. IDs de elementos derivados de TaskId.

Entrada de texto usa `InputState`/`Input`; criar/editar confirma em Enter ou
botão, Esc cancela a edição. Autosave significa persistir a ação confirmada,
não cada tecla. Rascunho não confirmado pode ser descartado ao fechar.

Tab/Shift+Tab percorrem controles com foco visível; Space/Enter ativam o
controle focado quando não for edição de texto. Ctrl+N foca nova tarefa;
Ctrl+1/2/3 selecionam filtros. Dialogs mantêm foco dentro deles e o restauram
ao fechar. Navegação deve alcançar tarefas fora da área visível rolando a lista.
Não afirmar suporte completo a leitor de tela sem teste; registrar cobertura real.

Cada ação de escrita entra em estado `busy` imediatamente e desabilita novas
escritas, campos de texto e ações dos diálogos até terminar; filtro/rolagem
continuam disponíveis. Isso impede que o retorno apague texto digitado durante
o salvamento. Mostrar “Salvando…”.
Executar I/O no background executor do GPUI, nunca dentro de `render` nem bloqueando
a thread gráfica. Aplicar novo snapshot somente após commit bem-sucedido.

Sucesso limpa o campo/fecha a edição e mostra “Salvo”. Erro mantém rascunho,
diálogo e último snapshot válido, mostra mensagem em português e permite tentar
a ação novamente. Não limpar input em erro nem ocultá-lo apenas no stderr.
Durante escrita, adiar fechamento até receber o resultado. Se falhar, oferecer
“Tentar novamente” ou “Sair sem salvar”; se funcionar, completar o fechamento.

## Persistência e falhas

Diretório: `RUST_GPUI_DATA_DIR` absoluto quando fornecido; senão
`$XDG_DATA_HOME/rust-gpui` quando XDG for absoluto/não vazio; senão
`$HOME/.local/share/rust-gpui` com HOME absoluto/não vazio. Sem HOME válido e sem
override/XDG: erro explícito.
Variáveis lidas via `var_os`; resolução testável por argumentos sem mudar o
ambiente global dos testes. Dados nunca ficam no repositório por padrão.

Arquivos: `tasks.sqlite3` e `app.lock`. Abrir lock sem truncar, adquirir
`std::fs::File::try_lock()` e manter o handle enquanto o repositório existir.
Não apagar o arquivo de lock ao sair: exclusividade vem do lock do SO, não da
existência do arquivo. Segunda instância no mesmo diretório abre mensagem
“O aplicativo já está aberto para estes dados.” e ação de sair. Diretórios
diferentes são independentes. Crash libera lock sem exigir limpeza manual.

SQLite: `journal_mode=DELETE`, `synchronous=FULL`, `busy_timeout=250ms`;
uma conexão protegida por mutex, utilizada fora da thread de UI. Schema 1:

```sql
CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('pending', 'completed'))
);
PRAGMA user_version = 1;
```

Inicializar schema em transação somente em banco novo/vazio sem tabelas de
usuário e `user_version=0`. Banco existente: validar versão, tabela/colunas
esperadas, `PRAGMA quick_check` e cada registro pelo domínio. Versão diferente,
arquivo corrompido ou registro inválido bloqueia carga/escrita com mensagem e
caminho dos dados; nunca apagar, zerar ou substituir automaticamente.

Cada comando abre uma transação, lê a tarefa quando aplicável, aplica regras
do domínio, grava, lê snapshot ordenado dentro da transação e faz commit antes
de retornar esse snapshot. Erro anterior ao commit faz rollback; erro do commit
retorna falha sem confirmar sucesso na UI. Nada de debounce, fila de snapshots
ou tentativas automáticas ilimitadas. Falha de leitura inicial mostra tela de
erro com “Tentar novamente” e “Sair”, preservando o banco.

Logs de diagnóstico não registram títulos ou conteúdo das tarefas. Exibir na
UI o resumo seguro e o caminho, com detalhe técnico no stderr quando necessário.
Sem backup automático/exportação no MVP; preservar arquivo problemático para
recuperação externa. Não confundir isso com garantia contra qualquer falha física.

## Critérios de aceite

| ID | Evidência exigida |
|---|---|
| AC01 | `cargo run --locked --release` abre a janela real nesta sessão Ubuntu/X11 |
| AC02 | Criar “Comprar café ☕”, editar, concluir/reabrir e excluir pelo fluxo de confirmação |
| AC03 | Três filtros corretos; contagem global de pendentes; estados vazios distintos |
| AC04 | Fechar/reabrir preserva conteúdo, estado e ordem; excluídas continuam ausentes |
| AC05 | Vazio/controle/201 caracteres recusados; 200 caracteres, duplicatas, acentos e emojis aceitos |
| AC06 | Cancelar edição/exclusão não muda dados; falha de escrita não perde rascunho nem simula sucesso |
| AC07 | Erros de path/DB/versão e segunda instância são visíveis; dados existentes preservados |
| AC08 | Teclado, foco, redimensionamento e rolagem funcionam; com 1.000 tarefas filtro e ação simples retornam em até 1s nesta máquina, medidos na jornada release |
| AC09 | fmt, clippy, testes e build passam; testes de domínio, transação/rollback e estado de UI exercitam falhas |
| AC10 | README permite iniciar; `docs/qa` tem jornadas, resultados e evidência do binário examinado |
| AC11 | Skill Rust/GPUI recebem aprendizados verificados em seu escopo; log mapeia evidências |

O limite de 1s é critério do experimento, não promessa de 120 FPS. Nenhum AC
está aprovado por execução neste documento. Um bloqueio gráfico impede declarar
AC01/02/08 aprovados mesmo que testes automatizados estejam verdes.

## Checklist DDD/Kaizen do desenho

- [x] Problema concreto e contexto único: administrar tarefas locais.
- [x] Vocabulário, identidade e invariantes definidos; GPUI separado do domínio.
- [x] Cenários de negócio e de falha ligados aos ACs.
- [x] Complexidade limitada ao fluxo requerido, sem serviços distribuídos.
- [x] Plano deve entregar primeiro janela e depois capacidades verificáveis.
- [x] Prevenção prevista: título validado, transação, lock do SO e snapshot pós-commit.
- [x] Reversão por incremento Git; dados reais fora dos testes e do controle de versão.
- [ ] Verificar implementação, QA e efetividade dessas prevenções durante execução.

Referências DDD: Evans, DDD parte IV (integridade da linguagem); Vernon, IDDD
cap.4 (separação de responsabilidades). Kaizen: skill local, princípios de
Gemba, incrementos pequenos, Check por artefato e Act nas skills/documentação.

## Fontes técnicas consultadas

- [GPUI 0.2.2](https://docs.rs/gpui/0.2.2/gpui/).
- [Crate component 0.5.1](https://crates.io/crates/gpui-component/0.5.1):
  Cargo.toml, README, src/input/state.rs e src/root.rs do arquivo publicado.
- [Assets 0.5.1](https://crates.io/crates/gpui-component-assets/0.5.1).
- [Rusqlite 0.42.0](https://docs.rs/rusqlite/0.42.0/rusqlite/).
- [File::try_lock](https://doc.rust-lang.org/std/fs/struct.File.html#method.try_lock).
- [Script Linux do Zed no commit associado à crate](https://github.com/zed-industries/zed/blob/69e2130295c2649963eb639fc70b4f2ee8ea1624/script/linux): referência auxiliar, não executar script inteiro.
- [Instalação atual do GPUI Kit](https://gpui-kit.com/docs/installation/):
  consultada para identificar a divergência, não usada como receita de 0.5.1.
