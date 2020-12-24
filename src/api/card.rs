use super::button::Button;
use serde::ser::{Serialize, Serializer};

pub trait Card: Clone {}

#[derive(Clone)]
pub struct DefaultAction {
    status: &'static str,
    url: String,
    title: String,
}

impl DefaultAction {
    pub fn new(title: &str, url: &str) -> Self {
        DefaultAction{
            status: "web_url",
            url: String::from(url),
            title: String::from(title),
        }
    }

    pub fn to_json(&self) -> &str {
        format!(r#"{{"type":"{}","url":"{}","title":"{}"}}"#,self.status,self.url,self.title)
    }
}

impl Serialize for DefaultAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>

    where S: Serializer,
    {      
        let mut s = format!(r#"{{"type":"{}","url":"{}","title":"{}"}}"#,self.status,self.url,self.title);
        serializer.serialize_str(&s)
    }
}

#[derive(Clone)]
pub struct CardGeneric {
    title: String,
    subtitle: Option<String>,
    image_url: Option<String>,
    buttons: Option<Vec<Button>>,
    default_action: Option<DefaultAction> // Accept an url. When the card was tapped we send an url
}

impl Serialize for CardGeneric {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>

    where S: Serializer,
    {
        let mut s = format!(r#"{{"template_type":"generic","elements": ["#);

        s.push_str( format!(r#"{{"title":"{}""# 
        ,self.title));

        if self.subtitle.is_some() {
           s.push_str( &format!(r#","subtitle":"{}""#, self.subtitle.as_ref().unwrap()) );
        }

        if self.image_url.is_some() {
            s.push_str( &format!(r#","image_url":"{}""#, self.image_url.as_ref().unwrap()) );
        }     

        match self.buttons.as_ref() {
            Some(e) => {
                s.push_str( r#","buttons":"[""# );
                for elem in e {
                    s.push_str( &elem.to_json_str() );
                    s.push(',');
                }

                s.push(']');
            }
            None => {}
        }

        if self.default_action.is_some() {
            s.push_str( r#","default_action":"# );
            s.push_str( self.default_action.unwrap().to_json() );
        }

        s.push_str("]}");

        serializer.serialize_str(s.as_str())
    }
}

impl CardGeneric {
    pub fn new(title: &str) -> Self {
        CardGeneric{
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

    pub fn default_action(mut self, default_action: DefaultAction) -> Self {
        self.default_action = Some(default_action);
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