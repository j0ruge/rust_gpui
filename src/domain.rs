use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
/// Violações das invariantes de título e identidade.
pub enum DomainError {
    #[error("Informe um título para a tarefa.")]
    EmptyTitle,
    #[error("O título deve ter no máximo 200 caracteres.")]
    TitleTooLong,
    #[error("O título não pode conter tabulações, quebras de linha ou controles.")]
    ControlCharacter,
    #[error("A identidade da tarefa deve ser positiva.")]
    InvalidId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Identidade positiva, estável e independente do título da tarefa.
pub struct TaskId(i64);
impl TaskId {
    /// Recusa valores não positivos antes de construir a identidade.
    pub fn new(value: i64) -> Result<Self, DomainError> {
        if value <= 0 {
            return Err(DomainError::InvalidId);
        }
        Ok(Self(value))
    }
    /// Expõe o valor validado para persistência e identificação na interface.
    pub fn get(self) -> i64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Título normalizado, não vazio e com até 200 escalares Unicode sem controles.
pub struct TaskTitle(String);
impl TaskTitle {
    /// Recusa controles antes do trim, remove espaços das bordas e valida o limite.
    pub fn parse(raw: &str) -> Result<Self, DomainError> {
        if raw.chars().any(char::is_control) {
            return Err(DomainError::ControlCharacter);
        }
        let title = raw.trim();
        if title.is_empty() {
            return Err(DomainError::EmptyTitle);
        }
        if title.chars().count() > 200 {
            return Err(DomainError::TitleTooLong);
        }
        Ok(Self(title.to_owned()))
    }
    /// Retorna o título já validado e normalizado.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Estado confirmado da tarefa, persistido como pending ou completed.
pub enum TaskStatus {
    Pending,
    Completed,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Critério de visualização que não modifica as tarefas.
pub enum TaskFilter {
    All,
    Pending,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Tarefa confirmada com identidade, título validado e estado.
pub struct Task {
    id: TaskId,
    title: TaskTitle,
    status: TaskStatus,
}
impl Task {
    /// Reconstrói uma tarefa usando identidade e título previamente validados.
    pub fn restore(id: TaskId, title: TaskTitle, status: TaskStatus) -> Self {
        Self { id, title, status }
    }
    /// Troca o título validado, preservando identidade e estado.
    pub fn rename(&mut self, title: TaskTitle) {
        self.title = title;
    }
    /// Define o estado sem alterar identidade ou título.
    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
    }
    /// Retorna a identidade estável.
    pub fn id(&self) -> TaskId {
        self.id
    }
    /// Retorna o título validado.
    pub fn title(&self) -> &TaskTitle {
        &self.title
    }
    /// Retorna o estado atual.
    pub fn status(&self) -> TaskStatus {
        self.status
    }
}
impl TaskFilter {
    /// Indica se a tarefa pertence a esta visualização.
    pub fn matches(self, task: &Task) -> bool {
        match self {
            Self::All => true,
            Self::Pending => task.status == TaskStatus::Pending,
            Self::Completed => task.status == TaskStatus::Completed,
        }
    }
}
