
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs;
use uuid::Uuid;


#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Config {
    // pub key:String,
    pub key_admin: String,
    pub key_query: String,
    pub listen_ip: std::net::IpAddr,
    pub listen_port: u16,
    pub reverse_proxy: bool,
    pub filterlist: FilterList,
}
impl Default for Config {
    fn default() -> Self {
        // let key="passwordpassword";
        // let passwd=key.as_bytes().to_vec();
        Self {
            // passwd: passwd.clone(),
            // key:String::from("1234"),
            key_admin: String::from("adminpasswordpassword"),
            key_query: String::from("passwordpassword"),
            listen_ip: "0.0.0.0".parse().unwrap(),
            listen_port: 3000,
            reverse_proxy: false,
            filterlist: FilterList::default(), // filterlist:FilterList::new()
                                               // sysinfo: Sysinfo::new(&passwd)
        }
    }
}

pub fn from_str(content: &str) -> Option<Config> {
    let mut o = toml::from_str::<Config>(content).unwrap();
    // o.hosts_map = HashMap::new();

    // for (idx, host) in o.hosts.iter_mut().enumerate() {
    //     host.pos = idx;
    //     if host.alias.is_empty() {
    //         host.alias = host.name.to_owned();
    //     }
    //     if host.monthstart < 1 || host.monthstart > 31 {
    //         host.monthstart = 1;
    //     }
    //     host.weight = 10000_u64 - idx as u64;
    //     o.hosts_map.insert(host.name.to_owned(), host.clone());
    // }

    // for (idx, group) in o.hosts_group.iter_mut().enumerate() {
    //     group.pos = idx;
    //     group.weight = (10000 - (1 + idx) * 100) as u64;
    //     o.hosts_group_map.insert(group.gid.to_owned(), group.clone());
    // }

    // if o.offline_threshold < 30 {
    //     o.offline_threshold = 30;
    // }
    // if o.notify_interval < 30 {
    //     o.notify_interval = 30;
    // }
    // if o.group_gc < 30 {
    //     o.group_gc = 30;
    // }

    // if o.admin_user.is_none() || o.admin_user.as_ref()?.is_empty() {
    //     o.admin_user = Some("admin".to_string());
    // }
    // if o.admin_pass.is_none() || o.admin_pass.as_ref()?.is_empty() {
    //     o.admin_pass = Some(Uuid::new_v4().to_string());
    // }
    // if o.jwt_secret.is_none() || o.jwt_secret.as_ref()?.is_empty() {
    //     o.jwt_secret = Some(Uuid::new_v4().to_string());
    // }

    // eprintln!("✨ admin_user: {}", o.admin_user.as_ref()?);
    // eprintln!("✨ admin_pass: {}", o.admin_pass.as_ref()?);

    Some(o)
}


pub fn from_env() -> Option<Config> {
    from_str(
        env::var("SRV_CONF")
            .expect("can't load config from env `SRV_CONF")
            .as_str(),
    )
}

pub fn from_file(cfg: &str) -> Option<Config> {
    fs::read_to_string(cfg)
        .map(|contents| from_str(contents.as_str()))
        .ok()?
}

pub fn test_from_file(cfg: &str) -> Result<Config> {
    fs::read_to_string(cfg)
        .map(|contents| toml::from_str::<Config>(&contents))
        .unwrap()
        .map_err(anyhow::Error::new)
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
    pub auto_reset: String,
    pub rate_trigger: String,
}

impl Default for FilterList {
    fn default() -> Self {
        // let key="passwordpassword";
        // let passwd=key.as_bytes().to_vec();
        Self {
            // passwd: passwd.clone(),
            allow_ports: vec![],
            ban_ports: vec![],
            allow_ips: vec![],
            ban_ips: vec![],
            auto_add_threshold: 0,
            reject: false,
            // deny_action:String::from("DROP"),
            auto_reset: String::from("m"),
            rate_trigger: String::from(""),
        }
    }
}
