
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

pub async fn add(
    method: axum::http::Method,
    axum::extract::ConnectInfo(addr): axum::extract::ConnectInfo<SocketAddr>,   
    axum::extract::Query(para): axum::extract::Query<HashMap<String, String>>,    
    user_agent: Result<TypedHeader<axum::headers::UserAgent>,TypedHeaderRejection>,

    xrealip: Result<TypedHeader<XRealIp>,TypedHeaderRejection>,
    xforwardfor: Result<TypedHeader<XForwardedFor>,TypedHeaderRejection>,
) -> impl IntoResponse {

    println!("{}, {:?}, {:?}", method, addr, user_agent);
    // println!("{}, {:?}, {:?}", method, addr, user_agent.unwrap());

    // println!("{}, {:?}, {:?}, {:?}", method, addr, user_agent, realip);
    let ip =get_client_ip(false,addr.ip(),xrealip, xforwardfor);
 
    println!("ip:{:?}",ip);

    // location /api {
    //     proxy_set_header  X-real-ip $remote_addr;
    //     proxy_set_header  X-Forwarded-For $proxy_add_x_forwarded_for;
    //     proxy_pass http://127.0.0.1:8080/api;
    //  }

    // let mut key:String = String::from("value");
    let key  =para.get("key").unwrap_or(&"".to_string()).clone();

    (StatusCode::OK, key)
}
pub async fn root() -> &'static str {
    "Hello, World!"
}

fn get_client_ip(
    reverse_proxy: bool,
    addr: std::net::IpAddr,     
    xrealip: Result<TypedHeader<XRealIp>,TypedHeaderRejection>,
    xforwardfor: Result<TypedHeader<XForwardedFor>,TypedHeaderRejection>
) -> std::net::IpAddr{
    let mut ip:std::net::IpAddr;
    // let reverse =false;
    if !reverse_proxy {
        ip=addr;
    }
    else{
        let mut ok_reverse = false;
        
        match xrealip {
            Ok(xrealip) => {
                ip=xrealip.0.0;
                ok_reverse=true;
                println!("xforwardfor: {}",  ip)
            },
            Err(err) =>  {
                ip = std::net::IpAddr::from([127, 0, 0, 1]);
                println!("{:?}",err.to_string())
            },
        }
        if ok_reverse{

        }else{

            match xforwardfor {
                Ok(xforwardfor) => {
                    let xforwardfor = &xforwardfor.0.0[..];

                    let comma_index = xforwardfor.find(','); // 查找逗号的索引
                    if let Some(index) = comma_index {
                        let substring = &xforwardfor[..index]; // 使用切片操作获取逗号之前的部分
                        println!("substring: {}", substring);
                    } else {
                        println!("Comma not found");
                    }
                    println!("xforwardfor: {}",  xforwardfor);                    
                },
                Err(err) =>  {
                    ip = std::net::IpAddr::from([127, 0, 0, 1]);
                    println!("{:?}",err.to_string())
                },
            }
            
        }
    }
    println!("ip:{:?}",ip);
    ip
}




#[derive(Debug)]
pub struct XForwardedFor(String);
static XFORWARDED_FOR: axum::http::HeaderName = axum::http::HeaderName::from_static("x-forwarded-for");
impl axum::headers::Header for XForwardedFor {
     fn name() -> &'static axum::http::HeaderName {
         &XFORWARDED_FOR
     }

     fn decode<'i, I>(values: &mut I) -> Result<Self, axum::headers::Error>
     where
         I: Iterator<Item = &'i axum::headers::HeaderValue>,
     {
         let value = values.next().ok_or_else(axum::headers::Error::invalid)?;

         let real_ip_str =
             std::str::from_utf8(value.as_bytes()).map_err(|_| axum::headers::Error::invalid())?;

         let real_ip = real_ip_str.parse().map_err(|_| axum::headers::Error::invalid())?;

         Ok(XForwardedFor(real_ip))
     }

     fn encode<E>(&self, values: &mut E)
     where
         E: Extend<axum::http::HeaderValue>,
     {
         let value = axum::headers::HeaderValue::from_str(&self.0.to_string());

         values.extend(std::iter::once(value.unwrap()));
     }
}


#[derive(Debug)]
pub struct XRealIp(std::net::IpAddr);
static XREAL_IP: axum::http::HeaderName = axum::http::HeaderName::from_static("x-real-ip");
impl axum::headers::Header for XRealIp {
     fn name() -> &'static axum::http::HeaderName {
         &XREAL_IP
     }

     fn decode<'i, I>(values: &mut I) -> Result<Self, axum::headers::Error>
     where
         I: Iterator<Item = &'i axum::headers::HeaderValue>,
     {
         let value = values.next().ok_or_else(axum::headers::Error::invalid)?;

         let real_ip_str =
             std::str::from_utf8(value.as_bytes()).map_err(|_| axum::headers::Error::invalid())?;
         let real_ip = real_ip_str.parse().map_err(|_| axum::headers::Error::invalid())?;

         Ok(XRealIp(real_ip))
     }

     fn encode<E>(&self, values: &mut E)
     where
         E: Extend<axum::http::HeaderValue>,
     {
         let value = axum::headers::HeaderValue::from_str(&self.0.to_string());

         values.extend(std::iter::once(value.unwrap()));
     }
}