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

- Plano autocontido pronto em
  `docs/superpowers/plans/2026-11-12-tarefas-gpui.md`, com spec vinculada.
  Retomada em `EXECUTAR_COM_SOL.md`. Ainda não há aplicação ou build.
- O usuário fará `/clear` e executará com o Sol. Nesta sessão de planejamento
  não iniciar scaffolding, instalação de dependências ou código do produto.
  Na nova sessão, o pedido de executar o plano autoriza avançar pelas tarefas;
  não reiniciar a entrevista ou pedir novamente escolhas já resolvidas.
- Plataforma inicial definida pelo planejador: Ubuntu 24.04 x86_64/X11;
  GPUI + componentes + SQLite, conforme versões e limites da spec.
- Escrever documentação e as skills solicitadas já está autorizado.
- Antes do handoff, registrar escopo, decisões, arquivos previstos, dependências,
  incrementos, comandos de verificação, critérios de aceite e pendências.
- Após a implementação: `qa-report` planeja as jornadas e `qa-execution` as
  executa. Depois de outro `/clear`, o usuário iniciará `$goal` com
  `codereview:codereview` até todas as categorias atingirem A com evidência.
- Não afirmar troca de modelo, execução de QA ou conclusão de goal sem ter
  realizado e verificado a ação. Uma revisão somente leitura não autoriza
  mudanças fora do escopo de correção solicitado.

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
