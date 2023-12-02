use crate::utils::iferror::iferror;
use crate::{config, filer_service::*};
use ::iptables;
use cidr_utils::cidr::IpCidr;
use ctrlc;
use local_ip_address;
use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    net::IpAddr,
    net::SocketAddr,
    ops::Deref,
    rc::Rc,
    sync::atomic::{AtomicBool, Ordering},
    sync::{Arc, Mutex},
};

pub struct WhitelistRecord {
    pub ip: cidr_utils::cidr::IpCidr,
    pub packets_out: u16,
    pub packets_in: u16,
    pub bandwidth_out: u16,
    pub bandwidth_in: u16,
}
pub struct empty_struct {}

pub struct FilterService {
    // mutex: Mutex,
    pub tables_v4: iptables::IPTables,
    pub tables_v6: iptables::IPTables,
    // IP6Tables        *iptables.IPTables
    pub whitelisted_ips: HashMap<IpCidr, empty_struct>,
    pub blacklisted_ips: HashMap<IpCidr, empty_struct>,
    pub whitelisted_ports: HashMap<u16, empty_struct>,
    pub blacklisted_ports: HashMap<u16, empty_struct>,
    pub packets_per_ip: HashMap<cidr_utils::cidr::IpCidr, u32>,
    pub auto_add_threshold: u16,
    pub deny_action: String,
    pub rate_trigger: String,
}
impl FilterService {
    pub fn new() -> FilterService {
        // let key="passwordpassword";
        // let passwd=key.as_bytes().to_vec();
        FilterService {
            // passwd: passwd.clone(),
            tables_v4: iptables::new(false).unwrap(),
            tables_v6: iptables::new(true).unwrap(),
            whitelisted_ips: HashMap::new(),
            blacklisted_ips: HashMap::new(),
            whitelisted_ports: HashMap::new(),
            blacklisted_ports: HashMap::new(),
            packets_per_ip: HashMap::new(),
            auto_add_threshold: 0,
            deny_action: "DROP".to_string(),
            rate_trigger: "".to_string(),
        }
    }
    pub fn init(&mut self) {
        // let key="passwordpassword";
        // let passwd=key.as_bytes().to_vec();
        *self = FilterService {
            // passwd: passwd.clone(),
            tables_v4: iptables::new(false).unwrap(),
            tables_v6: iptables::new(true).unwrap(),
            whitelisted_ips: HashMap::new(),
            blacklisted_ips: HashMap::new(),
            whitelisted_ports: HashMap::new(),
            blacklisted_ports: HashMap::new(),
            packets_per_ip: HashMap::new(),
            auto_add_threshold: 0,
            deny_action: "DROP".to_string(),
            rate_trigger: "".to_string(),
        }
    }
    pub fn init_tables(&mut self) {
        // PROTECT_CHAIN
        iferror(self.tables_v4.new_chain("filter", PROTECT_CHAIN));
        iferror(self.tables_v4.new_chain("filter", BLACKLIST_CHAIN));

        iferror(self.tables_v4.new_chain("filter", BANDWIDTH_IN_CHAIN));
        iferror(self.tables_v4.new_chain("filter", BANDWIDTH_OUT_CHAIN));
        // self.tables_v4.new_chain("filter", "NEWCHAINNAME"));
        // 获取每ip下载流量

        iferror(self.tables_v4.append_unique(
            "filter",
            "INPUT",
            &("-j".to_string() + " " + BANDWIDTH_IN_CHAIN)[..],
        ));
        iferror(self.tables_v4.append_unique(
            "filter",
            "OUTPUT",
            &("-j".to_string() + " " + BANDWIDTH_OUT_CHAIN)[..],
        ));

        iferror(self.tables_v4.append_unique(
            "filter",
            "INPUT",
            &("-j".to_string() + " " + BLACKLIST_CHAIN)[..],
        ));
        iferror(self.tables_v4.append_unique(
            "filter",
            "INPUT",
            &("-j".to_string() + " " + PROTECT_CHAIN)[..],
        ));
        // let ip = ip::local_ip());
        // 本机ip放行
        let network_interfaces = local_ip_address::list_afinet_netifas();
        if let Ok(network_interfaces) = network_interfaces {
            for (_, ip) in network_interfaces.iter() {
                // println!("{}:\t{:?}", name, ip);

                match ip {
                    IpAddr::V4(ip) => {
                        iferror(self.tables_v4.append_unique(
                            "filter",
                            PROTECT_CHAIN,
                            &("-s".to_string() + " " + &ip.to_string()[..] + " -j ACCEPT")[..],
                        ));
                    }
                    IpAddr::V6(ip) => {
                        // TODO:
                        // println!("{ip}");
                        // iferror(self.tables_v6.append_unique("filter", PROTECT_CHAIN, &("-s".to_string()+" "+&ip.to_string()[..]+" -j ACCEPT")[..]));
                    }
                }
            }
        } else {
            println!("Error getting network interfaces: {:?}", network_interfaces);
        }
        // self.tables_v4.append_unique("filter", PROTECT_CHAIN, &format!("{}{}{}{}","-p",ip.to_string(), "-j", "ACCEPT")[..]));

        iferror(self.tables_v4.append_unique(
            "filter",
            PROTECT_CHAIN,
            &("-p".to_string() + " icmp -j ACCEPT")[..],
        ));

        // self.add_whitelisted_ip(ip.clone());
        let mut ips_to_add = Vec::new();
        for (ip, _) in self.whitelisted_ips.iter() {
            // let ip = *ip;
            ips_to_add.push(ip.clone());
        }
        for ip in ips_to_add {
            self.add_whitelisted_ip(ip);
        }

        let mut ips_to_add = Vec::new();
        for (ip, _) in self.blacklisted_ips.iter() {
            // let ip = *ip;
            ips_to_add.push(ip.clone());
        }
        for ip in ips_to_add {
            self.add_blacklisted_ip(ip);
        }

        for (port, _) in self.whitelisted_ports.iter() {
            iferror(self.tables_v4.append_unique(
                "filter",
                PROTECT_CHAIN,
                &("-p tcp --dport ".to_string() + &port.to_string()[..] + " -j ACCEPT")[..],
            ));
            iferror(self.tables_v4.append_unique(
                "filter",
                PROTECT_CHAIN,
                &("-p udp --dport ".to_string() + &port.to_string()[..] + " -j ACCEPT")[..],
            ));

            iferror(self.tables_v4.append_unique(
                "filter",
                PROTECT_CHAIN,
                &("-p tcp --sport ".to_string() + &port.to_string()[..] + " -j ACCEPT")[..],
            ));
            iferror(self.tables_v4.append_unique(
                "filter",
                PROTECT_CHAIN,
                &("-p udp --sport ".to_string() + &port.to_string()[..] + " -j ACCEPT")[..],
            ));

            // iferror(self.tables_v4.append_unique("filter", PROTECT_CHAIN, &("-p all --dport ".to_string()+&port.to_string()[..]+" -j ACCEPT")[..]));
        }

        // // 包速率触发器相关的设置
        // pStr, tStr, validTrigger := parseTrigger(s.RateTrigger)
        let (pstr, tstr, valid_trigger) = parse_trigger(&self.rate_trigger);
        // log::info!("{pstr}, {tstr}, {valid_trigger}");
        // parse_trigger(self.c)
        // 需要保护的端口初始化
        if self.blacklisted_ports.len() > 0 {
            for (port, _) in self.blacklisted_ports.iter() {
                iferror(self.tables_v4.append_unique(
                    "filter",
                    PROTECT_CHAIN,
                    &("-p tcp --dport ".to_string()
                        + &port.to_string()[..]
                        + " -j NFLOG --nflog-group 100 --nflog-prefix "
                        + PREFIX_DEFAULT)[..],
                ));
                iferror(self.tables_v4.append_unique(
                    "filter",
                    PROTECT_CHAIN,
                    &("-p udp --dport ".to_string()
                        + &port.to_string()[..]
                        + " -j NFLOG --nflog-group 100 --nflog-prefix "
                        + PREFIX_DEFAULT)[..],
                ));

                if valid_trigger {
                    log::info!("valid_trigger: {pstr}, {tstr}, {valid_trigger}");
                    iferror(self.tables_v4.append_unique(
                        "filter",
                        PROTECT_CHAIN,
                        &("-p tcp --dport ".to_string()
                            + &port.to_string()[..]
                            + " -m recent --name "
                            + &port.to_string()[..]
                            + "TRIGGER --set")[..],
                    ));
                    iferror(self.tables_v4.append_unique(
                        "filter",
                        PROTECT_CHAIN,
                        &("-p tcp --dport ".to_string()
                            + &port.to_string()[..]
                            + " -m recent --name "
                            + &port.to_string()[..]
                            + "TRIGGER --rcheck --seconds "
                            + &tstr[..]
                            + " --hitcount "
                            + &pstr[..]
                            + " -j NFLOG --nflog-group 100 --nflog-prefix  "
                            + PREFIX_TRIGGER)[..],
                    ));
                }
                iferror(self.tables_v4.append_unique(
                    "filter",
                    PROTECT_CHAIN,
                    &("-p tcp --dport ".to_string()
                        + &port.to_string()[..]
                        + " -j "
                        + &self.deny_action)[..],
                ));
                iferror(self.tables_v4.append_unique(
                    "filter",
                    PROTECT_CHAIN,
                    &("-p udp --dport ".to_string()
                        + &port.to_string()[..]
                        + " -j "
                        + &self.deny_action)[..],
                ));
            }
        } else {
            // 不指明端口时，全端口防护
            iferror(self.tables_v4.append_unique(
                "filter",
                PROTECT_CHAIN,
                &("-j ".to_string() + &self.deny_action)[..],
            ));
        }
    }

