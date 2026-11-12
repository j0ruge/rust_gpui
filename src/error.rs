use crate::domain::DomainError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Validation(#[from] DomainError),
    #[error("O diretório dos dados precisa ser um caminho absoluto válido.")]
    InvalidDataDir,
    #[error("Não foi possível acessar os arquivos locais.")]
    Io(#[from] std::io::Error),
    #[error("O banco de tarefas é inválido. O arquivo foi preservado.")]
    InvalidDatabase(#[source] Option<rusqlite::Error>),
    #[error("Esta versão do banco não é compatível com o aplicativo.")]
    IncompatibleVersion,
    #[error("O aplicativo já está aberto para estes dados.")]
    AlreadyRunning,
    #[error("A tarefa não foi encontrada.")]
    NotFound,
    #[error("Não foi possível salvar ou ler as tarefas. Tente novamente.")]
    Sqlite(#[from] rusqlite::Error),
    #[error("O armazenamento não está disponível. Feche e reabra o aplicativo.")]
    StorageUnavailable,
}
