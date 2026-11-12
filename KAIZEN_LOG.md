# Kaizen Log — Rust + GPUI

## 2026-11-12 — Fechar plano para entregar o aplicativo antes da aula

- **Tipo:** melhoria e padronização.
- **Antes:** descoberta aberta e proposta de aprendizado intercalado à construção.
- **Depois:** usuário escolheu aplicativo primeiro, execução futura com Sol. Plano
  e desenho definidos, checklist DDD/Kaizen atualizado e retomada autocontida.
- **Padronizado em:** `AGENTS.md`, `CONTEXT.md`, `EXECUTAR_COM_SOL.md`,
  `docs/superpowers/specs/2026-11-12-tarefas-gpui-design.md` e
  `docs/superpowers/plans/2026-11-12-tarefas-gpui.md`.
- **Yokoten:** lições de build nativo na skill pessoal `rust`; compatibilidade e
  bootstrap de componentes na referência `gpui/references/components.md`.

### Evidência e destino dos aprendizados

| Aprendizado | Fonte verificada | Onde foi incorporado |
|---|---|---|
| Runtime gráfico instalado não comprova headers/ferramentas de build | PATH/dpkg-query: faltam clang, cmake, pkg-config e pacotes -dev | `$rust`, seção Reconhecer o projeto; inventário específico no plano |
| Bibliotecas de componentes precisam compartilhar a família GPUI | Manifests publicados component/assets 0.5.1 usam gpui 0.2.2; docs atuais Kit usam gpui-pre | `$gpui`, references/components.md |
| Falha de docs.rs não comprova indisponibilidade de uma versão | URLs 0.5.1 falharam; crate publicada e fontes foram lidas com sucesso | `$gpui`, references/components.md |
| Bootstrap dos componentes exige inicialização e Root; input tem eventos distintos | README, root.rs e input/state.rs publicados em 0.5.1 | `$gpui`, references/components.md |
| Desejo de aprender não define preferência por aula antes da entrega | Correção explícita do usuário nesta sessão | AGENTS/CONTEXT; não generalizado nas skills |

### Check e limites

- Desenho lido por avaliador independente: `nothing important left`.
- Autorrevisão do plano ligou AC01–AC11 às tarefas e conferiu contratos; adicionou
  proteção de rascunho durante busy e validação do HOME absoluto.
- As duas skills passaram novamente no `quick_validate.py` após as alterações.
- A leitura dos manifests comprova requisitos declarados, não build real.
- Nenhuma instalação, inicialização Git/Cargo, execução do app, QA funcional,
  troca do modelo principal ou goal de revisão ocorreu nesta fase.

### Próximo passo

Usuário faz `/clear`, seleciona Sol e envia a instrução de `EXECUTAR_COM_SOL.md`.
O executor comprova os incrementos e registra novos aprendizados nas skills
respectivas; a revisão final com goal ocorre após outro clear, conforme pedido.

## 2026-11-12 — Preservar decisões e separar referências reutilizáveis

- **Tipo:** padronização.
- **Antes:** pasta com um documento de conversa e snippet de contador, sem
  instruções locais nem registro das escolhas feitas para a aplicação.
- **Depois:** `AGENTS.md` estabelece os acordos; `CONTEXT.md` registra decisões,
  evidências, perguntas abertas e checklist DDD/Kaizen. Skills pessoais `rust`
  e `gpui` mantêm separadas as orientações reutilizáveis.
- **Padronizado em:** `AGENTS.md`, `CONTEXT.md`,
  `/home/joruge/.codex/skills/rust/SKILL.md` e
  `/home/joruge/.codex/skills/gpui/SKILL.md`.
- **Yokoten:** skills pessoais podem orientar outros projetos Rust/GPUI sem
  impor a lista de tarefas, plataforma ou sequência de aprovação deste projeto.

### Aprendizados e destino

| Aprendizado | Evidência | Destino |
|---|---|---|
| Pasta de trabalho não implica projeto Cargo ou Git | Inventário inicial sem manifesto; `git status` falhou por ausência de repo | Skill `rust`; contexto específico em `CONTEXT.md` |
| Ferramentas disponíveis não comprovam build do produto | Versões de Rust/Cargo/rustfmt/Clippy consultadas; nenhuma compilação realizada | Skill `rust` |
| Dependências e exemplos precisam se referir à mesma versão | API publicada GPUI 0.2.2 e snippet da referência diferem na closure de janela | Skill `gpui` |
| Inventário Vulkan não prova backend efetivamente utilizado | `vulkaninfo` lista GPU Intel e llvmpipe; aplicação inexistente | Skill `gpui` |
| QA precisa alcançar a superfície nativa | GPUI é interface desktop; ferramentas X11 localizadas, mas não exercitadas com o app | Skill `gpui` e critérios futuros no contexto |
| Decisões do produto não devem virar regras universais de framework | Usuário escolheu tarefas, autosave e operações; plataforma ainda sem resposta | `AGENTS.md` e `CONTEXT.md` |

### Verificação e limites

