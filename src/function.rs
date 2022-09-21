use std::fs::File;
use std::io::{Write, Error, BufRead, self};
use std::path::Path;

struct UserData {
    grade:String,
    class:String,
    number:String,
    name:String,
    phone:String
}

fn exist_check(file_url:String) -> bool{
    if Path::new(&format!("./upload/{}",file_url)).exists() {
        return true;
    }
    false
}

fn read_data_to_csv(file_url:String) -> Result<Vec<UserData>,&'static str>{
    if let Ok(lines) 
    = read_lines(&format!("./upload/{}",file_url)) {
        let mut is_first = true;
        let mut data:Vec<UserData> = Vec::new();
        for line in lines {
            if let Ok(ip) = line { //라인별로 읽어왔음
                if !is_first{
                    let one_data:Vec<&str> = ip.split(",").collect(); // 각자 나누고
                    data.push(UserData{
                        grade: one_data[0].to_string(),
                        class: one_data[1].to_string(),
                        number: one_data[2].to_string(),
                        name: one_data[3].to_string(),
                        phone: one_data[4].to_string(),
                    })
                }
                is_first=false;
            }
        }
        return Ok(data);
    }
    Err("read_data_to_csv function Error!")
}
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

pub(crate) fn insert_stu_data(file_url:String) -> Result<(),&'static str>{
    if !exist_check(file_url.clone()) {
        return Err("파일없으 ㅋㅋ");
    }
    let data:Vec<UserData> =read_data_to_csv(file_url).expect("csv 파일 읽다가 뭔가 잘못됨");
    //이제 vector 에 데이터를 db에 넣어야함
    
    Ok(())
}

fn insert_reward_data(){
    
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}