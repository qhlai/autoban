
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
use tower_http::{trace::TraceLayer};

use serde::{Deserialize, Serialize};
// use crate::server::handlers::{add,root,ban,list,remove_whitelist,log_record};

use std::{
    collections::HashMap,
    net::SocketAddr,
    ops::Deref,
    error::Error,
    sync::{Arc, Mutex},
    rc::Rc
};
use crate::filer_service::service;
use crate::database::query;
use tower::ServiceBuilder;
// use::crates::{
//     filer_service::service,
//     // server::server::{start_server},

// };
// #[tokio::main]
//:Arc<Mutex<crate::Config>>
pub async fn start_server(config :Arc<Mutex<crate::Config>>, data:Arc<Mutex<service::FilterService>>,database:Arc<Mutex<query::Database>>) {

    // let cfg = data.clone();
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(crate::server::handlers::root))

        .route("/api/add", get(crate::server::handlers::add_whitelist))
        .route("/api/remove", get(crate::server::handlers::remove_whitelist))     
        .route("/api/list", get(crate::server::handlers::list_whitelist).post(crate::server::handlers::list_whitelist))  

        .route("/api/ban", get(crate::server::handlers::add_blacklist))
        .route("/api/unban", get(crate::server::handlers::remove_blacklist))
        .route("/api/uban", get(crate::server::handlers::remove_blacklist))

        .route("/api/listb", get(crate::server::handlers::list_blacklist).post(crate::server::handlers::list_blacklist))       
        .route("/api/listblacklist", get(crate::server::handlers::list_blacklist).post(crate::server::handlers::list_blacklist)) 

        .route("/api/resetw", get(crate::server::handlers::reset_whitelist))
        .route("/api/resetwhitelist", get(crate::server::handlers::reset_whitelist))

        .route("/api/reset", get(crate::server::handlers::reset_all))

        .route("/api/log", get(crate::server::handlers::get_record))
        
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(Extension(data))
                .layer(Extension(config.clone()))
                .layer(Extension(database.clone()))
        );

        // `POST /users` goes to `create_user`
        // .route("/users", post(
        //     create_user));
        // .layer(axum::AddExtensionLayer::new(ConnectInfo::default()));
    let addr:SocketAddr;
    {
        let cfg = &*config.lock().unwrap();
        addr = SocketAddr::from((cfg.listen_ip, cfg.listen_port));
    }
    // let cfg = &*config.lock().unwrap();
    // run our app with hyper
    // let addr = SocketAddr::from((cfg.listen_ip, cfg.listen_port));

    // tracing::debug!("listening on {}", listener.local_addr().unwrap());
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)    
    .serve(app.into_make_service_with_connect_info::<SocketAddr>())
    .await.unwrap();

}