use tokio::{io::{ copy, AsyncWriteExt, AsyncReadExt}, net::{TcpListener, TcpStream}};

use crate::{utils::{padding, remove_padding, create_or_incnum, }, mdns::{mdns_offer, mdns_scanner}};



pub struct Sender<'a>{
    name:&'a str,
    my_streams_addr:Vec<String>,
    receiver_ip:&'a str,
    receiver_port:&'a str,
    files:Vec<&'a str>
}

impl<'a> Sender<'a>{
    pub fn new(name:&'a str)->Sender<'a>{
        Sender{
            name,
            my_streams_addr:vec![],
            files:vec![],
            receiver_ip:"",
            receiver_port:""
        }
    }

    pub fn add_file(&mut self,file_name:&'a str){
        self.files.push(file_name);
    }

    pub async fn search_receiver(){
        tokio::spawn(async move{
            mdns_scanner().await;
        });
    }

    pub fn set_receiver_addr(&mut self,receiver_ip:&'a str,receiver_port:&'a str){
        self.receiver_ip=receiver_ip;
        self.receiver_port=receiver_port;
    }

    async fn connect_nth_stream(&mut self,n:i32)->Result<TcpStream, Box<dyn std::error::Error>>{
        println!("stream {} connecting to receiver...",n);
        let  stream = tokio::net::TcpStream::connect(self.receiver_ip.to_owned()+":"+&self.receiver_port).await?;
        self.my_streams_addr.push(stream.peer_addr()?.to_string());
        println!("stream {} connected to receiver",n);
       Ok(stream)
    }

    async fn handle_transfer(file_path:&str,stream:&mut tokio::net::TcpStream,sender_name:&str)->Result<(), Box<dyn std::error::Error>>{
        let file_name = std::path::Path::new(file_path).file_name().unwrap().to_str().unwrap();
        let file_name = padding(file_name.to_string());
        //chech the file exists or not
        let  file = tokio::fs::File::open(file_path).await?;
        //then sending file_name
        stream.write_all(file_name.as_bytes()).await?;
        //sending sender_name
        stream.write_all(padding(sender_name.to_string()).as_bytes()).await?;
        //then sending data
        let mut file_reader = tokio::io::BufReader::new(file);
        let bytes_transferred = copy(&mut file_reader,  stream).await?;
        println!("Transferred {} bytes.", bytes_transferred);
        Ok(())
    }

    pub async fn send(&mut self)->Result<(),Box<dyn std::error::Error>>{
        let mut handles = vec![];
        let mut i=1;
        while self.files.len()!=0{
            let mut stream = self.connect_nth_stream(i as i32).await?;
            let file_path = self.files.pop().unwrap().to_string();
            let sender_name = self.name.to_string();
            let handle =tokio::spawn(async move{
                Self::handle_transfer(file_path.as_str(),&mut stream,sender_name.as_str()).await.unwrap();
            });
            handles.push(handle);
            i+=1;
        }

        for handle in handles{
            handle.await?;
        }
        Ok(())
    }

}







pub struct Receiver<'a>{
    name:&'a str,
    my_ip:&'a str,
    my_port:&'a str,
    sender_streams_addr:Vec<String>,
    files:Vec<String>,
}


impl<'a> Receiver<'a>{

    pub fn new(name:&'a str)->Receiver<'a>{
        Receiver{
            name,
            my_ip:"0.0.0.0",
            my_port:"8080",
            sender_streams_addr:vec![],
            files:vec![],
        }
    }

    pub async fn notify_all(&self){
        let port =self.my_port.to_owned();
        let name = self.name.to_owned();
        let handle = tokio::spawn(async move{
            mdns_offer(port.as_str(),name.as_str());
        });
        handle.await.unwrap();
    }

    pub async fn listen_on(&mut self,port:&'a str)->Result<(),Box<dyn std::error::Error>>{
        self.my_port=port;
        let listener = TcpListener::bind(self.my_ip.to_owned()+":"+self.my_port).await?;
        println!("Listening on port {}",port);
        let mut handles = vec![];
        let mut i=0;
        while i<5{
            let (mut stream, _) = listener.accept().await?;
            println!("connection accepted from sender {}",stream.peer_addr()?);
            self.sender_streams_addr.push(stream.peer_addr()?.to_string());
            let handle =tokio::spawn(async move{
                Self::receive(&mut stream).await.unwrap()
            });
            handles.push(handle);
            i+=1;
        }
        println!("waiting for all handles to join");
        for handle in handles{
            let file_name = handle.await?;
            self.files.push(file_name);
        }
        Ok(())
    }

    async fn receive(stream:& mut TcpStream)->Result<String, Box<dyn std::error::Error>>{
        
        let mut file_name = [0u8; 255];
        stream.read_exact(&mut file_name).await?;
        let file_name = remove_padding(String::from_utf8(file_name.to_vec())?);

        let mut sender_name = [0u8;255];
        stream.read_exact(&mut sender_name).await?;
        let sender_name = remove_padding(String::from_utf8(sender_name.to_vec())?);

        println!("receiving {} from {}",file_name,sender_name);

        let download_path = home::home_dir().unwrap().join("Downloads").join(file_name.as_str());
        let mut dest_file = create_or_incnum(download_path).await?;
        let bytes_transferred = copy( stream, &mut dest_file).await?;
        println!("Received {} bytes from {} .", bytes_transferred,sender_name);
        Ok(file_name)
    }
}





// pub async fn connect_to_receiver(recv_addr:&str)->Result<TcpStream, Box<dyn std::error::Error>>{
//     println!("connecting to receiver...");
//     let  stream = tokio::net::TcpStream::connect(recv_addr).await?;
//     println!("connected to receiver");
//    Ok(stream)
// }



// pub async fn start_receiver()->Result<(), Box<dyn std::error::Error>>{
//     let listener = TcpListener::bind("127.0.0.1:8080").await?;
//     println!("Listening on port 8080");
//     let mut handles = vec![];
//     let mut i=0;
//     while i<5{
//         let (mut stream, _) = listener.accept().await?;
//         println!("connection accepted from sender {}",stream.peer_addr()?);
//         let handle =tokio::spawn(async move{
//             receive(&mut stream).await.unwrap();
//         });
//         handles.push(handle);
//         i+=1;
//     }
//     println!("waiting for all handles to join");
//     for handle in handles{
//         handle.await?;
//     }
//     Ok(())
// }