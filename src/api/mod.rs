use crate::utils;
pub mod button;
pub mod card;

use button::Button;
use card::Card;
use utils::{BotUser};
use serde::ser::{Serialize ,Serializer};
use log::{info, warn};
use std::fmt;
use ureq::*;
use std::sync::Arc;

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

impl Serialize for MessagingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            MessagingType::RESPONSE => serializer.serialize_str("RESPONSE"),
            MessagingType::UPDATE => serializer.serialize_str("UPDATE"),
            MessagingType::MESSAGETAG => serializer.serialize_str("MESSAGE_TAG"),
        }
    }
}

pub trait ApiMessage {
    fn send(&self, user: &BotUser, token: &str);
}

#[derive(Clone)]
pub struct Message {
    text: Option<String>,
    buttons: Option<Vec<Button>>,
    cards: Option<Vec<Arc<dyn Card>>>,
}

impl ApiMessage for Message {
    fn send(&self, user: &BotUser, token: &str) {

        fn send_json(value: serde_json::Value, token: &str) {
            let url = format!("https://graph.facebook.com/v9.0/me/messages?access_token={}",token);
            info!("Json value : {}",value.to_string());
            let resp = ureq::post(&url)
                .send_json(value);

                if resp.ok() {
                    info!("success: {}", resp.into_string().unwrap());
                  } else {
                    warn!("error {}: {}", resp.status(), resp.into_string().unwrap());
                  }
        }

        if token.is_empty() {
            warn!("Message doesn't have a access_token");
        }
        else if self.text.is_some() {
            
            let json =  self::json!(
                {
                    "messaging_type": MessagingType::RESPONSE,
                    "recipient": {
                        "id": user.get_sender()
                    },
                    "message": {
                        "text": self.text,
                        "quick_replies": self.buttons,
                    }
                }
            );
            send_json(json,token);
        }
        else if self.cards.is_some() {
            let card =  self.cards.as_ref().unwrap()[0].clone();
            let json =  self::json!(
                {
                    "messaging_type": MessagingType::RESPONSE,
                    "recipient": {
                        "id": user.get_sender()
                    },
                    "message": {
                        "attachment": card.to_json()
                    }
                }
            );
            send_json(json,token);
        }
    }
}

impl Message {
    pub fn new(text : Option<String>,buttons: Option<Vec<Button>>, cards: Option<Vec<Arc<dyn Card>>>) -> Self {
        Message{
            text: text,
            buttons: buttons,
            cards: cards
        }
    }
}