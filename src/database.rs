

// use axum::{
//     TypedHeader,
//     http::StatusCode,
//     response::IntoResponse,
//     routing::{get, post},
//     Json, Router,
//     headers,
//     extract::{Extension, ConnectInfo, State,rejection::{QueryRejection,TypedHeaderRejection}},
//     http::{HeaderMap, header::HOST},
//     // TypedHeader,

// };
// use serde::{Deserialize, Serialize};

// use std::{
//     collections::HashMap,
//     net::SocketAddr,
//     ops::Deref,
//     error::Error,
//     sync::{Arc, Mutex},
//     rc::Rc
// };

// #[derive(serde::Deserialize)]
// pub struct Config {
//     pub key:String,
//     pub admin_key: String,
//     pub query_key: String,
//     pub listen_port: u32,
//     pub auto_add_threshold: u8,
//     pub reject: bool,
//     pub reverse_proxy: bool,
//     // pub filterlist:FilterList
// }
// impl Config {
//     pub fn new()->Config{
//         // let key="passwordpassword";
//         // let passwd=key.as_bytes().to_vec();
//         Config { 
//             // passwd: passwd.clone(), 
//             key:String::from("1234"),
//             admin_key:String::from("1234"),
//             query_key:String::from("123"),
//             listen_port:8081,
//             auto_add_threshold:0,
//             reject:false,
//             reverse_proxy:false,
//             // filterlist:FilterList::new()
//             // sysinfo: Sysinfo::new(&passwd)
//         }
//     }
// }
// pub struct FilterList {
//     pub allow_ports: Vec<u32>,
//     pub ban_ports: Vec<u32>,
//     pub allow_ips: Vec<std::net::IpAddr>,
//     pub ban_ips: Vec<std::net::IpAddr>,
// }
// impl FilterList {
//     pub fn new()->FilterList{
//         // let key="passwordpassword";
//         // let passwd=key.as_bytes().to_vec();
//         FilterList { 
//             // passwd: passwd.clone(), 
//             allow_ports:vec![],
//             ban_ports:vec![],
//             allow_ips:vec![],
//             ban_ips:vec![],

//         }
//     }
//     pub fn load()->FilterList{
//         // let key="passwordpassword";
//         // let passwd=key.as_bytes().to_vec();
//         FilterList { 
//             // passwd: passwd.clone(), 
//             allow_ports:vec![],
//             ban_ports:vec![],
//             allow_ips:vec![],
//             ban_ips:vec![],

//         }
//     }
//     pub fn save()->FilterList{
//         // let key="passwordpassword";
//         // let passwd=key.as_bytes().to_vec();
//         FilterList { 
//             // passwd: passwd.clone(), 
//             allow_ports:vec![],
//             ban_ports:vec![],
//             allow_ips:vec![],
//             ban_ips:vec![],

//         }
//     }
// }
// #[tokio::main]
// async fn adsan() {
//   let params = Params::new();
// }

