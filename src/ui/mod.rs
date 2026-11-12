mod task_row;

use crate::{
    config::resolve_data_dir,
    domain::{TaskFilter, TaskId, TaskStatus},
    error::AppError,
    state::TaskScreenState,
    storage::{TaskCommand, TaskRepository},
};
use gpui::{prelude::*, *};
use gpui_component::{
    Disableable, Selectable, StyledExt,
    button::{Button, ButtonVariants},
    input::{Input, InputEvent, InputState},
};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

actions!(
    tasks,
    [
        NewTask,
        AllTasks,
        PendingTasks,
        CompletedTasks,
        Quit,
        CancelEdit,
        DialogNext
    ]
);

/// Registra atalhos locais da tela e navegação dos diálogos.
pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("ctrl-n", NewTask, Some("TaskScreen")),
        KeyBinding::new("ctrl-1", AllTasks, Some("TaskScreen")),
        KeyBinding::new("ctrl-2", PendingTasks, Some("TaskScreen")),
        KeyBinding::new("ctrl-3", CompletedTasks, Some("TaskScreen")),
        KeyBinding::new("ctrl-q", Quit, Some("TaskScreen")),
        KeyBinding::new("escape", CancelEdit, Some("TaskScreen")),
        KeyBinding::new("tab", DialogNext, Some("TaskDialog")),
        KeyBinding::new("shift-tab", DialogNext, Some("TaskDialog")),
    ]);
}

/// Agenda saída em outro ciclo para evitar reentrada no backend X11.
fn schedule_quit(cx: &mut App) {
    cx.spawn(async |cx| {
        let _ = cx.update(|cx| cx.quit());
    })
    .detach();
}

/// Adaptador nativo entre estado de apresentação, input, foco e jobs de persistência.
pub struct TaskScreen {
    state: TaskScreenState,
    input: Entity<InputState>,
    repository: Option<Arc<Mutex<TaskRepository>>>,
    data_dir: Option<PathBuf>,
    load_error: Option<AppError>,
    loading: bool,
    selected: Option<TaskId>,
    screen_focus: FocusHandle,
    list_focus: FocusHandle,
    cancel_focus: FocusHandle,
    confirm_focus: FocusHandle,
    previous_focus: Option<FocusHandle>,
    scroll: UniformListScrollHandle,
    _subscriptions: Vec<Subscription>,
    job: Option<gpui::Task<()>>,
}

