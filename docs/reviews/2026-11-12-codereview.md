# Codereview — tarefas Rust + GPUI — 2026-11-12

Estado: `fixes_verified`; revisão independente do delta ainda em andamento.
Revisão completa de `develop` (`802b129198e3e843b0a05f53ff8b77b7797fe628`)
até `326adde4606e5dd85ea3df02f6387df68ee9cf21`: seis commits, 53 arquivos,
+11.312/−110 linhas. Configuração: raiz `.`, extensões `rs,sh`, testes `tests/*.rs`,
Cargo/toolchain como configuração, framework `generic` com GPUI nativo.
Cargo.lock e dois `.gitkeep` excluídos da análise semântica; scanner lê o diff completo.

O pedido de goal até todas as categorias A autoriza as correções destes achados.
A análise dos revisores permanece somente leitura. As correções, testes e este
registro são uma etapa separada, feita pelo orquestrador no checkout existente.
Sem remoto, push, PR ou merge.

## Rodada 1 — achados originais e decisões

IDs e locais abaixo se referem ao HEAD original, antes das correções. Resultados
limpos dos outros arquivos não apagam os achados desta rodada.

| ID | Severidade original | Local original | Achado | Decisão e evidência |
|---|---|---|---|---|
| R01 | HIGH | src/storage.rs:140 | Validação aceita gatilhos extras capazes de excluir outras tarefas durante um UPDATE | Corrigido: recusar objetos além de tasks/sqlite_sequence. Teste real inclui trigger, view, index e tabela sqliteXforeign; verifica preservação dos bytes. |
| R02 | MEDIUM | src/storage.rs:152 | Normalização modifica conteúdo dos literais SQL | Corrigido: preservar texto entre aspas simples. Testes recusam PENDING, pend ing e pend;ing e aceitam mudanças de formatação externa. |
| R03 | MEDIUM | src/ui/mod.rs:341 | Seleção fica no ID que saiu do filtro; Enter/Delete podem agir sobre tarefa oculta | Reproduzido em release: concluir a primeira tarefa em Pendentes e pressionar Delete abre sua exclusão. Corrigido reconciliando seleção após resultado e exigindo índice visível no teclado. Replay nos dois filtros, seleção visível e reabertura. |
| R04 | MEDIUM | src/ui/mod.rs:442 | Hipótese: diálogos não ativam por Enter/Espaço | Refutado: GPUI 0.2.2 elements/div.rs:2180–2210 gera ClickEvent no KeyUp do controle focado. Enter cancelou e Tab/Espaço confirmou exclusão na janela; Tab/Espaço também saiu do diálogo após falha. Nenhuma alteração funcional por este achado. |
| R05 | LOW | CONTEXT.md:110 | Checklist declara funcionamento/QA ainda não executados | Corrigido: distinguir planejamento do resultado posterior e manter AC08 parcial. |
| R06 | LOW | docs/superpowers/plans/2026-11-12-tarefas-gpui.md:43 | Contexto anterior ao Git/Cargo aparece como estado atual | Corrigido: seção explicitamente temporal e referência ao resultado da execução. |
| R07 | LOW | docs/qa/scenarios/*.md:19 | Oito cenários ainda dizem aguardar execução | Corrigido: sete apontam execução/reteste registrado; UI-resposta-release mantém tentativa inconclusiva e verificação humana pendente. Consolida os quatro retornos dos revisores sobre a mesma frase. |
| R08 | HIGH | src/config.rs:4 e APIs públicas em domain/error/state/storage/ui | Contratos públicos sem doc comments; lacuna confirmada pelo orquestrador no pass 6.5 | Corrigido: documentação em português nos 38 tipos/funções públicos e nos helpers da aplicação. Contagem em docstrings.json; não exige comentários em callbacks de teste. |

R01 e R02 eram `needs reproduction`: os testes novos falharam no comportamento
esperado antes das mudanças (`storage-red.log`, 10 passaram/2 falharam). Após as
correções, 33 testes passaram. O teste de rollback continua injetando trigger
após a abertura; remove a falha injetada antes de reabrir o esquema válido.
Isso preserva a prova de rollback sem exigir que a abertura aceite gatilhos extras.

## Evidências desta correção

Logs locais ignorados em `.superpowers/review/2026-11-12/`. O gate foi repetido
após a documentação final dos módulos, com quatro saídas zero em
`final-gate.json` e `final-gate-0.log` a `final-gate-3.log`:

| Evidência | Resultado |
|---|---|
| baseline-tests.log | 30 testes passaram antes das mudanças |
| storage-red.log | Duas falhas comportamentais, antes da correção |
| gate-0.log — cargo fmt --check | exit 0 |
| gate-1.log — cargo clippy --locked --all-targets -- -D warnings | exit 0 |
| gate-2.log — cargo test --locked | 33 testes, zero falhas |
| gate-3.log — cargo build --locked --release | exit 0 |
| mutation-results.json e mutation-*.log | Controle verde; três mutantes compilam e são detectados pelos testes de persistência |
| docstrings.json | 38/38 declarações públicas documentadas; helpers da aplicação documentados |

Mutações em cópia temporária dos módulos reais domain/error/storage e dos testes
de persistência: retirar a recusa de objetos extras; restaurar normalização dos
literais; restaurar o filtro LIKE que confunde `_` com caractere literal.
Cada mutante falhou por assertion, com resumo de teste, sem erro de compilação.
O controle passou. Nenhum mutante foi aplicado ao checkout.

QA nativo em Ubuntu 24.04/X11, DISPLAY=:1, release Rust 1.97.0. SHA256 do binário
e de `/proc/1753264/exe`:
`32d5135ff79944965fe1349395a46bd1df971208af5d00fccc04678420e6983c`.
Fixtures artificiais fora dos dados do usuário. Capturas locais ignoradas em
`docs/qa/evidence/2026-11-12-codereview/`:

- `baseline-hidden-delete.png`: diálogo da tarefa oculta antes da correção.
- `native-red.txt`: assertion falha sobre o diálogo, confirmado por OCR do recorte.
- `fixed-hidden-actions.png` e `fixed-reopen-hidden-actions.png`: Enter/Delete não
  agem sobre a seleção removida em Pendentes e Concluídas.
- `fixed-visible-edit.png`: Enter edita a seleção visível.
- `fixed-delete-dialog-real.png`, `fixed-enter-cancel-real.png` e
  `fixed-space-delete-real.png`: Cancelar em foco; Enter cancela; Tab/Espaço exclui.
- `fixed-failed-save-close.png`: decisão de saída após erro de validação; Tab/Espaço sai.
- `fixed-durable-reopened.png`: reabertura preserva as três tarefas e seus estados
  após o replay dos filtros; `reopened-exit.txt`: encerramento exit 0.

## Limites preservados

O relatório de QA anterior continua sendo a fonte daquela execução. As evidências
acima pertencem à correção, não substituem as jornadas anteriores. Não houve nova
medição de latência/FPS; AC08 permanece parcial e `UI-resposta-release` continua
`blocked-verify`. Leitor de tela, contraste formal e outros sistemas não certificados.
Aviso upstream de futura incompatibilidade de proc-macro-error2 2.0.1 continua
visível; não houve atualização de dependências ou toolchain.

Varredura de código morto: Bucket A limpo por pesquisa de referências; compilador
e clippy também passaram. Bucket B preexistente fora do escopo (`sweep=full` seria
necessário). Scanner de segredos da rodada 1: regex + gitleaks, zero achados,
zero erros, 53 arquivos/11.312 linhas adicionadas. Não equivale a uma auditoria
de vulnerabilidades de todas as dependências.

## Rodadas seguintes

Revisar apenas as correções a partir de `326adde`, incluindo documentação e testes.
Uma rodada limpa deve cobrir o último texto alterado; o goal só termina após essa
verificação. As notas finais e o HEAD coberto serão informados na conversa.
