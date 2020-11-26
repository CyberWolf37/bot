#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_json;

pub mod utils;
pub mod api;

use utils::{Block, Conf, BotUser, PipeBox, Messaging, CartBox};
use api::*;
use rocket_codegen;
use rocket_contrib::json::{Json, JsonValue};
use rocket::config::{Config, Environment};
use rocket::http::RawStr;
use rocket::State;
use log::{info, warn, trace};
use rocket::outcome::Outcome::*;
use rocket::request::{self, Request, FromRequest};
use rocket::request::Form;

#[derive(Clone)]
pub struct BotMessenger {
    conf: Conf,
    blocks: Vec<Block>,
}

impl BotMessenger {

    // New BotMessenger struct
    pub fn new() -> Self {
        BotMessenger{
            conf: Conf::default(),
            blocks: Vec::new(),
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
                rocket::custom(e).manage(bot).mount(&route,routes![root_connection, root_message])
                    .mount("/", routes![get_basic]).launch();
            }
            Err(e) => panic!("Failed init config : {}", e)
        }

        
    }

    pub fn get_conf(& self) -> &Conf {
        &self.conf
    }

    pub fn get_conf_mut(&mut self) -> &mut Conf {
        &mut self.conf
    }
}

#[derive(FromForm)]
struct FbForm {
    #[form(field = "hub.verify_token")]
    verify_token: String,

    #[form(field = "hub.challenge")]
    challenge: String,

    #[form(field = "hub.mode")]
    mode: String,
}

// routes
#[get("/?<hub..>")]
fn root_connection(bot: State<BotMessenger>, hub: Form<FbForm>) -> String {
    if hub.mode == "subscribe" && hub.verify_token == bot.get_conf().get_token_webhook() {
        let s = hub.challenge.clone();
        s
    }
    else {
        "Sorry i don't understand".to_string()
    }
}

#[post("/" ,format = "json", data = "<user>")]
fn root_message(bot: State<BotMessenger> ,user: Json<BotUser>) -> &'static str {
    let mut bot: BotMessenger = bot.clone();
    info!("New user: {}",*user);
    bot.add_user(user.clone());
    "ok"
}

#[get("/")]
fn get_basic() -> &'static str {
    "ok"
}

#[cfg(test)]
mod tests {

    use crate::BotMessenger;
    use crate::utils;
    use crate::api;

    use utils::{Block,CartBox,BotUser};
    use api::{Message,ApiMessage};
    use std::sync::Arc;
    use log::*;

    #[test]
    fn it_works() {
        let message = Message::new("Mamaguriba");
        let mut bot = BotMessenger::new();
        let mut block = Block::new("Hello");
        block.add(Arc::new(CartBox::new(Arc::new(move |x: &BotUser| {
           message.send(x,"Hello mother fucker");
        }))));
        bot.add_block(block);
        println!("{}",bot.get_conf());
        bot.launch();
    }
}
