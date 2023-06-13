mod crawler;
mod router;
mod search;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();
  crawler::spider::main().await.unwrap();

  let app = router::entry();
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  tracing::info!("Listening on http://{:#?}", addr);

  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}
