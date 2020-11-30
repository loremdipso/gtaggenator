use crate::taggenator::database::Database;
use crate::taggenator::errors::BError;
use crate::taggenator::models::record::Record;
use rusqlite::NO_PARAMS;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use string_builder::Builder;

const RECORD_ID_INDEX: usize = 0;
const RECORD_NAME_INDEX: usize = 1;
const TAG_INDEX: usize = 7;
const LOCATION_INDEX: usize = 2;

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
		// let query = &"\nSelect * From Records";
		let query = &self.format_query(&query);
		// let query = format!(
		// 	"{} Left Join Tags On Records.RecordID = Tags.RecordID",
		// 	query
		// );
		// println!("{}", &query);
		let mut stmt = db.conn.prepare(&query)?;

		// TODO: how can we make this less bad?
		let mut records: Vec<Record> = vec![];
		let mut current_record: Option<Record> = None;
		let mut rows = stmt.query(&self.get_query_args())?;
		loop {
			let row = rows.next()?;
			match row {
				None => break,
				Some(row) => {
					let recordID: i32 = row.get(0)?;
					let mut should_create = true;
					if let Some(ref mut record) = current_record {
						if record.RecordID == recordID {
							if let Ok(tag) = row.get(TAG_INDEX) {
								record.Tags.push(tag);
								should_create = false;
							}
						} else {
							records.push(current_record.take().unwrap());
						}
					}

					if should_create {
						let mut record = Record {
							RecordID: row.get(RECORD_ID_INDEX)?,
							Name: row.get(RECORD_NAME_INDEX)?,
							Location: row.get(LOCATION_INDEX)?,
							Tags: vec![],
						};
						if let Ok(tag) = row.get(TAG_INDEX) {
							record.Tags.push(tag);
						}
						current_record = Some(record);
					}
				}
			}
		}

		if let Some(ref record) = current_record {
			records.push(current_record.take().unwrap());
		}

		return Ok(records);
	}

	fn get_query_args(&self) -> Vec<String> {
		let mut rv = vec![];
		for filter in &self.filters {
			rv.extend(filter.args.clone());
		}
		return rv;
	}

	fn format_query(&mut self, query: &str) -> String {
		let mut sql = query.to_string();

		let mut is_first = true;
		for mut filter in &mut self.filters.drain(..) {
			if filter.sqlizable() {
				sql = filter.sqlize(sql, is_first);
				is_first = false;
			} else {
				break;
			}
		}

		// println!("{}", &sql);
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
			"search" | "search_inclusive" | "search_exclusive" => return true,
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

					rv.append(" (");
					rv.append(format!("(Records.Location LIKE \"%{}%\")", &arg));

					rv.append("\nor ");

					rv.append(format!(
						"(\"{}\" in (
	SELECT Tags.TagName from Tags
	WHERE Tags.RecordID = Records.RecordID))",
						&arg
					));
					rv.append("\n)");
				}
				self.args.clear();
				rv.append(")");
				return format!("{} {}", sql, rv.string().unwrap());
			}

			_ => return sql,
		};
	}
}
