use std::env;

use lazy_static::lazy_static;


lazy_static!{
    pub static ref MNEMONIC:String = set_mnemonic();
    pub static ref ADDRESS:String = set_address();
    pub static ref PORT:u16 = set_port();
}

pub fn set_mnemonic() -> String{
    dotenv::dotenv().ok();
    env::var("MNEMONIC").unwrap()
}

pub fn set_port() -> u16{
    dotenv::dotenv().ok();
    env::var("PORT").unwrap().parse::<u16>().unwrap()
}

pub fn set_address() -> String{
    dotenv::dotenv().ok();
    env::var("ADDRESS").unwrap()
}

