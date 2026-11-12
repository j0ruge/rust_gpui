use crate::{
    domain::{Task, TaskFilter, TaskId, TaskStatus, TaskTitle},
    error::AppError,
    storage::TaskCommand,
};

/// Estado de apresentação; o snapshot só muda após uma escrita confirmada.
pub struct TaskScreenState {
    pub tasks: Vec<Task>,
    pub filter: TaskFilter,
    pub draft: String,
    pub editing: Option<TaskId>,
    pub delete_confirmation: Option<TaskId>,
    pub busy: bool,
    pub pending_command: Option<TaskCommand>,
    pub error: Option<AppError>,
    pub close_requested: bool,
    pub saved: bool,
}
impl TaskScreenState {
    /// Atualiza o rascunho; eco idêntico do input preserva o feedback Salvo.
    pub fn update_draft(&mut self, draft: String) {
        if self.draft != draft {
            self.saved = false;
        }
        self.draft = draft;
    }
    /// Inicia a tela com snapshot confirmado, filtro Todas e formulário vazio.
    pub fn new(tasks: Vec<Task>) -> Self {
        Self {
            tasks,
            filter: TaskFilter::All,
            draft: String::new(),
            editing: None,
            delete_confirmation: None,
            busy: false,
            pending_command: None,
            error: None,
            close_requested: false,
            saved: false,
        }
    }
    /// Valida o título antes de reservar a escrita; recusa reentrada enquanto busy.
    pub fn begin(&mut self, command: TaskCommand) -> Option<TaskCommand> {
        if self.busy {
            return None;
        }
        if let TaskCommand::Create(raw) | TaskCommand::Rename(_, raw) = &command
            && let Err(error) = TaskTitle::parse(raw)
        {
            self.saved = false;
            self.error = Some(error.into());
            self.pending_command = None;
            // Validação pede correção do rascunho, não confirma intenção de sair sem salvar.
            self.close_requested = false;
            return None;
        }
        self.busy = true;
        self.saved = false;
        self.error = None;
        self.pending_command = Some(command.clone());
        Some(command)
    }
    /// Aplica o resultado da escrita em andamento; falha preserva snapshot, rascunho e retry.
    pub fn finish(&mut self, result: Result<Vec<Task>, AppError>) {
        if !self.busy {
            return;
        }
        self.busy = false;
        match result {
            Err(error) => {
                self.error = Some(error);
            }
            Ok(tasks) => {
                match self.pending_command.take() {
                    Some(TaskCommand::Create(_)) => {
                        self.draft.clear();
                    }
                    Some(TaskCommand::Rename(id, _)) if self.editing == Some(id) => {
                        self.draft.clear();
                        self.editing = None;
                    }
                    Some(TaskCommand::Delete(id)) => {
                        if self.delete_confirmation == Some(id) {
                            self.delete_confirmation = None;
                        }
                        if self.editing == Some(id) {
                            self.draft.clear();
                            self.editing = None;
                        }
                    }
                    _ => {}
                }
                self.tasks = tasks;
                self.error = None;
                self.saved = true;
            }
        }
    }
    /// Filtra o snapshot mantendo a ordem de criação.
    pub fn visible_tasks(&self) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| self.filter.matches(task))
            .collect()
    }
    /// Conta pendentes em todo o snapshot, independentemente do filtro.
    pub fn pending_count(&self) -> usize {
        self.tasks
            .iter()
            .filter(|task| task.status() == TaskStatus::Pending)
            .count()
    }
    /// Registra a intenção de sair e informa se não há escrita a resolver.
    pub fn request_close(&mut self) -> bool {
        self.close_requested = true;
        self.can_close()
    }
    /// Permite fechar quando solicitado, sem escrita ativa ou comando pendente.
    pub fn can_close(&self) -> bool {
        self.close_requested && !self.busy && self.pending_command.is_none()
    }
    /// Descarta a intenção que falhou e autoriza fechar; não interrompe uma escrita ativa.
    pub fn exit_without_saving(&mut self) {
        if !self.busy {
            self.pending_command = None;
            self.error = None;
            self.close_requested = true;
        }
    }
    /// Descarta formulário e retry fora de busy, preservando as tarefas confirmadas.
    pub fn cancel_edit(&mut self) {
        if self.busy {
            return;
        }
        self.editing = None;
        self.draft.clear();
        self.error = None;
        self.pending_command = None;
    }
}
