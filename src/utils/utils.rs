use crate::api;

use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::fmt;
use std::fmt::{Display};
use api::{Message, ApiMessage, Button, Card};
use serde::de::{self, Deserialize, Deserializer};
use rocket_contrib::json::{Json, JsonValue};
use serde_json::Value;
use std::sync::Arc;
use log::{info, warn, trace};
use std::rc::Rc;

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
    fn consume(&self,message: &BotUser, token: &str) -> PipeStatus;
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
    pub fn new(port: u16 ,ip: &str ,uri: &str ,size: u16 ,token_webhook: &str ,token_fb_page: &str) -> Self {
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
    pub fn set_token_webhook(&mut self, token: &str) {
        self.token_webhook = String::from(token);
    }

    pub fn set_token_fb_page(&mut self, token: &str) {
        self.token_fb_page = String::from(token);
    }

    // set Vars conf
    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    pub fn set_ip(&mut self, ip: &str) {
        self.ip = String::from(ip);
    }

    pub fn set_uri(&mut self, uri: &str) {
        self.uri = String::from(uri);
    }

    pub fn set_workers(&mut self, workers: u16) {
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
pub struct MessagingPostback {
    payload: String,
}

impl<'a> Messaging for MessagingPostback {
    fn message_type(&self) -> MessagingType {
        MessagingType::POSTBACK(&self)
    }
    fn message(&self) -> &str {
        &self.payload
    }
}

#[derive(Clone)]
pub struct MessagingMessage {
    text: String,
}

impl<'a> Messaging for MessagingMessage {
    fn message_type(&self) -> MessagingType {
        MessagingType::MESSAGE(&self)
    }
    fn message(&self) -> &str {
        &self.text
    }
}