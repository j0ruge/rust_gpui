# Acordos do projeto Rust + GPUI

## Missão e fonte de verdade

Experimentar Rust e GPUI em uma aplicação desktop de tarefas locais. Leia
`CONTEXT.md` para decisões confirmadas, evidências e perguntas em aberto.
`Rust e o Editor Zed.md` é uma referência inicial: seu código e suas afirmações
de desempenho precisam ser conferidos contra a versão escolhida do GPUI.

O usuário quer aprender Rust para usar no dia a dia, tem experiência praticamente
zero e escolheu explicitamente **a aplicação funcionando primeiro**. Executar os
incrementos do plano e deixar aulas/exercícios para depois. Não interromper a
construção para exigir participação didática. Manter código e documentação claros
para o estudo posterior.

Escopo confirmado: criar, editar, concluir/reabrir e excluir tarefas; filtros
Todas/Pendentes/Concluídas; salvamento automático no computador, sem conta ou
servidor. Prazos, prioridades e projetos ficam fora da primeira versão.

## Etapa atual e continuidade

- Aplicação implementada em `feat/tarefas-gpui`, a partir de develop
  `802b129198e3e843b0a05f53ff8b77b7797fe628`. Checkpoint do código: ef5f8ef.
  Não há remoto/push/PR/merge. Não repetir scaffolding ou reinstalar por suposição.
- Execução: `cargo run --locked --release`. README e docs/stack.md descrevem
  uso, versões, dados, testes, SHAs e limitações. A janela final foi aberta para uso.
- Plano em docs/superpowers/plans/2026-11-12-tarefas-gpui.md executado;
  resultado de QA em docs/qa/reports/2026-11-12-full.md. Trinta testes, fmt,
  clippy e build; jornadas nativas CRUD, filtros, falhas, reabertura e mil itens.
- QA encerrou com sete cenários aprovados e uma verificação humana pendente:
  limite de 1s da AC08 não pôde ser medido com instrumento confiável. Protocolo
  no relatório; scripts/qa-volume.sh cria fixture nova e isolada. Não declarar
  AC08 integralmente aprovado nem prometer FPS.
- Aprendizados verificados incorporados às skills pessoais rust/gpui em seus
  escopos; exemplos e validação estrutural registrados em docs/stack.md.
- Após outro `/clear`, o usuário iniciará explicitamente o goal com
  codereview:codereview até todas as categorias A com evidência. Essa revisão
  não ocorreu nesta execução. Usar base develop, configuração Rust e SHAs
  registrados; preservar achados originais entre rodadas e incluir docs/testes.
- Não afirmar troca de modelo, QA não realizado ou conclusão de goal não executado.
  Uma revisão somente leitura não autoriza alterações fora da correção solicitada.

## Convenções

- Prosa, comentários, docstrings e mensagens de commit em português.
- Identificadores, chaves e tokens de estado em inglês; manter o mapeamento
  entre termos da interface e do código no glossário do `CONTEXT.md`.
- Base Git: `develop`; fluxo `develop → staging → main`. Verificar o estado
  real antes de operar. Inicialização Git e entrega remota pertencem ao plano;
  não presumir que a pasta já seja um repositório ou que exista um remoto.
- Antes de salvar um arquivo, verificar se contém credenciais. Usar referências
  a variáveis; nunca versionar senhas, tokens, `.env` ou dados pessoais de QA.
- Menor mudança que resolve; não refatorar trechos vizinhos sem solicitação.
- Atualizar a documentação afetada junto com a mudança e registrar ciclos
  concluídos no `KAIZEN_LOG.md`; oportunidades não são autorização de execução.
- Mapear aprendizados verificados: Cargo/toolchain na skill `rust`, API e
  interface nativa na skill `gpui`, decisões deste produto em `CONTEXT.md`.
  Registrar fonte e limitações antes de generalizar uma conclusão.

## Rust e GPUI

- Aplicar `$rust` para toolchain, Cargo e verificação; aplicar `$gpui` para API,
  ambiente gráfico e interface nativa. Skills pessoais desta máquina:
  `/home/joruge/.codex/skills/rust/SKILL.md` e
  `/home/joruge/.codex/skills/gpui/SKILL.md`.
- Respeitar `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` e `.cargo/config.toml`
  quando existirem. Não atualizar dependências ou toolchain por conveniência.
- Conferir a API na fonte da versão resolvida. Não transformar snippets de
  tutorial em código considerado funcional antes de compilar.
- Distinguir estado de apresentação, regras das tarefas e persistência.
  DDD orienta termos e invariantes; não adicionar camadas ou infraestrutura
  sem necessidade do domínio.
- Em implementação, testes de comportamento devem cobrir invariantes e riscos
  de perda de dados. Usar TDD conforme o plano aprovado e a suíte adotada.
- Erros de armazenamento precisam ser observáveis; a política de recuperação
  será definida no plano. Testes usam armazenamento isolado dos dados do usuário.

## Verificação e evidência

- Formatação, análise estática, testes, build e uso da janela são evidências
  distintas. Registrar comando, versão/build e resultado, inclusive bloqueios.
- QA de GPUI usa a janela nativa, teclado e mouse; testes de DOM/navegador não
  comprovam o funcionamento dessa interface.
- Validar salvamento reabrindo o aplicativo. Se o ambiente não permitir testar
  a janela, registrar a limitação e o procedimento humano restante.
- Disponibilidade de Vulkan não prova qual dispositivo o aplicativo usou.
  Não prometer 120 FPS, consumo de memória ou desempenho sem medição.
- Checks de código e do binário só serão executados quando houver aplicação;
  documentação validada não significa aplicação validada.
