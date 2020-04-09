use std::collections::HashMap;
use std::net::Ipv4Addr;
use trust_dns_proto::rr::Name;

#[derive(Debug, Default)]
pub struct Ipv4Mapping {
    pub addr_to_name: HashMap<Ipv4Addr, Name>,
    pub name_to_addr: HashMap<Name, Ipv4Addr>,
}
