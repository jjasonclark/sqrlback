#![deny(warnings)]
use warp::Filter;

fn getUrls() -> warp::Reply {
   warp::reply::html(r"urls!")
}

#[tokio::main]
async fn main() {
    let sqrl_routes = warp::path("sqrl").and(warp::post()).map(|| "sqrl!");
    let graph_ql_get_routes = warp::path("graphql").and(warp::get()).map(|| "graphql!");
    let graph_ql_post_routes = warp::path("graphql").and(warp::post()).map(|| "graphql!");
    let url_routes = warp::path("urls").and(warp::get()).map(|| getUrls());
    let health_routes = warp::path::end().and(warp::get()).map(|| "Hello, World!");

    warp::serve(
        sqrl_routes
            .or(graph_ql_get_routes)
            .or(graph_ql_post_routes)
            .or(url_routes)
            .or(health_routes),
    )
    .run(([127, 0, 0, 1], 3000))
    .await;
}
