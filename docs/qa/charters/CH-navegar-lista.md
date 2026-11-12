# CH-navegar-lista: Administrar uma lista pelo teclado

```yaml
charter:
  id: CH-navegar-lista
  mission: Tarefa distante é alcançável e alterações persistem com resposta medida até um segundo
  mode: charter-with-tour
  persona:
    name: Pessoa usando teclado
    device: desktop
    network: wifi-fast
    locale: pt-BR
  journey: J-navegar-lista
  scenarios: [UI-teclado-volume, UI-resposta-release]
  tour: Feature Tour
  time_box_minutes: 60
  guidance:
    must_try:
      - Abrir release com 1000 tarefas semeadas antes da abertura
      - Redimensionar e reabrir
      - Esc cancela edição ou diálogo e restaura foco sem alterar dados
    must_avoid:
      - Usar dados reais ou acessar a janela de outro aplicativo
      - Afirmar Pass sem observação na janela e evidência de reabertura
```
