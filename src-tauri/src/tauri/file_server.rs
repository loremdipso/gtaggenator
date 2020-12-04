use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use warp::cors;
use warp::http::header::{HeaderMap, HeaderValue};
use warp::Filter;

#[tokio::main]
pub async fn serve_fs() {
	// NOTE: this seems to work fine, but should we use actix-web instead?
	let fs_route = warp::path("static")
		.and(warp::fs::dir("."))
		.with(warp::log("warp-server"));

	let get_comic_info =
		warp::path("get_comic_info").and(warp::query().map(|query: ComicQuery| {
			let mut rv = HashMap::new();
			rv.insert("pages", 30); // TODO
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

			let path = query.path.unwrap().to_string();
			let path = Path::new(&path);
			let file = File::open(path).unwrap();

			let mut archive_contents: zip::read::ZipArchive<std::fs::File> =
				zip::ZipArchive::new(file).unwrap();

			let mut buffer = Vec::new();
			let mut archive_file: zip::read::ZipFile = archive_contents.by_index(0).unwrap();
			archive_file.read_to_end(&mut buffer);
			return buffer;
		}))
		.with(warp::reply::with::headers(headers));

	let cors = warp::cors().allow_any_origin();
	warp::serve(get_comic_info.or(get_comic_page).or(fs_route).with(cors))
		.run(([0, 0, 0, 0], 8000))
		.await;
}

#[derive(Deserialize, Debug)]
struct ComicQuery {
	path: Option<String>,
	page_number: Option<i32>,
}
