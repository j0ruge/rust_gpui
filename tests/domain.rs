use rust_gpui::domain::{Task, TaskFilter, TaskId, TaskStatus, TaskTitle};

#[test]
fn normalizes_edges_and_preserves_unicode_and_internal_spaces() {
    assert_eq!(
        TaskTitle::parse("  Comprar café ☕  ").unwrap().as_str(),
        "Comprar café ☕"
    );
    assert_eq!(
        TaskTitle::parse("  uma  outra ").unwrap().as_str(),
        "uma  outra"
    );
}

#[test]
fn rejects_empty_and_control_characters() {
    for title in [
        "",
        "   ",
        "uma\noutra",
        "uma\toutra",
        "\tuma",
        "uma\0outra",
        "uma\u{7f}",
    ] {
        assert!(TaskTitle::parse(title).is_err(), "Título inválido aceito");
    }
}

#[test]
fn bounds_title_by_unicode_scalars_instead_of_bytes() {
    assert!(TaskTitle::parse(&"é".repeat(200)).is_ok());
    assert!(TaskTitle::parse(&"é".repeat(201)).is_err());
    assert!(TaskTitle::parse(&"☕".repeat(200)).is_ok());
}

#[test]
fn requires_positive_identity() {
    assert!(TaskId::new(0).is_err());
    assert!(TaskId::new(-1).is_err());
    assert_eq!(TaskId::new(i64::MAX).unwrap().get(), i64::MAX);
}

#[test]
fn rename_preserves_identity_and_status() {
    let id = TaskId::new(7).unwrap();
    let mut task = Task::restore(
        id,
        TaskTitle::parse("Antes").unwrap(),
        TaskStatus::Completed,
    );
    task.rename(TaskTitle::parse("Depois").unwrap());
    assert_eq!(task.id(), id);
    assert_eq!(task.title().as_str(), "Depois");
    assert_eq!(task.status(), TaskStatus::Completed);
}

#[test]
fn completing_and_reopening_are_idempotent_and_filters_are_exact() {
    let mut task = Task::restore(
        TaskId::new(1).unwrap(),
        TaskTitle::parse("Uma").unwrap(),
        TaskStatus::Pending,
    );
    assert!(TaskFilter::All.matches(&task));
    assert!(TaskFilter::Pending.matches(&task));
    assert!(!TaskFilter::Completed.matches(&task));
    for _ in 0..2 {
        task.set_status(TaskStatus::Completed);
    }
    assert!(TaskFilter::All.matches(&task));
    assert!(!TaskFilter::Pending.matches(&task));
    assert!(TaskFilter::Completed.matches(&task));
    for _ in 0..2 {
        task.set_status(TaskStatus::Pending);
    }
    assert!(TaskFilter::Pending.matches(&task));
}
