use std::env;




mod tcp;
mod mdns;
mod utils;
mod app;

#[tokio::main]
async fn main() {
    let args:Vec<String> = env::args().collect();
    let role = match  args.get(1) {
        Some(role)=> {
            if role == "-s" {
                "sender"
            } else if role == "-r" {
                "receiver"
            } else {
                println!("invalid usage");
                println!("usage:");
                println!("-s (sender) or -r (receiver) -p (port)");
                return;
            }
        },
        None => {
            println!("invalid usage");
            println!("usage:");
            println!("-s (sender) or -r (receiver) -p (port)");
            return;
        }
    };
    if role == "sender"{
        let mut sender = tcp::Sender::new();
     sender.select_files().await;
     sender.search_and_set_receiver().await;
     sender.send().await.unwrap();
    }
   
     

    
}