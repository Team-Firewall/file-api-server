#[macro_use] extern crate rocket;
use serde_json::{json};
use std::io;
use std::str::from_utf8;
use rocket::data::Data;
use rocket::http::uri::Absolute;
use rocket::response::content::RawJson;
use rocket::tokio::fs::{self, File};
use std::time::{SystemTime, UNIX_EPOCH};

mod db_connect;
mod function;

const HOST: Absolute<'static> = uri!("http://localhost:8888"); //your url!

#[post("/<channel>", data = "<paste>")] //파일 받기
async fn upload(channel:String,mut paste:Data<'_>) -> io::Result<String> {
    let time:String = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string();
    
    // println!("{}", from_utf8(paste.peek(1024 * 1024).await).unwrap());
    
    let data:&str = from_utf8(paste.peek(1024 * 1024).await).unwrap();
    function::file_format(data,time.clone()).expect("error on file_format in function.rs");

    if channel == "stu_data"{ 
        //todo!("make stu_data");
    } else{
        //todo!("make reward_data");
    }
    Ok(format!("{}/{}",HOST,time))
}

#[get("/<file_url>")] //파일 주기
async fn retrieve(file_url:String) -> Option<File> {
    File::open(format!("./upload/{}",file_url)).await.ok()
}

#[delete("/<file_url>")] //파일 삭제
async fn delete(file_url:String) -> Option<()> {
    fs::remove_file(format!("./upload/{}",file_url)).await.ok()
}

#[get("/")]
fn mysql_json() -> RawJson<String> {
    let db_url:String = db_connect::connect();
    let pool = mysql::Pool::new(db_url).expect("연결실패");
    let query = "select * from name";
    let result = pool.prep_exec(query,()).expect("쿼리 오류");

    let mut vec = Vec::new();
    for row in result {
        let (id,name):(i32, String) = mysql::from_row(row.unwrap());
        let export = json!({
            "id":id,
            "name":name
        });
        vec.push(export)
    }
    RawJson(json!(vec).to_string())
}

#[get("/")]
fn index() -> &'static str {
    "
    USAGE
    POST /<index>
        쿼리는 reward_data, stu_data 로 받습니다.
        POST 메소드로 파일을 보내셈.
        return file_url

    GET /<file_url>
        응답받은 file_url 기반으로 응답이 옵니다. (응답만 옵니다)

    DELETE /<file_url>
        file_url 기반으로 파일 삭제
    
    "
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, upload, delete, retrieve]) //csv
        .mount("/mysql",routes![mysql_json]) //json
}