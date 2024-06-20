use axum::{routing::get_service, Router};
use tower_http::services::ServeDir;

use tokio::net::TcpListener;

mod routers;

use routers::check_health;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let routes_static = Router::new().nest_service("/static", get_service(ServeDir::new("static")));
    let app = Router::new()
        .merge(check_health::routes())
        .fallback_service(routes_static);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await
}
