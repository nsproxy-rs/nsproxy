use log::{info};
use tokio;
use tokio::net::{UdpSocket, ToSocketAddrs};
use tokio::sync::Mutex;
use trust_dns_proto::op::{Message};
use trust_dns_proto::rr::{Record, RecordType, DNSClass, RData, Name};
use std::net::{Ipv4Addr};
use trust_dns_proto::serialize::binary::BinEncodable;
use std::sync::Arc;

use crate::mapping::Ipv4Mapping;


const BASE_IPV4_ADDR: u32 = 0x0D000000;  // 13.0.0.0
const FIRST_IPV4_ADDR: u32 = BASE_IPV4_ADDR + 1;  // 13.0.0.1

#[derive(Debug)]
pub struct DnsServer {
    dns_server: UdpSocket,
    answer_index: u32,
    ipv4_mapping: Arc<Mutex<Ipv4Mapping>>,
}

impl DnsServer {
    pub async fn bind<A: ToSocketAddrs>(addr: A) -> Result<DnsServer, tokio::io::Error> {
        let server = DnsServer {
            dns_server: UdpSocket::bind(addr).await?,
            answer_index: 1,
            ipv4_mapping: Arc::new(Mutex::new(Ipv4Mapping::default())),
        };
        Ok(server)
    }

    pub fn port(&self) -> Result<u16, tokio::io::Error> {
        self.dns_server.local_addr().map(|addr| addr.port())
    }

    pub fn ipv4_mapping(&self) -> Arc<Mutex<Ipv4Mapping>> {
        self.ipv4_mapping.clone()
    }

    pub async fn run(mut self) -> Result<(), tokio::io::Error> {
        let mut query_buf = vec![0; 65536];
        loop {
            let (_, addr) = self.dns_server.recv_from(&mut query_buf).await?;
            let mut msg = match Message::from_vec(&query_buf) {
                Ok(query) => query,
                Err(_) => {
                    info!("Received invalid dns packet");
                    continue;
                }
            };
            let mut answers = Vec::with_capacity(msg.query_count() as usize);
            for query in msg.queries() {
                let mut record = Record::new();
                record.set_name(query.name().clone());
                record.set_record_type(RecordType::A);
                record.set_dns_class(DNSClass::IN);
                record.set_ttl(60 * 60);  // 1 hour
                record.set_rdata(self.create_rdata_for_domain(record.name(), record.record_type()).await);
                answers.push(record);
            }

            msg.add_answers(answers);
            self.dns_server.send_to(&msg.to_bytes()?, addr).await?;
        }
    }
    async fn create_rdata_for_domain(&mut self, name: &Name, record_type: RecordType) -> RData {
        match record_type {
            RecordType::A => RData::A(self.create_ipv4_addr_for_domain(&name).await),
            _ => unimplemented!(),
        }
    }

    async fn create_ipv4_addr_for_domain(&mut self, name: &Name) -> Ipv4Addr {
        let mut ipv4_mapping = self.ipv4_mapping.lock().await;
        match ipv4_mapping.name_to_addr.get(name) {
            Some(ipv4) => return *ipv4,
            None => (),
        }

        let addr = Ipv4Addr::from(BASE_IPV4_ADDR + self.answer_index);
        ipv4_mapping.name_to_addr.insert(name.clone(), addr);
        ipv4_mapping.addr_to_name.insert(addr, name.clone());

        self.answer_index += 1;
        if self.answer_index % 256 == 255 {
            self.answer_index += 2;
        }

        addr
    }
}

#[cfg(test)]
mod tests {
    use tokio::net::UdpSocket;

    use crate::dns_server::{DnsServer, FIRST_IPV4_ADDR};
    use trust_dns_client::udp::UdpClientStream;
    use trust_dns_client::client::{AsyncClient, ClientHandle};
    use std::str::FromStr;
    use trust_dns_client::op::DnsResponse;
    use trust_dns_client::rr::{DNSClass, RecordType, Name, IntoName};
    use std::net::{IpAddr, Ipv4Addr};
    use std::error::Error;

