use std::{process, sync::Arc, time::Duration};

use mdns_sd::{ServiceDaemon, ServiceEvent};
use tokio::sync::{mpsc, Mutex};

pub fn mdns_offer(port:&str){
    
     let responder = libmdns::Responder::new().unwrap_or_else(|err|{
            println!("connect to a network");
            process::exit(0);
        });
        let _svc = responder.register("_fileshare._tcp".into(),"_fileshare._tcp.local".into(),port.parse::<u16>().unwrap(),&["hello anbu"]);
        

}

pub async fn mdns_scanner()->Vec<(String, String,String)>{


    let mdns = ServiceDaemon::new().expect("Failed to create daemon");

    let service_type = "_fileshare._tcp.local.";
    let receiver = mdns.browse(service_type).expect("Failed to browse");

    let (tx,mut rx) = mpsc::channel::<(String,String,String)>(10);
    
    let count = Arc::new(Mutex::new(0));
    let counter = Arc::clone(&count);

    let (tx1,mut rx1) = mpsc::channel::<bool>(1);

    tokio::spawn( async move{
        let mut value = counter.lock().await;
        loop {
            println!("searching...");
            tokio::select! {
                
                event = async {
                    receiver.recv()
                 }=>{
                    if let ServiceEvent::ServiceResolved(info) = event.unwrap(){
                                println!("Resolved a new service: {:?} {} {} {:?}", info.get_addresses(),info.get_port(),info.get_hostname(),info.get_other_ttl());
                                for ip in info.get_addresses().iter(){
                                if ip.is_ipv6(){
                                    continue;
                                }
                                let port = info.get_port().to_string();
                                let host_name = info.get_hostname().to_string();
                                tx.send((ip.to_string(),port,host_name)).await.unwrap();
                                *value+=1;
                            }
                        }
                }
                f = rx1.recv() => {
                    if f.unwrap() {
                        drop(value);
                        break;
                    }
                }
            }
        }
    });
    tokio::time::sleep(Duration::from_secs(2)).await;
    tx1.send(true).await.unwrap();
    let mut receivers = vec![];
    let count = Arc::clone(&count);
    let count = count.lock().await;
    let count = *count;
    for _ in 0..count{
        receivers.push(rx.recv().await.unwrap());
    }
    mdns.shutdown().unwrap();

    println!("receivers: {:?}",receivers);

    receivers
}
