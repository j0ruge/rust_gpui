---
id: TASK-validar-titulo
area: TASK
title: Recusar títulos inválidos sem perder rascunho
persona: Pessoa começando
journey: J-organizar-tarefas
expected: Vazio controle e 201 escalares são recusados e 200 escalares e duplicatas são aceitos
entry_points: cargo run --locked --release
qa_status: pass
bug_ids:
fix_status:
retest_status:
fix_commits:
evidence: evidence/2026-11-12-full/title-control-validation.png; evidence/2026-11-12-full/title-201-real-rejection.png; evidence/2026-11-12-full/long-title-actions-fixed.png
last_report: reports/2026-11-12-full.md
overlaps:
---

Planejado a partir da jornada; aguardando execução na janela nativa.
