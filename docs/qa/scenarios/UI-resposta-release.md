---
id: UI-resposta-release
area: UI
title: Filtrar e alterar 1000 tarefas com resposta medida
persona: Pessoa usando teclado
journey: J-navegar-lista
expected: Filtro e ação simples respondem em até um segundo com método observável no release
entry_points: cargo run --locked --release
qa_status: blocked-verify
bug_ids:
fix_status:
retest_status:
fix_commits:
evidence: evidence/2026-11-12-full/latency-filter.json; evidence/2026-11-12-full/filter-after-measure-probe.png
last_report: reports/2026-11-12-full.md
overlaps:
---

Planejado a partir da jornada; aguardando execução na janela nativa.

Medição bloqueada pelo instrumento: captura contínua não registrou mudança que apareceu na captura da janela seguinte. Não há latência aprovada ou falha de desempenho atribuída ao produto. Procedimento humano no relatório e scripts/qa-volume.sh.
