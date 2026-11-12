use crate::{
    domain::{Task, TaskId, TaskStatus, TaskTitle},
    error::AppError,
};
use rusqlite::{Connection, params};
use std::{
    fs::{File, OpenOptions, TryLockError},
    path::Path,
    time::Duration,
};

const TABLE_SQL: &str = "CREATE TABLE tasks (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT NOT NULL, status TEXT NOT NULL CHECK (status IN ('pending', 'completed')))";

#[derive(Debug, Clone, PartialEq, Eq)]
/// Intenção de escrita serializada pela interface e confirmada por transação.
pub enum TaskCommand {
    Create(String),
    Rename(TaskId, String),
    SetStatus(TaskId, TaskStatus),
    Delete(TaskId),
}
/// Conexão SQLite e lock exclusivo do diretório, mantido até o drop.
pub struct TaskRepository {
    connection: Connection,
    // A exclusividade depende do handle vivo, nunca da presença do arquivo.
    _lock: File,
}
impl TaskRepository {
    /// Abre diretório absoluto com lock; inicializa banco vazio ou recusa versão, esquema e dados inválidos.
    pub fn open(data_dir: &Path) -> Result<Self, AppError> {
        if !data_dir.is_absolute() {
            return Err(AppError::InvalidDataDir);
        }
        std::fs::create_dir_all(data_dir)?;
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(data_dir.join("app.lock"))?;
        lock.try_lock().map_err(|error| match error {
            TryLockError::WouldBlock => AppError::AlreadyRunning,
            TryLockError::Error(error) => AppError::Io(error),
        })?;
        let mut connection = Connection::open(data_dir.join("tasks.sqlite3"))?;
        connection.busy_timeout(Duration::from_millis(250))?;
        validate_or_initialize(&mut connection)?;
        connection.pragma_update(None, "journal_mode", "DELETE")?;
        connection.pragma_update(None, "synchronous", "FULL")?;
        let repo = Self {
            connection,
            _lock: lock,
        };
        repo.list()?;
        Ok(repo)
    }

    /// Lê e valida tarefas em ordem crescente de identidade.
    pub fn list(&self) -> Result<Vec<Task>, AppError> {
        read_tasks(&self.connection)
    }

    /// Valida e executa uma intenção; retorna o snapshot somente após commit. Erro reverte a transação.
    pub fn apply(&mut self, command: TaskCommand) -> Result<Vec<Task>, AppError> {
        let tx = self.connection.transaction()?;
        let tasks = read_tasks(&tx)?;
        match command {
            TaskCommand::Create(raw) => {
                let title = TaskTitle::parse(&raw)?;
                tx.execute(
                    "INSERT INTO tasks (title,status) VALUES (?1,'pending')",
                    [title.as_str()],
                )?;
            }
            TaskCommand::Rename(id, raw) => {
                if !tasks.iter().any(|task| task.id() == id) {
                    return Err(AppError::NotFound);
                }
                let title = TaskTitle::parse(&raw)?;
                if tx.execute(
                    "UPDATE tasks SET title=?1 WHERE id=?2",
                    params![title.as_str(), id.get()],
                )? != 1
                {
                    return Err(AppError::NotFound);
                }
            }
            TaskCommand::SetStatus(id, status) => {
                if !tasks.iter().any(|task| task.id() == id) {
                    return Err(AppError::NotFound);
                }
                let token = match status {
                    TaskStatus::Pending => "pending",
                    TaskStatus::Completed => "completed",
                };
                if tx.execute(
                    "UPDATE tasks SET status=?1 WHERE id=?2",
                    params![token, id.get()],
                )? != 1
                {
                    return Err(AppError::NotFound);
                }
            }
            TaskCommand::Delete(id) => {
                if !tasks.iter().any(|task| task.id() == id) {
                    return Err(AppError::NotFound);
                }
                if tx.execute("DELETE FROM tasks WHERE id=?1", [id.get()])? != 1 {
                    return Err(AppError::NotFound);
                }
            }
        }
        let snapshot = read_tasks(&tx)?;
        // O snapshot só sai da fronteira de persistência depois do commit.
        tx.commit()?;
        Ok(snapshot)
    }
}

