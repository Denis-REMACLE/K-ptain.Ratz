use std::net::UdpSocket;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

pub struct BytePacketBuffer {
    pub buf: [u8; 512],
    pub pos: usize,
}

pub fn new() -> BytePacketBuffer {
    BytePacketBuffer {
        buf: [0; 512],
        pos: 0,
    }
}

fn handle_query(socket: &UdpSocket) -> Result<()> {
    let mut req_buffer = new();
    let (_, src) = socket.recv_from(&mut req_buffer.buf)?;

    println!("{}", src);

    Ok(())
}

fn main() -> Result<()> {
    let socket = UdpSocket::bind(("192.168.1.41", 53))?;

    loop {
        match handle_query(&socket) {
            Ok(_) => {}
            Err(e) => eprintln!("An error occurred: {}", e),
        }
    }
}