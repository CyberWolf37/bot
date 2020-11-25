use crate::api;

use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::fmt;
use std::fmt::{Display};
use api::{Message, ApiMessage};
use serde::de::{self, Deserialize, Deserializer};
use rocket_contrib::json::{Json, JsonValue};
use serde_json::Value;
use std::sync::Arc;
use log::{info, warn, trace};

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
    //fn sender(&self) -> &BotUser;
}

impl fmt::Display for dyn Messaging{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Message {}: {}"
            ,self.message_type(), self.message())
    }
}

pub trait PipeBox {
    fn consume(&self,message: &BotUser) -> PipeStatus;
}

#[derive(Clone)]
pub struct Conf {
    port: u16,
    ip: String,
    uri: String,
    workers: u16,
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
    fn new(port: u16 ,ip: &str ,uri: &str ,size: u16 ,token_webhook: &str ,token_fb_page: &str) -> Self {
        Conf{
            port: port,
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
    fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    fn set_ip(&mut self, ip: &str) {
        self.ip = String::from(ip);
    }

    fn set_uri(&mut self, uri: &str) {
        self.uri = String::from(uri);
    }

    fn set_workers(&mut self, workers: u16) {
        self.workers = workers;
    }

    pub fn get_uri(&self) -> &str {
        &self.uri
    }

    pub fn get_ip(&self) -> &str {
        &self.ip
    }

    pub fn get_port(&self) -> &u16 {
        &self.port
    }

    pub fn get_workers(&self) -> &u16 {
        &self.workers
    }

    pub fn get_token_webhook(&self) -> &str {
        &self.token_webhook
    }

    pub fn get_token_fb_page(&self) -> &str {
        &self.token_fb_page
    }
}

impl Default for Conf {
    fn default() -> Self {
        Conf{
            port: 7878,
            ip: String::from("0.0.0.0"),
            uri: String::from("/webhook"),
            workers: 12,
            token_webhook: String::from("MamaGuriba"),
            token_fb_page: String::from("MamaGuriba"),
        }
    }
}

#[derive(Clone)]
pub struct BotUser {
    sender_id: String,
    message: Arc<dyn Messaging + Send + Sync>,
}

impl<'de> Deserialize<'de> for BotUser {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {

        let json: Value =  Value::deserialize(deserializer)?;

        let object: &String = match &json["object"] {
            Value::String(e) if e == "page" => e,
            _ => return Err(de::Error::custom("Doesn't have a valid json format API FB")),
        };

        let id = match &json["entry"][0]["messaging"][0]["sender"]["id"] {
            Value::String(e) => e,
            _ => return Err(de::Error::custom("Doesn't have a sender id in json")),
        };

        let messageP: Option<MessagingPostback> = match &json["entry"][0]["messaging"][0]["postback"]["payload"] {
            Value::String(e) => {
                Some(MessagingPostback{payload: e.clone()})
            },
            _ => None,
        };
        
        let messageM: Option<MessagingMessage> = match &json["entry"][0]["messaging"][0]["message"]["text"] {
            Value::String(e) => {
                Some(MessagingMessage{text: e.clone()})
            },
            _ => None,
        };

        if let Some(i) = messageP {
            return Ok(BotUser::new(&id, Arc::new(i)));
        }
        else {
            if let Some(i) = messageM {
                return Ok(BotUser::new(&id, Arc::new(i)));
            }
            else{
                return Err(de::Error::custom("Don't have Messaging or Postback value in json"));
            }
        }
    }
}

impl fmt::Display for BotUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bot User : [ Sender id: {} , Sender Message: {} ]"
            , self.sender_id, self.message.clone().message())
    }
}

impl PartialEq for BotUser {
    fn eq(&self, other: &Self) -> bool {
        self.sender_id == other.sender_id
    }
}

impl BotUser {
    pub fn new(id: &str ,message: Arc<dyn Messaging  + Send + Sync>) -> Self {
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

    pub fn get_message(&self) -> Arc<dyn Messaging  + Send + Sync> {
        self.message.clone()
    }
}

#[derive(Clone)]
pub struct Block{
    name: String,
    childs: Vec<(BotUser,usize)>,
    pipe: Vec<Arc<dyn PipeBox  + Send + Sync>>,
}

impl Default for Block {
    fn default() -> Self {
        Block{
            name: String::from("Hello"),
            childs: Vec::new(),
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
        self.consume(user.clone());      
    }

    // Consume the PipeBox for the user
    fn consume(&mut self ,user: BotUser) {
        // find current user and is index
        let pair = self.childs.iter_mut().find(|x| {x.0 == user});

        match pair {
            Some((_,value)) => {
                info!("Match with pipebox");
                match self.pipe.get(*value) {
                    Some(pipe_box) => {
                        match pipe_box.consume(&user) {
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
            None => { 
                info!("Don't Match with any pipebox");
                let user_cp = user.clone();
                self.childs.push((user, 0));

                self.pipe[0].consume(&user_cp);
            }
        }
    }

    // Setter
    pub fn set_name(&mut self, name: &str) -> &mut Self{
        self.name = String::from(name);
        self
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add(&mut self, pipeBox: Arc<dyn PipeBox + Send + Sync>){
        self.pipe.push(pipeBox);
    }

    pub fn find(&self,user: &BotUser) -> Option<&(BotUser,usize)> {
        self.childs.iter().find(|x| x.0.get_sender() == user.get_sender())
    }
}

#[derive(Clone)]
pub struct MessagingPostback {
    payload: String,
    //botUser: &'static BotUser,
}

impl<'a> Messaging for MessagingPostback {
    fn message_type(&self) -> MessagingType {
        MessagingType::POSTBACK(&self)
    }
    fn message(&self) -> &str {
        &self.payload
    }
    /*fn sender(&self) -> &BotUser {
        &self.botUser
    }*/
}

#[derive(Clone)]
pub struct MessagingMessage {
    text: String,
    //botUser: &'static BotUser,
}

impl<'a> Messaging for MessagingMessage {
    fn message_type(&self) -> MessagingType {
        MessagingType::MESSAGE(&self)
    }
    fn message(&self) -> &str {
        &self.text
    }
    /*fn sender(&self) -> &BotUser {
        &self.botUser
    }*/
}

#[derive(Clone)]
pub struct CartBox {
    function_controle: Arc<dyn Fn(&BotUser) -> Option<&BotUser> + Send + Sync>,
    function_core: Arc<dyn Fn(&BotUser) + Send + Sync>,
}

impl PipeBox for CartBox {
    fn consume(&self,message: &BotUser) -> PipeStatus {
        info!("Consume in the block the pipebox");
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
    pub fn new(function_core: Arc<dyn Fn(&BotUser) + Send + Sync>) -> Self {
        let function_controle: Arc<dyn Fn(&BotUser) -> Option<&BotUser> + Send + Sync> = Arc::new(|u| {Some(u)});

        CartBox{
            function_controle: function_controle,
            function_core: function_core,
        }
    }

    pub fn set_func_ctrl(&mut self,func: Arc<dyn Fn(&BotUser) -> Option<&BotUser> + Send + Sync>){
        self.function_controle = func;
    }

    pub fn set_func_core(&mut self, func: Arc<dyn Fn(&BotUser) + Send + Sync>) {
        self.function_core = func;
    }
}
