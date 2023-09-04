

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


#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let lock = Arc::new(Mutex::new(0));

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/api/add", get(add))
        .route("/api/list", get(root).post(root))
        .route("/api/remove", get(root))
        .route("/api/log", get(root))

        // `POST /users` goes to `create_user`
        .route("/users", post(
            create_user));
        // .layer(axum::AddExtensionLayer::new(ConnectInfo::default()));

    // run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // tracing::debug!("listening on {}", listener.local_addr().unwrap());
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)    
    .serve(app.into_make_service_with_connect_info::<SocketAddr>())
    .await
    .unwrap();

    // axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
fn get_client_ip(
    addr: std::net::IpAddr,     
    xrealip: Result<TypedHeader<XRealIp>,TypedHeaderRejection>,
    xforwardfor: Result<TypedHeader<XForwardedFor>,TypedHeaderRejection>
) -> std::net::IpAddr{
    let mut ip:std::net::IpAddr;
    let reverse =false;
    if !reverse {
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
// basic handler that responds with a static string
async fn add(
    method: axum::http::Method,
    axum::extract::ConnectInfo(addr): axum::extract::ConnectInfo<SocketAddr>,   

    axum::extract::Query(para): axum::extract::Query<HashMap<String, String>>,    
    // params: Result<axum::extract::Query<Params>,QueryRejection>,
    user_agent: Result<TypedHeader<axum::headers::UserAgent>,TypedHeaderRejection>,

    xrealip: Result<TypedHeader<XRealIp>,TypedHeaderRejection>,
    xforwardfor: Result<TypedHeader<XForwardedFor>,TypedHeaderRejection>,


    // axum::extract::Query(aa): axum::http<HeaderMap<String, String>>,
    // headers:TypedHeader<axum::headers::HeaderMap>,
    // request: axum::extract::FromRequestParts
) -> impl IntoResponse {

    println!("{}, {:?}, {:?}", method, addr, user_agent);
    // println!("{}, {:?}, {:?}", method, addr, user_agent.unwrap());

    // println!("{}, {:?}, {:?}, {:?}", method, addr, user_agent, realip);
    let ip =get_client_ip(addr.ip(),xrealip, xforwardfor);
    // let mut ip:std::net::IpAddr;
    // let reverse =false;
    // if !reverse {
    //     ip=addr.ip();
    // }
    // else{
    //     let mut ok_reverse = false;
        
    //     match xrealip {
    //         Ok(xrealip) => {
    //             ip=xrealip.0.0;
    //             ok_reverse=true;
    //             println!("xforwardfor: {}",  ip)
    //         },
    //         Err(err) =>  {
    //             ip = std::net::IpAddr::from([127, 0, 0, 1]);
    //             println!("{:?}",err.to_string())
    //         },
    //     }
    //     if ok_reverse{

    //     }else{

    //         match xforwardfor {
    //             Ok(xforwardfor) => {
    //                 let xforwardfor = &xforwardfor.0.0[..];

    //                 let comma_index = xforwardfor.find(','); // 查找逗号的索引
    //                 if let Some(index) = comma_index {
    //                     let substring = &xforwardfor[..index]; // 使用切片操作获取逗号之前的部分
    //                     println!("substring: {}", substring);
    //                 } else {
    //                     println!("Comma not found");
    //                 }
    //                 println!("xforwardfor: {}",  xforwardfor);                    
    //             },
    //             Err(err) =>  {
    //                 ip = std::net::IpAddr::from([127, 0, 0, 1]);
    //                 println!("{:?}",err.to_string())
    //             },
    //         }
            
    //     }
    // }
    println!("ip:{:?}",ip);

    // location /api {
    //     proxy_set_header  X-real-ip $remote_addr;
    //     proxy_set_header  X-Forwarded-For $proxy_add_x_forwarded_for;
    //     proxy_pass http://127.0.0.1:8080/api;
    //  }

    // match params {
    //     Ok(params) => (StatusCode::OK, params.key.clone()),
    //     Err(err) => (StatusCode::UNAUTHORIZED, err.to_string()),
    // }
    // let a =para.get("value").unwrap_or("");\

    let mut key:String = String::from("value");
    match para.get("key") {
        Some(review) => {
            key = review.clone();
            println!("key: {review}")
        },
        None => {
            // key = review.clone();
            println!("key is unreviewed.")
        }
    }
    // println!("{}", a);

    (StatusCode::OK, key)
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}


#[derive(Debug)]
struct XForwardedFor(String);
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
struct XRealIp(std::net::IpAddr);
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

#[derive(serde::Deserialize)]
struct Params {
    key:String,
    admin_key: String,
    query_key: String,
    listen_port: u32,
    auto_add_threshold: u8,
    reject: bool,
    reverse_proxy: bool
}
impl Params {
    pub fn new()->Params{
        let key="passwordpassword";
        let passwd=key.as_bytes().to_vec();
        Params { 
            // passwd: passwd.clone(), 
            key:String::from("1234"),
            admin_key:String::from("1234"),
            query_key:String::from("123"),
            listen_port:8081,
            auto_add_threshold:0,
            reject:false,
            reverse_proxy:false,
            // sysinfo: Sysinfo::new(&passwd)
        }
    }

}
// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}