    pub fn clear_tables(&mut self) {
        iferror(self.tables_v4.delete(
            "filter",
            "INPUT",
            &("-j ".to_string() + BANDWIDTH_IN_CHAIN)[..],
        ));
        iferror(self.tables_v4.delete(
            "filter",
            "OUTPUT",
            &("-j ".to_string() + BANDWIDTH_OUT_CHAIN)[..],
        ));
        iferror(
            self.tables_v4
                .delete("filter", "INPUT", &("-j ".to_string() + PROTECT_CHAIN)[..]),
        );
        iferror(self.tables_v4.delete(
            "filter",
            "INPUT",
            &("-j ".to_string() + BLACKLIST_CHAIN)[..],
        ));
        iferror(self.tables_v4.flush_chain("filter", PROTECT_CHAIN));
        iferror(self.tables_v4.delete_chain("filter", PROTECT_CHAIN));
        iferror(self.tables_v4.flush_chain("filter", BLACKLIST_CHAIN));
        iferror(self.tables_v4.delete_chain("filter", BLACKLIST_CHAIN));

        iferror(self.tables_v4.flush_chain("filter", BANDWIDTH_IN_CHAIN));
        iferror(self.tables_v4.delete_chain("filter", BANDWIDTH_IN_CHAIN));

        iferror(self.tables_v4.flush_chain("filter", BANDWIDTH_OUT_CHAIN));
        iferror(self.tables_v4.delete_chain("filter", BANDWIDTH_OUT_CHAIN));
    }
    pub fn load_config(&mut self, config: &crate::config::Config) {
        // self.
        let config = config.clone();
        for ip in config.filterlist.allow_ips.into_iter() {
            let ip = cidr_utils::cidr::IpCidr::from_str(&ip[..]).unwrap();

            self.whitelisted_ips.insert(ip, empty_struct {});
        }
        for ip in config.filterlist.ban_ips.into_iter() {
            let ip = cidr_utils::cidr::IpCidr::from_str(&ip[..]).unwrap();
            self.blacklisted_ips.insert(ip, empty_struct {});
        }
        for port in config.filterlist.allow_ports.into_iter() {
            self.whitelisted_ports.insert(port, empty_struct {});
        }
        if config.reverse_proxy {
        } else {
            self.whitelisted_ports
                .insert(config.listen_port, empty_struct {});
        }

        for port in config.filterlist.ban_ports.into_iter() {
            self.blacklisted_ports.insert(port, empty_struct {});
        }
        self.auto_add_threshold = config.filterlist.auto_add_threshold;

        if config.filterlist.reject {
            self.deny_action = "REJECT".to_string();
        } else {
            self.deny_action = "DROP".to_string();
        }
        self.rate_trigger = config.filterlist.rate_trigger;

        // for ip in config.allow_ports.into_iter(){
        //     self.add_whitelisted_ip(ip);
        // }
        // for ip in config.allow_ips.into_iter(){
        //     self.add_whitelisted_ip(ip);
        // }
    }
    pub fn start(&mut self) {
        // self.init();
        self.init_tables();
        // self.clear_tables();
        // self.clear_after_exit();
    }
    pub fn clear_after_exit(&mut self) {
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        // let service = Arc::new(Mutex::new(self'a));

        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
            // let s= service.clone();
            // self.clear_tables();
            // self.clear_after_exit();
        })
        .expect("Error setting Ctrl-C handler");

        while running.load(Ordering::SeqCst) {
            std::thread::yield_now();
        }
        // self.clear_tables();
        // std::process::exit(0);
    }
    pub fn add_whitelisted_ip(&mut self, ip: cidr_utils::cidr::IpCidr) {
        self.whitelisted_ips.insert(ip, empty_struct {});
        match ip {
            cidr_utils::cidr::IpCidr::V4(ip) => {
                iferror(self.tables_v4.insert_unique(
                    "filter",
                    BANDWIDTH_IN_CHAIN,
                    &(" -s ".to_string() + &ip.to_string()[..] + " -j RETURN")[..],
                    1,
                ));
                iferror(self.tables_v4.insert_unique(
                    "filter",
                    BANDWIDTH_OUT_CHAIN,
                    &(" -d ".to_string() + &ip.to_string()[..] + " -j RETURN")[..],
                    1,
                ));
                iferror(self.tables_v4.insert_unique(
                    "filter",
                    PROTECT_CHAIN,
                    &(" -s ".to_string() + &ip.to_string()[..] + " -j ACCEPT")[..],
                    1,
                ));
            }
            cidr_utils::cidr::IpCidr::V6(ip) => {
                // TODO:
                log::debug!("{ip}");
                // println!("{ip}");
                // self.tables_v4.append_unique("filter", PROTECT_CHAIN, &format!("{}{}","-s",PROTECT_CHAIN)[..]));
            }
        }

        // // if !self.whitelisted_ips.contains_key(&ip) {
        //     self.whitelisted_ips.insert(ip, empty_struct{});
        //     iferror(self.tables_v4.append_unique("filter", BANDWIDTH_IN_CHAIN, &(" -s ".to_string()+&ip.to_string()[..]+" -j RETURN")[..]));
        //     iferror(self.tables_v4.append_unique("filter", BANDWIDTH_OUT_CHAIN, &(" -d ".to_string()+&ip.to_string()[..]+" -j RETURN")[..]));
        //     iferror(self.tables_v4.append_unique("filter", PROTECT_CHAIN, &(" -s ".to_string()+&ip.to_string()[..]+" -j ACCEPT")[..]));
        // // }
    }
    pub fn del_whitelisted_ip(&mut self, ip: cidr_utils::cidr::IpCidr) {
        self.whitelisted_ips.remove(&ip);
        self.packets_per_ip.remove(&ip);

        iferror(self.tables_v4.delete(
            "filter",
            BANDWIDTH_IN_CHAIN,
            &(" -s ".to_string() + &ip.to_string()[..] + " -j RETURN")[..],
        ));
        iferror(self.tables_v4.delete(
            "filter",
            BANDWIDTH_OUT_CHAIN,
            &(" -d ".to_string() + &ip.to_string()[..] + " -j RETURN")[..],
        ));
        iferror(self.tables_v4.delete(
            "filter",
            PROTECT_CHAIN,
            &(" -s ".to_string() + &ip.to_string()[..] + " -j ACCEPT")[..],
        ));
    }
    pub fn reset_whitelist(&mut self) {
        self.whitelisted_ips = HashMap::new();
        self.packets_per_ip = HashMap::new();
        self.clear_tables();
        self.start();
    }

    pub fn reset_table(&mut self) {
        // self.whitelisted_ips=HashMap::new();
        // self.blacklisted_ips=HashMap::new();

        self.packets_per_ip = HashMap::new();
        self.clear_tables();
        self.start();
    }
    pub fn add_blacklisted_ip(&mut self, ip: cidr_utils::cidr::IpCidr) {
        self.blacklisted_ips.insert(ip, empty_struct {});
        match ip {
            cidr_utils::cidr::IpCidr::V4(ip) => {
                iferror(self.tables_v4.append_unique(
                    "filter",
                    BLACKLIST_CHAIN,
                    &(" -s ".to_string() + &ip.to_string()[..] + " -j " + &self.deny_action)[..],
                ));
            }
            cidr_utils::cidr::IpCidr::V6(ip) => {
                // TODO:
                log::debug!("{ip}");
                // self.tables_v4.append_unique("filter", PROTECT_CHAIN, &format!("{}{}","-s",PROTECT_CHAIN)[..]));
            }
        }
    }
    pub fn del_blacklisted_ip(&mut self, ip: cidr_utils::cidr::IpCidr) {
        self.blacklisted_ips.remove(&ip);
        self.packets_per_ip.remove(&ip);

        iferror(self.tables_v4.delete(
            "filter",
            BLACKLIST_CHAIN,
            &(" -s ".to_string() + &ip.to_string()[..] + " -j " + &self.deny_action)[..],
        ));
    }
    pub fn get_whitelist_data(&mut self) -> Vec<WhitelistRecord> {
        // 分别获取INPUT和OUTPUT的查询数据,之后过滤出每ip的值
        // 低性能实现.
        let mut wrs = vec![];
        log::info!("get_whitelist_data");

        let cmd;
        cmd = "iptables";

        let input_raw = std::process::Command::new(cmd)
            .arg("-vnL")
            .arg(BANDWIDTH_IN_CHAIN)
            .output()
            .expect("Failed to execute command");

        let output_raw = std::process::Command::new(cmd)
            .arg("-vnL")
            .arg(BANDWIDTH_OUT_CHAIN)
            .output()
            .expect("Failed to execute command");

        let cmd = "ip6tables";

        let input_raw_v6 = std::process::Command::new(cmd)
            .arg("-vnL")
            .arg(BANDWIDTH_IN_CHAIN)
            .output()
            .expect("Failed to execute command");

        let output_raw_v6 = std::process::Command::new(cmd)
            .arg("-vnL")
            .arg(BANDWIDTH_OUT_CHAIN)
            .output()
            .expect("Failed to execute command");

        if input_raw.status.success() && output_raw.status.success() {
            let stdout_input = &*String::from_utf8_lossy(&input_raw.stdout);
            let stdout_input_v6 = &*String::from_utf8_lossy(&input_raw_v6.stdout);

            // let ts1 = stdout_input.split(" ").collect::<Vec<&str>>();
            let stdout_output = &*String::from_utf8_lossy(&output_raw.stdout);
            let stdout_output_v6 = &*String::from_utf8_lossy(&output_raw_v6.stdout);
            for (ip, _) in self.whitelisted_ips.iter() {
                let lines_in: Vec<&str>;
                let lines_out: Vec<&str>;
                // log::debug!("{}",ip.to_string());
                let ip_addr = ip.clone().first_as_ip_addr();
                match ip_addr {
                    IpAddr::V4(ip) => {
                        // let ip = ip.to_string().split("/").collect::<Vec<&str>>()[0].to_string();
                        // let ip = ip.clone().first_as_ip_addr();
                        lines_in = stdout_input
                            .lines()
                            .filter(|&line| line.contains(&ip.to_string()[..]))
                            .collect();
                        lines_out = stdout_output
                            .lines()
                            .filter(|&line| line.contains(&ip.to_string()[..]))
                            .collect();
                    }
                    IpAddr::V6(ip) => {
                        // let ip = ip.to_string().split("/").collect::<Vec<&str>>()[0].to_string();
                        lines_in = stdout_input_v6
                            .lines()
                            .filter(|&line| line.contains(&ip.to_string()[..]))
                            .collect();
                        lines_out = stdout_output_v6
                            .lines()
                            .filter(|&line| line.contains(&ip.to_string()[..]))
                            .collect();
                    }
                }
                // let lines_in: Vec<&str> = stdout_input.lines()
                // .filter(|&line| line.contains(&ip.to_string()[..]))
                // .collect();
                // let lines_out: Vec<&str> = stdout_output.lines()
                // .filter(|&line| line.contains(&ip.to_string()[..]))
                // .collect();
                if lines_in.len() == lines_out.len() {
                    for index in 0..lines_in.len() {
                        let in_field: Vec<&str> = lines_in[index].split_whitespace().collect();
                        let out_field: Vec<&str> = lines_out[index].split_whitespace().collect();
                        // log::info!("{:?}",in_field);
                        // log::info!("{:?}",out_field);

                        let wr = WhitelistRecord {
                            ip: *ip,
                            packets_out: out_field[0].parse::<u16>().unwrap_or(0),
                            packets_in: in_field[0].parse::<u16>().unwrap_or(0),
                            bandwidth_out: out_field[1].parse::<u16>().unwrap_or(0),
                            bandwidth_in: in_field[1].parse::<u16>().unwrap_or(0),
                        };
                        wrs.push(wr);
                        // println!("{}", line);
                    }
                }
            }
        } else {
            log::info!("cmd run failed");
            let stderr_in = String::from_utf8_lossy(&input_raw.stderr);
            let stderr_out = String::from_utf8_lossy(&output_raw.stderr);
            log::info!("{stderr_in}");
            log::info!("{stderr_out}");
        }

        return wrs;
    }
}
// impl iptables::IPTables {

