# BUG-20261112-saved-feedback-disappears: Limpeza do formulário apaga o feedback Salvo

- **Status:** verified
- **Impact (user-side):** Friction
- **Severity:** Low · **Priority:** P2
- **Persona Affected:** Pessoa começando
- **Journey Step:** J-organizar-tarefas, criar e ler a tarefa
- **Scenarios:** TASK-crud-filtros
- **Found:** 2026-11-12 · **Report:** reports/2026-11-12-full.md

## Summary

Limpeza do formulário apaga o feedback Salvo. O resultado foi observado na janela real Ubuntu/X11.

## Reproduction

- **Charter:** CH-organizar-tarefas · **Tour:** Feature Tour
- **Environment:** release Rust 1.97.0, X11, 900×640, pt-BR

1. Abrir um diretório temporário isolado.
2. Criar uma tarefa e confirmar com Enter; no caso da largura usar 200 escalares.
3. Observar lista, ações e mensagem de sucesso.

**Expected:** Ações alcançáveis e feedback Salvo após commit.
**Actual:** Limpeza do formulário apaga o feedback Salvo.

## Evidence

- evidence/2026-11-12-full/trace-created.png
- Binário inicial integrado 5cf0e53d; diagnóstico d9c9c30f, com trace sem conteúdo de tarefas.

## Fix

- **Root cause:** InputState.set_value emite Change e o adapter zera saved mesmo quando o draft já coincide com o estado confirmado.
- **Fix commit:** ef5f8ef
- **Regression test:** replay nativo para largura; estado puro para eco do input.

## Verification

Reteste em 2026-11-12 no mesmo perfil. Ações de título longo visíveis em 900×640 e 640×480; Salvo preservado após Create. Evidências long-title-actions-fixed.png e minimum-window-actions.png. Correção pequena, sem schema ou mudança de produto.
