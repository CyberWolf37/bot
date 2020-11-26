use crate::utils;

use utils::{BotUser};
use log::*;
use serde::{Serialize, Deserialize};
use std::fmt;
use futures::executor::block_on;
use ureq::*;

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

            let url = format!("https://graph.facebook.com/v9.0/me/messages?access_token={}",self.token);
            let resp = ureq::post(&url)
                .send_json(self::json!(
                    {
                        "messaging_type": "RESPONSE",
                        "recipient": {
                            "id": user.get_sender()
                        },
                        "message": {
                            "text": text
                        }
                    }
                ));

                if resp.ok() {
                    println!("success: {}", resp.into_string().unwrap());
                  } else {
                    println!("error {}: {}", resp.status(), resp.into_string().unwrap());
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