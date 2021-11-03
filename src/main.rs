use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // the `?` operator bubbles up the error so it does not need explicit handling
    run()?.await
}
