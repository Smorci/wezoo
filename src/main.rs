#![deny(warnings)]

use tokio;
use pretty_env_logger;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let frontend = warp::fs::dir("out");

    warp::serve(frontend)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

#[cfg(test)]
mod tests {

    use http::{Request};

    #[test]
    fn frontend_http_get() {
        let request = Request::builder()
            .method("GET")
            .uri("http://localhost:3030/")
            .unwrap();
    }
}
