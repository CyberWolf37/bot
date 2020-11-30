#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_json;

pub mod utils;
pub mod api;

use utils::{Block, Conf, BotUser};
use rocket_contrib::json::{Json};
use rocket::config::{Config, Environment};
use rocket::State;
use log::{info, warn};
use rocket::request::Form;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone)]
pub struct BotMessenger {
    conf: Conf,
    blocks: Vec<Block>,
    counter: usize,
}

impl Drop for BotMessenger {
    fn drop(&mut self) {
        println!("> Dropping BotMessenger");
    }
}

impl BotMessenger {

    // New BotMessenger struct
    pub fn new() -> Self {
        BotMessenger{
            conf: Conf::default(),
            blocks: Vec::new(),
            counter: 0,
        }
    }

    // Add conf struct
    pub fn block(mut self, value: Block) -> Self {
        let mut block = value;
        block.set_token(self.get_conf().get_token_fb_page());
        self.add_block(block);
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
                }}) 
                {
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

    pub fn with_conf(mut self, conf: Conf) -> Self {
        self.conf = conf;
        self
    }

    pub fn with_token_fb(mut self, token: &str) -> Self {
        self.conf.set_token_fb_page(token);
        self.blocks.iter_mut().for_each(|x| x.set_token(token));
        self
    }

    pub fn with_token_wh(mut self, token: &str) -> Self {
        self.conf.set_token_webhook(token);
        self
    }

    pub fn rooting_user(&self, user: &BotUser) {

    }

    pub fn count(&mut self) {
        self.counter = self.counter +1;
    }

    pub fn get_count(&self) -> &usize {
        &self.counter
    }

    // Launch server rocket
    pub fn launch(&self) {

        //let bot = self.clone();

        let config = Config::build(Environment::Development)
            .address(self.get_conf().get_ip())
            .port(*self.get_conf().get_port())
            .workers(*self.get_conf().get_workers())
            .finalize();

        let selfy = Arc::new(Mutex::new(self.clone()));
        //println!("Token {}",selfy.get_conf().get_token_fb_page());

        match config {
            Ok(e) => {
                let route = format!("/{}",self.get_conf().get_uri());
                rocket::custom(e).manage(selfy).mount(&route,routes![root_connection, root_message])
                    .mount("/", routes![get_basic]).launch();
            }
            Err(e) => panic!("Failed init config : {}", e)
        } 
    }

    pub fn get_conf(&self) -> &Conf {
        &self.conf
    }

    pub fn get_conf_mut(&mut self) -> &mut Conf {
        &mut self.conf
    }

    pub fn add_block(&mut self, block: Block){
        self.blocks.push(block);
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
fn root_connection(bot: State<Arc<Mutex<BotMessenger>>>, hub: Form<FbForm>) -> String {
    let bot = bot.clone();
    let bot = bot.lock().unwrap();
    if hub.mode == "subscribe" && hub.verify_token == bot.get_conf().get_token_webhook() {
        let s = hub.challenge.clone();
        s
    }
    else {
        "Sorry i don't understand".to_string()
    }
}

#[post("/" ,format = "json", data = "<user>")]
fn root_message(bot: State<Arc<Mutex<BotMessenger>>> ,user: Json<BotUser>) -> &'static str {
    let bot = bot.clone();
    let bot = &mut bot.lock().unwrap();
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

    use utils::{Block,CartBox};

    #[test]
    fn it_works() { 
        BotMessenger::new()
            .block(Block::new("Hello")
                .cartBox(CartBox::new()
                    .text("Hello new user"))
                .cartBox(CartBox::new()
                    .text("It's a new day")))
            .block(Block::new("#Start")
                .cartBox(CartBox::new()
                    .text("New start user")))
            .with_token_fb("EAAKAw0ggVncBAIux8WOG4JnbbWCHJvFOeKK5yMZC3TwZAPaypjicgXH69plFsp28r0KyEwlWGFntOEEM2sNatIQFZCtuY3zSl98V6VRmvQBwwGXVZBfNq8gECNweZBR7oSwqdtTtbGiOaVRo05PzUYiHoMKPSuz6IE8EGOovzvAZDZD")
            .with_token_wh("MamaGuriba")
            .launch();
    }  
}
