# QA de Tarefas

Árvore viva do QA nativo do aplicativo Rust + GPUI. Automação por padrão:
`bash scripts/qa-headless.sh`, após build release. Sessão visível apenas quando
solicitada: `cargo run --locked --release` na sessão X11 do Ubuntu.

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


Rodada inicial: [2026-11-12-full](reports/2026-11-12-full.md), encerrada com sete
cenários aprovados e um blocked-verify de temporização. Reproduzir o volume com
`bash scripts/qa-volume.sh`; ele cria 1.000 registros artificiais em diretório novo.
O protocolo humano está no relatório. A aplicação habitual abre sem esse script.

## Regressões headless do PR #1

Após `cargo build --locked --release`, execute `bash scripts/qa-headless.sh`.
O wrapper cria um Xvfb exclusivo sem TCP, remove a herança Wayland e seleciona
lavapipe. Python verifica o servidor virtual antes de qualquer entrada. Openbox
e todas as ferramentas de teclado, mouse e captura ficam nesse display.

Dependências de QA: `xvfb`, `xauth`, `openbox`, `xdotool`, `wmctrl`,
`imagemagick`, `tesseract-ocr`, `python3`, `python3-pil` e
`mesa-vulkan-drivers`. Confira a disponibilidade antes de instalar. Um ICD de
software em outro local pode ser indicado por `RUST_GPUI_QA_ICD_FILE`.

As 15 jornadas cobrem foco após concluir/reabrir/excluir, fechamento repetido,
erros iniciais, segunda instância, validação, seleção oculta e retry de
criação/edição com o rascunho atualizado. Fixtures SQLite, cópia do binário,
SHA256, capturas, logs e `report.json` ficam num diretório novo em `/tmp`.
O relatório exige resultado para cada jornada selecionada e exit 1 em falhas.

Para uma execução dirigida, use, por exemplo:

```bash
bash scripts/qa-headless.sh --case retry_create --case retry_rename
```

`--binary /caminho/binario` escolhe o artefato; `--output /caminho/novo`
escolhe a pasta de evidências. Não reutilize a pasta de uma execução anterior.
Não execute `qa_headless.py` diretamente no desktop. O teste confirma a janela
nativa e efeitos no SQLite, sem DOM. OCR reconhece rótulos; dados exatos são
conferidos no banco artificial. Lavapipe e tempos totais dessas jornadas não
certificam GPU física, FPS ou o limite de 1s da AC08.

Resultados desta rodada: [correções do Claude](../reviews/2026-11-12-claude-pr1.md).
