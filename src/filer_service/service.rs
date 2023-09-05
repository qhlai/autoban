use::iptables;
use std::{
    collections::HashMap,
    net::SocketAddr,
    ops::Deref,
    error::Error,
    sync::{Arc, Mutex},
    rc::Rc,
    net::IpAddr,
};
use local_ip_address;
use crate::filer_service::{*};
pub struct empty_struct{

}
pub struct FilterService{
    // mutex: Mutex,
	pub tables_v4:         iptables::IPTables,
    pub tables_v6:         iptables::IPTables,
	// IP6Tables        *iptables.IPTables
	pub whitelisted_ips:   HashMap<std::net::IpAddr, empty_struct>,
	pub blacklisted_ips:   HashMap<std::net::IpAddr, empty_struct>,
	pub whitelisted_ports: HashMap<u16, empty_struct>,
	pub blacklisted_ports:   HashMap<u16, empty_struct>,
	pub packets_per_ip:    HashMap<std::net::IpAddr, u32>,
	pub autoadd_threshold: u8,
	pub deny_action:       String,

}
impl FilterService {
    pub fn new()->FilterService{
        // let key="passwordpassword";
        // let passwd=key.as_bytes().to_vec();
        FilterService { 
            // passwd: passwd.clone(), 
            tables_v4:iptables::new(false).unwrap(),
            tables_v6:iptables::new(true).unwrap(),
            whitelisted_ips:HashMap::new(),
            blacklisted_ips:HashMap::new(),
            whitelisted_ports:HashMap::new(),
            blacklisted_ports:HashMap::new(),
            packets_per_ip:HashMap::new(),
            autoadd_threshold:0,
            deny_action:"DROP".to_string(),
        }
    }
    pub fn init(&mut self){
        // let key="passwordpassword";
        // let passwd=key.as_bytes().to_vec();
        *self = FilterService { 
            // passwd: passwd.clone(), 
            tables_v4:iptables::new(false).unwrap(),
            tables_v6:iptables::new(true).unwrap(),
            whitelisted_ips:HashMap::new(),
            blacklisted_ips:HashMap::new(),
            whitelisted_ports:HashMap::new(),
            blacklisted_ports:HashMap::new(),
            packets_per_ip:HashMap::new(),
            autoadd_threshold:0,
            deny_action:"DROP".to_string(),
        }
    }
    pub fn init_tables(&mut self){
        // PROTECT_CHAIN
        self.tables_v4.new_chain("filter", PROTECT_CHAIN).unwrap();
        self.tables_v4.new_chain("filter", BLACKLIST_CHAIN).unwrap();

        self.tables_v4.new_chain("filter", BANDWIDTH_IN_CHAIN).unwrap();
        self.tables_v4.new_chain("filter", BANDWIDTH_OUT_CHAIN).unwrap();
        // self.tables_v4.new_chain("filter", "NEWCHAINNAME").unwrap();
        // 获取每ip下载流量

        self.tables_v4.append("filter", "INPUT", &("-j".to_string()+BANDWIDTH_IN_CHAIN)[..]).unwrap();
        // self.tables_v4.append("filter", "INPUT", &format!("{} {}","-j",BANDWIDTH_IN_CHAIN)[..]).unwrap();
        self.tables_v4.append("filter", "INPUT", &format!("{} {}","-j",BANDWIDTH_OUT_CHAIN)[..]).unwrap();

        self.tables_v4.append("filter", "INPUT", &format!("{} {}","-j",BLACKLIST_CHAIN)[..]).unwrap();
        self.tables_v4.append("filter", "INPUT", &format!("{} {}","-j",PROTECT_CHAIN)[..]).unwrap();
        // let ip = ip::local_ip().unwrap();
        // 本机ip放行
        let network_interfaces = local_ip_address::list_afinet_netifas();
        if let Ok(network_interfaces) = network_interfaces {
            for (_, ip) in network_interfaces.iter() {
                // println!("{}:\t{:?}", name, ip);
                match ip {
                    IpAddr::V4(ip)=> {
                        self.tables_v4.append("filter", PROTECT_CHAIN, &format!("{}{}{}{}","-s",ip.to_string(), "-j", "ACCEPT")[..]).unwrap();
                    },
                    IpAddr::V6(ip)=> {
                        // TODO:
                        println!("{ip}");
                        // self.tables_v4.append("filter", PROTECT_CHAIN, &format!("{}{}","-s",PROTECT_CHAIN)[..]).unwrap();
                    },
                }
            }
        } else {
            println!("Error getting network interfaces: {:?}", network_interfaces);
        }
        // self.tables_v4.append("filter", PROTECT_CHAIN, &format!("{}{}{}{}","-p",ip.to_string(), "-j", "ACCEPT")[..]).unwrap();

        self.tables_v4.append("filter", "INPUT", &format!("{} {} {} {}","-s","icmp", "-j", "ACCEPT")[..]).unwrap();
        for (ip,_) in self.whitelisted_ips.iter(){
            match ip {
                IpAddr::V4(ip)=> {
                    self.tables_v4.append("filter", PROTECT_CHAIN, &format!("{} {} {} {}","-s",ip.to_string(), "-j", "ACCEPT")[..]).unwrap();
                },
                IpAddr::V6(ip)=> {
                    // TODO:
                    println!("{ip}");
                    // self.tables_v4.append("filter", PROTECT_CHAIN, &format!("{}{}","-s",PROTECT_CHAIN)[..]).unwrap();
                },
            }
        }

        for (port,_) in self.whitelisted_ports.iter(){
            self.tables_v4.append("filter", PROTECT_CHAIN, &format!("{} {} {} {} {}","-p","tcp", port.to_string(), "-j", "ACCEPT")[..]).unwrap();
            self.tables_v4.append("filter", PROTECT_CHAIN, &format!("{} {} {} {} {}","-p","udp", port.to_string(), "-j", "ACCEPT")[..]).unwrap();
        }
        // self.tables_v4.append("filter", "NEWCHAINNAME", "-j ACCEPT").unwrap();
        // // 包速率触发器相关的设置
        // pStr, tStr, validTrigger := parseTrigger(s.RateTrigger)
        // // 需要保护的端口初始化
        // if len(s.ProtectedPorts) > 0 {
        // 	for port := range s.ProtectedPorts {
        // 		utils.LogError(s.IP4Tables.AppendUnique("filter", PROTECT_CHAIN, "-p", "tcp", "--dport", strconv.Itoa(port), "-j", "NFLOG", "--nflog-group", "100", "--nflog-prefix", PREFIX_DEFAULT))
        // 		utils.LogError(s.IP4Tables.AppendUnique("filter", PROTECT_CHAIN, "-p", "udp", "--dport", strconv.Itoa(port), "-j", "NFLOG", "--nflog-group", "100", "--nflog-prefix", PREFIX_DEFAULT))
        // 		if validTrigger {
        // 			utils.LogError(s.IP4Tables.AppendUnique("filter", PROTECT_CHAIN, "-p", "tcp", "--dport", strconv.Itoa(port), "-m", "recent", "--name", strconv.Itoa(port)+"TRIGGER", "--set"))
        // 			utils.LogError(s.IP4Tables.AppendUnique("filter", PROTECT_CHAIN, "-p", "tcp", "--dport", strconv.Itoa(port), "-m", "recent", "--name", strconv.Itoa(port)+"TRIGGER", "--rcheck", "--seconds", tStr, "--hitcount", pStr, "-j", "NFLOG", "--nflog-group", "100", "--nflog-prefix", PREFIX_TRIGGER))
        // 		}
        // 		utils.LogError(s.IP4Tables.AppendUnique("filter", PROTECT_CHAIN, "-p", "tcp", "--dport", strconv.Itoa(port), "-j", s.denyAction))
        // 		utils.LogError(s.IP4Tables.AppendUnique("filter", PROTECT_CHAIN, "-p", "udp", "--dport", strconv.Itoa(port), "-j", s.denyAction))
        // 	}
        // } else {
        // 	// 不指明端口时，全端口防护
        // 	utils.LogError(s.IP4Tables.AppendUnique("filter", PROTECT_CHAIN, "-j", s.denyAction))
        // }

    }

