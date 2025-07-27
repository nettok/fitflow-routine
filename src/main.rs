mod api;

use axum::Router;
use axum::response::Html;
use axum::routing::get;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app()).await.unwrap();
}

fn app() -> Router {
    Router::new().route("/", get(handle_index)).nest(
        "/api/v1",
        Router::new().route("/routines", get(api::get_routines)),
    )
}

async fn handle_index() -> Html<&'static str> {
    Html("OK")
}
