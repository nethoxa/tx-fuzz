use std::fmt::{self, Display};

#[derive(PartialEq, Debug)]
pub enum AppStatus {
    RuntimeError,
    Exit,
    InvalidBool,
}

impl Display for AppStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppStatus::RuntimeError => write!(f, "runtime error"),
            AppStatus::Exit => write!(f, "exit"),
            AppStatus::InvalidBool => write!(f, "invalid bool"),
        }
    }
}
