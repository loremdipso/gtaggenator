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
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
use threadpool::ThreadPool;
extern crate shell_words;
extern crate threadpool;

pub fn apply_tags(taggenator: &mut Taggenator, mut args: Vec<String>) -> Result<(), BError> {
	let mut tags_to_add: Vec<String> = vec![];
	let num_threads = take_flag_with_arg(&mut args, "--threads")
		.unwrap_or("1".to_string())
		.parse::<usize>()
		.unwrap();

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

	let pool = ThreadPool::new(num_threads);

	let mut searcher = Searcher::new(args)?;
	let records = searcher.get_records(&taggenator.database)?;
	println!("Found {} files", &records.len());

	let (tx, rx) = channel();
	let receiver = Arc::new(Mutex::new(rx));
	let total_jobs = records.len();
	for i in 0..num_threads {
		let receiver = receiver.clone();
		let tags_to_add = tags_to_add.clone();
		pool.execute(move || {
			let mut taggenator = Taggenator::new_headless().unwrap();
			loop {
				// NOTE: need to split this onto two lines or else we keep the lock for too long
				let result = receiver.lock().unwrap().recv().unwrap();
				if let Some((job_num, mut record)) = result {
					println!("Worker #{}, job {} / {}", i, job_num, total_jobs);
					for tag in &tags_to_add {
						taggenator.insert_tag_line(&mut record, tag.to_string());
					}
				} else {
					break;
				}
			}
		});
	}

	let mut job_id = 0;
	for mut record in records {
		job_id += 1;
		tx.send(Some((job_id, record)));
	}

	// close threads, or try to anyway
	for _ in 0..num_threads {
		tx.send(None);
	}

	pool.join();

	return Ok(());
}
