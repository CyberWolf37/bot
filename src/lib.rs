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
use rocket::State;
use log::{info, warn, trace};

#[derive(Clone)]
pub struct BotMessenger {
    conf: Conf,
    blocks: Vec<Block>,
    //connections: Vec<BotUser>,
}

impl BotMessenger {

    // New BotMessenger struct
    pub fn new() -> Self {
        BotMessenger{
            conf: Conf::default(),
            blocks: Vec::new(),
            //connections: Vec::new(),
        }
    }

    // Add conf struct
    pub fn add_block(&mut self, value: Block) -> &mut Self {
        self.blocks.push(value);
        self
    }

    // Add user connection
    pub fn add_user(&mut self, user: BotUser) -> &mut Self {
        let block_match = self.blocks.iter_mut().find(|x| {
            x.get_name() == user.get_message().message()
        });
        
        if let Some(i) = block_match {
            info!("Message match with block");
            i.root(&user);
        }
        else {
            match self.blocks.iter_mut().enumerate().find(|x| {
                match x.1.find(&user) {
                    Some(_) => true,
                    None => false,
                }}) {
                Some(u) => {
                    info!("Find a user match in block");
                    u.1.root(&user);
                },
                None => {
                    warn!("Don't match with any of blocks");
                }
            } 
        }
        
        
        self
    }

    pub fn with_conf(&mut self, conf: Conf) -> &mut Self {
        self.conf = conf;
        self
    }

    pub fn rooting_user(&self, user: &BotUser) {

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
    /*pub fn get_connections(&self) -> &[BotUser] {
        &self.connections
    }*/

    pub fn get_conf(&self) -> &Conf {
        &self.conf
    }
}

// routes
#[get("/")]
fn root_connection() -> &'static str {
    "Hello World"
}

#[post("/" ,format = "json", data = "<user>")]
fn root_message(bot: State<BotMessenger> ,user: Json<BotUser>) -> &'static str {
    let mut bot: BotMessenger = bot.clone();
    info!("New user: {}",*user);
    bot.add_user(user.clone());
    "Hello World"
}

#[cfg(test)]
mod tests {

    use crate::BotMessenger;
    use crate::utils;

    use utils::{Block,CartBox};
    use std::sync::Arc;
    use log::*;
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
