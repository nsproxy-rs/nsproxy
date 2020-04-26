pub enum ProxyMethod {
    StrictChain,
    DynamicChain,
}

pub enum ProxyProtocol {
    Socks5,
    Socks4,
    Http,
}

pub struct Auth {
    pub username: String,
    pub password: String,
}

pub struct ProxyConfig {
    pub protocol: ProxyProtocol,
    pub address: String,
    pub port: u16,
    pub auth: Auth,
}

pub struct Config {
    pub method: ProxyMethod,
    pub proxies: Vec<ProxyConfig>,
}
