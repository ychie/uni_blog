mod app;
mod handlers;
mod routes;


#[tokio::main]
async fn main() {
    app::start().await;
}
