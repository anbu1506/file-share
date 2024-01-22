mod tcp;
mod mdns;
mod utils;
mod app;

#[tokio::main]
async fn main() {

    let handle =tokio::spawn(async move{
       let mut rec= tcp::Receiver::new("Jarvis");
       
       rec.listen_on("8080",false).await.unwrap();

    });

    handle.await.unwrap();
}