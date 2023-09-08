
use std::{
    collections::HashMap,
    net::SocketAddr,
    ops::Deref,
    error::Error,
    sync::{Arc, Mutex},
    rc::Rc,
    net::IpAddr,
};
use log;
pub fn iferror(x: Result<(), Box<dyn std::error::Error>>) {
    match x {
        Ok(_) => {
            // log::debug!("ok")
            // log::error!("")
        },
        Err(e) => {
            log::error!("error: {}", e)
            // log::error!("")
        },
    }
    // println!("{}",
}