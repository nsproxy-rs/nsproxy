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
    username: std::string::String,
    password: std::string::String,
}

pub struct ProxyConfig {
    protocol: ProxyProtocol,
    address: std::string::String,
    port: u16,
    auth: Auth,
}

pub struct Config {
    method: ProxyMethod,
    proxies: Vec<ProxyConfig>,
}
