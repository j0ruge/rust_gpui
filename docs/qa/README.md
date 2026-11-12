# QA de Tarefas

Árvore viva do QA nativo do aplicativo Rust + GPUI. Entrada: executar
`cargo run --locked --release` na sessão X11 do Ubuntu. Não há servidor web.

Áreas: **TASK** (ações e validação), **DATA** (continuidade e recuperação),
**UI** (teclado, tamanho e volume). Cenários ficam em `scenarios/`, jornadas
em `journeys/`, missões em `charters/` e resultados datados em `reports/`.

Usar sempre `RUST_GPUI_DATA_DIR` absoluto em diretório temporário isolado.
Dados e imagens de QA são artificiais. Capturas somente da janela por ID X11;
`evidence/` e a visão gerada `state.csv` ficam ignoradas no Git. Logs volumosos
ficam em `.superpowers/sdd/2026-11-12-tarefas-gpui/`, indexados pelo relatório.

Um Pass exige a jornada na janela real e reabertura quando houver persistência.
Testes de domínio/SQLite/estado são evidências separadas. O binário é identificado
por SHA256 comparado com `/proc/<pid>/exe`. Disponibilidade de Vulkan não comprova
qual dispositivo foi utilizado. Leitor de tela completo não é certificado.

Planejamento inicial: full, quatro jornadas e quatro personas. Cada jornada
considera comportamento, experiência, falhas e continuidade. Auth, conta,
rede, mobile e multiplataforma não se aplicam ao escopo local Ubuntu/X11.


Rodada atual: [2026-11-12-full](reports/2026-11-12-full.md), encerrada com sete
cenários aprovados e um blocked-verify de temporização. Reproduzir o volume com
`bash scripts/qa-volume.sh`; ele cria 1.000 registros artificiais em diretório novo.
O protocolo humano está no relatório. A aplicação habitual abre sem esse script.
