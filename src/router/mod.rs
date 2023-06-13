mod ping;
mod query;

use axum::{
  routing::{get, post},
  Router,
};
use ping::ping_handler;
use query::query_handler;

pub fn entry() -> Router {
  let app = Router::new().route("/ping", get(ping_handler));
  // .route("/query", post(query_handler));

  app
}
