#![allow(dead_code)]
#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]
use std::env;
use taggenator::Taggenator;
mod tauri;

fn main() {
	let args: Vec<String> = env::args().skip(1).collect();
	let taggenator = Taggenator::new();
	if args.len() == 0 {
		tauri::start_tauri(taggenator)
	} else {
		taggenator.parse_args(args)
	}
}
