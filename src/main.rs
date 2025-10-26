use email_newsletter::run;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    run().await
}
