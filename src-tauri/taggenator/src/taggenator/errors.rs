// TODO: remove
#![allow(dead_code, warnings, unused)]

use std::fmt;
use std::{error::Error, include_str};

pub type BError = Box<Error>;

#[derive(Debug)]
pub enum MyCustomError {
	SetupError,
	ParseError,
	DuplicateFile { name: String },
}

impl std::error::Error for MyCustomError {}

impl fmt::Display for MyCustomError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			MyCustomError::SetupError => write!(f, "Setup Error"),
			MyCustomError::ParseError => write!(f, "Parse Error"),
			MyCustomError::DuplicateFile { name } => {
				write!(f, "Duplicate file error: {}", name)
			}
		}
	}
}
