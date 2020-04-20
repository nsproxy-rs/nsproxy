pub struct TproxyRule {

}

pub struct IPRouteRule {

}


pub struct NSRoute {
    transparent_proxy: TproxyRule,
    ip_route_rule: IPRouteRule
}

impl NSRoute {
    pub fn new() -> NSRoute {
        NSRoute{}
    }
}

impl Drop for NSRoute {
    fn drop(&mut self) {

    }
}
