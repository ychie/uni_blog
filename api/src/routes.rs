use warp::{self, Filter};

use crate::app::AppState;
use crate::handlers;

pub fn routes(
    state: AppState,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
	.map(handlers::index::index)
}
