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

async fn heartbeat(
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
            let _enc_data = srv_pub_key
                .encrypt(
                    &mut rng,
                    PaddingScheme::new_pkcs1v15_encrypt(),
                    &json_message.as_bytes(),
                )
                .expect("failed to encrypt");
        }
    }
}

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

    let mut _stream = TcpStream::connect("10.0.0.12:53").await?;
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
    println!("{:?}", json_message.message_content);
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
    loop {
        let sleep_time = std::time::Duration::from_secs(rng.gen_range(1800..3600));
        std::thread::sleep(sleep_time);
        println!("Sending heartbeat");
        heartbeat(&username, &srv_pub_key, rng_thread).await;
    }

    //Shell
}
