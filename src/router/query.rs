use crate::search::Engine;
use lazy_static::lazy_static;

lazy_static! {
  static ref search_engine: Engine = Engine::new();
}

pub async fn query_handler(query: &str) -> &'static str {
  tracing::info!("incoming request to /query");

  let data = search_engine.search("深圳大学粤海校区");

  "querying"
}
