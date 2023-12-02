// #![deny(warnings)]
// #![allow(unused)]

use std::{
    collections::HashMap,
    error::Error,
    net::SocketAddr,
    ops::Deref,
    process,
    rc::Rc,
    sync::atomic::{AtomicBool, Ordering},
    sync::{Arc, Mutex},
};

use axum::{
    extract::{
        rejection::{QueryRejection, TypedHeaderRejection},
        ConnectInfo, Extension, State,
    },
    handler,
    // TypedHeader,
    headers,
    http::StatusCode,
    http::{header::HOST, HeaderMap},
    response::IntoResponse,
    routing::{get, post},
    Json,
    Router,
    TypedHeader,
};
use clap::Parser;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use ::autoban::{config, database, filer_service::service, server::server::start_server};
use ::iptables;
use autoban::pause;
use ctrlc;
use tokio::signal;
use toml::Table;

static G_CONFIG: OnceCell<config::Config> = OnceCell::new();
// static G_STATS_MGR: OnceCell<crate::stats::StatsMgr> = OnceCell::new();
#[derive(Parser, Debug)]
#[command(author, version = env!("APP_VERSION"), about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "config.toml")]
    config: String,
    #[arg(short = 't', long, help = "config test, default:false")]
    config_test: bool,
    #[arg(long = "notify-test", help = "notify test, default:false")]
    notify_test: bool,
    #[arg(long = "cloud", help = "cloud mode, load cfg from env var: SRV_CONF")]
    cloud: bool,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let args = Args::parse();

    eprintln!("✨ {} {}", env!("CARGO_BIN_NAME"), env!("APP_VERSION"));

    // config test
    if args.config_test {
        config::test_from_file(&args.config).unwrap();
        eprintln!("✨ the conf file {} syntax is ok", &args.config);
        eprintln!("✨ the conf file {} test is successful", &args.config);
        std::process::exit(0);
    }

    // config load
    if let Some(cfg) = if args.cloud {
        // export SRV_CONF=$(cat config.toml)
        // echo "$SRV_CONF"
        eprintln!("✨ run in cloud mode, load config from env");
        config::from_env()
    } else {
        eprintln!(
            "✨ run in normal mode, load conf from local file `{}",
            &args.config
        );
        config::from_file(&args.config)
    } {
        log::debug!("{}", serde_json::to_string_pretty(&cfg).unwrap());
        G_CONFIG.set(cfg).unwrap();
    } else {
        log::error!("can't parse config");
        process::exit(1);
    }

    let config = G_CONFIG.get().unwrap();

    let config = Arc::new(Mutex::new(config.to_owned()));

    log::debug!("{:?}", config);
    let database = Arc::new(Mutex::new(database::query::Database::new()));

    // let running = Arc::new(AtomicBool::new(true));
    let data = Arc::new(Mutex::new(service::FilterService::new()));

    // let data = data.clone();
    data.lock().unwrap().load_config(&*config.lock().unwrap());

    data.lock().unwrap().start();

    let mut handles = Vec::new();

    let data_clone = data.clone();
    // a.lock().unwrap().clear_tables();
    // Arc<crate::config::Config>
    // let config: Arc<crate::config::Config> = Arc::new(*config);
    let config_clone = config.clone();
    handles.push(tokio::spawn(async move {
        start_server(config_clone, data_clone, database).await;
    }));

    // let data_clone = data.clone();

    // let tmp = f.clone();
    // handles.push(tokio::spawn(async move {
    //     let f = tmp.clone();
    //     let (tx, rx) = std::sync::mpsc::channel();
    //     ctrlc::set_handler(move || {
    //         r.store(false, Ordering::SeqCst);
    //         // let s= service.clone();
    //         // self.clear_tables();
    //         // self.clear_after_exit();
    //         tx.send(()).expect("Could not send signal on channel.");
    //         f.lock().unwrap().clear_tables();
    //         std::process::exit(0);

    //     }).expect("Error setting Ctrl-C handler");

    //     println!("Waiting for Ctrl-C...");
    //     rx.recv().expect("Could not receive from channel.");
    //     println!("Got it! Exiting...");
    //     rx.recv().expect("Could not receive from channel.");
    // }));
    // handles[0].await;
    for handler in handles {
        handler.await.unwrap();
    }

    // match signal::ctrl_c().await {
    //     Ok(()) => {
    //         // data_clone.lock().unwrap().clear_tables();
    //         println!("signal received, starting graceful shutdown");
    //         std::process::exit(0);
    //     }
    //     Err(err) => {
    //         eprintln!("Unable to listen for shutdown signal: {}", err);
    //         // we also shut down in case of error
    //     }
    // }
}
