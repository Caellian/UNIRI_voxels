use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResourceError {
    #[error("Invalid content pack: {path}")]
    InvalidContentPack { path: PathBuf },
    #[error("Invalid path: {0}")]
    InvalidPath(PathBuf),
    #[error(transparent)]
    Ron(#[from] ron::error::SpannedError),
    #[error("Vox error: {0}")]
    Vox(&'static str),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Resource(#[from] ResourceError),
}
