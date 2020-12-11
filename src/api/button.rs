#[derive(Clone)]
pub enum Button {
    PAYLOAD(String,String),
    URL(String,String),
}

impl Serialize for Button {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Button::PAYLOAD(name,payload) => serializer.serialize_str(&format!("{{\"content_type\":\"text\",\"title\":\"{}\",\"payload\":\"{}\"}}",name,payload)),
            Button::URL(name,url) => {
                serializer.serialize_str(&format!("
                    {{
                        \"type\":\"web_url\",
                        \"url\":\"{}\",
                        \"title\":\"{}\",
                        \"webview_height_ratio\": \"compact\",
                        \"messenger_extensions\": \"false\",
                    }}",url,name))
            },
        }
    }
}

impl Button {
    pub fn new_button_pb(name: &str, postback: &str) -> Button {
        Button::PAYLOAD(String::from(name),String::from(postback))
    }
    pub fn new_button_url(name: &str, url: &str) -> Button {
        Button::PAYLOAD(String::from(name),String::from(url))
    }
}