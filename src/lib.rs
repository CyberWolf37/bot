#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] use rocket;

mod utils;

use utils::{Block, Conf, BotUser, Adder};

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
    pub fn add<T>(&mut self, value: T) -> &mut Self where T: Adder {
        value.add(self);
        self
    }

    pub fn launch(&self) {
        rocket::ignite();
    }


}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
