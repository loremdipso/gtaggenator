use crate::taggenator::commands::MyCustomError::UnknownError;
use crate::taggenator::database::searcher::Searcher;
use crate::taggenator::database::Database;
use crate::taggenator::errors::BError;
use crate::taggenator::models::record::Record;
use crate::taggenator::utils::input::readline;
use crate::Taggenator;
extern crate shell_words;

pub fn grab_bag(taggenator: &mut Taggenator, args: Vec<String>) -> Result<(), BError> {
	if args.len() > 0 {
		run_grab_bag(taggenator, args)?;
	} else {
		// TODO: interactive version
		println!("TODO");
		loop {
			match readline("> ") {
				Ok(line) => {
					println!("{}", line);
					run_grab_bag(taggenator, shell_words::split(&line)?)?;
				}

				Err(_) => {
					break;
				}
			};
		}
	}

	return Ok(());
}

pub fn run_grab_bag(taggenator: &mut Taggenator, args: Vec<String>) -> Result<(), BError> {
	let mut command: Option<String> = None;
	let mut location: Option<String> = None;
	let mut key: Option<String> = None;
	let mut value: Option<String> = None;

	for arg in args {
		if command.is_none() {
			match &arg[..] {
				"add" | "remove" | "get" | "get_all" => command = Some(arg),
				_ => return Err(Box::new(UnknownError)),
			}
		} else if location.is_none() {
			location = Some(arg);
		} else if key.is_none() {
			key = Some(arg);
		} else if value.is_none() {
			value = Some(arg);
		}

		if let Some(ref actual_command) = command {
			match &actual_command[..] {
				"add" => {
					if location.is_some() && key.is_some() && value.is_some() {
						taggenator.database.grabbag_upsert_by_location(
							location.take().unwrap(),
							key.take().unwrap(),
							value.take().unwrap(),
						)?;
					}
				}

				"remove" => {
					if location.is_some() && key.is_some() {
						taggenator.database.grabbag_delete_by_location(
							location.take().unwrap(),
							key.take().unwrap(),
						)?;
					}
				}

				"get" => {
					if location.is_some() && key.is_some() {
						let value = taggenator.database.grabbag_get_by_location(
							location.take().unwrap(),
							key.take().unwrap(),
						)?;
						println!("Value: {}", value);
					}
				}

				"get_all" => {
					if location.is_some() {
						let value = taggenator
							.database
							.grabbag_get_all_by_location(location.take().unwrap())?;
						println!("Values: {:?}", value);
					}
				}

				_ => {}
			}
		}
	}

	return Ok(());
}
