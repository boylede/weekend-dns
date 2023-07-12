use std::env;
use std::net::UdpSocket;

use weekend_dns::packet::*;

fn main() {
    let mut args = env::args();
    let _ = args.next();
    let domain_str = args.next().unwrap_or("www.google.com".to_string());

    let Ok(socket) = UdpSocket::bind("0.0.0.0:5353") else {
        println!("failed to bind to port");
        return;
    };

    {
        let query = Packet::new()
            .with_flags(Flags::new())
            .with_question(Question::new().with_domain_name(&domain_str));
        println!("Sending query: {}", query);
        let buf = query.to_bytes();
        let Ok(_) = socket.send_to(&buf, "8.8.8.8:53") else {
            panic!("failed to send packet");
        };
    }
    {
        let mut buf = [0u8; 1024];
        let Ok(_) = socket.recv_from(&mut buf) else {
            println!("failed to receive anything");
            return;
        };
        let Some(response) = Packet::from_bytes(&buf) else {
            println!("failed to parse packet");
            return;
        };
        println!("Got response packet: {}", response);
    }
}
