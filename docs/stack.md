# Stack e evidências de execução

## Histórico Git

- Base `develop`: `802b129198e3e843b0a05f53ff8b77b7797fe628` (documentação aprovada).
- Implementação na branch `feat/tarefas-gpui`, sem remoto/push/PR.
- SHA revisável será registrado ao final. O goal de codereview permanece para
  outra sessão, conforme combinado.

## Ambiente observado em 2026-11-12

Ubuntu 24.04.5 LTS x86_64; X11; `DISPLAY=:1`. `rustc` e Cargo 1.97.0 estão
instalados em stable; o pin nomeado 1.97.0 está sendo preparado pelo rustup.
Há 125 GiB livres no filesystem e 31 GiB de RAM total no início da execução.

O inventário `dpkg-query` confirmou ausência de pkg-config, clang, cmake e dos
headers listados no README. `sudo -n true` retornou “a password is required”.
Foi solicitada instalação no terminal do usuário. Não houve build/janela ainda.

## Dependências declaradas

Rust 1.97.0, edition 2024; GPUI 0.2.2; gpui-component/assets 0.5.1;
rusqlite 0.42.0 com SQLite bundled; thiserror 2.0.21; tempfile 3.27.0 em testes.
As dependências são exatas no manifesto. Grafo e lock serão conferidos no fetch.

## Verificação

| Incremento | Comando/evidência | Resultado |
|---|---|---|
| Baseline | `git init -b develop`, commit de documentação e branch de feature | concluído |
| Ambiente | PATH, dpkg-query, versões e `sudo -n true` | instalação depende do usuário |
| Primeira janela | build release e janela X11 | pendente |
| Domínio/config | testes | pendente |
| SQLite | persistência, validação, rollback e lock | pendente |
| Estado | transições e preservação em falha | pendente |
| Interface/QA | jornadas nativas e reabertura | pendente |

## Ajustes motivados pelo primeiro build

O build release falhou em `xattr 0.2.3` por `libc::ENOATTR` ausente em
`libc 0.2.191`. Grafo: GPUI → gpui_http_client → zed-async-tar → xattr.
A correção mínima foi `cargo +stable update -p libc --precise 0.2.189`,
registrada no lockfile, preservando as dependências diretas. O segundo build
compilou xattr e zed-async-tar. Fonte: código publicado no registry e
[remoção upstream](https://github.com/rust-lang/libc/issues/1356).

Fetch e builds iniciais usam `+stable`, cujo rustc é exatamente 1.97.0
(2d8144b78), enquanto se prepara o pin nomeado da mesma versão. Uma consulta
rustup concorrente à instalação provocou disputa do arquivo parcial de clippy;
a preparação foi repetida sequencialmente. Não se alterou a toolchain do projeto.

O plano de QA foi preparado com `qa-report` durante a compilação: quatro
jornadas/charters, oito cenários `untested`. O materializador validou os oito.
Isso não comprova interação na janela.

## Incremento 1 — janela comprovada

Todos os pacotes nativos previstos foram confirmados como installed por
dpkg-query após o usuário executar `scripts/install-system-deps.sh`. A toolchain
nomeada 1.97.0 e rustfmt/clippy foram conferidos pelos comandos de versão.

`cargo build --locked --release` passou. `cargo run --locked --release` abriu
janela X11 900×640 com o bootstrap. Captura:
`docs/qa/evidence/2026-11-12-full/initial-window.png`. SHA256 do binário e
`/proc/1416628/exe`: `7ef705e59714955090cdeb0a01a7895a0cf830f53862e105480e44daa95eaada`.
Fechamento via gerenciador de janelas retornou exit 0 e nenhum processo ficou.

Ao fechar inicialmente, o GPUI X11 apresentou `RefCell already borrowed` em
Platform::quit. Backtrace confirmou execução de quit durante close do backend.
`App::defer` ainda executava no mesmo flush; agendar quit com `App::spawn`
encerrou em outro ciclo e resolveu na repetição nativa. Fonte: GPUI 0.2.2,
x11/client.rs:1388, stack em close-backtrace.log. Isso não generaliza a outros backends.

O Cargo informa incompatibilidade futura upstream de `proc-macro-error2 2.0.1`
(reexport privado de proc_macro, rust-lang/rust#127909). É aviso transitivo;
não impede o build da toolchain fixada. Não foi aplicado patch amplo.
