use crate::taggenator::database::Database;
use crate::taggenator::errors::BError;
use crate::taggenator::errors::MyCustomError;
use crate::Taggenator;

mod apply_tags;
mod delete;
mod dump;
mod dump_tags;
mod grab_bag;
mod import;
mod move_record;
mod open;
pub mod open_all; // expose to outside
mod run_grabbag;

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
		"open_all" => {
			return open_all::open_all(taggenator, args);
		}
		"apply_tags" => {
			return apply_tags::apply_tags(taggenator, args);
		}
		"run_grabbag" => {
			return run_grabbag::run_grabbag(taggenator, args);
		}
		"delete" => {
			return delete::delete(taggenator, args);
		}
		"move" => {
			return move_record::move_record(taggenator, args);
		}
		_ => {
			return Err(Box::new(MyCustomError::InvalidCommand {
				name: command_name,
			}));
		}
	}
	return Ok(());
}
