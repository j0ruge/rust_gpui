use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
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
pub struct TaskId(i64);
impl TaskId {
    pub fn new(value: i64) -> Result<Self, DomainError> {
        if value <= 0 {
            return Err(DomainError::InvalidId);
        }
        Ok(Self(value))
    }
    pub fn get(self) -> i64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskTitle(String);
impl TaskTitle {
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
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Completed,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskFilter {
    All,
    Pending,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    id: TaskId,
    title: TaskTitle,
    status: TaskStatus,
}
impl Task {
    pub fn restore(id: TaskId, title: TaskTitle, status: TaskStatus) -> Self {
        Self { id, title, status }
    }
    pub fn rename(&mut self, title: TaskTitle) {
        self.title = title;
    }
    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
    }
    pub fn id(&self) -> TaskId {
        self.id
    }
    pub fn title(&self) -> &TaskTitle {
        &self.title
    }
    pub fn status(&self) -> TaskStatus {
        self.status
    }
}
impl TaskFilter {
    pub fn matches(self, task: &Task) -> bool {
        match self {
            Self::All => true,
            Self::Pending => task.status == TaskStatus::Pending,
            Self::Completed => task.status == TaskStatus::Completed,
        }
    }
}
