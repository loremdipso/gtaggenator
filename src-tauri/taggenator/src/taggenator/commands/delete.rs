use crate::taggenator::database::searcher::Searcher;
use crate::taggenator::database::Database;
use crate::taggenator::errors::BError;
use crate::Taggenator;

pub fn delete(db: &mut Taggenator, args: Vec<String>) -> Result<(), BError> {
	let mut searcher = Searcher::new(args)?;

	let records = searcher.get_records(&db.database)?;

	for record in records {
		println!("moving {}...", record.Location);
		std::fs::remove_file(record.Location);
	}

	return Ok(());
}
