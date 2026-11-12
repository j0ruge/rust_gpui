//! Regras, persistência e apresentação da aplicação local de tarefas.

/// Resolução do diretório de dados a partir das opções do ambiente.
pub mod config;
/// Tipos e invariantes das tarefas, sem dependência da interface.
pub mod domain;
/// Erros observáveis e suas causas técnicas.
pub mod error;
/// Transições de apresentação e confirmação das intenções de escrita.
pub mod state;
/// Repositório SQLite transacional com lock exclusivo por diretório.
pub mod storage;
/// Janela nativa e interação com GPUI.
pub mod ui;
