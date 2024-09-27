mod bip;
// use crate::bip::mnemonic::BipMenomic;
use actix_web::{get,post, web, App, HttpResponse, HttpServer, Responder,middleware, http};
mod controller;
use crate::controller::controller::{generate_keypair,create_wallet,get_balance,};

mod models;
mod account;
use actix_cors::Cors;
mod config;
use config::Config;


#[actix_web::main]
async fn main() -> std::io::Result<()> {    
    println!("Starting server at http://localhost:8000");
    env_logger::init();
    let config = Config::from_env();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .wrap(
                Cors::default()
                .allowed_origin("http://localhost:3000")
                .allowed_origin("http://127.0.0.1:3000")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![
                    http::header::CONTENT_TYPE,
                    http::header::AUTHORIZATION,
                    http::header::ACCEPT,
                ])
        .max_age(3600),
            )
            .wrap(middleware::Logger::default())
            .service(generate_keypair)
            .service(create_wallet)
            .service(get_balance)
            .service(controller::transfer::send_sol)
            .service(account::private_key::get_private_key)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await


}


