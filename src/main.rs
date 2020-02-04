#![deny(warnings)]
use warp::Filter;

#[tokio::main]
async fn main() {
    let sqrl_routes = warp::path("sqrl").and(warp::post()).map(|| "sqrl!");
    let graph_ql_get_routes = warp::path("graphql").and(warp::get()).map(|| "graphql!");
    let graph_ql_post_routes = warp::path("graphql").and(warp::post()).map(|| "graphql!");
    let health_routes = warp::path::end().and(warp::get()).map(|| "Hello, World!");

    warp::serve(
        sqrl_routes
            .or(graph_ql_get_routes)
            .or(graph_ql_post_routes)
            .or(health_routes),
    )
    .run(([127, 0, 0, 1], 3000))
    .await;
}
