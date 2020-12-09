use crate::taggenator::commands::MyCustomError::UnknownError;
use crate::taggenator::database::searcher::Searcher;
use crate::taggenator::database::Database;
use crate::taggenator::errors::BError;
use crate::taggenator::models::record::Record;
use crate::taggenator::utils::input::readline;
use crate::Taggenator;
use chrono::format::ParseError;
use chrono::Utc;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use std::convert::TryFrom;
use std::path::Path;
extern crate shell_words;

pub fn import(taggenator: &mut Taggenator, args: Vec<String>) -> Result<(), BError> {
	if args.len() > 0 {
		run_import(taggenator, args)?;
	} else {
		// TODO: interactive version
		loop {
			match readline("> ") {
				Ok(line) => {
					run_import(taggenator, shell_words::split(&line)?)?;
				}

				Err(_) => {
					break;
				}
			};
		}
	}

	return Ok(());
}

pub fn run_import(taggenator: &mut Taggenator, args: Vec<String>) -> Result<(), BError> {
	dbg!(&args);

	let mut it = args.iter().peekable();
	loop {
		if it.peek().is_none() {
			break;
		}

		let command = it.next();
		if let Some(ref actual_command) = command {
			match &actual_command[..] {
				"add_tag" => {
					println!("Adding tag!");
					let location = it.next().unwrap().to_string();
					let tag = it.next().unwrap().to_string();
					println!("Adding tag: {}", &tag);
					taggenator.database.add_tag_by_location(location, tag)?;
				}

				"add_record" => {
					let location = it.next().unwrap().to_string();
					let filename = Path::new(&location)
						.file_name()
						.unwrap()
						.to_string_lossy()
						.to_string();
					let size = it.next().unwrap().to_string().parse::<i64>()?;
					let length = it.next().unwrap().to_string().parse::<i64>()?;
					let times_opened = it.next().unwrap().to_string().parse::<i64>()?;
					let have_manually_touched = it.next().unwrap().to_string().parse::<bool>()?;

					// date_added: Option<DateTime<chrono::Utc>>,
					// dbg!(&date_added);
					let date_added = get_date(&it.next().unwrap().to_string())?;
					let date_created = get_date(&it.next().unwrap().to_string())?;
					let date_last_touched = get_date(&it.next().unwrap().to_string())?;
					let imported = true;

					taggenator.database.add_record_by_location(
						filename,
						location,
						size,
						length,
						times_opened,
						// date_added,
						None,
						date_created,
						date_last_touched,
						have_manually_touched,
						imported,
					)?;

					println!("success!");
				}

				_ => {}
			}
		}
	}

	return Ok(());
}

fn get_date(date_str: &String) -> Result<Option<DateTime<chrono::Utc>>, BError> {
	println!("{}", &date_str);
	if let Ok(date) = DateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S.%f %z %Z") {
		let utc_date: DateTime<Utc> = DateTime::<Utc>::from(date);
		return Ok(Some(utc_date));
	} else {
		let date = DateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S %z %Z")?;
		let utc_date: DateTime<Utc> = DateTime::<Utc>::from(date);
		return Ok(Some(utc_date));
	}
}
