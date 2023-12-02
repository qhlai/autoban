use log;
use std::{
    collections::HashMap,
    error::Error,
    net::IpAddr,
    net::SocketAddr,
    ops::Deref,
    rc::Rc,
    sync::{Arc, Mutex},
};
pub fn iferror(x: Result<(), Box<dyn std::error::Error>>) {
    match x {
        Ok(_) => {
            // log::debug!("ok")
            // log::error!("")
        }
        Err(e) => {
            log::error!("error: {}", e)
            // log::error!("")
        }
    }
    // println!("{}",
}
