use std::{fmt::Write, process};

use dialoguer::{theme::ColorfulTheme, Select};
use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use tokio::{io::{ copy, AsyncWriteExt, AsyncReadExt}, net::{TcpListener, TcpStream}};

use crate::{utils::{padding, remove_padding, create_or_incnum, }, mdns::mdns_scanner};

use rfd::AsyncFileDialog;

#[derive(Debug)]
pub struct Sender{
    name:String,
    my_streams_addr:Vec<String>,
    receiver_ip:String,
    receiver_port:String,
    files:Vec<String>
}

impl Sender{
    pub fn new()->Sender{
        let  name = hostname::get().unwrap();
        let name = name.to_str().unwrap().to_string();
        Sender{
            name,
            my_streams_addr:vec![],
            files:vec![],
            receiver_ip:"".to_owned(),
            receiver_port:"".to_owned()
        }
    }
    pub async fn  select_files(&mut self) {
        let future = async {
            let file = AsyncFileDialog::new()
                .set_directory("/").pick_files()
                .await;
        
            let files = file.unwrap();

            for file in files {
                let path = file.path().to_str().unwrap().to_owned();
                self.files.push(path);
            }
        };
        future.await;
    }

    pub async fn search_and_set_receiver(&mut self){
       
        let rec = mdns_scanner().await;
        let mut ips = vec!["none".to_owned()];
        let mut ports = vec!["none".to_owned()];
        let mut names = vec!["none online. exit".to_owned()];
        rec.iter().for_each(|(ip,port,name)|{
            // println!("{} {} {}",ip,port,name);
            ips.push(ip.to_owned());
            ports.push(port.to_owned());
            names.push(name.to_owned());
        });

        let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select a receiver")
                .items(&names)
                .default(0) 
                .interact()
                .unwrap();
        if selection == 0{
            println!("exited ...");
            process::exit(1);
        }
        let ip = ips[selection].to_owned();
        let port = ports[selection].to_owned();
        self.set_receiver_addr(ip, port);
    }

    pub fn set_receiver_addr(&mut self,receiver_ip:String,receiver_port:String){
        self.receiver_ip=receiver_ip;
        self.receiver_port=receiver_port;
    }

    async fn connect_nth_stream(&mut self,n:i32)->Result<TcpStream, Box<dyn std::error::Error>>{
        // println!("stream {} connecting to receiver...",n);
        let  stream = tokio::net::TcpStream::connect(self.receiver_ip.to_owned()+":"+&self.receiver_port).await?;
        self.my_streams_addr.push(stream.peer_addr()?.to_string());
        // println!("stream {} connected to receiver",n);
       Ok(stream)
    }

    async fn handle_transfer(file_path:&str,stream:&mut tokio::net::TcpStream,sender_name:&str,progress_bar:ProgressBar)->Result<(), Box<dyn std::error::Error>>{
        let file_name = std::path::Path::new(file_path).file_name().unwrap().to_str().unwrap();
        let file_name = padding(file_name.to_string());
        //chech the file exists or not
        let  file = tokio::fs::File::open(file_path).await?;
        //then sending file_name
        stream.write_all(file_name.as_bytes()).await?;
        //sending sender_name
        stream.write_all(padding(sender_name.to_string()).as_bytes()).await?;

        //sending file length
        // stream.write_all(padding(file.metadata().await?.len().to_string()).as_bytes()).await?;
        let file_len = file.metadata().await?.len();
        // stream.write_all(file_len.to_le_bytes().as_ref()).await?;
        stream.write_u64(file_len).await.unwrap();

        progress_bar.set_length(file_len);

        //then sending data
        let mut file_reader = tokio::io::BufReader::new(file);
        let bytes_transferred = copy(&mut file_reader,  &mut progress_bar.wrap_async_write(stream)).await?;
        progress_bar.finish_with_message("done");
        // println!("Transferred {} bytes.", bytes_transferred);
        Ok(())
    }

    pub async fn send(&mut self)->Result<(),Box<dyn std::error::Error>>{
        let mut handles = vec![];
        let mut i=1;
        let multi_progress = MultiProgress::new();
        let style = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-");
        while self.files.len()!=0{
            let mut stream = self.connect_nth_stream(i as i32).await?;
            let file_path = self.files.pop().unwrap().to_string();
            let sender_name = self.name.to_string();
            let progress_bar = multi_progress.add(ProgressBar::new(0));
            progress_bar.set_style(style.clone());
            let handle =tokio::spawn(async move{
                Self::handle_transfer(file_path.as_str(),&mut stream,sender_name.as_str(),progress_bar).await.unwrap();
            });
            handles.push(handle);
            i+=1;
        }

        multi_progress.clear().unwrap();

        for handle in handles{
            handle.await?;
        }
        Ok(())
    }

}







pub struct Receiver<'a>{
    name:String,
    my_ip:&'a str,
    my_port:&'a str,
    sender_streams_addr:Vec<String>,
    files:Vec<String>,
}


impl<'a> Receiver<'a>{

    pub fn new()->Receiver<'a>{
        let  name = hostname::get().unwrap();
        let name = name.to_str().unwrap().to_string();
        Receiver{
            name,
            my_ip:"0.0.0.0",
            my_port:"8080",
            sender_streams_addr:vec![],
            files:vec![],
        }
    }

    
    pub async fn listen_on(&mut self,port:&'a str)->Result<(),Box<dyn std::error::Error>>{
        self.my_port=port;
        let listener = TcpListener::bind(self.my_ip.to_owned()+":"+self.my_port).await?;
        println!("Listening on port {}",port);
       
        let mut handles = vec![];
        let mut i=0;

        let multi_progress = MultiProgress::new();
        let style = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-");
        while i<5{
            let (mut stream, _) = listener.accept().await?;
            let progress_bar = multi_progress.add(ProgressBar::new(0));
            progress_bar.set_style(style.clone());
            // println!("connection accepted from sender {}",stream.peer_addr()?);
            self.sender_streams_addr.push(stream.peer_addr()?.to_string());
            let handle =tokio::spawn(async move{
                Self::receive(&mut stream,progress_bar).await.unwrap()
            });
            handles.push(handle);
            i+=1;
        }
        multi_progress.clear().unwrap();
        // println!("waiting for all handles to join");
        for handle in handles{
            let file_name = handle.await?;
            self.files.push(file_name);
        }
        Ok(())
    }
     async fn receive(stream:& mut TcpStream,progress_bar:ProgressBar)->Result<String, Box<dyn std::error::Error>>{
        
        let mut file_name = [0u8; 255];
        stream.read_exact(&mut file_name).await?;
        let file_name = remove_padding(String::from_utf8(file_name.to_vec())?);

        let mut sender_name = [0u8;255];
        stream.read_exact(&mut sender_name).await?;
        let sender_name = remove_padding(String::from_utf8(sender_name.to_vec())?);

        let file_len = stream.read_u64().await.unwrap();

        println!("receiving {} from {}",file_name,sender_name);

        progress_bar.set_length(file_len);

        let download_path = home::home_dir().unwrap().join("Downloads").join(file_name.as_str());
        let dest_file = create_or_incnum(download_path).await?;
        
        copy( stream, &mut progress_bar.wrap_async_write(dest_file)).await?;

        progress_bar.finish_with_message("done");
        // println!("Received {} bytes from {} .", bytes_transferred,sender_name);
        Ok(file_name)
    }

}
