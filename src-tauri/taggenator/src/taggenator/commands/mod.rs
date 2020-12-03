use crate::taggenator::database::Database;
use crate::taggenator::errors::BError;
use crate::taggenator::errors::MyCustomError;
use crate::Taggenator;

mod dump;
mod dump_tags;
mod grab_bag;
mod import;
mod open;

pub fn RunCommand(
	taggenator: &mut Taggenator,
	command_name: String,
	args: Vec<String>,
) -> Result<(), BError> {
	match &command_name[..] {
		"dump" => {
			return dump::dump(taggenator, args);
		}
		"grabbag" | "grab_bag" => {
			return grab_bag::grab_bag(taggenator, args);
		}
		"import" => {
			return import::import(taggenator, args);
		}
		"dump_tags" => {
			return dump_tags::dump_tags(taggenator, args);
		}
		"open" => {
			return open::open(taggenator, args);
		}
		_ => {
			return Err(Box::new(MyCustomError::InvalidCommand {
				name: command_name,
			}));
		}
	}
	return Ok(());
}
