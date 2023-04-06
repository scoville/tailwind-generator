#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("pyaco error: {0}")]
    Pyaco(#[from] pyaco_core::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("glob pattern error: {0}")]
    GlobPattern(#[from] glob::PatternError),

    #[error("notify error: {0}")]
    Notify(#[from] notify::Error),

    #[error("regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("grep regex error: {0}")]
    GrepRegex(#[from] grep_regex::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
