use crate::taggenator::database::Database;
use crate::taggenator::errors::BError;
use crate::taggenator::models::record::Record;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rusqlite::NO_PARAMS;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use string_builder::Builder;

// TODO: is there some way to clean this up? Diesel, for ex?
const ID_INDEX: usize = 0;
const NAME_INDEX: usize = 1;
const LOCATION_INDEX: usize = 2;
const SIZE_INDEX: usize = 3;
const LENGTH_INDEX: usize = 4;
const TIMES_OPENED_INDEX: usize = 5;
const DATE_ADDED_INDEX: usize = 6;
const DATE_CREATED_INDEX: usize = 7;
const DATE_LAST_ACCESSED_INDEX: usize = 8;
const HAVE_MANUALLY_TOUCHED_INDEX: usize = 9;

const TAG_INDEX: usize = 12;

#[derive(Debug)]
pub struct Searcher {
	filters: Vec<Filter>,
}

// turns args into SQL commands
impl Searcher {
	pub fn new(args: Vec<String>) -> Result<Searcher, BError> {
		let mut filters: Vec<Filter> = vec![];

		let mut name: Option<String> = None;
		let mut current_args: Vec<String> = vec![];
		for arg in args {
			if arg == "-sort" {
				if let Some(ref actual_name) = name {
					let filter = Filter::new(actual_name.clone(), current_args.clone())?;
					name.take();
					current_args.clear();
					filters.push(filter);
				}
			} else {
				if let None = name {
					name = Some(arg);
				} else {
					current_args.push(arg);
				}
			}
		}

		if let Some(actual_name) = name {
			let filter = Filter::new(actual_name.clone(), current_args.clone())?;
			current_args.clear();
			filters.push(filter);
		}

		Ok(Searcher { filters: filters })
	}

	// pub fn get_records(self, db: &Database) -> Result<Vec<Record>, BError> {
	// 	dbg!(self.filters);
	// 	return Ok(vec![]);
	// }

	pub fn get_tags(&mut self, db: &Database) -> Result<HashSet<String>, BError> {
		// COULD DO THIS, but... meh. Seems like an unnecessary optimization
		// let mut query = "SELECT Tags.TagName FROM Tags".to_string();
		// if self.filters.len() > 0 {
		// let query = &"\nJoin Records ON Records.RecordID = Tags.RecordID";
		// }
		let records = self.get_records(&db)?;
		let mut tags = HashSet::new();
		for record in records {
			for tag in record.Tags {
				tags.insert(tag.clone());
			}
		}
		return Ok(tags);
	}

	pub fn get_records(&mut self, db: &Database) -> Result<Vec<Record>, BError> {
		let query = &"\nSelect * From Records Left Join Tags On Records.RecordID = Tags.RecordID";
		let query = &self.format_query(&query);
		let mut stmt = db.conn.prepare(&query)?;

		// TODO: how can we make this less bad?
		let mut records: Vec<Record> = vec![];
		let mut current_record: Option<Record> = None;

		// We're handling query args ourselves
		let mut rows = stmt.query(NO_PARAMS)?;

		loop {
			let row = rows.next()?;
			match row {
				None => break,
				Some(row) => {
					let recordID: i64 = row.get(0)?;
					let mut should_create = true;
					if let Some(ref mut record) = current_record {
						if record.RecordID == recordID {
							if let Ok(tag) = row.get(TAG_INDEX) {
								record.Tags.insert(tag);
								should_create = false;
							}
						} else {
							records.push(current_record.take().unwrap());
						}
					}

					if should_create {
						let mut record = Record {
							RecordID: row.get(ID_INDEX)?,
							Name: row.get(NAME_INDEX)?,
							Location: row.get(LOCATION_INDEX)?,

							Tags: HashSet::new(),

							Size: row.get(SIZE_INDEX)?,
							Length: row.get(LENGTH_INDEX)?,
							TimesOpened: row.get(TIMES_OPENED_INDEX)?,

							DateAdded: row.get(DATE_ADDED_INDEX)?,
							DateCreated: row.get(DATE_CREATED_INDEX)?,
							DateLastAccessed: row.get(DATE_LAST_ACCESSED_INDEX)?,

							HaveManuallyTouched: row.get(HAVE_MANUALLY_TOUCHED_INDEX)?,
						};
						if let Ok(tag) = row.get(TAG_INDEX) {
							record.Tags.insert(tag);
						}

						// dbg!(&record);

						current_record = Some(record);
					}
				}
			}
		}

		if let Some(ref record) = current_record {
			records.push(current_record.take().unwrap());
		}

		let mut temp_records = Some(records);
		for filter in &self.filters {
			// the idea here is we wrap/unwrap the records array, filtering/sorting
			// in-between. We could try and optimize by running multiple filters so we
			// don't need to build several temporary vectors, but for now we don't.
			temp_records = filter.execute(temp_records);
		}
		let records = temp_records.take().unwrap();

		return Ok(records);
	}

