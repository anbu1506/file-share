


mod tcp;
mod mdns;


#[tokio::main]
async fn main() {
    let mut sender = tcp::Sender::new("Jarvis");
    sender.add_file("Cargo.toml");
    sender.add_file("Cargo.toml");
    sender.add_file("Cargo.lock");
    sender.set_receiver_addr("127.0.0.1","5432");
    sender.send().await.unwrap();
}                                                 