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
		let num_added = self.update_files()?;
		println!("Added {} new files", num_added);
		self.database.flush_writes();

		// self.settings.save();
		// self.database.test_write(100000)?;
		// self.database.test_write(10)?;
		// dbg!(self.database.test_read()?.len());
		// self.database.add_tag("1000016", vec!["sup".to_string()]);

		Ok(())
	}

	fn update_files(&mut self) -> Result<i32, BError> {
		let files = self.database.get_filenames()?;

		let mut num_added = 0;
		// for entry in walkdir::WalkDir::new(".") {
		// 	if let Some(name) = entry?.file_name().to_str() {
		// 		if !files.contains(name) {
		// 			num_added += 1;
		// 			self.database.add_record(name);
		// 		}
		// 	}
		// }

		Ok(num_added)
	}
}
