use serde::Deserialize;
use std::path::Path;
use warp::http::header::{HeaderMap, HeaderValue};
use warp::Filter;

#[tokio::main]
pub async fn serve_fs() {
	// NOTE: this seems to work fine, but should we use actix-web instead?
	let route = warp::path("static").and(warp::fs::dir("."));
	let route = route.with(warp::log("warp-server"));

	// TODO: maybe we don't need separate threads
	// let route = route.or(route);

	warp::serve(route).run(([0, 0, 0, 0], 8000)).await;
}

#[tokio::main]
pub async fn comic_handler() {
	let mut headers = HeaderMap::new();
	headers.insert("Content-Type", HeaderValue::from_static("application/json"));
	headers.insert(
		"Content-Disposition",
		HeaderValue::from_static("attachement; filename = \"modified.json\""),
	);

	let route = warp::path("comic")
		// .and(warp::body::concat())
		// .map(move |body| export_json(body))
		.and(warp::query().map(|query: ComicQuery| {
			let path = Path::new("/home/madams/Pictures/face_game.png");
			let f = std::fs::read(path).unwrap();
			return f;
			// warp::fs::file(path_export_json)
			// return warp::fs::file(Path::new("/home/madams/Pictures/face_game.png"));
			// return format!("query = {:?}", query);
		}))
		.with(warp::reply::with::headers(headers));

	// let route = warp::path("comic")
	// 	.and(warp::path::end())
	// 	.and(warp::query().map(|query: ComicQuery| {
	// 		let path = Path::new("/home/madams/Pictures/face_game.png").unwrap();
	// 		let f = std::fs::read_to_string(path).unwrap();
	// 		headers.insert("Content-Disposition", HeaderValue::from_static("attachement; filename = \"modified.json\""));
	// 		warp::fs::file(path_export_json)
	// 		// return warp::fs::file(Path::new("/home/madams/Pictures/face_game.png"));
	// 		// return format!("query = {:?}", query);
	// 	}));

	warp::serve(route).run(([0, 0, 0, 0], 8001)).await;
}

#[derive(Deserialize, Debug)]
struct ComicQuery {
	path: Option<String>,
	get_pages: Option<bool>,
}

// struct Png {
//     inner: std::io::Result<File>,
// }

// impl Png {

//     pub fn new(path: &Path) -> Self {
//          Png {inner : File::open(path)}
//     }
// }

// impl Reply for Png {
//     #[inline]
//     fn into_response(self) -> warp::reply::Response {
//         match self.inner {
//             Ok(mut file) => {
//                 let mut data : Vec<u8> = Vec::new();
//                     match file.read_to_end(&mut data) {
//                         Err(why) => {
//                             println!("Error: {:?}", why);
//                             return Response::new(String::new().into());
//                         }
//                         Ok(_) => {
//                             let mut res = Response::new(data.into());
//                             res.headers_mut()
//                                 .insert(CONTENT_TYPE, HeaderValue::from_static("image/png"));
//                             return res;
//                         },
//                     }
//             }
//             Err(why) => {
//                 println!("Error: {:?}", why);
//                 return Response::new(String::new().into());
//             }
//         }
//     }
// }
