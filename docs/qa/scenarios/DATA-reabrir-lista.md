---
id: DATA-reabrir-lista
area: DATA
title: Retomar ações confirmadas após reabrir
persona: Pessoa retomando
journey: J-retomar-lista
expected: Conteúdo estado e ordem persistem e rascunho não confirmado é descartado
entry_points: cargo run --locked --release
qa_status: pass
bug_ids:
fix_status:
retest_status:
fix_commits:
evidence: evidence/2026-11-12-full/resume-draft-discarded.png
last_report: reports/2026-11-12-full.md
overlaps:
---

Executado na janela nativa; resultado e eventuais retestes registrados em
`../reports/2026-11-12-full.md`.
