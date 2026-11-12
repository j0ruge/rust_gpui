# **Rust e o Editor Zed**

[https\://gemini.google.com/app/817562bf253a9893](https://gemini.google.com/app/817562bf253a9893)

*User prompt: Qual relação entre a linguagem rust e a interface ZED?*

Response: A relação principal é direta e fundamental: o **Zed é um editor de código construído inteiramente na linguagem Rust**.

Criado pelos mesmos fundadores do editor Atom e da biblioteca Tree-sitter, a equipe escolheu o Rust desde o primeiro dia para resolver os problemas de lentidão e consumo de memória comuns em editores baseados em Electron (como o VS Code e o próprio Atom).

Aqui estão os três pontos centrais dessa relação:

> * **Desempenho e Uso de Memória:** O Rust compila diretamente para código de máquina e não possui *garbage collector*, o que permite que o Zed abra projetos gigantescos quase instantaneamente e consuma uma fração da memória (RAM) exigida por editores baseados em tecnologias web.  
> * **O Framework GPUI:** Como não havia uma interface gráfica em Rust que atendesse às necessidades de um editor de altíssimo desempenho, os criadores desenvolveram seu próprio framework de UI do zero em Rust, chamado **GPUI**. Ele renderiza toda a interface do Zed diretamente na GPU, garantindo animações e rolagem de texto a 120 quadros por segundo (fps).  
> * **Concorrência Segura (Multiplayer):** O Zed possui recursos nativos de colaboração em tempo real. O modelo de segurança de memória e "ownership" do Rust tornou muito mais fácil para os desenvolvedores criarem um sistema multithread complexo sem os temidos *data races* (conflitos de dados) que causariam travamentos no editor.

Em resumo, a linguagem Rust é o motor que permite ao Zed cumprir sua promessa de ser o editor de código colaborativo mais rápido do mercado.

---

*User prompt: Sim quero saber como funciona e gastaria de testar em um caso real*

Response: O **GPUI** funciona de uma maneira diferente da maioria das bibliotecas de interface gráfica. Ele foi criado porque a equipe do Zed percebeu que frameworks baseados em tecnologias web (como o Electron) gastavam muito tempo com o *garbage collector* e cálculos de layout do navegador (DOM).

Aqui estão os três pilares de como ele funciona por baixo dos panos:

> 1. **Renderização Direta na GPU:** Em vez de usar botões e janelas padrão do sistema operacional (ou HTML), o GPUI transforma sua interface em formas geométricas básicas (retângulos, sombras, textos) e manda a placa de vídeo desenhar tudo usando shaders nativos (Metal no Mac e Vulkan no Linux).  
> 2. **Estilização Estilo "Tailwind":** Você não escreve arquivos CSS separados. Toda a aparência é definida via código encadeando métodos em Rust (ex: .flex().bg(rgb(0x000000)).p\_2()).  
> 3. **Gestão de Estado (Contexto):** O GPUI possui um estado centralizado chamado App. Suas telas e botões são "Entidades". Quando você altera uma variável, precisa chamar um comando chamado cx.notify(). Isso avisa a placa de vídeo: *"Ei, o número mudou, desenhe apenas este pedaço da tela novamente"*.

### **Testando o GPUI em um Caso Real**

Vamos criar um aplicativo de Desktop completo (um contador simples) usando o GPUI.

**Passo 1: Criar o projeto** Abra o seu terminal e crie um novo projeto em Rust:

`cargo new meu_app_gpui`  
`cd meu_app_gpui`  
`cargo add gpui`

**Passo 2: O Código** Abra o arquivo src/main.rs e substitua todo o conteúdo por este código:

`use gpui::*;`

`// 1. Definimos o nosso componente e o estado dele`  
`struct Counter {`  
    `count: usize,`  
`}`

`// 2. Implementamos a interface gráfica`  
`impl Render for Counter {`  
    `fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {`  
        `div()`  
            `.flex()`  
            `.flex_col()`  
            `.items_center()`  
            `.justify_center()`  
            `.size_full()`  
            `.bg(rgb(0x1e1e2e)) // Cor de fundo escura`  
            `.text_color(rgb(0xffffff)) // Texto branco`  
            `.child(`  
                `div()`  
                    `.text_xl()`  
                    `.child(format!("Cliques: {}", self.count))`  
            `)`  
            `.child(`  
                `div()`  
                    `.mt_4()`  
                    `.p_2()`  
                    `.bg(rgb(0x45475a))`  
                    `.rounded_md()`  
                    `.cursor_pointer()`  
                    `.hover(|style| style.bg(rgb(0x585b70))) // Efeito ao passar o mouse`  
                    `.on_click(cx.listener(|this, _event, _window, cx| {`  
                        `// O que acontece ao clicar:`  
                        `this.count += 1;`  
                        `cx.notify(); // Avisa o GPUI para atualizar a tela!`  
                    `}))`  
                    `.child("Incrementar")`  
            `)`  
    `}`  
`}`

`// 3. Inicializamos o aplicativo e abrimos a janela`  
`fn main() {`  
    `Application::new().run(|cx: &mut App| {`  
        `let options = WindowOptions::default();`  
        `cx.open_window(options, |cx| {`  
            `cx.new(|_cx| Counter { count: 0 })`  
        `}).unwrap();`  
    `});`  
`}`

**Passo 3: Rodar o aplicativo** Volte ao terminal e execute:

`cargo run --release`

*(Nota: O GPUI roda muito mais fluido quando compilado em modo \--release)*.

Você verá uma janela nativa escura com um botão. Toda vez que você clicar nele, o número será atualizado instantaneamente a 120 FPS via placa de vídeo.

**Sources**

> 1. [Desktop GUI with Rust \- Introduction to gpui Part 1 (gpui's ... \- Zenn](https://zenn.dev/nazo6/articles/gpui-introduction-1?locale=en)  
> 2. [Building a Desktop ToDo App in Rust with GPUI, Zed's UI Framework](https://azukiazusa.dev/en/blog/gpui-todo-app/)  
> 3. [Your First GPUI App \- Building a Desktop UI in Rust \- YouTube](https://www.youtube.com/watch?v=NT2XPvtof-Y)  
> 4. [Tutorial for getting started with gpui a rust-driven UI ... \- GitHub](https://github.com/hedge-ops/gpui-tutorial)

---

