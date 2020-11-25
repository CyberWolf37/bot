#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_json;

mod utils;
mod api;

use utils::{Block, Conf, BotUser, PipeBox, Messaging, CartBox};
use api::*;
use rocket_codegen;
use rocket_contrib::json::{Json, JsonValue};
use rocket::config::{Config, Environment};
use rocket::http::RawStr;

#[derive(Clone)]
pub struct BotMessenger {
    conf: Conf,
    blocks: Vec<Block>,
    connections: Vec<BotUser>,
}

impl BotMessenger {

    // New BotMessenger struct
    pub fn new() -> Self {
        BotMessenger{
            conf: Conf::default(),
            blocks: Vec::new(),
            connections: Vec::new(),
        }
    }

    // Add conf struct
    pub fn add_block(&mut self, value: Block) -> &mut Self {
        self.blocks.push(value);
        self
    }

    // Add user connection
    pub fn add_user(&mut self, user: BotUser) -> &mut Self {
        match self.connections.iter().enumerate().find(|x| x.1.get_sender() == user.get_sender()) {
            Some(u) => { 
                self.connections.remove(u.0);
            },
            None => {
                self.connections.push(user);
            }
        }
        
        self
    }

    pub fn with_conf(&mut self, conf: Conf) -> &mut Self {
        self.conf = conf;
        self
    }

    // Launch server rocket
    pub fn launch(&self) {

        let bot = self.clone();

        let config = Config::build(Environment::Development)
            .address(self.get_conf().get_ip())
            .port(*self.get_conf().get_port())
            .workers(*self.get_conf().get_workers())
            .finalize();

        match config {
            Ok(e) => {
                let route = format!("/{}",self.get_conf().get_uri());
                rocket::custom(e).manage(bot).mount(&route,routes![root_connection, root_message]).launch();
            }
            Err(e) => panic!("Failed init config : {}", e)
        }

        
    }

    // Getter Setter
    pub fn get_connections(&self) -> &[BotUser] {
        &self.connections
    }

    pub fn get_conf(&self) -> &Conf {
        &self.conf
    }
}

// routes
#[get("/")]
fn root_connection() -> &'static str {
    "Hello World"
}

#[post("/" ,format = "json", data = "<message>")]
fn root_message(message: Json<BotUser>) -> &'static str {
    println!("{}",*message);
    "Hello World"
}

#[cfg(test)]
mod tests {

    use crate::BotMessenger;
    use crate::utils;

    use utils::{Block,CartBox};
    use std::sync::Arc;
    #[test]
    fn it_works() {
        let mut bot = BotMessenger::new();
        let mut block = Block::new("Hello");
        block.add(Arc::new(CartBox::new("Hello")));
        bot.add_block(block);
        println!("{}",bot.get_conf());
        bot.launch();
    }
}
