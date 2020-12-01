use warp::Filter;

#[tokio::main]
pub async fn serve_fs() {
	// NOTE: this seems to work fine, but should we use actix-web instead?
	let route = warp::path("static").and(warp::fs::dir("."));
	let route = route.with(warp::log("warp-server"));
	warp::serve(route).run(([0, 0, 0, 0], 8000)).await;
}
