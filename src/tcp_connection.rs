use std::net::{Ipv4Addr, TcpStream};

pub fn create_connections(ip: Ipv4Addr, ports: Vec<u16>) -> Vec<TcpStream> {
    let mut data: Vec<TcpStream> = Vec::new();

    for port in ports {
        let stream = TcpStream::connect((ip, port)).expect("Failed to connect to server");
        data.push(stream);
    }

    data
}
