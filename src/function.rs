use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::path::Path;

pub(crate) fn file_format(data:&str, time:String) -> Result<(),Error>{
    let path = Path::new(&format!("./upload/{}",time));

    println!("{}",data);
    
    Ok(())
}