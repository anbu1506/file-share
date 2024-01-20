


mod tcp;
mod mdns;
mod utils;

#[tokio::main]
async fn main() {
    let mut sender = tcp::Sender::new("Jarvis");
    sender.add_file("/home/hunter/Documents/rust/test/American_Psycho_2000_Uncut_1080p_UHD_BluRay_x265_HDR_DD+5_1_Pahe.mkv");
    sender.set_receiver_addr("127.0.0.1","8080");
    sender.send().await.unwrap();
}                                                 