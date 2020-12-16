use crate::taggenator::database::searcher::Searcher;
use crate::taggenator::database::Database;
use crate::taggenator::errors::BError;
use crate::taggenator::models::record::Record;
use crate::taggenator::utils::input::readline;
use crate::Taggenator;

pub fn open(taggenator: &mut Taggenator, args: Vec<String>) -> Result<(), BError> {
	let mut searcher = Searcher::new(args)?;
	let mut records = searcher.get_records(&taggenator.database)?;

	let index = 0;
	loop {
		if index < 0 || index > records.len() {
			break;
		}

		let record = &mut records[index];
		print_record(&record);
		let line = readline("Tag> ")?;
		taggenator.insert_tag_line(record, line, false);
	}

	for record in records {
		println!("{}", record.Location);
	}

	return Ok(());
}

fn print_record(record: &Record) {
	println!("\n\n{}", record.Name);
	for tag in &record.Tags {
		print!("{} |", &tag);
	}
}
