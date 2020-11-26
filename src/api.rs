use crate::utils;

use utils::{BotUser};
use std::env;
use log::*;
use serde_json::*;
use http::Request;
use serde_derive::*;
use serde::{Serialize, Deserialize};
use std::fmt;

pub trait ApiMessage {
    fn send(&self, user: &BotUser, text: &str);
}

pub struct Message {
    token: String,
}

impl ApiMessage for Message {
    fn send(&self, user: &BotUser, text: &str) {

        if self.token.is_empty() {
            warn!("Message doesn't have a access_token");
        }
        else {
            let json = json!(format!(r#"
            {{
                "messaging_type": "RESPONSE",
                "recipient": {{
                "id": "{}"
                }},
                "message": {{
                "text": "{}"
                }}
            }}"#,user.get_sender(),text));
            
            let request = Request::builder()
                .uri(format!("https://graph.facebook.com/v9.0/me/messages?access_token={}",self.token))
                .header("User-Agent", "botMessenger/1.0")
                .header("Content-type", "application/json")
                .body(json);

            match request {
                Ok(_) => info!("Send sucessful"),
                Err(_) => warn!("Err of sending message"),
            }
        }
    }
}

impl Message {
    pub fn new(token : &str) -> Self {
        Message{
            token: String::from(token),
        }
    }

    pub fn set_token(&mut self,token: &'static str) {
        self.token = String::from(token);
    }
}