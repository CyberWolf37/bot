use std::collections::HashMap;
use std::collections::hash_map::Iter;

pub enum MessagingType<'a> {
    POSTBACK(&'a MessagingPostback),
    MESSAGE(&'a MessagingMessage),
}

pub enum PipeStatus {
    NEXT,
    REPLAY,
    RESTART,
}

pub trait Messaging {
    fn message_type(&self) -> MessagingType;
    fn message(&self) -> &str;
    fn sender(&self) -> &BotUser;
}

pub trait PipeBox {
    fn consume(&self,message: &BotUser) -> PipeStatus;
}

pub struct Conf {
    port: String,
    ip: String,
    uri: String,
    workers: usize,
    token_webhook: String,
    token_fb_page: String,
}

impl Conf {
    // New Conf struct
    fn new(port: &str ,ip: &str ,uri: &str ,size: usize ,token_webhook: &str ,token_fb_page: &str) -> Self {
        Conf{
            port: String::from(port),
            ip: String::from(ip),
            uri: String::from(uri),
            workers: size,
            token_webhook: String::from(token_webhook),
            token_fb_page: String::from(token_fb_page),
        }
    }

    // Set Tokens
    fn set_token_webhook(&mut self, token: &str) -> &mut Self {
        self.token_webhook = String::from(token);
        self
    }

    fn set_token_fb_page(&mut self, token: &str) -> &mut Self {
        self.token_fb_page = String::from(token);
        self
    }
}

impl Default for Conf {
    fn default() -> Self {
        Conf{
            port: String::from("7878"),
            ip: String::from("0.0.0.0"),
            uri: String::from("/webhook"),
            workers: 12,
            token_webhook: String::from("MamaGuriba"),
            token_fb_page: String::from("MamaGuriba"),
        }
    }
}

pub struct BotUser {
    sender_id: String,
    message: Box<dyn Messaging>,
}

impl PartialEq for BotUser {
    fn eq(&self, other: &Self) -> bool {
        self.sender_id == other.sender_id
    }
}

impl BotUser {
    pub fn new(id: &str ,message: Box<dyn Messaging>) -> Self {
        BotUser{
            sender_id: String::from(id),
            message: message,
        }
    }

    pub fn get_sender(&self) -> &str {
        &self.sender_id
    }

    pub fn get_message(&self) -> &Box<dyn Messaging> {
        &self.message
    }
}

pub struct Block{
    name: String,
    childs: HashMap<&'static BotUser,usize>,
    pipe: Vec<Box<dyn PipeBox>>,
}

impl Default for Block {
    fn default() -> Self {
        Block{
            name: String::from("Hello"),
            childs: HashMap::new(),
            pipe: Vec::new(),
        }
    }
}

impl Block {

    // Init Block
    pub fn new(name: &str) -> Self {
        let mut block = Block::default();
        block.set_name(name);
        block
    }

    // Rooting user
    pub fn root(&mut self ,user: &BotUser) {
        self.consume(user);
    }

    // Consume the PipeBox for the user
    fn consume(&mut self ,user: &BotUser) {
        // find current user and is index
        let pair = self.childs.iter_mut().find(|x| {*x.0 == user});

        match pair {
            Some((_,value)) => {
                match self.pipe.get(*value) {
                    Some(pipe_box) => {
                        match pipe_box.consume(user) {
                            PipeStatus::NEXT => {
                                if *value < self.pipe.len() {
                                    *value = *value + 1;
                                }
                                else if *value == self.pipe.len(){
                                    *value = 0;
                                }
                            }
                            PipeStatus::REPLAY => {
                                // Nothing to do
                            }
                            PipeStatus::RESTART => {
                                *value = 0;
                            }
                        };
                    }
                    None => ()
                }
            }
            None => {}
        }
    }

    // Setter
    fn set_name(&mut self, name: &str) -> &mut Self{
        self.name = String::from(name);
        self
    }

    fn add(&mut self, pipeBox: Box<dyn PipeBox>) -> &mut Self {
        self.pipe.push(pipeBox);
        self
    }

    fn iter(&self) -> Iter<'_,&BotUser,usize> {
        self.childs.iter()
    }
}

pub struct MessagingPostback {
    payload: String,
    botUser: &'static BotUser,
}

impl<'a> Messaging for MessagingPostback {
    fn message_type(&self) -> MessagingType {
        MessagingType::POSTBACK(&self)
    }
    fn message(&self) -> &str {
        &self.payload
    }
    fn sender(&self) -> &BotUser {
        &self.botUser
    }
}

pub struct MessagingMessage {
    text: String,
    botUser: &'static BotUser,
}

impl<'a> Messaging for MessagingMessage {
    fn message_type(&self) -> MessagingType {
        MessagingType::MESSAGE(&self)
    }
    fn message(&self) -> &str {
        &self.text
    }
    fn sender(&self) -> &BotUser {
        &self.botUser
    }
}
