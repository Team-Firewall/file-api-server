use std::fs::File;
use std::io::{Write, Error};

pub(crate) fn file_format(data:&str, time:String) -> Result<(),Error>{
    let path:&str = &format!("./upload/{}",time);
    
    let mut output = File::create(path)?;
    let string_data:String = data.into();
    
    // let mut vec_data:Vec<String> = Vec::new();
    let mut result_data:String = String::new();
    let mut count = 0;
    for index in string_data.lines() {
        if count>3 && count < string_data.lines().count()-2{
            // vec_data.push(index.to_string());
            result_data.push_str(&format!("{}\n",index));
        }
        count+=1;
    }
    result_data.pop();
    write!(output, "{}", result_data)?;
    Ok(())
}

fn insert_stu_data(){

}

fn insert_reward_data(){
    
}