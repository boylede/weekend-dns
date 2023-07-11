use rand::Rng;
use std::fmt::Write;
use std::net::UdpSocket;

use weekend_dns::{
    packet::*,
    record::{Class, Kind},
    response::*,
    serialization::to_hex_bytes,
};

fn main() {
    let query = Packet::build("www.google.com", Kind::A);

    let Ok(socket) = UdpSocket::bind("0.0.0.0:5353") else {
        println!("failed to bind to port");
        return;
    };

    {
        println!(" sending query: {:?}", query);
        let buf = query.to_bytes();
        socket.send_to(&buf, "8.8.8.8:53");
    }
    println!("sent, waiting for response...");
    {
        let mut buf = [0u8; 1024];
        let Ok((length, a)) = socket.recv_from(&mut buf) else {
            println!("failed to receive anything");
            return;
        };
        for i in 0..length {
            print!("{:02x}", buf[i]);
        }
        println!("");

        let response = Packet::from_bytes(&buf);
    }
    println!("\nfinished successfully");
    // let test_query = Query::build("www.example.com", Kind::A).with_id(0x8298);
    // let test_expectation = "82980100000100000000000003777777076578616d706c6503636f6d0000010001";
    // let result = to_hex_bytes(&test_query.to_bytes());
    // println!("{result}, {}", result == test_expectation);
}
