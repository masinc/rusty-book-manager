use crate::handler::book::{register_book, show_book, show_book_list};
use axum::routing::{get, post};
use axum::Router;
use registry::AppRegistry;

pub fn build_book_routes() -> Router<AppRegistry> {
    let book_routes = Router::new()
        .route("/", post(register_book))
        .route("/", get(show_book_list))
        .route("/:book_id", get(show_book));

    Router::new().nest("/books", book_routes)
}
