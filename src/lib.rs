#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_json;

pub mod utils;
pub mod api;

use utils::block::Block;
use utils::{Conf, BotUser};
use rocket_contrib::json::{Json};
use rocket_contrib::serve::{StaticFiles, Options};
use rocket::config::{Config, Environment};
use rocket::State;
use log::{info, warn};
use rocket::request::Form;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct BotMessenger {
    conf: Conf,
    blocks: Vec<Block>,
    block_default: Block,
    static_file: Option<String>,
}

impl Drop for BotMessenger {
    fn drop(&mut self) {
        println!("> Dropping BotMessenger");
    }
}

impl BotMessenger {

    // New BotMessenger struct
    pub fn new() -> Self {
        BotMessenger {
            conf: Conf::default(),
            blocks: Vec::new(),
            block_default: Block::default(),
            static_file: None,
        }
    }

    // Add conf struct
    pub fn block(mut self, value: Block) -> Self {
        let mut block = value;
        block.set_token(self.get_conf().get_token_fb_page());
        self.add_block(block);
        self
    }

    pub fn block_default(mut self, value: Block) -> Self {
        let mut block = value;
        block.set_token(self.get_conf().get_token_fb_page());
        self.block_default = block;
        self
    }

    // Add user connection
    pub fn add_user(&mut self, user: BotUser) -> &mut Self {
        let block_match = self.blocks.iter_mut().find(|x| {
            x.get_name() == user.get_message().message()
        });
        
        if let Some(i) = block_match {
            info!("Message match with block");
            i.remove_child(&user);
            i.root(&user);
        }
        else {
            match self.blocks.iter_mut().enumerate().find(|x| {
                x.1.find(&user)}) 
                {
                    Some(u) => {
                        info!("Find a user match in block");
                        u.1.root(&user);
                },
                    None => {
                        warn!("Don't match with any of blocks");
                        self.block_default.root(&user);
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
        self.block_default.set_token(token);
        self
    }
    
    pub fn with_token_wh(mut self, token: &str) -> Self {
        self.conf.set_token_webhook(token);
        self
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.conf.set_port(port);
        self
    }

    pub fn with_static_file(mut self, file: String) -> Self{
        self.static_file = Some(file);
        self
    }

    pub fn rooting_user(&self, user: &BotUser) {

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
                let mut rocket = rocket::custom(e).manage(selfy).mount(&route,routes![root_connection, root_message])
                    .mount("/", routes![get_basic]);

                if self.static_file.is_some() {
                    let option = Options::None;
                    let s = self.static_file.clone().unwrap();
                    rocket = rocket.mount("/static", StaticFiles::new(s,option).rank(3));
                }
                
                rocket.launch();
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
        "Sorry I don't understand".to_string()
    }
}

#[post("/" ,format = "json", data = "<user>")]
fn root_message(bot: State<Arc<Mutex<BotMessenger>>> ,user: Json<BotUser>) -> &'static str {
    let bot = bot.clone();
    let bot = &mut bot.lock();

    if let Ok(b) = bot {
       info!("New user: {}",*user);
        b.add_user(user.clone());
        "ok" 
    }
    else {
        "Don't understand ?"
    }
    
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


    use utils::block::Block;
    use utils::block::CartBox;
    use api::card::Card;
    use api::card::DefaultAction;
    use api::card::CardGeneric;
    use api::card::CardButtons;
    use api::button::Button;

    #[test]
    fn it_works() { 
        BotMessenger::new()
            .block_default(Block::new("default")
                .cartBox(CartBox::new()
                    .text("Sorry I don't understand 🐻")))
            .block(Block::new("Hello")
                .cartBox(CartBox::new()
                    .text("Hello new user"))
                .cartBox(CartBox::new()
                    .text("Hello new user 2"))
                .cartBox(CartBox::new()
                    .text("Hello new user 3"))
                .cartBox(CartBox::new()  
                    .text("It's a new day")
                    .button_postback("Push", "Hello"))
                .cartBox(CartBox::new()
                    .card(CardGeneric::new("Hello")
                        .button(Button::new_button_pb("Welcom back Mr potter", "Hello"))
                        .image("https://images.ladepeche.fr/api/v1/images/view/5c34fb833e454650457f60ce/large/image.jpg")
                        .subtitle("Bouyah"))
                    .card(CardGeneric::new("Hello")
                        .button(Button::new_button_pb("Welcom back Mr potter", "Hello"))
                        .image("https://images.ladepeche.fr/api/v1/images/view/5c34fb833e454650457f60ce/large/image.jpg")
                        .subtitle("Bouyah")))
                .cartBox(CartBox::new()
                    .card(CardButtons::new("Can you choose !")
                        .button(Button::new_button_url("wake me up", "www.google.fr"))
                        .button(Button::new_button_pb("not me !", "Hello")))))
            .block(Block::new("#Start")
                .cartBox(CartBox::new()
                    .text("New start user")))
            .with_token_fb(&std::env::var("TOKEN_FB").unwrap())
            .with_token_wh("MamaGuriba")
            .launch();
    }
}
