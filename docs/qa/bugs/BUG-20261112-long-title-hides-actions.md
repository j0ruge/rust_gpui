# BUG-20261112-long-title-hides-actions: Título longo esconde Editar e Excluir

- **Status:** verified
- **Impact (user-side):** Blocks-Completion
- **Severity:** High · **Priority:** P1
- **Persona Affected:** Pessoa começando
- **Journey Step:** J-organizar-tarefas, criar e ler a tarefa
- **Scenarios:** UI-teclado-volume
- **Found:** 2026-11-12 · **Report:** reports/2026-11-12-full.md

## Summary

Título longo esconde Editar e Excluir. O resultado foi observado na janela real Ubuntu/X11.

## Reproduction

- **Charter:** CH-organizar-tarefas · **Tour:** Feature Tour
- **Environment:** release Rust 1.97.0, X11, 900×640, pt-BR

1. Abrir um diretório temporário isolado.
2. Criar uma tarefa e confirmar com Enter; no caso da largura usar 200 escalares.
3. Observar lista, ações e mensagem de sucesso.

**Expected:** Ações alcançáveis e feedback Salvo após commit.
**Actual:** Título longo esconde Editar e Excluir.

## Evidence

- evidence/2026-11-12-full/title-200-real-layout.png
- Binário inicial integrado 5cf0e53d; diagnóstico d9c9c30f, com trace sem conteúdo de tarefas.

## Fix

- **Root cause:** Linha virtualizada sem largura máxima disponível usa largura intrínseca do título.
- **Fix commit:** ef5f8ef
- **Regression test:** replay nativo para largura; estado puro para eco do input.

## Verification

Reteste em 2026-11-12 no mesmo perfil. Ações de título longo visíveis em 900×640 e 640×480; Salvo preservado após Create. Evidências long-title-actions-fixed.png e minimum-window-actions.png. Correção pequena, sem schema ou mudança de produto.
