#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no css classes found, are you sure the provided css source contains at least one class and is valid?")]
    NoCssClassesFound,

    #[error("askama error: {0}")]
    Askama(#[from] askama::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("ureq error: {0}")]
    Ureq(#[from] Box<ureq::Error>),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
