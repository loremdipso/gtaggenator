use crate::flags::take_flag_with_arg;
use crate::taggenator::commands::MyCustomError::UnknownError;
use crate::taggenator::database::searcher::Searcher;
use crate::taggenator::database::Database;
use crate::taggenator::errors::BError;
use crate::taggenator::models::record::Record;
use crate::taggenator::utils::input::readline;
use crate::Taggenator;
use chrono::format::ParseError;
use chrono::Utc;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use std::convert::TryFrom;
use std::path::Path;
extern crate shell_words;

pub fn apply_tags(taggenator: &mut Taggenator, mut args: Vec<String>) -> Result<(), BError> {
	let mut tags_to_add: Vec<String> = vec![];
	loop {
		let mut tag = take_flag_with_arg(&mut args, "--tag");
		if tag.is_none() {
			tag = take_flag_with_arg(&mut args, "-tag");
		}

		if let Some(tag) = tag {
			tags_to_add.push(tag);
		} else {
			break;
		}
	}

	println!("Adding tags: {:?}", &tags_to_add);

	let mut searcher = Searcher::new(args)?;
	let records = searcher.get_records(&taggenator.database)?;
	println!("Found {} files", &records.len());

	for mut record in records {
		for tag in &tags_to_add {
			taggenator.insert_tag_line(&mut record, tag.to_string());
		}
	}

	return Ok(());
}
