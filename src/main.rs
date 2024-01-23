use std::{env, process};



mod tcp;
mod mdns;
mod utils;

#[tokio::main]
async fn main() {
    let args:Vec<String> = env::args().collect();
    let mut port:&str = "8080";
    let role = match  args.get(1) {
        Some(role)=> {
            if role == "-s" {
                "sender"
            } else if role == "-r" {
                port = match args.get(2) {
                    Some(port) =>{
                        port.as_str()
                    }
                    None =>{
                        println!("invalid usage");
                        println!("usage:");
                        println!("-s (sender) or -r (receiver) 8080 (port)");
                        return;
                    }
                };
                "receiver"
            } else {
                println!("invalid usage");
                println!("usage:");
                println!("-s (sender) or -r (receiver) 8080 (port)");
                return;
            }
        },
        None => {
            println!("invalid usage");
            println!("usage:");
            println!("-s (sender) or -r (receiver) 8080 (port)");
            return;
        }
    };
    if role == "sender"{
        let mut sender = tcp::Sender::new();
     sender.select_files().await;
     sender.search_and_set_receiver().await;
     sender.send().await.unwrap();
    }

    else if role == "receiver"{

        let responder = libmdns::Responder::new().unwrap_or_else(|err|{
            println!("connect to a network");
            process::exit(0);
        });
        let _svc = responder.register("_fileshare._tcp".into(),"_fileshare._tcp.local".into(),port.parse::<u16>().unwrap(),&["hello anbu"]);
        
        // mdns_offer(port);
        let mut rec= tcp::Receiver::new();
        rec.listen_on(port).await.unwrap();
    }
   
     

    
}
