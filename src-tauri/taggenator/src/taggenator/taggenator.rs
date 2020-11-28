// TODO: remove
#![allow(warnings, unused)]
use crate::taggenator::errors::BError;
use crate::taggenator::settings::Settings;
use jwalk;
use std::fs::File;
use std::include_str;
use std::io::prelude::*;
use toml::{de::Error, Value};
use walkdir;

pub struct Taggenator {
	settings: Settings,
}

impl Taggenator {
	pub fn new() -> Result<Taggenator, BError> {
		let settings = Settings::new()?;
		return Ok(Taggenator { settings: settings });
	}

	pub fn parse_args(self, args: Vec<String>) -> Result<(), BError> {
		// dbg!(args);
		// dbg!(env::current_dir());
		// self.base();
		// self.jwalk();
		Ok(())
	}

	fn jwalk(self) -> Result<(), BError> {
		let mut num_chars = 0;
		for entry in jwalk::WalkDir::new(".") {
			if let Some(name) = entry?.file_name().to_str() {
				num_chars += name.len();
			}
		}
		dbg!(num_chars);
		Ok(())
	}

	fn base(self) -> Result<(), BError> {
		let mut num_chars = 0;
		for entry in walkdir::WalkDir::new(".") {
			if let Some(name) = entry?.file_name().to_str() {
				num_chars += name.len();
			}
		}
		dbg!(num_chars);
		Ok(())
	}
}
