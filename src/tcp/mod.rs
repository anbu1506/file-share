use tokio::{io::{ copy, AsyncWriteExt, AsyncReadExt}, net::{TcpListener, TcpStream}};

fn padding(name:String)->String{
    let mut name=name;
    while name.len()<=255{
        name.push(' ');
    }
    name

}

fn remove_padding(name:String)->String{
    let mut name=name;
    while name.len()>0 && name.chars().last().unwrap()==' '{
        name.pop();
    }
    name
}

pub struct Sender{
    name:String,
    my_streams_addr:Vec<String>,
    receiver_addr:String,
    files:Vec<String>
}

impl Sender{
    pub fn new(name:String)->Sender{
        Sender{
            name,
            my_streams_addr:vec![],
            files:vec![],
            receiver_addr:"".to_string(),
        }
    }

    pub fn add_file(&mut self,file_name:String){
        self.files.push(file_name);
    }

    pub fn set_receiver_addr(&mut self,receiver_addr:String){
        self.receiver_addr=receiver_addr;
    }

    async fn connect_nth_stream(&mut self,n:i32)->Result<TcpStream, Box<dyn std::error::Error>>{
        println!("stream {} connecting to receiver...",n);
        let  stream = tokio::net::TcpStream::connect(self.receiver_addr.as_str()).await?;
        self.my_streams_addr.push(stream.peer_addr()?.to_string());
        println!("stream {} connected to receiver",n);
       Ok(stream)
    }

    pub async fn send(&mut self)->Result<(),Box<dyn std::error::Error>>{
        let mut handles = vec![];
        let mut i=1;
        while self.files.len()!=0{
            let mut stream = self.connect_nth_stream(i as i32).await?;
            let file_path = self.files.pop().unwrap();
            let handle =tokio::spawn(async move{
                handle_transfer(file_path.as_str(),&mut stream).await.unwrap();
            });
            handles.push(handle);
            i+=1;
        }
        println!("i:{} flen:{}",i,self.files.len());
        for handle in handles{
            handle.await?;
        }
        Ok(())
    }
}
async fn handle_transfer(file_path:&str,stream:&mut tokio::net::TcpStream)->Result<(), Box<dyn std::error::Error>>{
    let file_name = std::path::Path::new(file_path).file_name().unwrap().to_str().unwrap();
    let file_name = padding(file_name.to_string());
    //chech the file exists or not
    let  file = tokio::fs::File::open(file_path).await?;
    //then sending file_name
    stream.write_all(file_name.as_bytes()).await?;
    //then sending data
    let mut file_reader = tokio::io::BufReader::new(file);
    let bytes_transferred = copy(&mut file_reader,  stream).await?;
    println!("Transferred {} bytes.", bytes_transferred);
    Ok(())
}






pub struct Receiver{
    name:String,
    my_addr:String,
    sender_addr:String,
    files:Vec<String>,
}


impl Receiver {

    pub fn new(name:String)->Receiver{
        Receiver{
            name,
            my_addr:"".to_string(),
            sender_addr:"".to_string(),
            files:vec![],
        }
    }

    pub async fn listen_on(port:&str)->Result<(),Box<dyn std::error::Error>>{
        let listener = TcpListener::bind("127.0.0.1:".to_owned()+port).await?;
        println!("Listening on port {}",port);
        let mut handles = vec![];
        let mut i=0;
        while i<5{
            let (mut stream, _) = listener.accept().await?;
            println!("connection accepted from sender {}",stream.peer_addr()?);
            let handle =tokio::spawn(async move{
                receive(&mut stream).await.unwrap();
            });
            handles.push(handle);
            i+=1;
        }
        println!("waiting for all handles to join");
        for handle in handles{
            handle.await?;
        }
        Ok(())
    }
}



async fn receive(stream:& mut TcpStream)->Result<(), Box<dyn std::error::Error>>{
    println!("receiving file");
    let mut file_name = [0u8; 255];
    stream.read_exact(&mut file_name).await?;
    let file_name = remove_padding(String::from_utf8(file_name.to_vec())?);
    let download_path = home::home_dir().unwrap().join("Downloads").join(file_name);
    let mut dest_file = tokio::fs::File::create(download_path).await?;
    let bytes_transferred = copy( stream, &mut dest_file).await?;
    println!("Received and saved {} bytes.", bytes_transferred);
    Ok(())
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