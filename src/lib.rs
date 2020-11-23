#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] use rocket;

mod utils;
mod api;

use utils::{Block, Conf, BotUser, PipeBox, Messaging, CartBox};
use api::*;
use rocket_codegen;

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
        let route = format!("/{}",self.get_conf().get_uri());
        rocket::ignite().mount(&route,routes![self.rootConnectio()]).launch();
    }

    // Getter Setter
    pub fn get_connections(&self) -> &[BotUser] {
        &self.connections
    }

    pub fn get_conf(&self) -> &Conf {
        &self.conf
    }

    // route
    #[get("/webhook")]
    fn rootConnectio(&self) {

    }


}

#[cfg(test)]
mod tests {

    use crate::BotMessenger;
    use crate::utils;

    use utils::{Block,CartBox};
    #[test]
    fn it_works() {
        let mut bot = BotMessenger::new();
        let mut block = Block::new("Hello");
        block.add(Box::new(CartBox::new("Hello")));
        bot.add_block(block);
        println!("{}",bot.get_conf());
    }
}