// }
fn parse_trigger(trigger_str: &String) -> (String, String, bool) {
    let pstr: String;
    let tstr: String;
    let mut valid = true;
    if trigger_str.len() == 0 {
        valid = false;
        // pstr = "".to_string();
        // tstr  ="".to_string();
        // log::debug!("trigger_str: {}", trigger_str);
        return ("".to_string(), "".to_string(), valid);
    }
    // log::debug!("trigger_str: {}", trigger_str);
    let ts = trigger_str.split("/").collect::<Vec<&str>>();
    // let a=ts[1];
    // ts := strings.Split(triggerStr, "/")
    if ts.len() != 2 {
        log::error!("wrong trigger param, please use [packet num]/[seconds]");
        log::debug!("trigger_str: {}", trigger_str);
        valid = false;
        pstr = "".to_string();
        tstr = "".to_string();
    } else {
        // let a = ts[0];
        // pstr = ts[0].to_string();
        let err1 = match ts[0].parse::<i32>() {
            Ok(i) => i,
            Err(_e) => -1,
        };
        let err2 = match ts[1].parse::<i32>() {
            Ok(i) => i,
            Err(_e) => -1,
        };

        if err1 > 0 && err2 > 0 {
            pstr = ts[0].to_string();
            tstr = ts[1].to_string();
        } else {
            log::error!("wrong trigger param, please use [packet num]/[seconds]");
            log::debug!("trigger_str: {}", trigger_str);
            valid = false;
            pstr = "".to_string();
            tstr = "".to_string();
        }
    }
    return (pstr, tstr, valid);
}
