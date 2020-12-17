use super::button::Button;
use serde::ser::{Serialize, Serializer, SerializeSeq};
use std::marker::Copy;

#[derive(Clone)]
pub struct Card {
    title: String,
    subtitle: Option<String>,
    image_url: Option<String>,
    buttons: Option<Vec<Button>>,
    default_action: Option<String> // Accept an url. When the card was tapped we send an url
}

impl Serialize for Card {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>

    where S: Serializer,
    {
        let mut s = String::from("");
        s.push_str(r#""attachment":{{
            "type":"template",
            "payload":{{
              "template_type":"generic",
              "elements":)"#);


        match &self.buttons {
            Some(e) => {
                s.push('[');
                for elem in e {
                    s.push_str(&elem.to_json_str());
                    s.push(',');
                }

                s.push(']');
            }
            None => {
                s.push_str("none");
            }
        }

        serializer.serialize_str(s.as_str())
    }
}

impl Card {
    pub fn new(title: &str) -> Self {
        Card{
            title: String::from(title),
            subtitle: None,
            image_url: None,
            buttons: None,
            default_action: None,
        }
    }

    pub fn subtitle(mut self, subtitle: &str) -> Self {
        self.subtitle = Some(String::from(subtitle));
        self
    }

    pub fn image(mut self, url: &str) -> Self {
        self.image_url = Some(String::from(url));
        self
    }

    pub fn default_action(mut self, url: &str) -> Self {
        self.default_action = Some(String::from(url));
        self
    }

    pub fn button(mut self, button: Button) -> Self {
        match &mut self.buttons {
            Some(e) => e.push(button),
            None => self.buttons = Some(vec!(button))
        }
        self
    }
}