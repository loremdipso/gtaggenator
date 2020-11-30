use super::database::searcher::Searcher;
use super::database::Database;
use crate::taggenator::commands::RunCommand;
use crate::taggenator::errors::BError;
use crate::taggenator::errors::MyCustomError;
use crate::taggenator::settings::Settings;
use std::collections::HashSet;
use std::fs::File;
use std::include_str;
use std::io::prelude::*;
use std::sync::mpsc::channel;
use std::thread;
use toml::{de::Error, Value};
use walkdir;

pub struct Taggenator {
	pub settings: Settings,
	pub database: Database,
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

	pub fn parse_args(&mut self, mut args: Vec<String>) -> Result<(), BError> {
		let (num_added, num_deleted) = self.update_files()?;

		// self.settings.save();
		// self.database.test_write(100000)?;
		// self.database.test_write(10)?;
		// dbg!(self.database.test_read()?.len());
		// self.database.add_tag("2", vec!["sup".to_string()]);
		// self.database.add_tag("100", vec!["yup".to_string()]);
		// self.database.add_tag("100", vec!["group".to_string()]);

		let command = args[0].clone();
		args.remove(0);
		RunCommand(self, command, args)?;

		Ok(())
	}

	fn update_files(&mut self) -> Result<(i32, i32), BError> {
		let (sender, receiver) = channel();

		// start a thread to run through the fs while the main thread talks to the DB
		let worker = thread::spawn(move || loop {
			for entry in walkdir::WalkDir::new(".") {
				sender.send(Some(entry));
			}
			sender.send(None);
		});

		// get all current filenames from the DB and then pend work for later
		let mut num_added = 0;
		let mut num_deleted = 0;
		let files = self.database.get_filenames()?;
		let mut seen = HashSet::new();
		loop {
			let value = receiver.recv()?;
			match value {
				None => break,
				Some(entry) => {
					let entry = entry.unwrap();
					if let Some(name) = &entry.file_name().to_str() {
						if !files.contains(*name) {
							num_added += 1;
							self.database
								.add_record(name, &entry.path().to_str().unwrap());
						}

						if seen.contains(*name) {
							return Err(Box::new(MyCustomError::DuplicateFile {
								name: name.to_string(),
							}));
						}
						seen.insert(name.to_string());
					}
				}
			}
		}

		for file in files {
			if !seen.contains(&file) {
				self.database.delete_record(&file);
				num_deleted += 1;
			}
		}

		println!("Added {} new files", num_added);
		println!("Deleted {} files", num_deleted);
		self.database.flush_writes();

		return Ok((num_added, num_deleted));
	}
}
