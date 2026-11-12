# CH-organizar-tarefas: Organizar tarefas pessoais

```yaml
charter:
  id: CH-organizar-tarefas
  mission: Tarefas corretas reaparecem na reabertura e a excluída permanece ausente
  mode: charter-with-tour
  persona:
    name: Pessoa começando
    device: desktop
    network: wifi-fast
    locale: pt-BR
  journey: J-organizar-tarefas
  scenarios: [TASK-crud-filtros, TASK-validar-titulo, TASK-cancelar-alteracoes]
  tour: Feature Tour
  time_box_minutes: 60
  guidance:
    must_try:
      - Abrir a janela vazia
      - Confirmar exclusão e reabrir o app
      - Fechar durante um rascunho não confirmado; ao retornar só ações confirmadas permanecem
    must_avoid:
      - Usar dados reais ou acessar a janela de outro aplicativo
      - Afirmar Pass sem observação na janela e evidência de reabertura
```
