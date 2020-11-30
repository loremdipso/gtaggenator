mod dump;
mod dump_tags;
use crate::taggenator::database::Database;
use crate::taggenator::errors::BError;
use crate::taggenator::errors::MyCustomError;
use crate::Taggenator;

pub fn RunCommand(
	taggenator: &mut Taggenator,
	command_name: String,
	args: Vec<String>,
) -> Result<(), BError> {
	match &command_name[..] {
		"dump" => {
			return dump::dump(taggenator, args);
		}
		"dump_tags" => {
			return dump_tags::dump_tags(taggenator, args);
		}
		_ => {
			return Err(Box::new(MyCustomError::InvalidCommand {
				name: command_name,
			}));
		}
	}
	return Ok(());
}
