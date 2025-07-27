mod api;

use axum::Router;
use axum::response::Html;
use axum::routing::{get, put};

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app()).await.unwrap();
}

fn app() -> Router {
    Router::new().route("/", get(handle_index)).nest(
        "/api/v1",
        Router::new()
            .route("/routines/by-goal", get(api::routines::get_routines_by_goal))
            .route("/routines/goals", get(api::routines::get_goals))
            .route("/assignments/accept", put(api::assignments::assignment_accept))
            .route("/assignments/start", put(api::assignments::assignment_start))
            .route("/assignments/complete", put(api::assignments::assignment_complete)),
    )
}

async fn handle_index() -> Html<&'static str> {
    Html("OK")
}
