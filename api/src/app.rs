use std::env;
use std::net::SocketAddr;

use warp::{self};

use crate::routes;

#[derive(Clone)]
pub struct AppState {
    
}

pub async fn start() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");

    let bind_address: SocketAddr = env::var("BIND_ADDRESS")
	.expect("BIND_ADDRESS is not set")
	.parse()
	.expect("BIND_ADDRESS is invalid");
    
    let app_state = AppState { };
    
    let routes = routes::routes(app_state);
    warp::serve(routes)
	.run(bind_address)
	.await;
}