    async fn query<N: IntoName>(domain: N, dns_server_port: u16) -> Result<Vec<IpAddr>, Box<dyn Error>> {
        let stream = UdpClientStream::<UdpSocket>::new(([127, 0, 0, 1], dns_server_port).into());
        let (mut client, bg) = AsyncClient::connect(stream).await?;
        tokio::spawn(bg);
        let name = domain.into_name()?;
        let res: DnsResponse = client.query(name.clone(), DNSClass::IN, RecordType::A).await?;
        let answers = res.answers().iter().map(|a| a.rdata().to_ip_addr().unwrap()).collect();
        Ok(answers)
    }

    #[tokio::test]
    async fn sanity() -> Result<(), Box<dyn Error>> {
        let server = DnsServer::bind("127.0.0.1:0").await?;
        let port = server.port()?;
        tokio::spawn(server.run());
        let addrs = query("www.example.com.", port).await?;
        assert_eq!(addrs, [IpAddr::V4(Ipv4Addr::from(FIRST_IPV4_ADDR))]);
        Ok(())
    }

    #[tokio::test]
    async fn domain_stay_in_cache() -> Result<(), Box<dyn Error>> {
        let server = DnsServer::bind("127.0.0.1:0").await?;
        let port = server.port()?;
        tokio::spawn(server.run());
        let addrs1 = query("www.example.com.", port).await?;
        let addrs2 = query("www.example.com.", port).await?;
        assert_eq!(addrs1, [IpAddr::V4(Ipv4Addr::from(FIRST_IPV4_ADDR))]);
        assert_eq!(addrs2, [IpAddr::V4(Ipv4Addr::from(FIRST_IPV4_ADDR))]);
        Ok(())
    }

    #[tokio::test]
    async fn different_addrs_for_different_domains() -> Result<(), Box<dyn Error>> {
        let server = DnsServer::bind("127.0.0.1:0").await?;
        let port = server.port()?;
        tokio::spawn(server.run());
        let addrs1 = query("www.example.com.", port).await?;
        let addrs2 = query("www.example2.com.", port).await?;
        assert_eq!(addrs1, [IpAddr::V4(Ipv4Addr::from(FIRST_IPV4_ADDR))]);
        assert_eq!(addrs2, [IpAddr::V4(Ipv4Addr::from(FIRST_IPV4_ADDR + 1))]);
        Ok(())
    }

    #[tokio::test]
    async fn ipv4_addrs_dont_end_with_0_or_255() -> Result<(), Box<dyn Error>> {
        let server = DnsServer::bind("127.0.0.1:0").await?;
        let port = server.port()?;
        tokio::spawn(server.run());

        for i in 0..256 {
            let addrs = query(format!("www.example{}.com.", i).as_str(), port).await?;
            match addrs[0] {
                IpAddr::V4(ipv4) => {
                    let mod256 = u32::from(ipv4) % 256;
                    assert_ne!(mod256, 0);
                    assert_ne!(mod256, 255);
                },
                _ => unreachable!(),
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn response_match_ipv4_mapping() -> Result<(), Box<dyn Error>> {
        let server = DnsServer::bind("127.0.0.1:0").await?;
        let port = server.port()?;
        let ipv4_mapping = server.ipv4_mapping();
        tokio::spawn(server.run());
        let addrs = query("www.example.com.", port).await?;
        let expected_ipv4_addr = Ipv4Addr::from(FIRST_IPV4_ADDR);
        let expected_name = Name::from_str("www.example.com.")?;
        assert_eq!(addrs, [expected_ipv4_addr]);
        let ipv4_mapping = ipv4_mapping.lock().await;
        assert_eq!(ipv4_mapping.addr_to_name.get_key_value(&expected_ipv4_addr), Some((&expected_ipv4_addr, &expected_name)));
        assert_eq!(ipv4_mapping.name_to_addr.get_key_value(&expected_name), Some((&expected_name, &expected_ipv4_addr)));
        Ok(())
    }
}
