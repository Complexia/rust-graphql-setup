use serde_json::json;
use warp::filters::BoxedFilter;
use warp::reply::json;
use warp::{Filter, Rejection, Reply};

async fn inception() -> Result<impl Reply, Rejection> {
    Ok(json(&json!({"ok": true})))
}

pub fn make_routes() -> BoxedFilter<(impl Reply,)> {
    let inception = warp::path::end().and_then(inception);
    inception.boxed()
}
