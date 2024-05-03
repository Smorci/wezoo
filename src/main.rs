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

#[tokio::test]
async fn test_dir_matches() {
    let filter = warp::fs::dir("out");

    assert!(
        warp::test::request()
            .path("/")
            .matches(&filter)
            .await
        );
}
