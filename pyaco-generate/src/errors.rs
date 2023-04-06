#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("pyaco error: {0}")]
    Pyaco(#[from] pyaco_core::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("notify error: {0}")]
    Notify(#[from] notify::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
