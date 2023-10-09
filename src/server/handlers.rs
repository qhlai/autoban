
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
use crate::{filer_service::service, server::check_key};

pub async fn add_whitelist(
    method: axum::http::Method,
    axum::extract::ConnectInfo(addr): axum::extract::ConnectInfo<SocketAddr>,   
    axum::extract::Query(para): axum::extract::Query<HashMap<String, String>>,    
    user_agent: Result<TypedHeader<axum::headers::UserAgent>,TypedHeaderRejection>,

    xrealip: Result<TypedHeader<XRealIp>,TypedHeaderRejection>,
    xforwardfor: Result<TypedHeader<XForwardedFor>,TypedHeaderRejection>,
    data: axum::extract::Extension<Arc<Mutex<service::FilterService>>>,
    config: axum::extract::Extension<Arc<Mutex<crate::Config>>>,
) -> impl IntoResponse {

    println!("{}, {:?}, {:?}", method, addr, user_agent);

    let query_ip =get_client_ip(false,addr.ip(),xrealip, xforwardfor);
    println!("query_ip: {:?}",query_ip);

    let key  =para.get("key").unwrap_or(&"".to_string()).clone();
    if check_key(&key[..], true, String::from("add"), &*config.lock().unwrap()){
        
        let mut data = data.0.lock().unwrap();
        match para.get("ip") {
            Some(ip)=>{
                match ip.parse() {
                    Ok(ip) =>{
                        // final_ip=ip;
                        log::info!("client: {query_ip} add whitelisted {ip}");
                        // let mut data = data.0.lock().unwrap();
                        data.add_whitelisted_ip(ip);            
                        return (StatusCode::OK, "ok");
                    },
                    Err(err) =>{
                        log::warn!("err: {err}, param ip is {ip}");
                        log::info!("client: {query_ip} add whitelisted {query_ip}");
                        
                        data.add_whitelisted_ip(query_ip); 

                        return (StatusCode::OK, "ok");
                    }
                }
            },
            None =>{
                log::info!("client: {query_ip} add whitelisted {query_ip}");
                // let mut data = data.0.lock().unwrap();
                data.add_whitelisted_ip(query_ip); 

                return (StatusCode::OK, "ok");
            }
        }
    }else {
        return (StatusCode::UNAUTHORIZED, "UNAUTHORIZED");
    }

}

