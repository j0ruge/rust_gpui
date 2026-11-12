---
id: DATA-falha-inicializacao
area: DATA
title: Compreender erro de abertura com banco preservado
persona: Pessoa recuperando dados
journey: J-recuperar-falhas
expected: Banco inválido versão path e segunda instância exibem erro e preservam dados
entry_points: cargo run --locked --release
qa_status: pass
bug_ids:
fix_status:
retest_status:
fix_commits:
evidence: evidence/2026-11-12-full/initial-error-corrupt.png; evidence/2026-11-12-full/initial-error-future.png; evidence/2026-11-12-full/initial-error-path-file.png; evidence/2026-11-12-full/second-instance-visible.png
last_report: reports/2026-11-12-full.md
overlaps:
---

Executado na janela nativa; resultado e eventuais retestes registrados em
`../reports/2026-11-12-full.md`.
