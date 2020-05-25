#![deny(warnings)]
extern crate config;
extern crate juniper;
extern crate log;
extern crate rustache;
extern crate serde;
use rustache::{HashBuilder, Render, RustacheError};
use std::env;
use warp::Filter;

#[derive(Debug, serde::Serialize)]
struct BaseUrls {
    cps: String,
    login: String,
    poll: String,
    success: String,
}

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

fn index_view(_addr: std::net::SocketAddr) -> Result<String, RustacheError> {
    let buj = BaseUrls {
        cps: "https://cps".to_owned(),
        login: "https://login".to_owned(),
        poll: "https://poll".to_owned(),
        success: "https://success".to_owned(),
    };
    let data = HashBuilder::new()
        .insert("baseUrlsJson", serde_json::to_string(&buj).unwrap())
        .insert("baseUrl", "https://baseUrl")
        .insert("nutJson", r#"{"nut":"","code":""}"#);
    let mut body = std::io::Cursor::new(Vec::new());
    let template = std::include_str!("./views/index.mustache");
    data.render(template, &mut body)?;
    let vec = body.into_inner();
    Ok(String::from_utf8(vec).unwrap())
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
    let graphql_route = warp::path("graphql")
        .and(warp::path::end())
        .and(warp::post2())
        .and(juniper_warp::make_graphql_filter(schema, state));
    let playground_route = warp::path("graphql")
        .and(warp::path::end())
        .and(warp::get2())
        .and(juniper_warp::playground_filter("/graphql"));
    let root_route = warp::path::end()
        .and(warp::filters::addr::remote())
        .map(|addr: Option<std::net::SocketAddr>| warp::reply::html(index_view(addr.unwrap()).unwrap()));

    let sqrl_routes = warp::path("sqrl")
        .and(warp::path::end())
        .and(warp::post2())
        .map(|| "sqrl!");

    warp::serve(
        sqrl_routes
            .or(root_route)
            .or(playground_route)
            .or(graphql_route)
            .with(warp_logger),
    )
    .run(addr);
}