pub async fn add_blacklist(
    method: axum::http::Method,
    axum::extract::ConnectInfo(addr): axum::extract::ConnectInfo<SocketAddr>,   
    axum::extract::Query(para): axum::extract::Query<HashMap<String, String>>,    
    user_agent: Result<TypedHeader<axum::headers::UserAgent>,TypedHeaderRejection>,

    xrealip: Result<TypedHeader<XRealIp>,TypedHeaderRejection>,
    xforwardfor: Result<TypedHeader<XForwardedFor>,TypedHeaderRejection>,
    data: axum::extract::Extension<Arc<Mutex<service::FilterService>>>,
    config: axum::extract::Extension<Arc<Mutex<crate::Config>>>,
) -> impl IntoResponse {
    println!("{}, {:?}, {:?}", method, addr, user_agent);

    let query_ip =get_client_ip(false,addr.ip(),xrealip, xforwardfor);

    let key  =para.get("key").unwrap_or(&"".to_string()).clone();
    if check_key(&key[..], true, String::from("ban"), &*config.lock().unwrap()){
        let mut data = data.0.lock().unwrap();

        match para.get("ip") {
            Some(ip)=>{
                match ip.parse() {
                    Ok(ip) =>{
                        log::info!("client: {query_ip} add blacklisted {ip}");
                        data.add_blacklisted_ip(ip);            
                        return (StatusCode::OK, "ok");
                    },
                    Err(err) =>{
                        log::warn!("err: {err}, param ip is {ip}");
                        return (StatusCode::BAD_REQUEST, "bad request");
                    }
                }
            },
            None =>{
                return (StatusCode::BAD_REQUEST, "bad request");
            }
        }
    }else {
        return (StatusCode::UNAUTHORIZED, "UNAUTHORIZED");
    }
}
pub async fn list_whitelist(
    method: axum::http::Method,
    axum::extract::ConnectInfo(addr): axum::extract::ConnectInfo<SocketAddr>,   
    axum::extract::Query(para): axum::extract::Query<HashMap<String, String>>,    
    user_agent: Result<TypedHeader<axum::headers::UserAgent>,TypedHeaderRejection>,

    xrealip: Result<TypedHeader<XRealIp>,TypedHeaderRejection>,
    xforwardfor: Result<TypedHeader<XForwardedFor>,TypedHeaderRejection>,
    data: axum::extract::Extension<Arc<Mutex<service::FilterService>>>,
    config: axum::extract::Extension<Arc<Mutex<crate::Config>>>,
    database: axum::extract::Extension<Arc<Mutex<crate::database::query::Database>>>,
) -> impl IntoResponse {
    println!("{}, {:?}, {:?}", method, addr, user_agent);

    let query_ip =get_client_ip(false,addr.ip(),xrealip, xforwardfor);

    let key  =para.get("key").unwrap_or(&"".to_string()).clone();
    if check_key(&key[..], false, String::from("list_whitelist"), &*config.lock().unwrap()){
        let mut data = data.0.lock().unwrap();
        let mut database = database.0.lock().unwrap();
        
        // final_ip=ip;
        log::info!("client: {query_ip} list_whitelist");
        // data.del_whitelisted_ip(ip);            
        let mut output=String::new();
        let records = data.get_whitelist_data();
        output+=&format!("[whitelist] has  {} ip  gen by: {query_ip}\n",records.len())[..];
        output+=&format!("ip            packets_out   packets_in    bandwidth_out bandwidth_in  \n")[..];

        for record in records.iter() {
            let ip_location = database.query_ip(&record.ip.first_as_ip_addr()).unwrap();
            output+=&format!("{:width$} {:width$} {:width$} {:width$} {:width$} {:width$}\n",record.ip, record.packets_out, record.packets_in, record.bandwidth_out, record.bandwidth_in,ip_location,width=10)[..];
        }
        log::debug!("{output}");
        return (StatusCode::OK, output.to_string());

    }else {
        return (StatusCode::UNAUTHORIZED, "UNAUTHORIZED".to_string());
    }

}
pub async fn remove_whitelist(
    method: axum::http::Method,
    axum::extract::ConnectInfo(addr): axum::extract::ConnectInfo<SocketAddr>,   
    axum::extract::Query(para): axum::extract::Query<HashMap<String, String>>,    
    user_agent: Result<TypedHeader<axum::headers::UserAgent>,TypedHeaderRejection>,

    xrealip: Result<TypedHeader<XRealIp>,TypedHeaderRejection>,
    xforwardfor: Result<TypedHeader<XForwardedFor>,TypedHeaderRejection>,
    data: axum::extract::Extension<Arc<Mutex<service::FilterService>>>,
    config: axum::extract::Extension<Arc<Mutex<crate::Config>>>,
) -> impl IntoResponse {

    println!("{}, {:?}, {:?}", method, addr, user_agent);

    let query_ip =get_client_ip(false,addr.ip(),xrealip, xforwardfor);

    let key  =para.get("key").unwrap_or(&"".to_string()).clone();
    if check_key(&key[..], false, String::from("remove_whitelist"), &*config.lock().unwrap()){
        let mut data = data.0.lock().unwrap();

        match para.get("ip") {
            Some(ip)=>{
                match ip.parse() {
                    Ok(ip) =>{
                        // final_ip=ip;
                        log::info!("client: {query_ip} remove_whitelist {ip}");
                        data.del_whitelisted_ip(ip);            
                        return (StatusCode::OK, "ok");
                    },
                    Err(err) =>{
                        log::warn!("err: {err}, param ip is {ip}"); 
                        return (StatusCode::BAD_REQUEST, "bad request");
                    }
                }
            },
            None =>{
                log::info!("client: {query_ip} add whitelisted {query_ip}");
                return (StatusCode::BAD_REQUEST, "bad request");
            }
        }
    }else {
        return (StatusCode::UNAUTHORIZED, "UNAUTHORIZED");
    }
}

pub async fn remove_blacklist(
    method: axum::http::Method,
    axum::extract::ConnectInfo(addr): axum::extract::ConnectInfo<SocketAddr>,   
    axum::extract::Query(para): axum::extract::Query<HashMap<String, String>>,    
    user_agent: Result<TypedHeader<axum::headers::UserAgent>,TypedHeaderRejection>,

    xrealip: Result<TypedHeader<XRealIp>,TypedHeaderRejection>,
    xforwardfor: Result<TypedHeader<XForwardedFor>,TypedHeaderRejection>,
    data: axum::extract::Extension<Arc<Mutex<service::FilterService>>>,
    config: axum::extract::Extension<Arc<Mutex<crate::Config>>>,
) -> impl IntoResponse {

    println!("{}, {:?}, {:?}", method, addr, user_agent);

    let query_ip =get_client_ip(false,addr.ip(),xrealip, xforwardfor);

    let key  =para.get("key").unwrap_or(&"".to_string()).clone();
    if check_key(&key[..], false, String::from("remove_blacklist"), &*config.lock().unwrap()){
        let mut data = data.0.lock().unwrap();

        match para.get("ip") {
            Some(ip)=>{
                match ip.parse() {
                    Ok(ip) =>{
                        // final_ip=ip;
                        log::info!("client: {query_ip} remove_blacklist {ip}");
                        data.del_blacklisted_ip(ip);            
                        return (StatusCode::OK, "ok");
                    },
                    Err(err) =>{
                        log::warn!("err: {err}, param ip is {ip}"); 
                        return (StatusCode::BAD_REQUEST, "bad request");
                    }
                }
            },
            None =>{
                // log::info!("client: {query_ip} add whitelisted {query_ip}");
                return (StatusCode::BAD_REQUEST, "bad request");
            }
        }
    }else {
        return (StatusCode::UNAUTHORIZED, "UNAUTHORIZED");
    }
}


