use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Toml parse error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("Invalid project: {0}")]
    InvalidProject(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Other: {0}")]
    Other(String),
}
