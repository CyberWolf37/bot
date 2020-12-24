use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::vec::Vec;
use std::fmt;


#[derive(Clone)]
pub enum Button {
    PAYLOAD(String,String),
    URL(String,String),
    QUICKPAYLOAD(String,String),
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(button, button)")
    }
}

impl Serialize for Button {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Button::PAYLOAD(name,payload) => {
                let mut state = serializer.serialize_struct("ButtonPayload", 3)?;

                state.serialize_field("type", "postback")?;
                state.serialize_field("title", name)?;
                state.serialize_field("payload", payload)?;
                state.end()
            }
            Button::URL(name,url) => {
                let mut state = serializer.serialize_struct("ButtonUrl", 5)?;

                state.serialize_field("type", "web_url")?;
                state.serialize_field("title", name)?;
                state.serialize_field("url", url)?;
                state.serialize_field("webview_height_ratio", "compact")?;
                state.serialize_field("messenger_extensions", &false)?;
                state.end()
            },
            Button::QUICKPAYLOAD(name,payload) => {
                let mut state = serializer.serialize_struct("ButtonQuickPayload", 3)?;

                state.serialize_field("content_type", "text")?;
                state.serialize_field("title", name)?;
                state.serialize_field("payload", payload)?;
                state.end()
            }
        }
    }
}

impl Button {
    pub fn new_button_pb(name: &str, postback: &str) -> Button {
        Button::PAYLOAD(String::from(name),String::from(postback))
    }

    pub fn new_button_quick_pb(name: &str, postback: &str) -> Button {
        Button::QUICKPAYLOAD(String::from(name),String::from(postback))
    }

    pub fn new_button_url(name: &str, url: &str) -> Button {
        Button::PAYLOAD(String::from(name),String::from(url))
    }

    pub fn to_json_str(&self) -> String {
        match self {
            Button::PAYLOAD(name,payload) => format!("{{\"content_type\":\"text\",\"title\":\"{}\",\"payload\":\"{}\"}}",name,payload),
            Button::URL(name,url) => {
                format!("
                    {{
                        \"type\":\"web_url\",
                        \"url\":\"{}\",
                        \"title\":\"{}\",
                        \"webview_height_ratio\": \"compact\",
                        \"messenger_extensions\": \"false\",
                    }}",url,name)
            },
            Button::QUICKPAYLOAD(name,payload) => format!("{{\"content_type\":\"text\",\"title\":\"{}\",\"payload\":\"{}\"}}",name,payload),
        }
    }
}