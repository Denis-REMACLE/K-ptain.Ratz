/*
extern crate daemonize_me;

use daemonize_me::{Daemon, Group, User};
use std::convert::TryFrom;
use std::fs::File;
*/
use std::thread;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Result;
/*
fn invocate_daemon() {
    let stdout = File::create("info.log").unwrap();
    let stderr = File::create("err.log").unwrap();
    let daemon = Daemon::new()
        .pid_file("example.pid", Some(false))
        .user(User::try_from("daemon").unwrap())
        .group(Group::try_from("daemon").unwrap())
        .umask(0o000)
        .work_dir(".")
        .stdout(stdout)
        .stderr(stderr)
        .start();

    match daemon {
        Ok(_) => println!("Daemonized with success"),
        Err(e) => eprintln!("Error, {}", e),
    }
}
*/
fn handle_connection(stream: TcpStream) -> Result<()> {
    println!("Hello, world! {:?}", stream);
    Ok(())
}

fn main()  -> std::io::Result<()> {
    let listener = TcpListener::bind("192.168.1.121:80").unwrap();

    for stream in listener.incoming() {
        thread::spawn(move || handle_connection(stream?));
    }
    Ok(())
}