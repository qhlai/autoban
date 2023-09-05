pub mod service;
pub mod iptables;
pub mod nftables;


pub static PROTECT_CHAIN : &str ="PROTECT_LIST";
pub static BLACKLIST_CHAIN : &str ="BLACK_LIST";
pub static BANDWIDTH_IN_CHAIN : &str ="BANDWIDTH_IN";
pub static BANDWIDTH_OUT_CHAIN : &str ="BANDWIDTH_OUT";
pub static PREFIX_DEFAULT : &str ="[ipt]";
pub static PREFIX_TRIGGER : &str ="[ipt-trigger]";
