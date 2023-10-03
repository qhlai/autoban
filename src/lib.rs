mod database;
mod func;
pub mod server;
pub mod filer_service;
pub mod utils;
use cidr_utils::cidr::IpCidr;
use serde::{Deserialize, Serialize};

use std::io::prelude::*;
pub fn pause() {
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}


#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Config {
    // pub key:String,
    pub key_admin: String,
    pub key_query: String,
    pub listen_ip: std::net::IpAddr,
    pub listen_port: u16,
    pub reverse_proxy: bool,
    pub filterlist:FilterList
}
impl Default for Config {
    fn default()->Self{
        // let key="passwordpassword";
        // let passwd=key.as_bytes().to_vec();
        Self { 
            // passwd: passwd.clone(), 
            // key:String::from("1234"),
            key_admin:String::from("adminpasswordpassword"),
            key_query:String::from("passwordpassword"),
            listen_ip:"0.0.0.0".parse().unwrap(),
            listen_port:3000,
            reverse_proxy:false,
            filterlist:FilterList::default()
            // filterlist:FilterList::new()
            // sysinfo: Sysinfo::new(&passwd)
        }
    }
}
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct FilterList {
    pub allow_ports: Vec<u16>,
    pub ban_ports: Vec<u16>,
    pub allow_ips: Vec<String>,
    pub ban_ips: Vec<String>,
    pub auto_add_threshold: u16,
    pub reject: bool,
    // pub deny_action:String,
    pub auto_reset:String,
    pub rate_trigger:String,
}

impl Default for FilterList {
    fn default()->Self{
        // let key="passwordpassword";
        // let passwd=key.as_bytes().to_vec();
        Self { 
            // passwd: passwd.clone(), 
            allow_ports:vec![],
            ban_ports:vec![],
            allow_ips:vec![],
            ban_ips:vec![],
            auto_add_threshold:0,
            reject:false,
            // deny_action:String::from("DROP"),
            auto_reset:String::from("m"),
            rate_trigger:String::from(""),
        }
    }
}