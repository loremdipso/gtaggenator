use super::database::searcher::Searcher;
use super::database::Database;
use crate::taggenator::commands::RunCommand;
use crate::taggenator::errors::BError;
use crate::taggenator::errors::MyCustomError;
use crate::taggenator::models::record::MiniRecord;
use crate::taggenator::models::record::Record;
use crate::taggenator::settings::Settings;
use crate::taggenator::tag_recommender::TagRecommender;
use crate::taggenator::utils::commands::run_command_string;
use crate::taggenator::utils::files::get_extension_from_filename;
use crate::taggenator::utils::flags::take_flag;
use crate::taggenator::utils::lists::dedup;
use multimap::MultiMap;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::include_str;
use std::io::prelude::*;
use std::ops::Deref;
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;
use toml::{de::Error, Value};
use walkdir;

pub struct Taggenator {
	pub settings: Settings,
	pub database: Database,
	newest_temp: i32,
}

impl Taggenator {
	pub fn new() -> Result<Taggenator, BError> {
		let settings = Settings::new()?;
		let database = Database::new()?;
		return Ok(Taggenator {
			settings: settings,
			database: database,
			newest_temp: 0,
		});
	}

	pub fn new_headless() -> Result<Taggenator, BError> {
		let settings = Settings::new_headless()?;
		let database = Database::new()?;
		return Ok(Taggenator {
			settings: settings,
			database: database,
			newest_temp: 0,
		});
	}

