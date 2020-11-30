use crate::taggenator::database::searcher::Searcher;
use crate::taggenator::database::Database;
use crate::taggenator::errors::BError;
use crate::Taggenator;

pub fn dump_tags(db: &mut Taggenator, args: Vec<String>) -> Result<(), BError> {
	let mut searcher = Searcher::new(args)?;

	let tags = searcher.get_tags(&db.database)?;

	for tag in tags {
		println!("{}", tag);
	}

	return Ok(());
}
