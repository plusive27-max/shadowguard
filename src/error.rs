use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShadowGuardError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Walkdir error: {0}")]
    Walkdir(#[from] walkdir::Error),
    #[error("Ollama error: {0}")]
    Ollama(String),
}