	pub fn parse_args(&mut self, mut args: Vec<String>) -> Result<(), BError> {
		if !take_flag(&mut args, "--ignore-update") {
			let (num_added, num_deleted) = self.update_files()?;
		}

		// self.settings.save();
		// self.database.test_write(100000)?;
		// self.database.test_write(10)?;
		// self.database.add_tag(100, "yup".to_string());

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

	pub fn update_files(&mut self) -> Result<(i32, i32), BError> {
		let (sender, receiver) = channel();

		// start a thread to run through the fs while the main thread talks to the DB
		let extensions = self.settings.extensions.clone();
		let worker = thread::spawn(move || loop {
			for entry in walkdir::WalkDir::new(".") {
				if let Ok(entry) = entry {
					if !entry.path().is_dir()
						&& Taggenator::valid_extension(&extensions, entry.path())
					{
						sender.send(Some(entry));
					}
				}
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

		self.database.start_batch();
		loop {
			let value = receiver.recv()?;
			match value {
				None => break,
				Some(entry) => {
					if let Some(name) = &entry.file_name().to_str() {
						let location = entry.path().to_str().unwrap();
						let mut did_create = false;
						if !file_map.contains_key(*name) {
							num_added += 1;
							self.database.add_record(name, location)?;
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
					self.database.add_record(name, location)?;
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
		self.database.end_batch();

		println!("# Added {} new files", num_added);
		println!("# Renamed {} files", num_renamed);
		println!("# Deleted {} files", num_deleted);
		self.database.flush_writes();

		return Ok((num_added, num_deleted));
	}

	fn valid_extension(extensions: &Vec<String>, path: &Path) -> bool {
		if let Some(extension) = get_extension_from_filename(&path) {
			if extensions.iter().any(|ext| ext == extension) {
				return true;
			}
		}
		return false;
	}

	pub fn insert_tag_line(&mut self, record: &mut Record, line: String) -> Result<(), BError> {
		let mut tags: Vec<String> = line
			.split(",")
			.map(|piece| piece.trim().to_string())
			.collect();

		self.handle_tagger(record, &mut tags)?;

		let mut to_add: Vec<String> = vec![];
		let mut to_remove: Vec<String> = vec![];
		for tag in tags.iter() {
			// don't add empty tags
			if tag.len() == 0 {
				continue;
			}

			if tag.chars().nth(0).unwrap_or_default() == '-' {
				let tag = &tag[1..];
				to_remove.push(tag.to_string());
			} else if tag.chars().last().unwrap_or_default() == '-' {
				let tag = &tag[..tag.chars().count() - 1];
				to_remove.push(tag.to_string());
			} else if !record.Tags.contains(tag) {
				to_add.push(tag.to_string());
			}
		}

		self.replace_synonyms(&mut to_remove)?;
		self.replace_synonyms(&mut to_add)?;
		self.handle_temp(&mut to_add)?;
		let do_reset = self.handle_reset(&mut to_add);
		self.add_derived(record, &mut to_add)?;

		dedup(&mut to_add);

		for tag in &to_add {
			if !record.Tags.contains(tag) {
				record.Tags.insert(tag.clone());
			}
		}

		for tag in &to_remove {
			if record.Tags.contains(tag) {
				record.Tags.remove(&tag.to_string());
			}

			if let Some(index) = to_add.iter().position(|x| tag == x) {
				to_add.remove(index);
			}
		}

		if to_add.len() > 0 {
			// only add the tags that weren't there already
			self.database.add_tags(record.RecordID, &to_add)?;
		}

		if to_remove.len() > 0 {
			self.database.remove_tags(record.RecordID, &to_remove)?;
		}

		if do_reset {
			record.HaveManuallyTouched = false;
			record.TimesOpened = 0;
			self.database.set_times_opened(record.RecordID, 0)?;
			self.database.set_touched(record.RecordID, false)?;
		} else if !record.HaveManuallyTouched && to_add.len() > 0 {
			// touched
			record.HaveManuallyTouched = true;
			self.database.set_touched(record.RecordID, true)?;
		}

		Ok(())
	}

	fn add_derived(&mut self, record: &mut Record, tags: &mut Vec<String>) -> Result<(), BError> {
		let mut to_add: Vec<String> = vec![];
		for tag in tags.iter() {
			let matches = self.settings.derived.get(tag);
			if let Some(matches) = matches {
				for new_tag in matches {
					if !record.Tags.contains(new_tag) && !tags.contains(new_tag) {
						to_add.push(new_tag.to_string());
					}
				}
			}
		}

		tags.append(&mut to_add);
		return Ok(());
	}

	fn handle_tagger(&mut self, record: &mut Record, tags: &mut Vec<String>) -> Result<(), BError> {
		let mut to_add: Vec<String> = vec![];
		let mut to_remove: Vec<usize> = vec![];
		for (i, tag) in tags.iter().enumerate() {
			if let Some(tagger_command) = self.settings.tagger.get(tag) {
				to_remove.push(i);

				// NOTE: need to jump through hoops because of COW
				let temp_tagger_command =
					Regex::new("%s")?.replace(&tagger_command, record.Location.as_str());
				let temp_tagger_command = temp_tagger_command.deref();

				let result = run_command_string(&temp_tagger_command.to_string())?;
				let mut do_remove = false;
				for new_tag in result.split("\n") {
					do_remove = true;
					if new_tag.len() > 0
						&& new_tag.chars().nth(0) != Some('#')
						&& !record.Tags.contains(new_tag)
					{
						println!("tagger found: {}", &new_tag);
						// NOTE: could lead to duplicates, but we're okay with that
						to_add.push(new_tag.to_string());
					}
				}
			}
		}

		for i in to_remove.iter().rev() {
			tags.remove(*i);
		}

		tags.append(&mut to_add);
		return Ok(());
	}

	fn replace_synonyms(&mut self, tags: &mut Vec<String>) -> Result<(), BError> {
		for tag in tags {
			if let Some(synonym) = self.settings.synonyms.get(tag) {
				*tag = synonym.clone();
			}
		}

		return Ok(());
	}

	// tempnew & temp
	fn handle_temp(&mut self, tags_to_add: &mut Vec<String>) -> Result<(), BError> {
		for tag in tags_to_add {
			if tag == "temp" {
				self.update_temp_tags()?;
				*tag = format!("temp{}", self.newest_temp);
			} else if tag == "tempnew" {
				self.update_temp_tags()?;
				self.newest_temp += 1;
				*tag = format!("temp{}", self.newest_temp);
			}
		}

		return Ok(());
	}

	fn handle_reset(&mut self, tags_to_add: &mut Vec<String>) -> bool {
		let mut to_remove = vec![];
		let mut rv = false;
		for (i, tag) in tags_to_add.iter().enumerate() {
			if tag == "reset" || tag == "unsorted" {
				to_remove.push(i);
				rv = true;
			}
		}

		// TODO: clean up, filter or drain or something
		for i in to_remove.iter().rev() {
			tags_to_add.remove(*i);
		}

		return rv;
	}

	fn update_temp_tags(&mut self) -> Result<(), BError> {
		if self.newest_temp == 0 {
			let query = &"Select TagName From Tags Where TagName Like ?";
			let mut stmt = self.database.conn.prepare(&query)?;

			// We're handling query args ourselves
			let mut rows = stmt.query(&["temp%"])?;

			loop {
				let row = rows.next()?;
				match row {
					None => break,
					Some(row) => {
						let tagName: String = row.get(0)?;
						let tagName = tagName.strip_prefix("temp").unwrap().to_string();
						let number = tagName.parse::<i32>();
						if let Ok(number) = number {
							if number > self.newest_temp {
								self.newest_temp = number;
							}
						}
					}
				}
			}
		}

		Ok(())
	}
}
