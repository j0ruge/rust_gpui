# Retomada com Sol

O planejamento está pronto. O usuário quer a aplicação funcionando primeiro e
estudará Rust depois. Esta fase não criou código, Git ou dependências instaladas.

## Como retomar

Após `/clear`, selecione o Sol na interface e envie:

```text
Execute o plano em /home/joruge/repos/rust_gpui/docs/superpowers/plans/2026-11-12-tarefas-gpui.md.
Leia primeiro o AGENTS.md e a spec vinculada no plano. Quero a aplicação de
tarefas Rust + GPUI funcionando no meu Ubuntu, com CRUD, filtros e salvamento
local. Faça os incrementos, testes e QA previstos sem reiniciar o brainstorming
ou intercalar aulas. Registre aprendizados em $rust e $gpui, cada um no seu escopo.
Ao terminar, mostre a aplicação rodando e o comando para abri-la. Deixe o goal
final de codereview para minha próxima sessão após outro clear.
```

O agente não alterou o modelo da conversa: a seleção do Sol é feita pelo usuário.

## Arquivos de entrada

- [AGENTS.md](AGENTS.md): acordos atuais.
- [Plano](docs/superpowers/plans/2026-11-12-tarefas-gpui.md): tarefas, contratos,
  testes, preparação do ambiente, QA e revisão futura.
- [Desenho](docs/superpowers/specs/2026-11-12-tarefas-gpui-design.md): comportamento,
  versões, regras, falhas e AC01–AC11.
- [CONTEXT.md](CONTEXT.md): decisões, evidências e checklist DDD/Kaizen.

Skills pessoais: `/home/joruge/.codex/skills/rust/SKILL.md` e
`/home/joruge/.codex/skills/gpui/SKILL.md`. QA: skills em
`/home/joruge/.claude/skills/qa-report/` e `qa-execution/`.

## Entrega esperada

Primeiro uma janela real; depois aplicação completa com SQLite local e testes.
`cargo run --locked --release` deve abrir “Tarefas”. Não declarar esse comando
funcional antes do build e da sessão gráfica. Se houver bloqueio real de sudo,
dependência ou acesso à janela, registrar exatamente o que falta.

Depois da entrega/QA e do novo `/clear`, pedir explicitamente o goal de review
com `codereview:codereview` até todas as categorias A. Usar os SHAs e resultados
gravados pelo executor; não usar defaults TypeScript/React para Rust.
