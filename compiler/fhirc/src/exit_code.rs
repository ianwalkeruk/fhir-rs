use crate::error::Error;

pub const SUCCESS: i32 = 0;

pub const IO_ERROR: i32 = 1;

pub const VALIDATION_ERROR: i32 = 2;

pub const PARSE_ERROR: i32 = 3;

pub const NOT_IMPLEMENTED: i32 = 4;

pub fn from_error(error: &Error) -> i32 {
    match error {
        Error::Io(_) => IO_ERROR,
        Error::YamlParse(_) => PARSE_ERROR,
        Error::ValidationError(_) => VALIDATION_ERROR,
        Error::FileNotFound(_) => IO_ERROR,
        Error::NotImplemented(_) => NOT_IMPLEMENTED,
    }
}
