---
id: DATA-repetir-gravacao
area: DATA
title: Recuperar gravação que falhou sem perder rascunho
persona: Pessoa recuperando dados
journey: J-recuperar-falhas
expected: Retry salva uma vez após erro visível com rascunho e snapshot preservados
entry_points: cargo run --locked --release
qa_status: pass
bug_ids:
fix_status:
retest_status:
fix_commits:
evidence: evidence/2026-11-12-full/write-failure-draft-retained.png; evidence/2026-11-12-full/close-after-write-failure.png; evidence/2026-11-12-full/write-retry-durable.png
last_report: reports/2026-11-12-full.md
overlaps:
---

Planejado a partir da jornada; aguardando execução na janela nativa.
