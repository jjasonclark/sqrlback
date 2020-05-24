#![deny(warnings)]
extern crate config;
extern crate juniper;
extern crate log;
use std::env;
use warp::Filter;

struct Query;
struct Mutation;

struct Context;

impl juniper::Context for Context {}

#[juniper::object(Context = Context)]
impl Query {
    fn hello() -> &'static str {
        "Hello, world!"
    }
}

#[juniper::object(Context = Context)]
impl Mutation {
    fn hello(context: &Context, text: String) -> String {
        format!("hello, {}!", text)
    }
}

type Schema = juniper::RootNode<'static, Query, Mutation>;

fn load_config() -> Result<config::Config, config::ConfigError> {
    let mut settings = config::Config::new();
    settings.merge(config::File::with_name("config/default"))?;
    settings.merge(config::File::with_name("config/local").required(false))?;
    Ok(settings)
}

#[tokio::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    let settings = load_config().unwrap();
    let address = settings.get("server.address").unwrap();
    let port = settings.get("server.port").unwrap();
    let addr = std::net::SocketAddr::new(address, port);
    let warp_logger = warp::log("web");

    let schema = Schema::new(Query, Mutation);
    let state = warp::any().map(move || Context {}).boxed();
    let graphql_route = warp::path("graphql").and(juniper_warp::make_graphql_filter(schema, state));
    let playground_route = warp::path("graphql")
        .and(warp::get2())
        .and(juniper_warp::playground_filter("/graphql"));
    let root_route = warp::get2()
        .and(warp::path::end())
        .map(|| warp::reply::html("<html><body><a href=\"/graphql\">Graphql</a></body></html>"));

    let sqrl_routes = warp::path("sqrl").and(warp::post2()).map(|| "sqrl!");

    warp::serve(
        sqrl_routes
            .or(root_route)
            .or(graphql_route)
            .or(playground_route)
            .with(warp_logger),
    )
    .run(addr);
}
