use rust_gpui::{
    domain::{Task, TaskFilter, TaskId, TaskStatus, TaskTitle},
    error::AppError,
    state::TaskScreenState,
    storage::TaskCommand,
};

fn task(status: TaskStatus) -> Task {
    Task::restore(
        TaskId::new(1).unwrap(),
        TaskTitle::parse("Confirmada").unwrap(),
        status,
    )
}

#[test]
fn refuses_reentry_until_the_first_result() {
    let mut state = TaskScreenState::new(vec![]);
    let command = TaskCommand::Create("Nova".into());
    assert_eq!(state.begin(command.clone()), Some(command.clone()));
    assert!(state.busy);
    assert_eq!(state.begin(TaskCommand::Create("Outra".into())), None);
    assert_eq!(state.begin(TaskCommand::Create(String::new())), None);
    assert!(state.busy);
    assert!(state.error.is_none());
    assert_eq!(state.pending_command, Some(command));
}

#[test]
fn cancel_during_busy_preserves_the_form_and_pending_write() {
    let original = task(TaskStatus::Pending);
    let mut state = TaskScreenState::new(vec![original.clone()]);
    state.editing = Some(original.id());
    state.delete_confirmation = Some(original.id());
    state.draft = "Em andamento".into();
    let command = TaskCommand::Rename(original.id(), state.draft.clone());
    state.begin(command.clone());
    state.cancel_edit();
    assert!(state.busy);
    assert_eq!(state.tasks, vec![original.clone()]);
    assert_eq!(state.draft, "Em andamento");
    assert_eq!(state.editing, Some(original.id()));
    assert_eq!(state.delete_confirmation, Some(original.id()));
    assert_eq!(state.pending_command, Some(command));
    assert!(state.error.is_none());
    assert!(!state.close_requested);
}

#[test]
fn exit_without_saving_during_busy_cannot_abandon_the_pending_write() {
    for closing in [false, true] {
        let mut state = TaskScreenState::new(vec![]);
        state.draft = "Em andamento".into();
        let command = TaskCommand::Create(state.draft.clone());
        state.begin(command.clone());
        if closing {
            assert!(!state.request_close());
        }
        state.exit_without_saving();
        assert!(state.busy);
        assert_eq!(state.pending_command, Some(command));
        assert_eq!(state.draft, "Em andamento");
        assert_eq!(state.close_requested, closing);
        assert!(!state.can_close());
    }
}

#[test]
fn unsolicited_result_cannot_replace_snapshot_or_clear_a_failed_write() {
    for failed_write in [false, true] {
        for result in [Ok(vec![]), Err(AppError::StorageUnavailable)] {
            let original = task(TaskStatus::Pending);
            let mut state = TaskScreenState::new(vec![original.clone()]);
            state.editing = Some(original.id());
            state.draft = "Preservar".into();
            let command = TaskCommand::Rename(original.id(), state.draft.clone());
            if failed_write {
                state.begin(command.clone());
                state.finish(Err(AppError::StorageUnavailable));
            }
            state.finish(result);
            assert!(!state.busy);
            assert_eq!(state.tasks, vec![original.clone()]);
            assert_eq!(state.editing, Some(original.id()));
            assert_eq!(state.draft, "Preservar");
            assert_eq!(state.pending_command, failed_write.then_some(command));
            assert_eq!(state.error.is_some(), failed_write);
            assert!(!state.saved);
        }
    }
}

#[test]
fn failure_preserves_draft_edit_identity_snapshot_and_retry() {
    let original = task(TaskStatus::Pending);
    let mut state = TaskScreenState::new(vec![original.clone()]);
    state.editing = Some(original.id());
    state.draft = "Rascunho ☕".into();
    let command = TaskCommand::Rename(original.id(), state.draft.clone());
    state.begin(command.clone());
    state.finish(Err(AppError::StorageUnavailable));
    assert!(!state.busy);
    assert_eq!(state.tasks, vec![original.clone()]);
    assert_eq!(state.draft, "Rascunho ☕");
    assert_eq!(state.editing, Some(original.id()));
    assert_eq!(state.pending_command, Some(command));
    assert!(state.error.is_some());
}

#[test]
fn success_clears_confirmed_form_and_keeps_filter_chosen_while_busy() {
    let mut state = TaskScreenState::new(vec![]);
    state.draft = "Confirmada".into();
    state.begin(TaskCommand::Create(state.draft.clone()));
    state.filter = TaskFilter::Completed;
    state.finish(Ok(vec![task(TaskStatus::Pending)]));
    assert_eq!(state.tasks.len(), 1);
    assert!(state.draft.is_empty());
    assert_eq!(state.filter, TaskFilter::Completed);
    assert!(!state.busy);
    assert!(state.pending_command.is_none());
    assert!(state.error.is_none());
}

#[test]
fn status_success_does_not_erase_unrelated_draft() {
    let mut state = TaskScreenState::new(vec![task(TaskStatus::Pending)]);
    state.draft = "Ainda digitando".into();
    state.begin(TaskCommand::SetStatus(
        TaskId::new(1).unwrap(),
        TaskStatus::Completed,
    ));
    state.finish(Ok(vec![task(TaskStatus::Completed)]));
    assert_eq!(state.draft, "Ainda digitando");
}