    pub fn clear(&mut self){
        // utils.LogError(s.IP4Tables.Delete("filter", "INPUT", "-j", BANDWIDTH_IN_CHAIN))
        // utils.LogError(s.IP4Tables.Delete("filter", "OUTPUT", "-j", BANDWIDTH_OUT_CHAIN))
        // utils.LogError(s.IP4Tables.Delete("filter", "INPUT", "-j", PROTECT_CHAIN))
        // utils.LogError(s.IP4Tables.Delete("filter", "INPUT", "-j", BLACKLIST_CHAIN))
    
        // utils.LogError(s.IP4Tables.ClearAndDeleteChain("filter", PROTECT_CHAIN))
        // utils.LogError(s.IP4Tables.ClearAndDeleteChain("filter", BLACKLIST_CHAIN))
        // utils.LogError(s.IP4Tables.ClearAndDeleteChain("filter", BANDWIDTH_IN_CHAIN))
        // utils.LogError(s.IP4Tables.ClearAndDeleteChain("filter", BANDWIDTH_OUT_CHAIN))
        self.tables_v4.delete("filter", "INPUT", "-j ACCEPT")
        self.tables_v4.delete_chain("nat", "NEWCHAINNAME").unwrap();

    }
    pub fn start(&mut self){
        self.init();
        self.init_tables();


    }
}