pub async fn list_blacklist(
    method: axum::http::Method,
    axum::extract::ConnectInfo(addr): axum::extract::ConnectInfo<SocketAddr>,   
    axum::extract::Query(para): axum::extract::Query<HashMap<String, String>>,    
    user_agent: Result<TypedHeader<axum::headers::UserAgent>,TypedHeaderRejection>,

    xrealip: Result<TypedHeader<XRealIp>,TypedHeaderRejection>,
    xforwardfor: Result<TypedHeader<XForwardedFor>,TypedHeaderRejection>,
    data: axum::extract::Extension<Arc<Mutex<service::FilterService>>>,
    config: axum::extract::Extension<Arc<Mutex<crate::Config>>>,
) -> impl IntoResponse {
    println!("{}, {:?}, {:?}", method, addr, user_agent);

    let query_ip =get_client_ip(false,addr.ip(),xrealip, xforwardfor);

    let key  =para.get("key").unwrap_or(&"".to_string()).clone();
    if check_key(&key[..], false, String::from("list_blacklist"), &*config.lock().unwrap()){
        let mut data = data.0.lock().unwrap();

     
        // final_ip=ip;
        log::info!("client: {query_ip} list_blacklist");
        // data.del_whitelisted_ip(ip);            
        let mut output=String::new();
        let records = &data.blacklisted_ips;
        output+=&format!("[blacklist] has {} ip  gen by: {query_ip}\n",records.len())[..];
        output+=&format!("ips:\n")[..];

        for record in records.iter() {
            output+=&format!("{:width$}\n",record.0,width=10)[..];
        }
        log::debug!("{output}");
        return (StatusCode::OK, output.to_string());
                    


    }else {
        return (StatusCode::UNAUTHORIZED, "UNAUTHORIZED".to_string());
    }

}
pub async fn reset_whitelist(
    method: axum::http::Method,
    axum::extract::ConnectInfo(addr): axum::extract::ConnectInfo<SocketAddr>,   
    axum::extract::Query(para): axum::extract::Query<HashMap<String, String>>,    
    user_agent: Result<TypedHeader<axum::headers::UserAgent>,TypedHeaderRejection>,

    xrealip: Result<TypedHeader<XRealIp>,TypedHeaderRejection>,
    xforwardfor: Result<TypedHeader<XForwardedFor>,TypedHeaderRejection>,
    data: axum::extract::Extension<Arc<Mutex<service::FilterService>>>,
    config: axum::extract::Extension<Arc<Mutex<crate::Config>>>,
) -> impl IntoResponse {
    println!("{}, {:?}, {:?}", method, addr, user_agent);

    let query_ip =get_client_ip(false,addr.ip(),xrealip, xforwardfor);

    let key  =para.get("key").unwrap_or(&"".to_string()).clone();
    if check_key(&key[..], false, String::from("reset_whitelist"), &*config.lock().unwrap()){
        let mut data = data.0.lock().unwrap();
        log::info!("client: {query_ip} reset_whitelist");         
        let mut output=String::from("ok");
        data.reset_whitelist();
        return (StatusCode::OK, output.to_string());
    }else {
        return (StatusCode::UNAUTHORIZED, "UNAUTHORIZED".to_string());
    }
}
pub async fn reset_all(
    method: axum::http::Method,
    axum::extract::ConnectInfo(addr): axum::extract::ConnectInfo<SocketAddr>,   
    axum::extract::Query(para): axum::extract::Query<HashMap<String, String>>,    
    user_agent: Result<TypedHeader<axum::headers::UserAgent>,TypedHeaderRejection>,

    xrealip: Result<TypedHeader<XRealIp>,TypedHeaderRejection>,
    xforwardfor: Result<TypedHeader<XForwardedFor>,TypedHeaderRejection>,
    data: axum::extract::Extension<Arc<Mutex<service::FilterService>>>,
    config: axum::extract::Extension<Arc<Mutex<crate::Config>>>,
) -> impl IntoResponse {
    println!("{}, {:?}, {:?}", method, addr, user_agent);

    let query_ip =get_client_ip(false,addr.ip(),xrealip, xforwardfor);

    let key  =para.get("key").unwrap_or(&"".to_string()).clone();
    if check_key(&key[..], false, String::from("reset_whitelist"), &*config.lock().unwrap()){
        let mut data = data.0.lock().unwrap();
        log::info!("client: {query_ip} reset_whitelist");
        data.init();
        data.load_config(config.lock().unwrap().clone());
        data.reset_table();

        return (StatusCode::OK, "ok".to_string());
    }else {
        return (StatusCode::UNAUTHORIZED, "UNAUTHORIZED".to_string());
    }
}
pub async fn root() -> &'static str {
    "SelfHelp iptables Whitelist\n/api/add?key=yourkey\n/api/list?key=yourkey \n/api/remove/ip?key=yourkey\n/api/log?key=yourkey\n/api/record?key=yourkey"
}
pub async fn get_record(
    method: axum::http::Method,
    axum::extract::ConnectInfo(addr): axum::extract::ConnectInfo<SocketAddr>,   
    axum::extract::Query(para): axum::extract::Query<HashMap<String, String>>,    
    user_agent: Result<TypedHeader<axum::headers::UserAgent>,TypedHeaderRejection>,

    xrealip: Result<TypedHeader<XRealIp>,TypedHeaderRejection>,
    xforwardfor: Result<TypedHeader<XForwardedFor>,TypedHeaderRejection>,
    data: axum::extract::Extension<Arc<Mutex<service::FilterService>>>,
    config: axum::extract::Extension<Arc<Mutex<crate::Config>>>,
) -> impl IntoResponse {

    println!("{}, {:?}, {:?}", method, addr, user_agent);

    let query_ip =get_client_ip(false,addr.ip(),xrealip, xforwardfor);

    let key  =para.get("key").unwrap_or(&"".to_string()).clone();
    if check_key(&key[..], true, String::from("log_record"), &*config.lock().unwrap()){
        // log::debug!("")
        let mut output=String::new();
        
        let mut data = data.0.lock().unwrap();
        output+=&format!("found {} ip  gen by: {query_ip}\n",&data.packets_per_ip.len())[..];
        for (ip,count) in  &data.packets_per_ip {
            if data.whitelisted_ips.contains_key(ip){
                output+=&ip.to_string()[..];
                output+=" record times: ";
                output+=&count.clone().to_string()[..];
                output+="  [Whitelist]\n ";

            }else{
                output+=&ip.to_string()[..];
                output+=" record times: ";
                output+=&count.clone().to_string()[..];
                output+="  \n ";            
            }
            return (StatusCode::UNAUTHORIZED, output);
        }
    }else {
        return (StatusCode::UNAUTHORIZED, "UNAUTHORIZED".to_string());
    }



    (StatusCode::OK, "ok".to_string())
}
fn get_client_ip(
    reverse_proxy: bool,
    addr: std::net::IpAddr,     
    xrealip: Result<TypedHeader<XRealIp>,TypedHeaderRejection>,
    xforwardfor: Result<TypedHeader<XForwardedFor>,TypedHeaderRejection>
) -> cidr_utils::cidr::IpCidr{
    let mut ip:cidr_utils::cidr::IpCidr;
    // let reverse =false;
    if !reverse_proxy {
        ip=cidr_utils::cidr::IpCidr::from_str(&addr.to_string()[..]).unwrap();
    }
    else{
        let mut ok_reverse = false;
        
        match xrealip {
            Ok(xrealip) => {
                ip=cidr_utils::cidr::IpCidr::from_str(&xrealip.0.0.to_string()[..]).unwrap();
                ok_reverse=true;
                println!("xforwardfor: {}",  ip)
            },
            Err(err) =>  {
                ip = cidr_utils::cidr::IpCidr::from_str("127.0.0.1").unwrap();
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
                    ip = cidr_utils::cidr::IpCidr::from_str("127.0.0.1").unwrap();
                    println!("{:?}",err.to_string())
                },
            }
            
        }
    }
    println!("ip:{:?}",ip);
    ip
}

pub fn which_ip(ip:String)  {

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


fn handle_poisoned_error<T>(error:std::sync::PoisonError<T>)
where
    T: std::fmt::Display,
{
    // 获取内部数据
    let inner = error.into_inner();

    // 处理内部数据的错误情况
    log::error!("{}",inner.to_string());
}