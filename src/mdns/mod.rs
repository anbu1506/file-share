pub fn mdns_offer(){
    let responder = libmdns::Responder::new().unwrap();
    let _svc = responder.register("_fileshare._tcp".into(),"Jarvis".into(),5432,&["hello"]);
    loop{
        std::thread::sleep(std::time::Duration::from_secs(5));
        println!("light_house is alive ...");
    }
}

pub async fn search_receiver(){
    use mdns_sd::{ServiceDaemon, ServiceEvent};

let mdns = ServiceDaemon::new().expect("Failed to create daemon");

let service_type = "_fileshare._tcp.local.";
let receiver = mdns.browse(service_type).expect("Failed to browse");

let handle=std::thread::spawn(move || {
    while let Ok(event) = receiver.recv() {
        if let ServiceEvent::ServiceResolved(info) = event{
            println!("Resolved a new service: {}", info.get_fullname());
        
        }
    }
});
handle.join().unwrap();
std::thread::sleep(std::time::Duration::from_secs(1));
mdns.shutdown().unwrap();
}
