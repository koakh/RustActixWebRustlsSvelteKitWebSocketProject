use actix_web::get;

#[get("/api/test")]
pub async fn handle_test() -> actix_web::Result<String> {
    let text = "hello world";
    println!("{}", text);
    Ok(text.to_string())
}
