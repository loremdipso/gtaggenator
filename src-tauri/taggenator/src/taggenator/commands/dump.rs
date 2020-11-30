use crate::taggenator::database::searcher::Searcher;
use crate::taggenator::database::Database;
use crate::taggenator::errors::BError;
use crate::Taggenator;

pub fn dump(db: &mut Taggenator, args: Vec<String>) -> Result<(), BError> {
	let mut searcher = Searcher::new(args)?;

	let records = searcher.get_records(&db.database)?;

	// println!("Num records: {}", records.len());
	for record in records {
		println!("{}", record.Location);
	}

	return Ok(());
}
