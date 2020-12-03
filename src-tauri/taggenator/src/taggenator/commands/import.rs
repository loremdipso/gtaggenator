use crate::taggenator::commands::MyCustomError::UnknownError;
use crate::taggenator::database::searcher::Searcher;
use crate::taggenator::database::Database;
use crate::taggenator::errors::BError;
use crate::taggenator::models::record::Record;
use crate::taggenator::utils::input::readline;
use crate::Taggenator;
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
					let location = it.next().unwrap().to_string();
					let tag = it.next().unwrap().to_string();
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
					// date_added: Option<DateTime<chrono::Utc>>,
					let have_manually_touched = it.next().unwrap().to_string().parse::<bool>()?;
					let date_added = None;
					let date_created = None;
					let date_last_touched = None;
					let imported = true;

					taggenator.database.add_record_by_location_core(
						filename,
						location,
						size,
						length,
						times_opened,
						// date_added: Option<DateTime<chrono::Utc>>,
						date_added,
						date_created,
						date_last_touched,
						have_manually_touched,
						imported,
					)?;
				}

				_ => {}
			}
		}
	}

	return Ok(());
}
