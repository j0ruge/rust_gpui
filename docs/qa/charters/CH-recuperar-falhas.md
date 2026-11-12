# CH-recuperar-falhas: Recuperar uma operação sem perda

```yaml
charter:
  id: CH-recuperar-falhas
  mission: Erro é compreensível e os dados anteriores e o rascunho são preservados
  mode: charter-with-tour
  persona:
    name: Pessoa recuperando dados
    device: desktop
    network: wifi-fast
    locale: pt-BR
  journey: J-recuperar-falhas
  scenarios: [DATA-falha-inicializacao, DATA-repetir-gravacao]
  tour: Garbage Tour
  time_box_minutes: 60
  guidance:
    must_try:
      - Abrir fixtures de banco inválido, versão e path inválidos
      - Reabrir e conferir durabilidade
      - Sair após erro preserva o banco; sair sem salvar descarta só a ação que falhou
    must_avoid:
      - Usar dados reais ou acessar a janela de outro aplicativo
      - Afirmar Pass sem observação na janela e evidência de reabertura
```
