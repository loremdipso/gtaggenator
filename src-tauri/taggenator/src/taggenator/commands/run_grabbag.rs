use crate::flags::take_flag_with_arg;
use crate::taggenator::commands::MyCustomError::UnknownError;
use crate::taggenator::database::searcher::Searcher;
use crate::taggenator::database::Database;
use crate::taggenator::errors::BError;
use crate::taggenator::models::record::Record;
use crate::taggenator::utils::commands::run_command_string;
use crate::taggenator::utils::input::readline;
use crate::Taggenator;
use chrono::format::ParseError;
use chrono::Utc;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use regex::Regex;
use serde_json::{Map, Value};
use std::convert::TryFrom;
use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
use threadpool::ThreadPool;
extern crate shell_words;
extern crate threadpool;

pub fn run_grabbag(taggenator: &mut Taggenator, mut args: Vec<String>) -> Result<(), BError> {
	let mut num_threads = take_flag_with_arg(&mut args, "--threads")
		.unwrap_or("1".to_string())
		.parse::<usize>()
		.unwrap();

	let mut exes_to_run: Vec<String> = vec![];
	loop {
		let mut exe = take_flag_with_arg(&mut args, "--exe");
		if exe.is_none() {
			exe = take_flag_with_arg(&mut args, "-exe");
		}

		if let Some(exe) = exe {
			exes_to_run.push(exe);
		} else {
			break;
		}
	}

	println!("Running grabbag exes: {:?}", &exes_to_run);

	let pool = ThreadPool::new(num_threads);

	let mut searcher = Searcher::new(args)?;
	let records = searcher.get_records(&taggenator.database)?;
	println!("Found {} files", &records.len());

	// Don't spawn unnecessary threads
	let total_jobs = records.len();
	num_threads = std::cmp::min(num_threads, total_jobs);

	let (tx, rx) = channel();
	let receiver = Arc::new(Mutex::new(rx));
	for i in 0..num_threads {
		let receiver = receiver.clone();
		let exes_to_run = exes_to_run.clone();
		pool.execute(move || {
			let mut taggenator = Taggenator::new_headless().unwrap();
			loop {
				// NOTE: need to split this onto two lines or else we keep the lock for too long
				let result = receiver.lock().unwrap().recv().unwrap();
				if let Some((job_num, mut record)) = result {
					println!("Worker #{}, job {} / {}", i, job_num, total_jobs);
					for exe in &exes_to_run {
						handle_exe(&mut taggenator, &mut record, &exe);
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

fn handle_exe(
	taggenator: &mut Taggenator,
	record: &mut Record,
	exe_path: &String,
) -> Result<(), BError> {
	let location = &record.Location;
	let location = std::fs::canonicalize(PathBuf::from(location))?;

	let temp_command = format!("{} \"{}\"", exe_path, location.to_str().unwrap());
	let temp_command = temp_command.deref();

	let result = run_command_string(&temp_command.to_string())?;
	let result = result.trim();
	if result.len() == 0 {
		return Ok(());
	}

	let parsed: Value = serde_json::from_str(&result)?;
	let obj: Map<String, Value> = parsed.as_object().unwrap().clone();

	for (key, value) in obj {
		println!("	Adding key: {}", &key);
		if let Value::String(value) = value {
			taggenator
				.database
				.grabbag_upsert(record.RecordID, key, value)?;
		}
	}

	return Ok(());
}
