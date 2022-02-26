use std::net;
use std::io;

// fn listen(socket: &net::UdpSocket) -> Vec<u8> {

//     let mut buf: [u8; 20] = [0; 20];
//     let _number_of_bytes: usize = 0;
//     let mut result: Vec<u8> = Vec::new();
//     match socket.recv_from(&mut buf) {
//         Ok((number_of_bytes, _src_addr)) => {
//             println!("received message: {:?}", buf);
//             result = Vec::from(&buf[0..number_of_bytes]);
//         }
//         Err(fail) => println!("failed listening {:?}", fail)
//     }

//     let display_result = result.clone();
//     let result_str = String::from_utf8(display_result).unwrap();
//     println!("received message: {:?}", result_str);
//     result
// }

fn send(socket: &net::UdpSocket, receiver: &str, msg: &Vec<u8>) -> usize {

    println!("sending message: {:?}", msg);
    let result: usize = 0;
    match socket.send_to(&msg, receiver) {
        Ok(number_of_bytes) => println!("{:?}", number_of_bytes),
        Err(fail) => println!("failed sending {:?}", fail),
    }

    result
}

fn init_host(host: &str) -> net::UdpSocket {

    println!("initializing host: {:?}", host);
    let socket = net::UdpSocket::bind(host).expect("failed to bind host socket");
    let duration = std::time::Duration::new(1, 0);
    let dur = std::option::Option::Some(duration);
    let _res = socket.set_read_timeout(dur).expect("failed to set timeout");

    socket
}

fn show_menu(config: &HostConfig, message: &str) {

    println!("Menu:\n\
    Local ip: -local {local_ip} \n\
    Local port: -lport {local_port} \n\
    Remote ip: -remote {remote_ip} \n\
    Remote port: -rport {remote_port} \n\
    Start host: -lstart \n\
    Connect to remote: -rconnect \n\
    Send message: -msg {message}",
             local_ip = config.local_ip,
             local_port = config.local_port,
             remote_ip = config.remote_ip,
             remote_port = config.remote_port,
             message = message);
}

#[derive(Debug, Default)]
struct HostConfig {
    local_ip: String,
    local_port: String,
    local_host: String,
    remote_ip: String,
    remote_port: String,
    remote_host: String,
}

#[derive(Debug)]
enum CommandInput {
    local_ip(String),
    local_port(String),
    remote_ip(String),
    remote_port(String),
    start_host,
    connect_remote,
    message(String),
    unknown(String),
    error(String),
}

fn identify_comand(command: &str, data: &str) -> CommandInput {

    match command {
        "-local" => CommandInput::local_ip(data.to_owned()),
        "-lport" => CommandInput::local_port(data.to_owned()),
        "-remote" => CommandInput::remote_ip(data.to_owned()),
        "-rport" => CommandInput::remote_port(data.to_owned()),
        "-lstart" => CommandInput::start_host,
        "-rconnect" => CommandInput::connect_remote,
        "-msg" => CommandInput::message(data.to_owned()),
        _ => CommandInput::unknown(data.to_owned()),
    }
}

fn read_console() -> CommandInput {

    
    let mut input = String::with_capacity(25);
    match io::stdin().read_line(&mut input) {
        Ok(_bytes_read) => {
            println!("read: {}", input);
            let mut split_input = input.split_whitespace();
            let cmd = split_input.next().unwrap();
            let data = split_input.collect::<String>();
            println!("cmd: {} ------ data: {}", cmd, data);
            identify_comand(&cmd, &data)
        }
        Err(fail) => {
            println!("Failed to read console: {}", fail);
            let invalid_data = "failed to read console".to_owned();
            CommandInput::error(invalid_data)
        }
    }
}

fn set_host_parameters(ip: &str, port: &str) -> String {

    
    let mut host = String::with_capacity(128);
    host.push_str(ip);
    host.push_str(":");
    host.push_str(port);

    host
}

fn build_config(cmd_input: CommandInput, host_config: &mut HostConfig) {

    println!("build: {:?}", cmd_input);
    match cmd_input {
        CommandInput::local_ip(ip) => {
            host_config.local_ip = ip;
            host_config.local_host = set_host_parameters(&host_config.local_ip,
                                                         &host_config.local_port);
        }
        CommandInput::local_port(port) => {
            host_config.local_port = port;
            host_config.local_host = set_host_parameters(&host_config.local_ip,
                                                         &host_config.local_port);
        }
        CommandInput::remote_ip(ip) => {
            host_config.remote_ip = ip;
            host_config.remote_host = set_host_parameters(&host_config.remote_ip,
                                                          &host_config.remote_port);
        }
        CommandInput::remote_port(port) => {
            host_config.remote_port = port;
            host_config.remote_host = set_host_parameters(&host_config.remote_ip,
                                                          &host_config.remote_port);
        }
        _ => println!("Not a configuration."),
    }

}

fn main() {

    let mut host_config = HostConfig {
        local_ip: "127.0.0.1".to_owned(),
        local_port: "7777".to_owned(),
        
        local_host: String::with_capacity(128),
        remote_ip: "127.0.0.1".to_owned(),
        remote_port: "8888".to_owned(),
        
        remote_host: String::with_capacity(128),
    };
    let default_msg = "hello world";

    host_config.local_host = set_host_parameters(&host_config.local_ip, &host_config.local_port);
    host_config.remote_host = set_host_parameters(&host_config.remote_ip, &host_config.remote_port);

    
    let mut message = String::with_capacity(128);

    loop {
        show_menu(&host_config, &default_msg);
        match read_console() {
            CommandInput::start_host => {
                println!("starting host");
                break;
            }
            CommandInput::connect_remote => println!("connecting to remote host"),
            CommandInput::message(msg) => {
                message = msg;
            } 
            CommandInput::unknown(unknown_data) => println!("unknown_data: {:?}", unknown_data),
            CommandInput::error(fail) => println!("error: {:?}", fail),
            input_cmd @ _ => build_config(input_cmd, &mut host_config),
        }
    }

    let mut socket: net::UdpSocket = init_host(&host_config.local_host);
    println!("host config: {:?}", host_config);
    println!("socket: {:?}", socket);
    let msg_bytes = message.into_bytes();

    
    let sleep_time = std::time::Duration::from_secs(1);
    std::thread::sleep(sleep_time);

    loop {
        
        // let received_msg = listen(&socket);
        send(&socket, &host_config.remote_host, &msg_bytes);
        // send(&socket, &client_arg, &msg_bytes);
    }
}