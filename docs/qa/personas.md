# Personas

```yaml
personas:
  - name: Pessoa começando
    base: New User
    goal: Registrar e organizar tarefas pessoais pela primeira vez
    device: desktop
    network: wifi-fast
    modality: mouse-keyboard
    locale: pt-BR
    patience_seconds: 60
  - name: Pessoa retomando
    base: Casual User
    goal: Encontrar amanhã as tarefas salvas hoje
    device: desktop
    network: wifi-fast
    modality: mouse-keyboard
    locale: pt-BR
    patience_seconds: 30
  - name: Pessoa recuperando dados
    base: Recovering User
    goal: Entender uma falha sem perder o último registro ou rascunho
    device: desktop
    network: wifi-fast
    modality: mouse-keyboard
    locale: pt-BR
    patience_seconds: 30
  - name: Pessoa usando teclado
    base: Accessibility-Reliant
    goal: Alcançar e administrar tarefas sem mouse inclusive em lista longa
    device: desktop
    network: wifi-fast
    modality: keyboard-only
    locale: pt-BR
    patience_seconds: 30
```

Aplicação desktop local: persona mobile não se aplica. A modalidade de teclado
é exercitada; suporte completo a leitor de tela permanece fora da afirmação de QA.
