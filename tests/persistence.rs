use rusqlite::Connection;
use rust_gpui::{
    domain::{TaskId, TaskStatus},
    error::AppError,
    storage::{TaskCommand, TaskRepository},
};
use std::fs;
use tempfile::tempdir;

const SCHEMA: &str = "CREATE TABLE tasks (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT NOT NULL, status TEXT NOT NULL CHECK (status IN ('pending', 'completed'))); PRAGMA user_version=1;";

#[test]
fn round_trip_keeps_identity_status_order_and_deletion() {
    let dir = tempdir().unwrap();
    let mut repo = TaskRepository::open(dir.path()).unwrap();
    assert!(repo.list().unwrap().is_empty());
    let id = repo
        .apply(TaskCommand::Create("  Comprar café ☕  ".into()))
        .unwrap()[0]
        .id();
    let second = repo.apply(TaskCommand::Create("Outra".into())).unwrap()[1].id();
    repo.apply(TaskCommand::Rename(id, "Comprar pão".into()))
        .unwrap();
    repo.apply(TaskCommand::SetStatus(id, TaskStatus::Completed))
        .unwrap();
    drop(repo);
    let mut repo = TaskRepository::open(dir.path()).unwrap();
    let tasks = repo.list().unwrap();
    assert_eq!(
        tasks.iter().map(|task| task.id()).collect::<Vec<_>>(),
        vec![id, second]
    );
    assert_eq!(tasks[0].title().as_str(), "Comprar pão");
    assert_eq!(tasks[0].status(), TaskStatus::Completed);
    repo.apply(TaskCommand::Delete(second)).unwrap();
    drop(repo);
    let mut repo = TaskRepository::open(dir.path()).unwrap();
    assert_eq!(repo.list().unwrap().len(), 1);
    let tasks = repo.apply(TaskCommand::Create("Depois".into())).unwrap();
    assert!(tasks[1].id().get() > second.get());
}

#[test]
fn duplicate_titles_are_independent_by_id() {
    let dir = tempdir().unwrap();
    let mut repo = TaskRepository::open(dir.path()).unwrap();
    let first = repo.apply(TaskCommand::Create("Igual".into())).unwrap()[0].id();
    let second = repo.apply(TaskCommand::Create("Igual".into())).unwrap()[1].id();
    repo.apply(TaskCommand::Rename(second, "Modificada".into()))
        .unwrap();
    repo.apply(TaskCommand::SetStatus(second, TaskStatus::Completed))
        .unwrap();
    let tasks = repo.apply(TaskCommand::Delete(first)).unwrap();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].id(), second);
    assert_eq!(tasks[0].title().as_str(), "Modificada");
    assert_eq!(tasks[0].status(), TaskStatus::Completed);
}

#[test]
fn refuses_corrupt_bytes_without_replacing_them() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("tasks.sqlite3");
    let bytes = b"isto nao e um banco sqlite\0\xff";
    fs::write(&path, bytes).unwrap();
    assert!(TaskRepository::open(dir.path()).is_err());
    assert_eq!(fs::read(path).unwrap(), bytes);
}

#[test]
fn refuses_future_schema_without_changing_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("tasks.sqlite3");
    let conn = Connection::open(&path).unwrap();
    conn.execute_batch("CREATE TABLE future (data TEXT); PRAGMA user_version=2;")
        .unwrap();
    drop(conn);
    let before = fs::read(&path).unwrap();
    assert!(matches!(
        TaskRepository::open(dir.path()),
        Err(AppError::IncompatibleVersion)
    ));
    assert_eq!(fs::read(path).unwrap(), before);
}

#[test]
fn refuses_foreign_schema_and_wrong_columns() {
    for sql in [
        "CREATE TABLE foreign_data (value TEXT);",
        "CREATE TABLE tasks (id INTEGER); PRAGMA user_version=1;",
    ] {
        let dir = tempdir().unwrap();
        let conn = Connection::open(dir.path().join("tasks.sqlite3")).unwrap();
        conn.execute_batch(sql).unwrap();
        drop(conn);
        let before = fs::read(dir.path().join("tasks.sqlite3")).unwrap();
        assert!(TaskRepository::open(dir.path()).is_err());
        assert_eq!(fs::read(dir.path().join("tasks.sqlite3")).unwrap(), before);
    }
}

#[test]
fn refuses_records_that_break_domain_invariants() {
    for sql in [
        "INSERT INTO tasks VALUES (1, '', 'pending');",
        "INSERT INTO tasks VALUES (-1, 'Título', 'pending');",
        "PRAGMA ignore_check_constraints=ON; INSERT INTO tasks VALUES (1, 'Título', 'unknown');",
    ] {
        let dir = tempdir().unwrap();
        let conn = Connection::open(dir.path().join("tasks.sqlite3")).unwrap();
        conn.execute_batch(SCHEMA).unwrap();
        conn.execute_batch(sql).unwrap();
        drop(conn);
        assert!(TaskRepository::open(dir.path()).is_err());
    }
}

#[test]
fn holds_os_lock_until_drop_and_ignores_stale_lock_file() {
    let dir = tempdir().unwrap();
    let other = tempdir().unwrap();
    let repo = TaskRepository::open(dir.path()).unwrap();
    assert!(matches!(
        TaskRepository::open(dir.path()),
        Err(AppError::AlreadyRunning)
    ));
    assert!(TaskRepository::open(other.path()).is_ok());
    drop(repo);
    assert!(dir.path().join("app.lock").exists());
    assert!(TaskRepository::open(dir.path()).is_ok());
}

#[test]
fn missing_id_and_invalid_title_never_create_rows() {
    let dir = tempdir().unwrap();
    let mut repo = TaskRepository::open(dir.path()).unwrap();
    let id = TaskId::new(99).unwrap();
    for command in [
        TaskCommand::Rename(id, "Novo".into()),
        TaskCommand::SetStatus(id, TaskStatus::Completed),
        TaskCommand::Delete(id),
    ] {
        assert!(matches!(repo.apply(command), Err(AppError::NotFound)));
    }
    assert!(repo.apply(TaskCommand::Create("\t".into())).is_err());
    assert!(repo.list().unwrap().is_empty());
}

#[test]
fn directory_that_is_a_file_is_reported_and_preserved() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("file");
    fs::write(&path, b"preservar").unwrap();
    assert!(TaskRepository::open(&path).is_err());
    assert_eq!(fs::read(path).unwrap(), b"preservar");
}
