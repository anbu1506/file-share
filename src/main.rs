


mod tcp;
mod mdns;
mod utils;

#[tokio::main]
async fn main() {
    let mut sender = tcp::Sender::new("Jarvis");
    sender.add_file("/home/hunter/Documents/rust/test/WhatsApp Video 2023-09-23 at 8.52.11 AM.mp4");
    sender.set_receiver_addr("127.0.0.1","8000");
    sender.send().await.unwrap();
}