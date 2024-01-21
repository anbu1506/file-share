mod tcp;
mod mdns;
mod utils;

#[tokio::main]
async fn main() {

    let handle =tokio::spawn(async move{
       let mut rec= tcp::Receiver::new("Jarvis");
       
       rec.listen_on("8080",true).await.unwrap();

    });


    let mut sender = tcp::Sender::new("Jarvis");
    tokio::spawn(async move{
        tcp::Sender::search_receiver().await;
    });
    sender.add_file("/home/hunter/Documents/rust/test/The.Green.Mile.1999.480p.BluRay.x264.700MB-[Mkvking.com].mkv");
    sender.add_file("/home/hunter/Documents/rust/test/American_Psycho_2000_Uncut_1080p_UHD_BluRay_x265_HDR_DD+5_1_Pahe.mkv");
    sender.set_receiver_addr("192.168.189.28", "8080");
    sender.send().await.unwrap();

    handle.await.unwrap();
}