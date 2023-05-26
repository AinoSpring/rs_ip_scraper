use std::net::{IpAddr, Ipv4Addr};
use std::thread;
use futures::executor::block_on;

#[tokio::main]
async fn main() {
    let conn = sqlite::open("database.db")
        .unwrap();
    let mut ip: u32 = 1;
    while ip > 0 {
        let ip_addr = get_ip(&ip);
        thread::spawn(move || block_on(scan_ip(ip_addr)));
        ip += 1;
    }
}

async fn scan_ip(ip: IpAddr) -> bool {
    match surge_ping::ping(ip, &[])
        .await {
        Ok(_) => {},
        Err(_) => return false,
    }
    true
}

fn get_ip(ip: &u32) -> IpAddr {
    let a: u8 = (ip >> 24) as u8;
    let b: u8 = (ip >> 16) as u8;
    let c: u8 = (ip >> 8) as u8;
    let d: u8 = ip.to_owned() as u8;
    IpAddr::V4(Ipv4Addr::new(a, b, c, d))
}
