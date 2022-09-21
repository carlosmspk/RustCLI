#[derive(Debug)]
pub enum Error {
    ItemlessOptionListError,
    IOError(std::io::Error),
    TerminalScreenStackEmpty,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(e) => write!(f, "{}", e),
            Self::ItemlessOptionListError => write!(
                f,
                "ItemlessMenuError: Menu Screen has no options to choose from!"
            ),
            Self::TerminalScreenStackEmpty => write!(
                f,
                "TerminalScreenStackEmpty: Terminal has no screens in its screen stack!"
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