/// Valida o formato suportado antes de alterar pragmas; só inicializa um banco vazio e sem versão.
fn validate_or_initialize(connection: &mut Connection) -> Result<(), AppError> {
    let check: String = connection
        .query_row("PRAGMA quick_check", [], |row| row.get(0))
        .map_err(|error| AppError::InvalidDatabase(Some(error)))?;
    if check != "ok" {
        return Err(AppError::InvalidDatabase(None));
    }
    let version: i64 = connection.pragma_query_value(None, "user_version", |row| row.get(0))?;
    let objects: i64 = connection.query_row(
        "SELECT count(*) FROM sqlite_master WHERE name != 'sqlite_sequence'",
        [],
        |row| row.get(0),
    )?;
    if version == 0 && objects == 0 {
        let tx = connection.transaction()?;
        tx.execute_batch(TABLE_SQL)?;
        tx.pragma_update(None, "user_version", 1)?;
        tx.commit()?;
    } else if version != 1 {
        return if version == 0 {
            Err(AppError::InvalidDatabase(None))
        } else {
            Err(AppError::IncompatibleVersion)
        };
    }
    let object_count: i64 = connection.query_row(
        "SELECT count(*) FROM sqlite_master WHERE name != 'sqlite_sequence'",
        [],
        |row| row.get(0),
    )?;
    let sql: String = connection
        .query_row(
            "SELECT sql FROM sqlite_master WHERE type='table' AND name='tasks'",
            [],
            |row| row.get(0),
        )
        .map_err(|error| AppError::InvalidDatabase(Some(error)))?;
    let normalize = |sql: &str| {
        let mut in_literal = false;
        sql.chars()
            .filter_map(|c| {
                if c == '\'' {
                    in_literal = !in_literal;
                    Some(c)
                } else if in_literal {
                    Some(c)
                } else if c.is_ascii_whitespace() || c == ';' {
                    None
                } else {
                    Some(c.to_ascii_lowercase())
                }
            })
            .collect::<String>()
    };
    if object_count != 1 || normalize(&sql) != normalize(TABLE_SQL) {
        return Err(AppError::InvalidDatabase(None));
    }
    // Validar antes de alterar pragmas evita tratar um banco inválido como vazio.
    read_tasks(connection)?;
    Ok(())
}

/// Reconstrói registros validados ou devolve erro, sem ocultar registros inválidos.
fn read_tasks(connection: &Connection) -> Result<Vec<Task>, AppError> {
    let mut statement = connection.prepare("SELECT id,title,status FROM tasks ORDER BY id ASC")?;
    let rows = statement.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;
    rows.map(|row| {
        let (id, raw, token) = row.map_err(|error| AppError::InvalidDatabase(Some(error)))?;
        let id = TaskId::new(id).map_err(|_| AppError::InvalidDatabase(None))?;
        let title = TaskTitle::parse(&raw).map_err(|_| AppError::InvalidDatabase(None))?;
        let status = match token.as_str() {
            "pending" => TaskStatus::Pending,
            "completed" => TaskStatus::Completed,
            _ => return Err(AppError::InvalidDatabase(None)),
        };
        Ok(Task::restore(id, title, status))
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    #[test]
    fn aborted_update_rolls_back_and_survives_reopening() {
        let dir = tempfile::tempdir().unwrap();
        let mut repo = TaskRepository::open(dir.path()).unwrap();
        let id = repo.apply(TaskCommand::Create("Antes".into())).unwrap()[0].id();
        repo.apply(TaskCommand::Create("Outra".into())).unwrap();
        let conn = Connection::open(dir.path().join("tasks.sqlite3")).unwrap();
        conn.execute_batch("CREATE TRIGGER fail_update BEFORE UPDATE ON tasks BEGIN SELECT RAISE(ABORT, 'falha de teste'); END;").unwrap();
        drop(conn);
        assert!(
            repo.apply(TaskCommand::Rename(id, "Depois".into()))
                .is_err()
        );
        assert_eq!(repo.list().unwrap()[0].title().as_str(), "Antes");
        // Retirar a falha injetada permite testar a reabertura do esquema válido.
        let conn = Connection::open(dir.path().join("tasks.sqlite3")).unwrap();
        conn.execute_batch("DROP TRIGGER fail_update;").unwrap();
        drop(conn);
        drop(repo);
        let repo = TaskRepository::open(dir.path()).unwrap();
        let tasks = repo.list().unwrap();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].title().as_str(), "Antes");
        assert_eq!(tasks[0].status(), TaskStatus::Pending);
    }
}
