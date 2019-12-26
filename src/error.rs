#[derive(Debug)]
pub struct Error {
    pub error_type: ErrType,
    pub msg: String,
}

#[derive(Debug)]
pub enum ErrType {
    IO,
    Parse,
    None,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self {
            error_type: ErrType::IO,
            msg: e.to_string(),
        }
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Self {
            error_type: ErrType::Parse,
            msg: e.to_string(),
        }
    }
}

impl From<std::option::NoneError> for Error {
    fn from(_: std::option::NoneError) -> Self {
        Self {
            error_type: ErrType::None,
            msg: "None".to_string(),
        }
    }
}

impl From<toml::ser::Error> for Error {
    fn from(e: toml::ser::Error) -> Self {
        Self {
            error_type: ErrType::Parse,
            msg: e.to_string(),
        }
    }
}
