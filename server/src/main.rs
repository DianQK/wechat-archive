use axum::{
    http::StatusCode,
    routing::{get, get_service},
    Router,
};
use axum_extra::routing::SpaRouter;
use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::str::FromStr;
use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use tower::ServiceBuilder;
use tower_http::{services::ServeDir, trace::TraceLayer};

#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate lazy_static;

use rbatis::rbatis::Rbatis;

pub mod database;
mod handlers;
mod merge;
pub mod utils;

lazy_static! {
    pub static ref RB: Rbatis = Rbatis::new();
}

// Setup the command line interface with clap.
#[derive(Parser, Debug)]
#[clap(name = "wechat-archive", about = "微信归档后端服务")]
struct Opt {
    /// set the log level
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,

    /// set the listen addr
    #[clap(short = 'a', long = "addr", default_value = "0.0.0.0")]
    addr: String,

    /// set the listen port
    #[clap(short = 'p', long = "port", default_value = "3000")]
    port: u16,

    /// set the directory where static files are to be found
    #[clap(long = "static-dir", default_value = "../dist")]
    static_dir: String,

    #[clap(long = "assets-dir", default_value = "./assets")]
    assets_dir: String,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Merge { path: String },
}

#[derive(Deserialize, Debug)]
struct Config {
    password: HashMap<String, String>,
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }

    // tracing_subscriber::fmt::init();
    fast_log::init(fast_log::config::Config::new().console()).expect("日志初始化失败");
    RB.link("mysql://root:wechat-archive@localhost:3306/wechat-archive")
        .await
        .unwrap();
    RB.exec(include_str!("sql/init.sql"), vec![]).await.unwrap();

    match opt.command {
        Some(subcommand) => {
            match subcommand {
                Commands::Merge { path } => {
                    let mut file = File::open("wechat-archive.toml").expect("文件打开失败");
                    let mut config_string = String::new();
                    file.read_to_string(&mut config_string).expect("读取失败");
                    let c: Config = toml::from_str(&config_string).unwrap();
                    let micro_msg_source = path; // /path/MicroMsg
                    let merge_micro_msg =
                        merge::MergeMicroMsg::new(&micro_msg_source, &opt.assets_dir, &c.password);
                    merge_micro_msg.merge().await.expect("迁移失败");
                }
            }
        }
        None => {
            let app = Router::new()
                .route("/api/users", get(handlers::user::get_users))
                .route(
                    "/api/conversations/:username",
                    get(handlers::conversation::get_conversations),
                )
                .route(
                    "/api/messages/:owner/:talker",
                    get(handlers::messages::get_messages),
                )
                .nest(
                    "/assets",
                    get_service(ServeDir::new(opt.assets_dir)).handle_error(
                        |error: std::io::Error| async move {
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                format!("Unhandled internal error: {}", error),
                            )
                        },
                    ),
                )
                .merge(SpaRouter::new("/static", opt.static_dir))
                .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

            let sock_addr = SocketAddr::from((
                IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
                opt.port,
            ));

            log::info!("listening on {}", sock_addr);
            axum::Server::bind(&sock_addr)
                .serve(app.into_make_service())
                .await
                .expect("Unable to start server");
        }
    }
}
