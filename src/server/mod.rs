pub mod handlers;
pub mod server;
// location /api {
//     proxy_set_header  X-real-ip $remote_addr;
//     proxy_set_header  X-Forwarded-For $proxy_add_x_forwarded_for;
//     proxy_pass http://127.0.0.1:8080/api;
//  }

pub fn check_key(key: &str, privilege: bool, api_name: String, config: &crate::config::Config) -> bool {
    let result: bool;
    if key.len() > 0 {
        if privilege {
            if key == config.key_admin {
                result = true;
                // log::info!("")
            } else {
                result = false;
                // log::debug!("")
            }
        } else {
            if key == config.key_query || key == config.key_admin {
                result = true;
                // log::debug!("")
            } else {
                result = false;
                // log::debug!("")
            }
        }
    } else {
        result = false;
    }
    if result {
        log::info!("do {api_name} auth success. privilege:{privilege}")
    } else {
        log::warn!("do {api_name} auth fail. privilege:{privilege}")
    }
    result
}