	fn format_query(&mut self, query: &str) -> String {
		let mut sql = query.to_string();

		let mut is_first = true;
		let mut count = 0;
		for mut filter in &mut self.filters {
			if filter.sqlizable() {
				count += 1;
				sql = filter.sqlize(sql, is_first);
				is_first = false;
			} else {
				break;
			}
		}

		// TODO: something prettier
		while count > 0 {
			self.filters.remove(0);
			count -= 1;
		}

		// dbg!(&self.filters);
		// println!("{}", sql);
		return sql;
	}
}

#[derive(Debug)]
pub struct Filter {
	pub name: String,
	pub args: Vec<String>,
}

impl Filter {
	pub fn new(name: String, args: Vec<String>) -> Result<Filter, BError> {
		return Ok(Filter {
			name: name,
			args: args,
		});
	}

	pub fn sqlizable(&self) -> bool {
		return match &self.name[..] {
			"search" | "search_inclusive" | "search_exclusive" | "tags" | "tags_exclusive"
			| "tags_inclusive" => return true,
			_ => return false,
		};
	}

	pub fn sqlize(&mut self, sql: String, is_first: bool) -> String {
		return match &self.name[..] {
			"search" | "search_inclusive" | "search_exclusive" => {
				let mut rv = Builder::default();
				if self.args.len() == 0 {
					return sql;
				}

				if is_first {
					rv.append("where (");
				} else {
					rv.append("and (");
				}

				for (i, arg) in self.args.iter().enumerate() {
					let mut arg = arg.clone();
					let mut exclude = false;
					if arg.starts_with("-") {
						arg = (&arg[1..]).to_string();
						exclude = true;
					}

					if exclude {
						if i > 0 {
							rv.append("\nand not ");
						} else {
							rv.append("\nnot ");
						}
					} else {
						if i > 0 {
							match &self.name[..] {
								"search_inclusive" => {
									rv.append("\nor ");
								}
								_ => {
									// default
									rv.append("\nand ");
								}
							}
						}
					}

					rv.append(" (");

					rv.append(format!("(Records.Location LIKE \"%{}%\")", &arg));
					rv.append("\nor ");

					rv.append(format!(
						// tags exclusive
						// "(\"{}\" in (
						// SELECT Tags.TagName from Tags
						// WHERE Tags.RecordID = Records.RecordID))",

						// loose search
						"EXISTS(
							SELECT Tags.TagName from Tags
							WHERE Tags.RecordID = Records.RecordID
							AND Tags.TagName Like '%{}%'
						)",
						&arg
					));
					rv.append("\n)");
				}
				self.args.clear();
				rv.append(")");

				let rv = format!("{} {}", sql, rv.string().unwrap());
				return rv;
			}

			// TODO: deduplicate this horrible logic
			"tags" | "tags_inclusive" | "tags_exclusive" => {
				let mut rv = Builder::default();
				if self.args.len() == 0 {
					return sql;
				}

				if is_first {
					rv.append("where (");
				} else {
					rv.append("and (");
				}

				for (i, arg) in self.args.iter().enumerate() {
					let mut arg = arg.clone();
					let mut exclude = false;
					if arg.starts_with("-") {
						arg = (&arg[1..]).to_string();
						exclude = true;
					}

					if exclude {
						if i > 0 {
							rv.append("\nand not ");
						} else {
							rv.append("\nnot ");
						}
					} else {
						if i > 0 {
							match &self.name[..] {
								"tags_inclusive" => {
									rv.append("\nor ");
								}
								_ => {
									// default
									rv.append("\nand ");
								}
							}
						}
					}

					rv.append(" (");

					rv.append(format!(
						// tags exclusive
						"(\"{}\" in (
						SELECT Tags.TagName from Tags
						WHERE Tags.RecordID = Records.RecordID))",
						&arg
					));
					rv.append("\n)");
				}
				self.args.clear();
				rv.append(")");

