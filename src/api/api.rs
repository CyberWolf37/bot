use crate::utils;

use utils::{BotUser};
use serde::ser::{Serialize ,Serializer};
use serde_json::ser::{Serializer as SerializerJ};
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
    //fn build(&self) -> Option<&[String]>;
}

#[derive(Clone)]
pub struct Message {
    text: Option<String>,
    buttons: Option<Vec<Button>>,
    cards: Option<Vec<Card>>,
}

/*impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = String::from("{");

        if self.text.is_some() {
            s.push_str(format!("\"text\": \"{}\",",self.text.unwrap()))
        }
        else if self.buttons.is_some() {
            s.push_str("\"quick_replies\":[");
            self.buttons.unwrap().iter().for_each(|x| {
                s.push_str(format!("{}",x.serialize().unwrap()));
                s.push(',');
            })
        }
        else {
            eprint!("Error don't have data to serialize")
        }

        serializer.serialize_str(&s)
    }
}*/

impl ApiMessage for Message {
    fn send(&self, user: &BotUser, token: &str) {

        if token.is_empty() {
            warn!("Message doesn't have a access_token");
        }
        else {

            let url = format!("https://graph.facebook.com/v9.0/me/messages?access_token={}",token);
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
            info!("Json value : {}",json);
            let resp = ureq::post(&url)
                .send_json(json);

                if resp.ok() {
                    info!("success: {}", resp.into_string().unwrap());
                  } else {
                    warn!("error {}: {}", resp.status(), resp.into_string().unwrap());
                  }
        }
    }
}

impl Message {
    pub fn new(text : Option<String>,buttons: Option<Vec<Button>>, cards: Option<Vec<Card>>) -> Self {
        Message{
            text: text,
            buttons: buttons,
            cards: cards
        }
    }
}