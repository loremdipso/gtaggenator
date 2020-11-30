use super::database::searcher::Searcher;
use super::database::Database;
use crate::taggenator::commands::RunCommand;
use crate::taggenator::errors::BError;
use crate::taggenator::errors::MyCustomError;
use crate::taggenator::models::record::MiniRecord;
use crate::taggenator::settings::Settings;
use crate::taggenator::utils::files::get_extension_from_filename;
use multimap::MultiMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::include_str;
use std::io::prelude::*;
use std::path::Path;
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
		// self.database.add_tag("99", vec!["yup".to_string()]);

		if args.len() == 0 {
			// TODO: print help
			println!("ERROR: invalid command");
			return Ok(());
		}

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
		let mut num_renamed = 0;
		let mut num_deleted = 0;
		let file_map = self.database.get_filenames_to_locations()?;
		let mut seen: MultiMap<String, String> = MultiMap::new();
		let mut possibly_moved: MultiMap<String, String> = MultiMap::new();

		loop {
			let value = receiver.recv()?;
			match value {
				None => break,
				Some(entry) => {
					let entry = entry.unwrap();
					if let Some(name) = &entry.file_name().to_str() {
						if entry.path().is_dir() || !self.valid_extension(entry.path()) {
							continue;
						}

						let location = entry.path().to_str().unwrap();
						let mut did_create = false;
						if !file_map.contains_key(*name) {
							num_added += 1;
							self.database.add_record(name, location);
							did_create = true;
						} else {
							// if we've seen a file like this, but never at this location,
							// deal with it later
							let matches = file_map.get_vec(*name).unwrap();
							if !matches.iter().any(|el| (*el).Location == location) {
								possibly_moved.insert(name.to_string(), location.to_string());
							}
						}

						seen.insert(name.to_string(), location.to_string());
					}
				}
			}
		}

		self.database.flush_writes();

		// try our best to handle moved files
		// dbg!(&possibly_moved);
		for (name, values) in possibly_moved.iter_all() {
			// If multiple files with the same name have all changed position,
			// we have no way to handle that
			if values.len() > 1 {
				return Err(Box::new(MyCustomError::DuplicateFiles {
					files: values.to_vec(),
				}));
			}
			let location = values.first().unwrap();

			let did_see = seen.get_vec(name).unwrap(); // guaranteed to be non-null
			let did_have = file_map.get_vec(name).unwrap(); // guaranteed to not be null

			let mut do_update: Vec<MiniRecord> = vec![];
			for mini_record in did_have {
				// check to see if the record we have on file matches
				// the location of any record we just saw
				if !did_see
					.iter()
					.any(|location| *location == mini_record.Location)
				{
					do_update.push(mini_record.clone());
				}
			}

			// dbg!(&do_update);
			match do_update.len() {
				0 => {
					// println!("Adding {} at {}", name, location);
					self.database.add_record(name, location);
					seen.insert(name.to_string(), location.to_string());
					num_added += 1;
				}

				1 => {
					let mini_record = do_update.first().unwrap();
					// println!("Moving {} to {}", (*mini_record).Location, location);
					self.database
						.update_location((*mini_record).RecordID, location);

					// need to insert the old location so our delete code still works
					seen.insert(name.to_string(), (*mini_record).Location.to_string());
					num_renamed += 1;
				}

				_ => {
					// error: can't update more than one thing at a time
					return Err(Box::new(MyCustomError::DuplicateFiles {
						files: do_update.iter().map(|e| e.Location.clone()).collect(),
					}));
				}
			}
		}

		// delete records
		for (name, records) in file_map.iter_all() {
			let temp_vec = vec![];
			let seen_locations = seen.get_vec(name).unwrap_or(&temp_vec);
			for record in records {
				if !seen_locations
					.iter()
					.any(|location| record.Location == *location)
				{
					// println!("Deleting {}", record.Location);
					self.database.delete_record(record.RecordID);
					num_deleted += 1;
				}
			}
		}

		println!("# Added {} new files", num_added);
		println!("# Renamed {} files", num_renamed);
		println!("# Deleted {} files", num_deleted);
		self.database.flush_writes();

		return Ok((num_added, num_deleted));
	}

	fn valid_extension(&self, path: &Path) -> bool {
		if let Some(extension) = get_extension_from_filename(&path) {
			if self.settings.extensions.iter().any(|ext| ext == extension) {
				return true;
			}
		}
		return false;
	}
}
