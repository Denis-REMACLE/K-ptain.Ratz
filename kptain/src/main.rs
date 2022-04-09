use std::net;
use std::env;

fn listen(socket: &net::UdpSocket, mut buffer: &mut [u8]) -> usize {

    let (number_of_bytes, src_addr) = socket.recv_from(&mut buffer).expect("no data received");

    println!("{:?}", number_of_bytes);
    println!("{:?}", src_addr);

    number_of_bytes
}

fn send(socket: &net::UdpSocket, receiver: &str, msg: &Vec<u8>) -> usize {

    println!("sending data");
    let result = socket.send_to(msg, receiver).expect("failed to send message");

    result
}

fn init_host() -> net::UdpSocket {

    println!("initializing host");
    let socket = net::UdpSocket::bind(("192.168.1.41", 53)).expect("failed to bind host socket");

    socket
}

fn main() {
    let mut buf: Vec<u8> = Vec::with_capacity(100);
    let socket = init_host();
    let message = String::from("hello");
    let msg_bytes = message.into_bytes();

    loop {
        while listen(&socket, &mut buf) != 0 {
            println!("boo");
        }
    }
}
