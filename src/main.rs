

use autoban::pause;
use axum::{
    TypedHeader,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
    headers,
    extract::{Extension, ConnectInfo, State,rejection::{QueryRejection,TypedHeaderRejection}},
    http::{HeaderMap, header::HOST}, handler,
    // TypedHeader,

};
use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    net::SocketAddr,
    ops::Deref,
    error::Error,
    sync::{Arc, Mutex},
    sync::atomic::{AtomicBool, Ordering},
    rc::Rc
};

use::autoban::{
    filer_service::service,
    database,
    server::server::{start_server},
    Config,
};
use::iptables;
use ctrlc;
use tokio::signal;
use toml::Table;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();
    let file_path = "./config.toml";
    let config = std::fs::read_to_string(file_path).unwrap_or_else(|error| {
        // 操作失败，处理错误值
        log::error!("Error reading file: {}", error);
        // 返回一个默认值
        std::process::exit(1);
        // String::from("")
    });

    // let config: Config = toml::from_str(r#" "#).unwrap_or_else(|error| {
    let config: Config = toml::from_str(&config[..]).unwrap_or_else(|error| {
        // 操作失败，处理错误值
        log::error!("Error reading file: {}", error);
        // 返回一个默认值
        std::process::exit(1);
    });
    let config = Arc::new(Mutex::new(config));

    log::debug!("{:?}",config);
    let database = Arc::new(Mutex::new(database::query::Database::new()));

    // let running = Arc::new(AtomicBool::new(true));    
    let filter_config = Arc::new(Mutex::new(service::FilterService::new()));
    

    let filter_config = filter_config.clone();
    filter_config.lock().unwrap().load_config(config.lock().unwrap().clone());
    filter_config.lock().unwrap().start();
    let mut handles = Vec::new();

    let filter_config_clone = filter_config.clone();
    // a.lock().unwrap().clear_tables();
    handles.push(  tokio::spawn(async move {
        start_server(config,filter_config_clone, database).await;
    }));
    let filter_config_clone = filter_config.clone();

    match signal::ctrl_c().await {
        Ok(()) => {
            filter_config_clone.lock().unwrap().clear_tables();
            std::process::exit(0);
        },
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        },
    }

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
    for handler in handles{
        handler.await.unwrap();
    }


}

