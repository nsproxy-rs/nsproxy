enum ProxyMethod {
    StrictChain,
    DynamicChain,
}

enum ProxyProtocol {
    Socks5,
    Socks4,
    Http,
}

struct Auth {
    username: std::string::String,
    password: std::string::String,
}

struct ProxyConfig {
    protocol: ProxyProtocol,
    address: std::string::String,
    port: u16,
    auth: Auth,
}

struct Config {
    method: ProxyMethod,
    proxies: Vec<ProxyConfig>,
}
