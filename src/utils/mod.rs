use std::{path::PathBuf, io::Error};

use tokio::fs::File;

pub async fn create_or_incnum(file_path:PathBuf)->Result<File,Error>{
    let mut x = tokio::fs::OpenOptions::new().create_new(true).write(true).open(file_path.to_owned()).await;
    let mut n =1;
    while let Err(ref err) = x{
        if err.kind() == tokio::io::ErrorKind::AlreadyExists{
            let ext = file_path.extension().unwrap_or("".as_ref());
            let mut file_name = file_path.file_stem().unwrap().to_os_string();
            file_name.push(n.to_string()+".");
            file_name.push(ext);
            let new_file_path = file_path.with_file_name(file_name);
            n+=1;
            x = tokio::fs::OpenOptions::new().create_new(true).write(true).open(new_file_path).await;
        }
    }
    x
    
}


pub fn padding(name:String)->String{
    let mut name=name;
    while name.len()<255{
        name.push(' ');
    }
    name

}

pub fn remove_padding(name:String)->String{
    let mut name=name;
    while name.len()>0 && name.chars().last().unwrap()==' '{
        name.pop();
    }
    name
}