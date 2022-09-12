#[derive(Debug)]
pub enum Error {
    ItemlessMenuError,
    IOError(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(e) => write!(f, "{}", e),
            Self::ItemlessMenuError => write!(
                f,
                "ItemlessMenuError: Menu Screen has no options to choose from!"
            ),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}
