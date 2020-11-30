use crate::taggenator::database::Database;
use crate::taggenator::errors::BError;
use crate::taggenator::models::record::Record;
use rusqlite::NO_PARAMS;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use string_builder::Builder;

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

	pub fn get_records(self, db: &Database) -> Result<Vec<Record>, BError> {
		dbg!(self.filters);
		return Ok(vec![]);
	}

	pub fn get_tags(&mut self, db: &Database) -> Result<HashSet<String>, BError> {
		let mut query = "SELECT Tags.TagName FROM Tags".to_string();
		if self.filters.len() > 0 {
			query += &"\nJoin Records ON Records.RecordID = Tags.RecordID";
		}

		let mut stmt = db.conn.prepare(&self.format_query(&query))?;
		let rows = stmt.query_map(&self.get_query_args(), |row| row.get(0))?;

		let mut names = HashSet::new();
		for name_result in rows {
			names.insert(name_result?);
		}
		Ok(names)
	}

	fn get_query_args(&self) -> Vec<String> {
		let mut rv = vec![];
		for filter in &self.filters {
			rv.extend(filter.args.clone());
		}
		return rv;
	}

	fn format_query(&mut self, query: &str) -> String {
		let mut rv = Builder::default();
		rv.append(query);
		for mut filter in &mut self.filters {
			rv.append("\n");
			rv.append(filter.to_sql());
		}

		rv.append(";");
		let rv = rv.string().unwrap();
		println!("{}", &rv);
		return rv;
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

	fn to_sql(&mut self) -> String {
		// TODO: figure out escaping
		let rv = match &self.name[..] {
			"search" => {
				let mut rv = Builder::default();
				rv.append("and ( ");
				for (i, arg) in self.args.iter().enumerate() {
					if i > 0 {
						rv.append("\nand ");
					}

					rv.append(" (");
					rv.append(format!("(Records.Location LIKE \"%{}%\")", &arg));

					rv.append("\nor ");

					rv.append(format!(
						"(\"{}\" in (
	SELECT Tags.TagName from Tags
	WHERE Tags.RecordID = Records.RecordID
))",
						&arg
					));
					rv.append("\n)");
				}
				self.args.clear();
				rv.append(")");
				rv.string().unwrap()
			}

			_ => "".to_string(),
		};
		return rv;
	}
}
