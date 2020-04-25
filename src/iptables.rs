RULE_FMT: &str = "iptables -t {table} {action} {chain} -i {input_iface} -p {protocol} -j {target} {target_args}"

enum Action {
    Append,
    Delete,
    Insert,
}

pub struct IpTablesRule {
    table: String,
    chain: String,
    input_iface: String,
    protocol: String, // TODO: maybe enum
    target: String,
    target_args: String,
}

impl IpTablesRule {
    fn to_string(&self, action: Action) -> String {
        let action_str = match action {
            Action::Append => "-A",
            Action::Delete => "-D",
            Action::Instert => "-I"
            _ => panic!("Bad Action")
        }
        
        format!(
            RULE_FMT,
            table=self.table,
            action=action_str,
            chain=self.chain,
            input_iface=self.input_iface,
            protocol=self.protocol,
            target=self.target,
            target_args=self.target_args
        )
    }

    pub fn tproxy() -> Result<IpTablesRule>
    {
        
    }
}

impl Drop for IpTablesRule {
    fn Drop(&mut self) {
        let cmdline = self.to_string(Action::Delete)
        // run       
    }
}