- `quick_validate.py` retornou `Skill is valid!` para as duas skills.
- A leitura independente anterior à escrita não demonstrou falha comportamental
  nesses cenários. A criação atende ao pedido explícito de referências separadas;
  não se afirma um ciclo de correção RED/GREEN nem ganho de qualidade medido.
- Avaliação da skill Rust com pasta sem manifesto e backends mutuamente exclusivos:
  preservou a fase de planejamento, evitou `--all-features` e distinguiu check,
  testes e artefato. A escolha da matriz permanece dependente do projeto.
- Avaliação da skill GPUI com tutorial divergente, Vulkan com dois dispositivos
  e sessão Wayland sem automação: identificou necessidade de API versionada,
  rejeitou evidência DOM como aceite nativo e manteve FPS/QA sem comprovação.
  Critérios de desempenho e política de autosave continuam decisões do projeto.
- Ainda não há aplicação, build, benchmark ou QA funcional executado.
- O usuário explicitou que o objetivo é aprender Rust para o dia a dia. O próximo
  ciclo deve calibrar experiência e participação, resolver a plataforma alvo,
  fechar abordagem e produzir o plano autocontido antes do `/clear` solicitado.

### Desperdícios evitados

- Sem transformar as escolhas do MVP em regras globais das skills.
- Sem scaffolding, instalação ou benchmarks antes do plano.
- Sem inventar resultados de testes a partir da disponibilidade das ferramentas.


## 2026-11-12 — Incremento 1: primeira janela e fechamento verificáveis

- Plan: bootstrap com versões exatas e janela real antes de CRUD.
- Do: baseline develop, branch feature, Cargo/lock/toolchain, Root/assets e script de dependências.
- Check: build release, captura da janela e SHA256 comparado ao processo; fechamento exit 0.
- Act: resolução libc 0.2.189 para xattr legado; encerramento assíncrono evita reentrada X11.
- Evidências e limites em `docs/stack.md`; QA funcional ainda não executado.


## 2026-11-12 — Incremento 2: regras independentes da janela

- Plan/Do: tipos de valor e filtros, resolver caminhos por argumentos.
- Check: RED comportamental seguido de dez testes verdes em release.
- Act: regras e mapeamento pt-BR/inglês preservados sem camadas adicionais.


## 2026-11-12 — Incremento 3: prevenir perda e falsa confirmação

- Plan/Do: SQLite transacional, schema/registro verificados e lock do SO.
- Check: dez testes reais com diretórios temporários; rollback por trigger,
  bytes corrompidos preservados e reabertura após drop. Suíte release verde.
- Act: padronizar snapshot somente após commit; erro explícito sem reset do banco.


## 2026-11-12 — Incremento 4: separar intenção de confirmação

- Plan/Do: estado puro com begin/finish, retry e fechamento adiado.
- Check: nove cenários RED/GREEN e suíte release com 29 testes verdes.
- Act: não apagar rascunho de outra ação nem converter erro em lista vazia.


## 2026-11-12 — Incremento 5: integrar e revisar pelo uso

- Plan/Do: UI nativa, jobs serializados fora da renderização e foco/atalhos.
- Check: CRUD, filtros, cancelamento, confirmação e reabertura na janela; mínimo
  640×480; 30 testes padrão e clippy sem avisos no código do projeto.
- Act: corrigir largura intrínseca que escondia ações e preservar feedback Salvo
  no eco programático do input. Detalhes em docs/qa; revisão final com goal não iniciada.


## 2026-11-12 — Incremento 6: entregar com evidência e limite explícito

- Plan: QA nativo, reabertura, falhas deliberadas isoladas e lista de 1.000 itens.
- Do: qa-report criou jornadas/charters/8 cenários; qa-execution percorreu quatro
  jornadas por quatro personas no release. Hash conferido contra processo.
- Check: sete cenários aprovados, dois bugs de UI corrigidos/verificados; um
  blocked-verify por captura contínua inconclusiva para o limite de 1s. Não
  houve benchmark/FPS inventado. Falha real de journal preservou draft e retry
  salvou uma vez, confirmado na UI reaberta. Cenários e relatório atualizados.
- Act: script qa-volume.sh reproduz dados artificiais em diretório sempre novo;
  README/CONTEXT/AGENTS/handoff atualizados. Protocolo humano da medição no relatório.
- Yokoten Rust: references/build-compatibility.md — pin transitivo por erro real,
  Cargo.lock, instalação serializada de toolchain. Fonte: erro/build e grafo.
- Yokoten GPUI: references/state-and-x11.md — quit em outro ciclo do executor,
  eco Change, largura de linha e dispositivo do processo Vulkan. Fonte: API
  publicada da versão, testes e replays nativos. Quick_validate passou para ambas;
  validação estrutural não é avaliação independente de agentes.
- Decisões/custos preservados em docs/stack.md: branch no diretório original,
  ledger manual por cabeçalhos PT, pin libc a reavaliar e revisão adiada pelo usuário.
- Próximo ciclo autorizado nesta entrega é apenas a verificação humana pendente;
  goal de codereview deve ser iniciado pelo usuário após novo clear. Oportunidade
  de melhorar a instrumentação não autoriza refatoração adjacente.
