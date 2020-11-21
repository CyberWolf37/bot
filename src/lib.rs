#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] use rocket;

mod utils;

use utils::{Block, Conf, BotUser, PipeBox, Messaging};

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
        let pair =  self.get_connections().iter().enumerate().find(|x| x.1.get_sender() == user.get_sender());
        match pair {
            Some(u) => {
                self.connections.remove(u.0);
            }
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
        rocket::ignite();
    }

    // Getter Setter
    fn get_connections(&self) -> &[BotUser] {
        &self.connections
    }


}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
