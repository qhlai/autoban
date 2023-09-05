
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
use crate::server::handlers::{add,root};

use std::{
    collections::HashMap,
    net::SocketAddr,
    ops::Deref,
    error::Error,
    sync::{Arc, Mutex},
    rc::Rc
};

// #[tokio::main]
pub async fn start_server() {

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/api/add", get(add))
        .route("/api/list", get(root).post(root))
        .route("/api/remove", get(root))
        .route("/api/log", get(root));

        // `POST /users` goes to `create_user`
        // .route("/users", post(
        //     create_user));
        // .layer(axum::AddExtensionLayer::new(ConnectInfo::default()));

    // run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // tracing::debug!("listening on {}", listener.local_addr().unwrap());
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)    
    .serve(app.into_make_service_with_connect_info::<SocketAddr>())
    .await.unwrap();

}