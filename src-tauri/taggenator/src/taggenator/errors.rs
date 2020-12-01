use std::fmt;
use std::{error::Error, include_str};

pub type BError = Box<Error>;

#[derive(Debug)]
pub enum MyCustomError {
	UnknownError,
	SetupError,
	ParseError,
	DuplicateFiles { files: Vec<String> },
	InvalidCommand { name: String },
}

impl std::error::Error for MyCustomError {}

impl fmt::Display for MyCustomError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			MyCustomError::UnknownError => write!(f, "Unknown Error"),
			MyCustomError::SetupError => write!(f, "Setup Error"),
			MyCustomError::SetupError => write!(f, "Setup Error"),
			MyCustomError::ParseError => write!(f, "Parse Error"),
			MyCustomError::DuplicateFiles { files } => {
				write!(f, "Duplicate file error: {:?}", files)
			}
			MyCustomError::InvalidCommand { name } => {
				write!(f, "Invalid command error: {}", name)
			}
		}
	}
}
