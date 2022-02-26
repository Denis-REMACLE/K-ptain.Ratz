use std::net::TcpStream;



fn main() {

    
    let stream = TcpStream::connect("127.0.0.1:8080")
                           .expect("Couldn't connect to the server...");
    stream.set_read_timeout(None).expect("set_read_timeout call failed");
    assert_eq!(stream.read_timeout().unwrap(), None);

}