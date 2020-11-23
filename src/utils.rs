use crate::api;

use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::fmt;
use api::{Message, ApiMessage};

pub enum MessagingType<'a> {
    POSTBACK(&'a MessagingPostback),
    MESSAGE(&'a MessagingMessage),
}

impl fmt::Display for MessagingType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessagingType::POSTBACK(_) => write!(f,"POSTBACK"),
            MessagingType::MESSAGE(_) => write!(f,"MESSAGE"),
        }
    }
}

pub enum PipeStatus {
    NEXT,
    REPLAY,
    RESTART,
}

impl fmt::Display for PipeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PipeStatus::NEXT => write!(f,"NEXT Status"),
            PipeStatus::REPLAY => write!(f,"REPLAY Satus"),
            PipeStatus::RESTART => write!(f,"NEXT Status"),
        }
    }
}

pub trait Messaging {
    fn message_type(&self) -> MessagingType;
    fn message(&self) -> &str;
    fn sender(&self) -> &BotUser;
}

impl fmt::Display for Messaging{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Message {}: {}"
            ,self.message_type(), self.message())
    }
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

impl fmt::Display for Conf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Configuration : \nPort: {}\nIp: {}\nURI: {}\nWorkers: {}\nToken webhook: {}\nToken FB: {}"
            , self.port, self.ip, self.uri, self.workers, self.token_webhook, self.token_fb_page)
    }
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

    // set Vars conf
    fn set_port(&mut self, port: &str) {
        self.port = String::from(port);
    }

    fn set_ip(&mut self, ip: &str) {
        self.ip = String::from(ip);
    }

    fn set_uri(&mut self, uri: &str) {
        self.uri = String::from(uri);
    }

    fn set_workers(&mut self, workers: usize) {
        self.workers = workers;
    }

    pub fn get_uri(&self) -> &str {
        &self.uri
    }

    pub fn get_ip(&self) -> &str {
        &self.ip
    }

    pub fn get_port(&self) -> &str {
        &self.port
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

impl fmt::Display for BotUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bot User : [ Sender id: {}\r\n ,Message: {}]"
            , self.sender_id, self.message)
    }
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

    pub fn send(message: Box<dyn Messaging>) {

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
    pub fn set_name(&mut self, name: &str) -> &mut Self{
        self.name = String::from(name);
        self
    }

    pub fn add(&mut self, pipeBox: Box<dyn PipeBox>){
        self.pipe.push(pipeBox);
    }

    pub fn iter(&self) -> Iter<'_,&BotUser,usize> {
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

pub struct CartBox {
    function_controle: Box<dyn Fn(&BotUser) -> Option<&BotUser> + Send + Sync>,
    function_core: Box<dyn Fn(&BotUser) + Send + Sync>,
}

impl PipeBox for CartBox {
    fn consume(&self,message: &BotUser) -> PipeStatus {
        match (self.function_controle)(message) {
            Some(e) => {
                (self.function_core)(e);
                PipeStatus::NEXT
            }
            None => {
                PipeStatus::REPLAY
            }
        }
    }
}

impl CartBox {
    pub fn new(text: &'static str) -> Self {
        let function_controle: Box<dyn Fn(&BotUser) -> Option<&BotUser> + Send + Sync> = Box::new(|u| {Some(u)});
        let function_core: Box<dyn Fn(&BotUser) + Send + Sync> = Box::new(move |u| {Message::new(text).send(u)});

        CartBox{
            function_controle: function_controle,
            function_core: function_core,
        }
    }

    pub fn set_func_ctrl(&mut self,func: Box<dyn Fn(&BotUser) -> Option<&BotUser> + Send + Sync>){
        self.function_controle = func;
    }

    pub fn set_func_core(&mut self, func: Box<dyn Fn(&BotUser) + Send + Sync>) {
        self.function_core = func;
    }
}
