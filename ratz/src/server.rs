use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, Error};
use std::io;
extern crate time;
extern crate schedule_recv;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Connection depuis: {}", stream.peer_addr()?);
    let mut buf = [0; 516];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 { return Ok(()) }
        stream.write(&buf[..bytes_read])?;
    }
}

fn im_online(ip:str) {
    let online = schedule_recv::periodic_ms(300000);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("IP : {}", n);
        }
        Err(error) => println!("error: {}", error),
    }
    loop {
        online.recv().unwrap();
        match TcpStream::connect("{}:80", ip) {
            Ok(mut stream) => {
                println!("Connected\n Trying to send message...");

                let msg = b"Im Online !";
                stream.write(msg).unwrap();
                println!("Envoie du message ..");

                let mut data = [0 as u8; 6]; // on utilise un buffer de 6 bytes
                match stream.read_exact($mut data) {
                    Ok(_) => {
                        if &data == msg {
                            println!("Reponse ok");

                        }else {
                            let text = from_utf8(&data).unwrap();
                            println!("Unexepted reply : {}", text);
                        }
                    },
                    Err(e) => {
                        println!("Echec de reception de data: {}", e);

                    }
                },
                Err(e) => {
                    println!("Failed to connect: {}", e);
                }

            }
        }
        println!("Fin");
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:5000").expect("Echec de la connection");
    for stream in listener.incoming() {
        match stream {
            Err(e) => { eprintln!("failed: {}", e)}
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}