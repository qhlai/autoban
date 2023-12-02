use axum::{
    extract::{
        rejection::{QueryRejection, TypedHeaderRejection},
        ConnectInfo, Extension, State,
    },
    headers,
    http::StatusCode,
    http::{header::HOST, HeaderMap},
    // TypedHeader,
    response::IntoResponse,
    routing::{get, post},
    Json,
    Router,
    TypedHeader,
};
use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    error::Error,
    net::SocketAddr,
    ops::Deref,
    rc::Rc,
    sync::{Arc, Mutex},
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
