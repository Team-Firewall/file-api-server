use serde_json::{json};
#[macro_use]
extern crate rocket;
mod db_connect;

#[get("/")]
fn index() -> String {  
    format!("hello! csv-api!\ntodo: upload file, download file, converter csv")
}
#[get("/")]
fn mysql_connect() -> String {
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
    json!(vec).to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/",routes![index]) //hello
        .mount("/mysql",routes![mysql_connect]) //json test
}