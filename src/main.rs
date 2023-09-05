

use axum::{
    TypedHeader,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
    headers,
    extract::{Extension, ConnectInfo, State,rejection::{QueryRejection,TypedHeaderRejection}},
    http::{HeaderMap, header::HOST},
    // TypedHeader,

};
use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    net::SocketAddr,
    ops::Deref,
    error::Error,
    sync::{Arc, Mutex},
    rc::Rc
};

use::autoban::{
    filer_service::service,
    server::server::{start_server},

};
use::iptables;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let lock = Arc::new(Mutex::new(0));
    // let ipt = iptables::new(false).unwrap();
    // ipt.new_chain("nat", "NEWCHAINNAME");
    // build our application with a route
    start_server().await;

    // axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}

