use qqwry::qqwry;
use std::net::IpAddr;
pub struct Database {
    qqwry_data: qqwry::QQWry,
}
impl Database {
    pub fn new() -> Database {
        // let key="passwordpassword";
        // let passwd=key.as_bytes().to_vec();
        Database {
            qqwry_data: qqwry::QQWry::from(String::from("./qqwry.dat")),
        }
    }
    pub fn query_ip(&mut self, ip: &IpAddr) -> Result<String, String> {
        log::info!("query ip: {}", ip);
        match ip {
            IpAddr::V4(ip) => match self.qqwry_data.read_ip_location(&ip.to_string()[..]) {
                Some(ip) => {
                    log::info!("query ip: {}", ip.get_start_ip_str());
                    return Ok(ip.country);
                }
                None => {
                    log::info!("unable to read ipv4");
                    return Err("unable to read".to_string());
                }
            },
            IpAddr::V6(ip) => {
                log::info!("unable to read ipv6");
                return Err("unable to read".to_string());
            }
        }
    }
    pub fn query_ip_str(&mut self, ip: &str) -> Result<String, String> {
        match self.qqwry_data.read_ip_location(ip) {
            Some(ip) => {
                return Ok(ip.get_start_ip_str());
            }
            None => {
                return Err("unable to read".to_string());
            }
        }
    }
}
