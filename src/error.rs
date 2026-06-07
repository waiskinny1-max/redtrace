use thiserror::Error;

#[derive(Debug, Error)]
pub enum RedtraceError {
    #[error("not inside a redtrace workspace; run `redtrace init <name>` first")]
    MissingWorkspace,

    #[error("redtrace workspace already exists at {0}")]
    WorkspaceExists(String),

    #[error("object not found: {0}")]
    NotFound(String),

    #[error("invalid scope rule: {0}")]
    InvalidScope(String),
}
