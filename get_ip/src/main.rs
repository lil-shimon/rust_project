fn main() {
    println!("{:?}", get_ip());
}

fn get_ip() -> Vec<String> {
    use pnet::datalink;
    let mut ip: Vec<String> = Vec::new();
    for interface in datalink::interfaces() {
        if !interface.ips.is_empty() && interface.is_up() {
            for ip_net in interface.ips {
                if ip_net.is_ipv4() && !ip_net.ip().is_loopback() {
                    ip.push(ip_net.ip().to_string());
                }
            }
        }
    }
    ip
}