				let rv = format!("{} {}", sql, rv.string().unwrap());
				return rv;
			}

			_ => return sql,
		};
	}

	// if we can't SQL something we execute it here.
	// We should have implementations for everything in sqlize,
	// since the sqlizability depends on both the position of the action
	// and the action itself
	pub fn execute(&self, records: Option<Vec<Record>>) -> Option<Vec<Record>> {
		match records {
			Some(mut records) => {
				match &self.name[..] {
					// sorting
					"random" => {
						records.shuffle(&mut thread_rng());
					}

					"reverse" => {
						records.reverse();
					}

					"largest" | "biggest" => {
						records.sort_by(|a, b| b.Size.cmp(&a.Size));
					}

					"smallest" => {
						records.sort_by(|a, b| a.Size.cmp(&b.Size));
					}

					"alpha" | "alphabetical" => {
						records.sort_by(|a, b| a.Name.cmp(&b.Name));
					}

					"newest" => {
						records.sort_by(|a, b| {
							(b.DateAdded, b.DateCreated).cmp(&(a.DateAdded, a.DateCreated))
						});
					}

					"oldest" => {
						records.sort_by(|a, b| {
							(a.DateAdded, a.DateCreated).cmp(&(b.DateAdded, b.DateCreated))
						});
					}

					"most_recently_opened" => {
						records.sort_by(|a, b| {
							(b.DateLastAccessed, b.DateAdded)
								.cmp(&(a.DateLastAccessed, a.DateAdded))
						});
					}

					"least_recently_opened" => {
						records.sort_by(|a, b| {
							(a.DateLastAccessed, a.DateAdded)
								.cmp(&(b.DateLastAccessed, b.DateAdded))
						});
					}

					"most_frequently_opened" => {
						records.sort_by(|a, b| b.TimesOpened.cmp(&a.TimesOpened));
					}

					"least_frequently_opened" => {
						records.sort_by(|a, b| a.TimesOpened.cmp(&b.TimesOpened));
					}

					// filters
					"touched" => {
						records = records
							.drain(..)
							.filter(|record| record.HaveManuallyTouched)
							.collect();
					}

					"untouched" => {
						records = records
							.drain(..)
							.filter(|record| !record.HaveManuallyTouched)
							.collect();
					}

					"tags" | "tags_exclusive" => {
						records = records
							.drain(..)
							.filter(|record| search_tags_exclusive(&record, &self.args))
							.collect();
					}

					"tags_inclusive" => {
						records = records
							.drain(..)
							.filter(|record| search_tags_inclusive(&record, &self.args))
							.collect();
					}

					"search" | "search_exclusive" => {
						records = records
							.drain(..)
							.filter(|record| loose_search_exclusive(&record, &self.args))
							.collect();
					}

					"search_inclusive" => {
						records = records
							.drain(..)
							.filter(|record| loose_search_inclusive(&record, &self.args))
							.collect();
					}

					_ => {
						// TODO: figure out error logging
						panic!("Unknown option: {}", self.name);
					}
				};

				return Some(records);
			}

			None => return None,
		}
	}
}

// every search term must match
fn loose_search_exclusive(record: &Record, search_terms: &Vec<String>) -> bool {
	for term in search_terms {
		if record.Location.to_lowercase().contains(term) {
			continue;
		}

		if record
			.Tags
			.iter()
			.any(|tag| tag.to_lowercase().contains(term))
		{
			continue;
		}

		return false;
	}

	return true;
}

// any search term needs to match
fn loose_search_inclusive(record: &Record, search_terms: &Vec<String>) -> bool {
	// special case: if there are no args, let's just assume this is true
	if search_terms.len() == 0 {
		return true;
	}

	for term in search_terms {
		if record.Location.to_lowercase().contains(term) {
			return true;
		}

		for tag in &record.Tags {
			if tag.to_lowercase().contains(term) {
				return true;
			}
		}
	}

	return false;
}

// every search term must match
fn search_tags_exclusive(record: &Record, search_terms: &Vec<String>) -> bool {
	for term in search_terms {
		if record
			.Tags
			.iter()
			.any(|tag| tag.to_lowercase().contains(term))
		{
			continue;
		}

		return false;
	}

	return true;
}

// any search term needs to match
fn search_tags_inclusive(record: &Record, search_terms: &Vec<String>) -> bool {
	// special case: if there are no args, let's just assume this is true
	if search_terms.len() == 0 {
		return true;
	}

	for term in search_terms {
		for tag in &record.Tags {
			if tag.to_lowercase().contains(term) {
				return true;
			}
		}
	}

	return false;
}
