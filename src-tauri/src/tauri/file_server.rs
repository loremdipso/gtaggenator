use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::{cmp::Ordering, collections::HashMap};
use warp::cors;
use warp::http::header::{HeaderMap, HeaderValue};
use warp::Filter;
use zip::ZipArchive;

#[tokio::main]
pub async fn serve_fs(port: u16) {
	// NOTE: this seems to work fine, but should we use actix-web instead?
	let fs_route = warp::path("static")
		.and(warp::fs::dir("."))
		.with(warp::log("warp-server"));

	// open the zip_archive locally (TODO: are we locking the file? IS that a problem?)
	// TODO: actually use this mutex
	let current_path: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
	let zip_archive: Arc<Mutex<Option<ZipArchive<File>>>> = Arc::new(Mutex::new(None));

	let get_comic_info =
		warp::path("get_comic_info").and(warp::query().map(|query: ComicQuery| {
			let mut archive_contents = get_archive(query.path.unwrap());

			let mut pages_map: HashMap<usize, String> = HashMap::new();
			// let mut pages: Vec<usize> = vec![];
			for i in 0..archive_contents.len() {
				let mut file = archive_contents.by_index(i).unwrap();
				if file.is_file() && is_image(file.name()) {
					pages_map.insert(i, file.name().to_string());
				}
			}

			// Make sure to sort by:
			// 	- numerical rep of name
			//  - name
			//  - then the index in the zip
			let mut pages: Vec<usize> = pages_map.keys().cloned().collect();
			let cmp = |l: &usize, r: &usize| {
				let l_name = &pages_map[l];
				let r_name = &pages_map[r];
				return chain_ordering(
					get_leading_number(&l_name).cmp(&get_leading_number(&r_name)),
					chain_ordering(l_name.cmp(r_name), l.cmp(r)),
				);
			};
			pages.sort_by(cmp);

			let mut rv = HashMap::new();
			rv.insert("pages", pages);
			let rv = serde_json::to_string(&rv).unwrap();

			return rv;
		}));

	let mut headers = HeaderMap::new();
	headers.insert("Content-Type", HeaderValue::from_static("application/json"));
	headers.insert(
		"Content-Disposition",
		HeaderValue::from_static("attachement; filename = \"modified.json\""),
	);

	let get_comic_page = warp::path("get_comic_page")
		.and(warp::query().map(|query: ComicQuery| {
			let path = query.path.unwrap();
			let page_number = query.page_number.unwrap();
			println!("Opening zip archive: {}, page: {}", path, page_number);

			let mut archive_contents = get_archive(path);
			let mut buffer = Vec::new();
			let mut archive_file: zip::read::ZipFile =
				archive_contents.by_index(page_number).unwrap();
			archive_file.read_to_end(&mut buffer);

			return buffer;
		}))
		.with(warp::reply::with::headers(headers));

	let cors = warp::cors().allow_any_origin();
	warp::serve(get_comic_info.or(get_comic_page).or(fs_route).with(cors))
		.run(([0, 0, 0, 0], port))
		.await;
}

fn get_archive(path: String) -> ZipArchive<File> {
	let path = path.to_string();
	let path = Path::new(&path);
	let file = File::open(path).unwrap();

	let mut archive_contents: zip::read::ZipArchive<std::fs::File> =
		zip::ZipArchive::new(file).unwrap();
	return archive_contents;
}

#[derive(Deserialize, Debug)]
struct ComicQuery {
	path: Option<String>,
	page_number: Option<usize>,
}

const IMAGE_EXTENSIONS: [&str; 5] = ["jpg", "png", "jpeg", "gif", "svg"];
fn is_image(name: &str) -> bool {
	for ext in IMAGE_EXTENSIONS.iter() {
		if name.ends_with(ext) {
			return true;
		}
	}
	return false;
}

fn get_leading_number(string: &str) -> usize {
	let res = string
		.chars()
		.take_while(|c| c.is_digit(10))
		.collect::<String>();
	return usize::from_str_radix(&res, 10).unwrap_or_default();
}

#[test]
fn test_get_leading_number() {
	assert_eq!(4, get_leading_number("4and then something else"));
	assert_eq!(42, get_leading_number("42and then something else"));
	assert_eq!(0, get_leading_number("something else then 4"));
	assert_eq!(0, get_leading_number("something else 4 then more"));
}

fn chain_ordering(o1: Ordering, o2: Ordering) -> Ordering {
	match o1 {
		Ordering::Equal => o2,
		_ => o1,
	}
}
