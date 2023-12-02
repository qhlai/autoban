use axum::{
    extract::{
        rejection::{QueryRejection, TypedHeaderRejection},
        ConnectInfo, Extension, State,
    },
    headers,
    http::StatusCode,
    http::{Method, Uri},
    response::IntoResponse,
    http::{header::HOST, HeaderMap},
    // TypedHeader,
    // response::IntoResponse,
    routing::{get, post},
    Json,
    Router,
    TypedHeader,
};
use tower_http::trace::TraceLayer;

use serde::{Deserialize, Serialize};
// use crate::server::handlers::{add,root,ban,list,remove_whitelist,log_record};

use crate::{database::query, config};
use crate::filer_service::service;
use std::{
    collections::HashMap,
    error::Error,
    net::SocketAddr,
    ops::Deref,
    rc::Rc,
    sync::{Arc, Mutex},
};
use tower::ServiceBuilder;
use tokio::signal;
// use::crates::{
//     filer_service::service,
//     // server::server::{start_server},

// };
// #[tokio::main]
//:Arc<Mutex<crate::config::Config>>


pub async fn shutdown_signal(data: Arc<Mutex<service::FilterService>>) {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
    data.lock().unwrap().clear_tables();

    std::process::exit(0);
}

pub async fn start_server(
    // config: Arc<crate::config::Config>,
    config: Arc<Mutex<crate::config::Config>>,
    // config: &'static crate::config::Config,
    data: Arc<Mutex<service::FilterService>>,
    database: Arc<Mutex<query::Database>>,
) {
    // let cfg = data.clone();
    let data_clone = data.clone();
    // let cors_layer = CorsLayer::new()
    // .allow_origin(Any)
    // .allow_methods([Method::GET, Method::POST]);
    // let arc_ptr: Arc<i32> = Arc::new(value);

    let app = Router::new()
        // `GET /` goes to `root`
        // .route("/", get(crate::server::handlers::root))
        .route("/", get(crate::assets::index_handler))
        .route("/api/add", get(crate::server::handlers::add_whitelist))
        .route(
            "/api/remove",
            get(crate::server::handlers::remove_whitelist),
        )
        .route(
            "/api/list",
            get(crate::server::handlers::list_whitelist)
                .post(crate::server::handlers::list_whitelist),
        )
        .route("/api/ban", get(crate::server::handlers::add_blacklist))
        .route("/api/unban", get(crate::server::handlers::remove_blacklist))
        .route(
            "/api/listb",
            get(crate::server::handlers::list_blacklist)
                .post(crate::server::handlers::list_blacklist),
        )
        .route(
            "/api/listblacklist",
            get(crate::server::handlers::list_blacklist)
                .post(crate::server::handlers::list_blacklist),
        )
        .route("/api/resetw", get(crate::server::handlers::reset_whitelist))
        .route(
            "/api/resetwhitelist",
            get(crate::server::handlers::reset_whitelist),
        )
        .route("/api/reset", get(crate::server::handlers::reset_all))
        .route("/api/log", get(crate::server::handlers::get_record))
        .fallback(fallback)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(Extension(data_clone))
                .layer(Extension(config.clone()))
                .layer(Extension(database.clone())),
        );

    // `POST /users` goes to `create_user`
    // .route("/users", post(
    //     create_user));
    // .layer(axum::AddExtensionLayer::new(ConnectInfo::default()));
    let addr: SocketAddr;
    {
        let config = &*config.lock().unwrap();
        addr = SocketAddr::from((config.listen_ip, config.listen_port));
    }
    // let cfg = &*config.lock().unwrap();
    // run our app with hyper
    // let addr = SocketAddr::from((cfg.listen_ip, cfg.listen_port));

    // tracing::debug!("listening on {}", listener.local_addr().unwrap());
    tracing::info!("listening on {}", addr);
    let data_clone = data.clone();
    // let borrowed = data.as_ref().unwrap().lock().unwrap();

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal(data_clone))
        .await
        .unwrap();
}

async fn fallback(uri: Uri) -> impl IntoResponse {
    crate::assets::static_handler(uri).await
}