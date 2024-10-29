use actix_web::web;
use decentralized_wallet_rust::{account, controller::{self, controller::{create_wallet, generate_keypair, get_balance}}};


pub fn config(config: &mut web::ServiceConfig){
    config
        .service(web::scope("/api")
        .service(generate_keypair)
        .service(create_wallet)
        .service(get_balance)
        .service(controller::transfer::send_sol)
        .service(account::private_key::get_private_key)
    );
}