impl TaskScreen {
    /// Cria input e assinaturas persistentes e inicia a carga fora da thread da janela.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input =
            cx.new(|cx| InputState::new(window, cx).placeholder("O que você precisa fazer?"));
        let subscription =
            cx.subscribe_in(
                &input,
                window,
                |this, input, event, window, cx| match event {
                    InputEvent::Change if !this.state.busy => {
                        this.state.update_draft(input.read(cx).value().to_string());
                        cx.notify();
                    }
                    InputEvent::PressEnter { .. } => this.submit(window, cx),
                    _ => {}
                },
            );
        let override_dir = std::env::var_os("RUST_GPUI_DATA_DIR").map(PathBuf::from);
        let xdg = std::env::var_os("XDG_DATA_HOME").map(PathBuf::from);
        let home = std::env::var_os("HOME").map(PathBuf::from);
        let resolved = resolve_data_dir(override_dir.as_deref(), xdg.as_deref(), home.as_deref());
        let (data_dir, load_error) = match resolved {
            Ok(path) => (Some(path), None),
            Err(error) => (None, Some(error)),
        };
        let mut this = Self {
            state: TaskScreenState::new(vec![]),
            input,
            repository: None,
            data_dir,
            load_error,
            loading: false,
            selected: None,
            screen_focus: cx.focus_handle(),
            list_focus: cx.focus_handle().tab_stop(true),
            cancel_focus: cx.focus_handle().tab_stop(true),
            confirm_focus: cx.focus_handle().tab_stop(true),
            previous_focus: None,
            scroll: UniformListScrollHandle::new(),
            _subscriptions: vec![subscription],
            job: None,
        };
        let weak = cx.weak_entity();
        window.on_window_should_close(cx, move |window, cx| {
            weak.update(cx, |this, cx| this.request_close(window, cx))
                .unwrap_or(true)
        });
        this.load(window, cx);
        this
    }

    /// Expõe o caminho dos dados ou orienta a resolução quando ele é inválido.
    fn path_label(&self) -> String {
        self.data_dir.as_ref().map(|path| path.display().to_string()).unwrap_or_else(|| "Diretório de dados não resolvido. Confira RUST_GPUI_DATA_DIR, XDG_DATA_HOME e HOME.".into())
    }

    /// Carrega uma vez por tentativa; falhas ficam visíveis sem substituir o banco.
    fn load(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(path) = self.data_dir.clone() else {
            return;
        };
        if self.loading {
            return;
        }
        self.loading = true;
        self.load_error = None;
        let work = cx.background_executor().spawn(async move {
            let repo = TaskRepository::open(&path)?;
            let tasks = repo.list()?;
            Ok::<_, AppError>((Arc::new(Mutex::new(repo)), tasks))
        });
        self.job = Some(cx.spawn_in(window, async move |this, cx| {
            let result = work.await;
            let _ = this.update_in(cx, |this, window, cx| {
                this.loading = false;
                match result {
                    Ok((repo, tasks)) => {
                        this.repository = Some(repo);
                        this.state = TaskScreenState::new(tasks);
                        this.focus_input(window, cx);
                    }
                    Err(error) => {
                        eprintln!("Falha ao abrir tarefas: {error:?}");
                        this.load_error = Some(error);
                    }
                }
                cx.notify();
            });
        }));
        cx.notify();
    }

    /// Foca o campo persistente da tela.
    fn focus_input(&self, window: &mut Window, cx: &mut Context<Self>) {
        self.input.update(cx, |input, cx| input.focus(window, cx));
    }

    /// Sincroniza o widget somente quando seu valor difere do rascunho.
    fn sync_input(&self, window: &mut Window, cx: &mut Context<Self>) {
        let value = self.state.draft.clone();
        if self.input.read(cx).value().as_ref() != value.as_str() {
            self.input
                .update(cx, |input, cx| input.set_value(value, window, cx));
        }
    }

    /// Identifica confirmação de exclusão ou decisão de saída após falha.
    fn has_dialog(&self) -> bool {
        self.state.delete_confirmation.is_some()
            || (self.state.close_requested && self.state.error.is_some())
    }

    /// Transforma o formulário em criação ou edição quando a tela permite.
    fn submit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.has_dialog() || self.repository.is_none() {
            return;
        }
        let command = match self.state.editing {
            Some(id) => TaskCommand::Rename(id, self.state.draft.clone()),
            None => TaskCommand::Create(self.state.draft.clone()),
        };
        self.dispatch(command, window, cx);
    }

    /// Executa escrita serializada em background e reconcilia seleção, formulário e fechamento.
    fn dispatch(&mut self, command: TaskCommand, window: &mut Window, cx: &mut Context<Self>) {
        let Some(repository) = self.repository.clone() else {
            return;
        };
        let Some(command) = self.state.begin(command) else {
            return;
        };
        let work = cx.background_executor().spawn(async move {
            repository
                .lock()
                .map_err(|_| AppError::StorageUnavailable)?
                .apply(command)
        });
        self.job = Some(cx.spawn_in(window, async move |this, cx| {
            let result = work.await;
            let _ = this.update_in(cx, |this, window, cx| {
                if let Err(error) = &result {
                    eprintln!("Falha na operação: {error:?}");
                }
                let was_dialog = this.has_dialog();
                this.state.finish(result);
                this.selected = this.selected.filter(|id| {
                    this.state
                        .visible_tasks()
                        .iter()
                        .any(|task| task.id() == *id)
                });
                this.sync_input(window, cx);
                if this.state.can_close() {
                    schedule_quit(cx);
                } else if this.state.close_requested && this.state.error.is_some() {
                    this.cancel_focus.focus(window);
                } else if was_dialog && !this.has_dialog() {
                    this.restore_focus(window, cx);
                }
                cx.notify();
            });
        }));
        cx.notify();
    }

    /// Repete a intenção falha usando o rascunho atual em criação ou edição.
    fn retry(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(command) = self.state.pending_command.clone() {
            let command = match command {
                TaskCommand::Create(_) => TaskCommand::Create(self.state.draft.clone()),
                TaskCommand::Rename(id, _) => TaskCommand::Rename(id, self.state.draft.clone()),
                command => command,
            };
            self.dispatch(command, window, cx);
        }
    }

    /// Abandona edição ativa e foca nova tarefa fora de busy e de diálogos.
    fn new_task(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.state.busy || self.has_dialog() {
            return;
        }
        if self.state.editing.is_some() {
            self.state.cancel_edit();
            self.sync_input(window, cx);
        }
        self.focus_input(window, cx);
        cx.notify();
    }

    /// Carrega o título confirmado no formulário, fora de busy e de diálogos.
    fn edit(&mut self, id: TaskId, window: &mut Window, cx: &mut Context<Self>) {
        if self.state.busy || self.has_dialog() {
            return;
        }
        if let Some(task) = self.state.tasks.iter().find(|task| task.id() == id) {
            self.state.draft = task.title().as_str().to_owned();
            self.state.editing = Some(id);
            self.state.error = None;
            self.state.pending_command = None;
            self.sync_input(window, cx);
            self.focus_input(window, cx);
            cx.notify();
        }
    }

    /// Abre confirmação preservando o foco anterior e focando Cancelar.
    fn ask_delete(&mut self, id: TaskId, window: &mut Window, cx: &mut Context<Self>) {
        if self.state.busy || self.has_dialog() {
            return;
        }
        self.previous_focus = window.focused(cx);
        self.state.delete_confirmation = Some(id);
        self.cancel_focus.focus(window);
        cx.notify();
    }

    /// Restaura o foco anterior ao diálogo ou volta ao campo de entrada.
    fn restore_focus(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(focus) = self.previous_focus.take() {
            focus.focus(window);
        } else {
            self.focus_input(window, cx);
        }
    }

    /// Fecha diálogo ou abandona edição; escrita em andamento não é interrompida.
    fn cancel(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.state.busy {
            return;
        }
        if self.has_dialog() {
            self.state.delete_confirmation = None;
            self.state.close_requested = false;
            self.restore_focus(window, cx);
        } else if self.state.editing.is_some() {
            self.state.cancel_edit();
            self.sync_input(window, cx);
            self.focus_input(window, cx);
        }
        cx.notify();
    }

    /// Adia fechamento até resolver escrita ou decisão após falha.
    fn request_close(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        if self.state.request_close() {
            return true;
        }
        if self.state.error.is_some() {
            self.previous_focus = window.focused(cx);
            self.cancel_focus.focus(window);
        }
        cx.notify();
        false
    }

    /// Muda visualização e limpa seleção/rolagem, sem alterar os dados.
    fn set_filter(&mut self, filter: TaskFilter, cx: &mut Context<Self>) {
        if self.has_dialog() {
            return;
        }
        self.state.filter = filter;
        self.selected = None;
        self.scroll.scroll_to_item(0, ScrollStrategy::Top);
        cx.notify();
    }

    /// Navega a lista e só permite ações por teclado sobre uma seleção visível.
    fn list_key(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        let tasks = self.state.visible_tasks();
        if tasks.is_empty() {
            return;
        }
        let index = self
            .selected
            .and_then(|id| tasks.iter().position(|task| task.id() == id));
        let next = match event.keystroke.key.as_str() {
            "down" => Some(index.map_or(0, |i| (i + 1).min(tasks.len() - 1))),
            "up" => Some(index.map_or(0, |i| i.saturating_sub(1))),
            "home" => Some(0),
            "end" => Some(tasks.len() - 1),
            _ => None,
        };
        if let Some(index) = next {
            self.selected = Some(tasks[index].id());
            self.list_focus.focus(window);
            self.scroll.scroll_to_item(index, ScrollStrategy::Center);
            cx.stop_propagation();
            window.prevent_default();
            cx.notify();
        } else if self.list_focus.is_focused(window)
            && let Some(index) = index
        {
            let id = tasks[index].id();
            match event.keystroke.key.as_str() {
                "enter" => self.edit(id, window, cx),
                "delete" => self.ask_delete(id, window, cx),
                "space" => {
                    let status = tasks
                        .iter()
                        .find(|task| task.id() == id)
                        .map(|task| task.status());
                    if let Some(status) = status {
                        self.dispatch(
                            TaskCommand::SetStatus(
                                id,
                                if status == TaskStatus::Pending {
                                    TaskStatus::Completed
                                } else {
                                    TaskStatus::Pending
                                },
                            ),
                            window,
                            cx,
                        );
                    }
                }
                _ => return,
            }
            cx.stop_propagation();
            window.prevent_default();
        }
    }

    /// Cancela exclusão ou tenta salvar novamente na decisão de saída.
    fn dialog_cancel(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.state.busy {
            return;
        }
        if self.state.close_requested {
            self.retry(window, cx);
        } else {
            self.cancel(window, cx);
        }
    }

    /// Confirma exclusão ou sai abandonando a intenção falha.
    fn dialog_confirm(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.state.busy {
            return;
        }
        if self.state.close_requested {
            self.state.exit_without_saving();
            schedule_quit(cx);
        } else if let Some(id) = self.state.delete_confirmation {
            self.dispatch(TaskCommand::Delete(id), window, cx);
        }
    }

    /// Monta o diálogo com foco explícito; GPUI ativa on_click por Enter/Espaço.
    fn render_dialog(&self, cx: &mut Context<Self>) -> AnyElement {
        let closing = self.state.close_requested && self.state.error.is_some();
        let title = if closing {
            "Não foi possível salvar"
        } else {
            "Excluir tarefa?"
        };
        let detail = if closing {
            "Sua última ação não foi salva. Tente novamente ou saia sem salvar essa ação."
                .to_owned()
        } else {
            self.state
                .tasks
                .iter()
                .find(|task| Some(task.id()) == self.state.delete_confirmation)
                .map(|task| task.title().as_str().to_owned())
                .unwrap_or_default()
        };
        let cancel_focus = self.cancel_focus.clone();
        let confirm_focus = self.confirm_focus.clone();
        div()
            .id("dialog-overlay")
            .absolute()
            .inset_0()
            .bg(rgba(0x000000b0))
            .flex()
            .items_center()
            .justify_center()
            .occlude()
            .child(
                div()
                    .id("task-dialog")
                    .key_context("TaskDialog")
                    .tab_group()
                    .w_full()
                    .max_w(px(480.))
                    .mx_4()
                    .p_6()
                    .rounded_lg()
                    .border_1()
                    .border_color(rgb(0x4c5666))
                    .bg(rgb(0x22272f))
                    .flex()
                    .flex_col()
                    .gap_4()
                    .on_action(cx.listener(move |_, _: &DialogNext, window, _| {
                        if cancel_focus.is_focused(window) {
                            confirm_focus.focus(window);
                        } else {
                            cancel_focus.focus(window);
                        }
                    }))
                    .child(div().text_xl().child(title))
                    .child(div().text_sm().child(detail))
                    .children(self.state.error.as_ref().map(|error| {
                        div()
                            .text_sm()
                            .text_color(rgb(0xf5aaa0))
                            .child(error.to_string())
                    }))
                    .child(
                        div()
                            .flex()
                            .justify_end()
                            .gap_3()
                            .child(
                                div()
                                    .id("dialog-cancel-control")
                                    .track_focus(&self.cancel_focus)
                                    .rounded_md()
                                    .border_2()
                                    .border_color(rgb(0x677489))
                                    .focus(|style| style.border_color(rgb(0x91baff)))
                                    .on_click(cx.listener(Self::dialog_cancel_click))
                                    .child(
                                        Button::new("dialog-cancel")
                                            .tab_stop(false)
                                            .label(if closing {
                                                "Tentar novamente"
                                            } else {
                                                "Cancelar"
                                            })
                                            .disabled(self.state.busy),
                                    ),
                            )
                            .child(
                                div()
                                    .id("dialog-confirm-control")
                                    .track_focus(&self.confirm_focus)
                                    .rounded_md()
                                    .border_2()
                                    .border_color(rgb(0x677489))
                                    .focus(|style| style.border_color(rgb(0x91baff)))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.dialog_confirm(window, cx)
                                    }))
                                    .child(
                                        Button::new("dialog-confirm")
                                            .tab_stop(false)
                                            .danger()
                                            .label(if closing {
                                                "Sair sem salvar"
                                            } else {
                                                "Excluir"
                                            })
                                            .disabled(self.state.busy),
                                    ),
                            ),
                    ),
            )
            .into_any_element()
    }

    /// Encaminha clique nativo ou de teclado para a ação segura do diálogo.
    fn dialog_cancel_click(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        self.dialog_cancel(window, cx);
    }
}

