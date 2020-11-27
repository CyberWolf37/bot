use crate::utils;

use utils::{BotUser};
use log::{info, warn};
use std::fmt;
use ureq::*;

pub enum MessagingType {
    RESPONSE,
    UPDATE,
    MESSAGETAG,
}

impl fmt::Display for MessagingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessagingType::RESPONSE => write!(f,"RESPONSE"),
            MessagingType::UPDATE => write!(f,"UPDATE"),
            MessagingType::MESSAGETAG => write!(f,"MESSAGE_TAG"),
        }
    }
}

pub trait ApiMessage {
    fn send(&self, user: &BotUser, token: &str);
}

#[derive(Clone)]
pub struct Message {
    text: String,
}

impl ApiMessage for Message {
    fn send(&self, user: &BotUser, token: &str) {

        if token.is_empty() {
            warn!("Message doesn't have a access_token");
        }
        else {

            let url = format!("https://graph.facebook.com/v9.0/me/messages?access_token={}",token);
            let resp = ureq::post(&url)
                .send_json(self::json!(
                    {
                        "messaging_type": MessagingType::RESPONSE,
                        "recipient": {
                            "id": user.get_sender()
                        },
                        "message": {
                            "text": self.text
                        }
                    }
                ));

                if resp.ok() {
                    info!("success: {}", resp.into_string().unwrap());
                  } else {
                    warn!("error {}: {}", resp.status(), resp.into_string().unwrap());
                  }
        }
    }
}

impl Message {
    pub fn new(text : &str) -> Self {
        Message{
            text: String::from(text),
        }
    }
}

pub struct Button {
    text: String
}

pub struct MessageBuilder {
    text: Option<String>,
    buttons: Option<[Button]>,  
}

impl MessageBuilder {
    pub fn message(text: &str) -> MessageBuilder {
        MessageBuilder {
            text: Some(String::from(text)),
            buttons: None,
        }
    }

    pub fn buttons(&self, buttons: [Button]) -> MessageBuilder {
        let mut selfy = self;
        selfy.buttons = Some(buttons);
        self
    }
}