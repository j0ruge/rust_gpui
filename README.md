# Tarefas

Aplicação desktop de tarefas locais em Rust + GPUI, em construção conforme o
[plano aprovado](docs/superpowers/plans/2026-11-12-tarefas-gpui.md).

## Ambiente

Ubuntu 24.04 x86_64 com sessão gráfica X11 e Vulkan. Rust 1.97.0 é selecionado
por `rust-toolchain.toml`. As versões e evidências ficam em [docs/stack.md](docs/stack.md).

Dependências nativas previstas:

```bash
sudo apt-get update
sudo apt-get install --no-install-recommends \
  build-essential pkg-config clang cmake libfontconfig-dev libfreetype-dev \
  libwayland-dev libx11-xcb-dev libxkbcommon-dev libxkbcommon-x11-dev \
  libxcb1-dev libssl-dev libzstd-dev
```

## Compilar e abrir

Na sessão gráfica do computador:

```bash
cd /home/joruge/repos/rust_gpui
cargo run --locked --release
```

O bootstrap já abriu a janela real; CRUD e persistência seguem em implementação.
