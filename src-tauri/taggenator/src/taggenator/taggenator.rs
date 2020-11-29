// TODO: remove
#![allow(warnings, unused)]
use super::database::Database;
use crate::taggenator::errors::BError;
use crate::taggenator::settings::Settings;
use std::fs::File;
use std::include_str;
use std::io::prelude::*;
use toml::{de::Error, Value};
use walkdir;

pub struct Taggenator {
	settings: Settings,
	database: Database,
}

impl Taggenator {
	pub fn new() -> Result<Taggenator, BError> {
		let settings = Settings::new()?;
		let database = Database::new()?;
		return Ok(Taggenator {
			settings: settings,
			database: database,
		});
	}

	pub fn parse_args(&mut self, args: Vec<String>) -> Result<(), BError> {
		self.update_files()?;
		// self.settings.save();
		// self.database.testWrite(100000)?;
		// dbg!(self.database.testRead()?.len());
		Ok(())
	}

	fn update_files(&mut self) -> Result<(), BError> {
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
