use std::env;
use std::net::SocketAddr;

use warp::{self};

use crate::routes;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};


#[derive(Clone)]
pub struct AppState {
    pool: Pool<Postgres>
}

pub async fn start() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    println!("{}", database_url);
    let pool = match PgPoolOptions::new()
	.max_connections(10)
	.connect(&"postgres://postgres:qwerty@db/uni")
	.await
    {
	Ok(pool) => {
	    println!("Connected PostgresQL");
	    pool
	}
	Err(err) => {
	    println!("PostgresQL connection failed {}", err);
	    std::process::exit(1);
	}
    };
    
    let bind_address: SocketAddr = env::var("BIND_ADDRESS")
	.expect("BIND_ADDRESS is not set")
	.parse()
	.expect("BIND_ADDRESS is invalid");
    
    let app_state = AppState {
	pool
    };
    
    let routes = routes::routes(app_state);
    warp::serve(routes)
	.run(bind_address)
	.await;
}
