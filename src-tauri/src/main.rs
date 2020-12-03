// TODO: remove
#![allow(dead_code, warnings, unused)]
#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]
use std::env;
use taggenator::flags::take_flag;
use taggenator::Taggenator;
mod tauri;

fn main() {
	let mut args: Vec<String> = env::args().skip(1).collect();
	let mut is_headless = take_flag(&mut args, "--headless");

	if is_headless || args.len() == 0 {
		let mut taggenator = Taggenator::new_headless().unwrap();
		tauri::start_tauri(taggenator);
	} else {
		let mut taggenator = Taggenator::new().unwrap();
		if let Err(error) = taggenator.parse_args(args) {
			dbg!(error);
		}
	}
}
