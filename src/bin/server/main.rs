use actix_web::{App, HttpServer};

mod env;

use env::Env;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::from_env()?;
    HttpServer::new(|| App::new())
        .bind((env.host.as_str(), env.port))?
        .run()
        .await?;
    Ok(())
}
