use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use warp::cors;
use warp::http::header::{HeaderMap, HeaderValue};
use warp::Filter;
use zip::ZipArchive;

#[tokio::main]
pub async fn serve_fs() {
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

			let mut pages: Vec<usize> = vec![];
			for i in 0..archive_contents.len() {
				let mut file = archive_contents.by_index(i).unwrap();
				if file.is_file() && is_image(file.name()) {
					pages.push(i);
				}
			}

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
			// let path = Path::new("/home/madams/Pictures/face_game.png");
			// let f = std::fs::read(path).unwrap();
			println!(
				"Opening zip archive for: {:?}, {:?}",
				query.path, query.page_number
			);

			let mut archive_contents = get_archive(query.path.unwrap());
			let mut buffer = Vec::new();
			let mut archive_file: zip::read::ZipFile = archive_contents
				.by_index(query.page_number.unwrap())
				.unwrap();
			archive_file.read_to_end(&mut buffer);

			return buffer;
		}))
		.with(warp::reply::with::headers(headers));

	let cors = warp::cors().allow_any_origin();
	warp::serve(get_comic_info.or(get_comic_page).or(fs_route).with(cors))
		.run(([0, 0, 0, 0], 8000))
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
