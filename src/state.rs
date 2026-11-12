use crate::{
    domain::{Task, TaskFilter, TaskId, TaskStatus},
    error::AppError,
    storage::TaskCommand,
};

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
    pub fn update_draft(&mut self, draft: String) {
        if self.draft != draft {
            self.saved = false;
        }
        self.draft = draft;
    }
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
    pub fn begin(&mut self, command: TaskCommand) -> Option<TaskCommand> {
        if self.busy {
            return None;
        }
        self.busy = true;
        self.saved = false;
        self.error = None;
        self.pending_command = Some(command.clone());
        Some(command)
    }
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
    pub fn visible_tasks(&self) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|task| self.filter.matches(task))
            .collect()
    }
    pub fn pending_count(&self) -> usize {
        self.tasks
            .iter()
            .filter(|task| task.status() == TaskStatus::Pending)
            .count()
    }
    pub fn request_close(&mut self) -> bool {
        self.close_requested = true;
        self.can_close()
    }
    pub fn can_close(&self) -> bool {
        self.close_requested && !self.busy && self.pending_command.is_none()
    }
    pub fn exit_without_saving(&mut self) {
        if !self.busy {
            self.pending_command = None;
            self.error = None;
            self.close_requested = true;
        }
    }
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
