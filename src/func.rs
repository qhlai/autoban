

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

#[derive(Debug)]
pub enum Protocol {
    ALL,
    TCP,
    UDP,
}
impl Protocol {
    fn tcp_prefix(&self) -> String {
        match &self {
            Protocol::ALL => "".to_string(),
            Protocol::TCP => "".to_string(),
            Protocol::UDP => "#".to_string(),
        }
    }
    fn udp_prefix(&self) -> String {
        match &self {
            Protocol::ALL => "".to_string(),
            Protocol::TCP => "#".to_string(),
            Protocol::UDP => "".to_string(),
        }
    }
}

impl From<Protocol> for String {
    fn from(protocol: Protocol) -> Self {
        match protocol {
            Protocol::UDP => "udp".into(),
            Protocol::TCP => "tcp".into(),
            Protocol::ALL => "all".into(),
        }
    }
}

impl From<String> for Protocol {
    fn from(protocol: String) -> Self {
        match protocol {
            protocol if protocol == "udp" => Protocol::UDP,
            protocol if protocol == "UDP" => Protocol::UDP,
            protocol if protocol == "tcp" => Protocol::TCP,
            protocol if protocol == "TCP" => Protocol::TCP,
            _ => Protocol::ALL,
        }
    }
}