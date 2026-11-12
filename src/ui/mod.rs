use gpui::{Context, IntoElement, ParentElement, Render, Styled, Window, div, rgb};

pub struct TaskScreen;

impl Render for TaskScreen {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .p_8()
            .gap_4()
            .bg(rgb(0x181b20))
            .text_color(rgb(0xe8e9ec))
            .child(div().text_2xl().child("Tarefas"))
            .child("Preparando tarefas…")
    }
}
