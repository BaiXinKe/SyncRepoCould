mod payload;

use axum::{
    extract::Json, http::StatusCode, response::IntoResponse, routing::post, Router, Server,
};
use payload::Payload;

async fn get_web_hook(Json(payload): Json<Payload>) -> impl IntoResponse {
    println!("{:?}", payload);
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    let router = Router::new().route("/git-web-hook", post(get_web_hook));

    let addr = ("127.0.0.1:6421")
        .parse()
        .expect("Failed to parse as socketaddr");

    let _ = Server::bind(&addr).serve(router.into_make_service()).await;
}
