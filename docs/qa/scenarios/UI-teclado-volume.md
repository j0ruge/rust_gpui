---
id: UI-teclado-volume
area: UI
title: Alcançar tarefa distante e salvar pelo teclado
persona: Pessoa usando teclado
journey: J-navegar-lista
expected: Foco atalhos setas e diálogos permitem salvar tarefa distante e reabrir em janela mínima
entry_points: cargo run --locked --release
qa_status: pass
bug_ids: BUG-20261112-long-title-hides-actions
fix_status: fixed
retest_status: pass
fix_commits: ef5f8ef
evidence: evidence/2026-11-12-full/volume-durable-reopened-last.png; evidence/2026-11-12-full/volume-minimum-completed-order.png
last_report: reports/2026-11-12-full.md
overlaps:
---

Executado na janela nativa; resultado e eventuais retestes registrados em
`../reports/2026-11-12-full.md`.
