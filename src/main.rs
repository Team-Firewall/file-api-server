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

#[post("/", data = "<paste>")] //파일 받기
async fn upload(mut paste:Data<'_>) -> io::Result<String> {
    let time:String = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string();
    
    // println!("{}", from_utf8(paste.peek(1024 * 1024).await).unwrap());
    
    let data:&str = from_utf8(paste.peek(1024 * 1024).await).unwrap();
    function::file_format(data,time.clone()).expect("error on file_format in function.rs");

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

#[post("/<channel>/<file_url>")] //파일로 db에 값 넣기
async fn file_import(channel:String,file_url:String) -> io::Result<String>{
    if channel == "stu_data"{ 
        //todo!("make stu_data");
        function::insert_stu_data(file_url).expect("임포트 실패");
        Ok("200!".to_string())
    } else{
        //todo!("make reward_data");
        Ok("not exist!".to_string())
    }
}

#[post("/")]
async fn db_export(){ //db 조회하고 csv화 해서 리턴

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
    POST{
        / (with file)
            쿼리는 reward_data, stu_data 로 받습니다.
            return file_url
        
        /import/<channel>/<file_url>
            업로드한 파일을 channel : (stu_data, reward_data)
            file_url 으로 인식하여 적용시킵니다

        /export
            지금 db의 값들을 csv로 넘깁니다.
            return file
    }

    GET{
        /<file_url>
            응답받은 file_url 기반으로 파일의 내용이 리턴됩니다.
    }

    DELETE{
        /<file_url>
            file_url 기반으로 파일 삭제
    } 
    
    "
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, upload, delete, retrieve]) //csv
        .mount("/import", routes![file_import])
        .mount("/export", routes![db_export])
        .mount("/mysql",routes![mysql_json]) //json
}