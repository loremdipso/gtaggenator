use crate::taggenator::database::searcher::Searcher;
use crate::taggenator::database::Database;
use crate::taggenator::errors::BError;
use crate::taggenator::flags::take_flag_with_arg;
use crate::Taggenator;
use std::path::Path;

pub fn move_record(taggenator: &mut Taggenator, mut args: Vec<String>) -> Result<(), BError> {
	let destination = take_flag_with_arg(&mut args, "-destination")
		.unwrap_or_else(|| take_flag_with_arg(&mut args, "--destination").unwrap());

	let destination_path = Path::new(&destination);
	if !destination_path.exists() {
		panic!(format!("{} does not exist", &destination));
	}

	if !destination_path.is_dir() {
		panic!(format!("{} is not a folder", &destination));
	}

	let mut searcher = Searcher::new(args)?;
	let records = searcher.get_records(&taggenator.database)?;

	for record in records {
		let new_location = Path::new(&destination).join(record.Name);
		let old_location = Path::new(&record.Location);

		if new_location == old_location {
			println!("NOT moving {}", &record.Location)
		} else {
			println!(
				"moving {} to {}...",
				record.Location,
				new_location.to_string_lossy()
			);
			std::fs::rename(Path::new(&record.Location), new_location);
		}
	}

	return Ok(());
}