#[test]
fn pending_filter_removes_completed_task_only_after_success() {
    let mut state = TaskScreenState::new(vec![task(TaskStatus::Pending)]);
    state.filter = TaskFilter::Pending;
    state.begin(TaskCommand::SetStatus(
        TaskId::new(1).unwrap(),
        TaskStatus::Completed,
    ));
    assert_eq!(state.visible_tasks().len(), 1);
    assert_eq!(state.pending_count(), 1);
    state.finish(Ok(vec![task(TaskStatus::Completed)]));
    assert!(state.visible_tasks().is_empty());
    assert_eq!(state.pending_count(), 0);
    state.filter = TaskFilter::All;
    assert_eq!(state.visible_tasks().len(), 1);
}

#[test]
fn deletion_failure_keeps_confirmation_and_success_closes_it() {
    let mut state = TaskScreenState::new(vec![task(TaskStatus::Pending)]);
    let id = TaskId::new(1).unwrap();
    state.delete_confirmation = Some(id);
    let command = TaskCommand::Delete(id);
    state.begin(command.clone());
    state.finish(Err(AppError::StorageUnavailable));
    assert_eq!(state.delete_confirmation, Some(id));
    assert_eq!(state.tasks.len(), 1);
    state.begin(command);
    state.finish(Ok(vec![]));
    assert!(state.delete_confirmation.is_none());
    assert!(state.tasks.is_empty());
}

#[test]
fn close_waits_for_commit_and_failure_requires_explicit_decision() {
    let mut state = TaskScreenState::new(vec![]);
    let command = TaskCommand::Create("Nova".into());
    state.begin(command.clone());
    assert!(!state.request_close());
    state.finish(Err(AppError::StorageUnavailable));
    assert!(!state.can_close());
    state.begin(command);
    state.finish(Ok(vec![task(TaskStatus::Pending)]));
    assert!(state.can_close());
}

#[test]
fn can_choose_to_exit_after_failed_save() {
    let mut state = TaskScreenState::new(vec![]);
    state.begin(TaskCommand::Create("Nova".into()));
    state.request_close();
    state.finish(Err(AppError::StorageUnavailable));
    state.exit_without_saving();
    assert!(state.can_close());
}

#[test]
fn cancel_edit_does_not_generate_a_command() {
    let mut state = TaskScreenState::new(vec![task(TaskStatus::Pending)]);
    state.editing = Some(TaskId::new(1).unwrap());
    state.draft = "Mudança descartada".into();
    state.cancel_edit();
    assert_eq!(state.tasks[0].title().as_str(), "Confirmada");
    assert!(state.editing.is_none());
    assert!(state.draft.is_empty());
    assert!(state.pending_command.is_none());
}

#[test]
fn confirmed_input_echo_keeps_saved_feedback_but_new_typing_clears_it() {
    let mut state = TaskScreenState::new(vec![]);
    state.draft = "Confirmada".into();
    state.begin(TaskCommand::Create(state.draft.clone()));
    state.finish(Ok(vec![task(TaskStatus::Pending)]));
    state.update_draft(String::new());
    assert!(state.saved);
    state.update_draft("Próxima".into());
    assert!(!state.saved);
}

#[test]
fn invalid_titles_never_start_a_write_or_require_a_save_decision_on_close() {
    for raw in [String::new(), " ".into(), "é".repeat(201)] {
        for editing in [false, true] {
            let original = task(TaskStatus::Pending);
            let mut state = TaskScreenState::new(vec![original.clone()]);
            state.draft = raw.clone();
            let command = if editing {
                state.editing = Some(original.id());
                TaskCommand::Rename(original.id(), raw.clone())
            } else {
                TaskCommand::Create(raw.clone())
            };
            assert_eq!(state.begin(command), None);
            assert!(!state.busy);
            assert!(state.pending_command.is_none());
            assert!(matches!(state.error, Some(AppError::Validation(_))));
            assert_eq!(state.draft, raw);
            assert_eq!(state.tasks, vec![original]);
            assert!(state.request_close());
        }
    }
}

#[test]
fn invalid_retry_after_a_failed_write_releases_an_already_requested_close() {
    for editing in [false, true] {
        let original = task(TaskStatus::Pending);
        let mut state = TaskScreenState::new(vec![original.clone()]);
        state.draft = "Antes".into();
        let command = if editing {
            state.editing = Some(original.id());
            TaskCommand::Rename(original.id(), state.draft.clone())
        } else {
            TaskCommand::Create(state.draft.clone())
        };
        state.begin(command);
        state.finish(Err(AppError::StorageUnavailable));
        state.update_draft(String::new());
        assert!(!state.request_close());
        let retry = if editing {
            TaskCommand::Rename(original.id(), state.draft.clone())
        } else {
            TaskCommand::Create(state.draft.clone())
        };
        assert_eq!(state.begin(retry), None);
        assert!(state.can_close());
        assert!(!state.busy);
        assert!(state.pending_command.is_none());
        assert!(matches!(state.error, Some(AppError::Validation(_))));
        assert_eq!(state.tasks, vec![original]);
    }
}