impl Render for TaskScreen {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut root =
            div()
                .id("task-screen")
                .key_context("TaskScreen")
                .track_focus(&self.screen_focus)
                .relative()
                .flex()
                .flex_col()
                .size_full()
                .p_6()
                .gap_4()
                .bg(rgb(0x181b20))
                .text_color(rgb(0xe8e9ec))
                .on_action(cx.listener(|this, _: &NewTask, window, cx| this.new_task(window, cx)))
                .on_action(
                    cx.listener(|this, _: &AllTasks, _, cx| this.set_filter(TaskFilter::All, cx)),
                )
                .on_action(cx.listener(|this, _: &PendingTasks, _, cx| {
                    this.set_filter(TaskFilter::Pending, cx)
                }))
                .on_action(cx.listener(|this, _: &CompletedTasks, _, cx| {
                    this.set_filter(TaskFilter::Completed, cx)
                }))
                .on_action(cx.listener(|this, _: &CancelEdit, window, cx| this.cancel(window, cx)))
                .on_action(cx.listener(|this, _: &Quit, window, cx| {
                    if this.request_close(window, cx) {
                        schedule_quit(cx);
                    }
                }))
                .child(
                    div()
                        .flex()
                        .items_baseline()
                        .justify_between()
                        .child(div().text_2xl().font_semibold().child("Tarefas"))
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(0xa2adba))
                                .child("Salvamento local automático"),
                        ),
                );
        if self.loading {
            return root.child("Preparando tarefas…");
        }
        if let Some(error) = &self.load_error {
            let already_running = matches!(error, AppError::AlreadyRunning);
            return root.child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .p_4()
                    .border_1()
                    .border_color(rgb(0x985f59))
                    .rounded_md()
                    .child(div().text_lg().child(error.to_string()))
                    .child(div().text_sm().child(self.path_label()))
                    .child(
                        div()
                            .flex()
                            .gap_3()
                            .when(!already_running, |row| {
                                row.child(
                                    Button::new("retry-open")
                                        .label("Tentar novamente")
                                        .on_click(
                                            cx.listener(|this, _, window, cx| {
                                                this.load(window, cx)
                                            }),
                                        ),
                                )
                            })
                            .child(
                                Button::new("exit")
                                    .label("Sair")
                                    .on_click(|_, _, cx| schedule_quit(cx)),
                            ),
                    ),
            );
        }
        let dialog = self.has_dialog();
        let disabled = self.state.busy || dialog;
        root = root.child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .flex_shrink_0()
                .child(div().text_sm().child(if self.state.editing.is_some() {
                    "Editar tarefa"
                } else {
                    "Nova tarefa"
                }))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .flex_1()
                                .min_w_0()
                                .child(Input::new(&self.input).disabled(disabled)),
                        )
                        .child(
                            Button::new("submit")
                                .primary()
                                .label(if self.state.editing.is_some() {
                                    "Salvar"
                                } else {
                                    "Adicionar"
                                })
                                .disabled(disabled)
                                .on_click(
                                    cx.listener(|this, _, window, cx| this.submit(window, cx)),
                                ),
                        )
                        .when(self.state.editing.is_some(), |row| {
                            row.child(
                                Button::new("cancel-edit")
                                    .label("Cancelar")
                                    .disabled(disabled)
                                    .on_click(
                                        cx.listener(|this, _, window, cx| this.cancel(window, cx)),
                                    ),
                            )
                        }),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0xa2adba))
                        .child("Enter confirma · Esc cancela edição · Ctrl+N nova tarefa"),
                ),
        );
        let mut filters = div().flex().items_center().gap_2().flex_shrink_0();
        for (id, label, filter) in [
            ("all", "Todas", TaskFilter::All),
            ("pending", "Pendentes", TaskFilter::Pending),
            ("completed", "Concluídas", TaskFilter::Completed),
        ] {
            filters = filters.child(
                Button::new(id)
                    .label(label)
                    .selected(self.state.filter == filter)
                    .disabled(dialog)
                    .on_click(cx.listener(move |this, _, _, cx| this.set_filter(filter, cx))),
            );
        }
        root = root.child(filters.child(div().flex_1())).child(
            div()
                .flex()
                .justify_between()
                .text_sm()
                .text_color(rgb(0xa2adba))
                .child(format!(
                    "{} pendentes · {} tarefas",
                    self.state.pending_count(),
                    self.state.tasks.len()
                ))
                .child(if self.state.busy {
                    "Salvando…"
                } else if self.state.error.is_some() {
                    "Não salvo"
                } else if self.state.saved {
                    "Salvo"
                } else {
                    ""
                }),
        );
        if let Some(error) = &self.state.error {
            root = root.child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .flex_shrink_0()
                    .text_sm()
                    .text_color(rgb(0xf5aaa0))
                    .child(error.to_string())
                    .child(self.path_label())
                    .child(
                        Button::new("retry-write")
                            .label("Tentar novamente")
                            .disabled(disabled)
                            .on_click(cx.listener(|this, _, window, cx| this.retry(window, cx))),
                    ),
            );
        }
        let count = self.state.visible_tasks().len();
        if count == 0 {
            root = root.child(
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_color(rgb(0xa2adba))
                    .child(if self.state.tasks.is_empty() {
                        "Nenhuma tarefa ainda. Adicione a primeira acima."
                    } else {
                        "Nenhuma tarefa neste filtro."
                    }),
            );
        } else {
            let weak = cx.weak_entity();
            root = root.child(
                div()
                    .id("task-list-focus")
                    .track_focus(&self.list_focus)
                    .flex_1()
                    .min_h_0()
                    .border_1()
                    .rounded_md()
                    .border_color(rgb(0x343d4b))
                    .focus(|style| style.border_color(rgb(0x91baff)))
                    .on_key_down(cx.listener(Self::list_key))
                    .child(
                        uniform_list("tasks", count, move |range, _, cx| {
                            weak.update(cx, |this, cx| {
                                let tasks: Vec<_> = this
                                    .state
                                    .visible_tasks()
                                    .into_iter()
                                    .skip(range.start)
                                    .take(range.len())
                                    .cloned()
                                    .collect();
                                tasks
                                    .into_iter()
                                    .map(|task| this.render_task_row(task, cx))
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_default()
                        })
                        .size_full()
                        .track_scroll(self.scroll.clone()),
                    ),
            );
        }
        root = root.child(div().flex_shrink_0().text_xs().text_color(rgb(0x8793a3)).child("Ctrl+1/2/3 filtros · Na lista: ↑↓ Home/End, Enter editar, Espaço concluir, Delete excluir"));
        if dialog {
            root = root.child(self.render_dialog(cx));
        }
        root
    }
}
