use tokio::process::Command;
use tokio::io::{Error, ErrorKind};
use std::net::{Ipv4Addr};
use std::path::{Path};
use std::fs::create_dir_all;
use std::fs::File;
use std::io::prelude::*;
use std::process::Output;

#[derive(Debug)]
pub struct NetworkNamespace {
    name: String,
}


macro_rules! check_ok_return_code {
    ($output:expr) => {
        match $output.status.code() {
            Some(code) => {
                if code != 0 {
                    let error_data = match std::str::from_utf8(&$output.stderr) {
                        Ok(s) => s,
                        Err(_) => "",
                    };
                    return Err(Error::new(ErrorKind::Other, format!("Non-Zero exit code ({0}), {1}", code, error_data)))
                }
            },
            None => return Err(Error::new(ErrorKind::Other, format!("Process was killed by a signal"))),
        }; 
    }
}


impl NetworkNamespace {
    pub async fn new(name: &str) -> Result<NetworkNamespace, Error> {
        let output = Command::new("ip")
            .arg("netns")
            .arg("add")
            .arg(name)
            .output().await?;

        check_ok_return_code!(output);

        let ns = NetworkNamespace{name: String::from(name)};
        Ok(ns)
    }

    pub async fn attach_interface(&self, iface_name: &str) -> Result<(), Error> {
        let output = Command::new("ip")
            .arg("link")
            .arg("set")
            .arg(iface_name)
            .arg("netns")
            .arg(&self.name)
            .output().await?;

        check_ok_return_code!(output);
        Ok(())  
    }

    pub fn configure_dns_servers(&self, servers: &[Ipv4Addr]) -> std::io::Result<()> {
        let ns_dir = Path::new("/etc/ns").join(&self.name);
        if !ns_dir.exists() {
            create_dir_all(ns_dir.as_path())?;
        }
        let mut ns_resolv_file = File::create(ns_dir.join("resolv.conf").as_path())?;

        let mut servers_strings: Vec<String> = Vec::with_capacity(servers.len());
        for server in servers {
            servers_strings.push(format!("nameserver {0}", server));
        }
        ns_resolv_file.write_all(servers_strings.join("\n").as_bytes())?;

        Ok(())
    }

    pub async fn exec_inside(&self, cmd: &str, args: &[&str]) -> Result<Output, Error> {
        let output = Command::new(cmd)
            .arg("ip")
            .arg("netns")
            .arg(&self.name)
            .arg("exec")
            .arg(cmd)
            .args(args)
            .output().await?;
        
        
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use crate::ns::NetworkNamespace;

    
}

