use crate::BotMessenger;
use std::collections::HashMap;

pub enum MessagingType<'a> {
    POSTBACK(&'a MessagingPostback),
    MESSAGE(&'a MessagingMessage),
}

pub trait Adder {
    fn add(self,value: &mut BotMessenger) -> &mut BotMessenger;
}

pub trait Messaging {
    fn message_type(&self) -> MessagingType;
    fn message(&self) -> &str;
    fn sender(&self) -> &BotUser;
}

pub trait PipeBox {
    fn controle(&self,message: &dyn Messaging) -> Result<bool>;
    fn core(&self);
    fn send(&self) -> Result<&BotUser,Err>;
}

impl Adder for Block {
    fn add(self,value: &mut BotMessenger) -> &mut BotMessenger {
        value.blocks.push(self);
        value
    }
}

impl Adder for Conf {
    fn add(self,value: &mut BotMessenger) -> &mut BotMessenger {
        value.conf = self;
        value
    }
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

pub struct Block {
    childs: HashMap<&'static BotUser,&'static dyn PipeBox>,
    pipe: Vec<Box<dyn PipeBox>>,
}

impl Default for Block {
    fn default() -> Self {
        Block{
            childs: HashMap::new(),
            pipe: Vec::new(),
        }
    }
}

impl Block {
    pub fn new() -> Self {
        Block::default()
    }

    pub fn attach(user: BotUser) {
        
    }
}

pub struct MessagingPostback {
    payload: String,
    botUser: &'static BotUser,
}

impl Messaging for MessagingPostback {
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

impl Messaging for MessagingMessage {
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
