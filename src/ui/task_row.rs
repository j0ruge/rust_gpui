use super::*;
use crate::domain::Task;
use gpui_component::checkbox::Checkbox;

impl TaskScreen {
    /// Monta linha de altura fixa com título limitado e ações disponíveis na largura da lista.
    pub(super) fn render_task_row(&mut self, task: Task, cx: &mut Context<Self>) -> AnyElement {
        let id = task.id();
        let completed = task.status() == TaskStatus::Completed;
        let disabled = self.state.busy || self.has_dialog();
        div()
            .id(("task", id.get() as u64))
            .w_full()
            .min_w_0()
            .h(px(64.))
            .px_3()
            .flex()
            .items_center()
            .gap_3()
            .border_b_1()
            .border_color(rgb(0x303744))
            .when(self.selected == Some(id), |row| row.bg(rgb(0x29364b)))
            .child(
                Checkbox::new(("status", id.get() as u64))
                    .checked(completed)
                    .disabled(disabled)
                    .on_click(cx.listener(move |this, checked, window, cx| {
                        this.dispatch(
                            TaskCommand::SetStatus(
                                id,
                                if *checked {
                                    TaskStatus::Completed
                                } else {
                                    TaskStatus::Pending
                                },
                            ),
                            window,
                            cx,
                        );
                    })),
            )
            .child(
                div()
                    .id(("title", id.get() as u64))
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.selected = Some(id);
                        this.list_focus.focus(window);
                        cx.notify();
                    }))
                    .child(
                        div()
                            .text_sm()
                            .truncate()
                            .when(completed, |text| {
                                text.line_through().text_color(rgb(0xa2adba))
                            })
                            .child(task.title().as_str().to_owned()),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x8793a3))
                            .child(if completed { "Concluída" } else { "Pendente" }),
                    ),
            )
            .child(
                Button::new(("edit", id.get() as u64))
                    .ghost()
                    .label("Editar")
                    .disabled(disabled)
                    .on_click(cx.listener(move |this, _, window, cx| this.edit(id, window, cx))),
            )
            .child(
                Button::new(("delete", id.get() as u64))
                    .ghost()
                    .label("Excluir")
                    .disabled(disabled)
                    .on_click(
                        cx.listener(move |this, _, window, cx| this.ask_delete(id, window, cx)),
                    ),
            )
            .into_any_element()
    }
}
