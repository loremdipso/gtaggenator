// TODO: remove
#![allow(dead_code, warnings, unused)]
#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]
use std::env;
use taggenator::Taggenator;
mod tauri;

fn main() {
	let args: Vec<String> = env::args().skip(1).collect();
	if args.len() == 0 {
		let mut taggenator = Taggenator::new_headless().unwrap();
		tauri::start_tauri(taggenator);
	} else {
		let mut taggenator = Taggenator::new().unwrap();
		taggenator.parse_args(args);
	}
}
