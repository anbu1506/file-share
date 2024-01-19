


mod tcp;
mod mdns;


#[tokio::main]
async fn main() {
    let mut sender = tcp::Sender::new("Jarvis".to_string());
    sender.add_file("Cargo.toml".to_string());
    sender.add_file("Cargo.toml".to_string());
    sender.add_file("Cargo.lock".to_string());
    sender.set_receiver_addr("127.0.0.1:8080".to_string());
    sender.send().await.unwrap();
}                                                 