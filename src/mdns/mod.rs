use mdns_sd::{ServiceDaemon, ServiceEvent};

pub fn mdns_offer(port:&str,name:&str){
    let responder = libmdns::Responder::new().unwrap();
    let _svc = responder.register("_fileshare._tcp".into(),name.into(),port.parse::<u16>().unwrap(),&[(name.to_owned()+": i'm Alive").as_str()]);
}

pub async fn mdns_scanner(){


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
