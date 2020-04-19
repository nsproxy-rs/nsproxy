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
    username: String,
    password: String,
}

pub struct ProxyConfig {
    protocol: ProxyProtocol,
    address: String,
    port: u16,
    auth: Auth,
}

pub struct Config {
    method: ProxyMethod,
    proxies: Vec<ProxyConfig>,
}
