use std::env;
use warp::{cors, path, Filter, Rejection, Reply};
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let default_port: u16 = 3000;
    let port = env::var("PORT")
        .unwrap_or(default_port.to_string())
        .parse::<u16>()
        .unwrap_or(default_port);

    let health_route = path!("health").and_then(health_handler);

    let hello = path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let routes = health_route.or(hello).with(cors().allow_any_origin());
    let address = ([0, 0, 0, 0], port);
    warp::serve(routes).run(address).await;
}

async fn health_handler() -> Result<impl Reply, Rejection> {
    Ok("ok")
}
