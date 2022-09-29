pub(crate)
fn connect() -> String{
    let db_user = "root";
    let db_password = "0000";
    let db_address = "localhost";
    let db_port = "3306";
    let db_name = "test";
    format!("mysql://{}:{}@{}:{}/{}",db_user,db_password,db_address,db_port,db_name)
}