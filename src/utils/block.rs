#[derive(Clone)]
pub struct Block{
    name: String,
    token: String,
    childs: Arc<Vec<(BotUser,usize)>>,
    pipe: Vec<Arc<dyn PipeBox + Send + Sync>>,
}

impl Default for Block {
    fn default() -> Self {
        Block{
            name: String::from("Hello"),
            token: String::from(""),
            childs: Arc::new(Vec::new()),
            pipe: Vec::new(),
        }
    }
}

impl Block {

    // Init Block
    pub fn new(name: &str) -> Self {
        let mut block = Block::default();
        block.set_name(name);
        block
    }

    // Rooting user
    pub fn root(&mut self ,user: &BotUser) {
        let find = self.find(user);

        match find {
            true => {
                self.consume(user)
            },
            false => {

                (*Arc::make_mut(&mut self.childs)).push((user.clone(),0));
                self.consume(user);
            }
        }
    }

    // Consume the PipeBox for the user
    fn consume(&mut self ,user: &BotUser) {

        let value = match (*Arc::make_mut(&mut self.childs)).iter_mut().enumerate().find(|x| {x.1.0 == *user}) {
            Some(x) => {
                match self.pipe[x.1.1].consume(user, &self.token) {
                    PipeStatus::NEXT => {
                        x.1.1 = x.1.1 + 1;
                        if x.1.1 >= self.pipe.len() {
                            Some(x.0)
                        }
                        else {
                            None
                        }
                    },
                    PipeStatus::REPLAY => {
                        x.1.1 = 0;
                        None
                    },
                    PipeStatus::RESTART => {
                        x.1.1 = 0;
                        None
                    },
                }
            }
            None => {
                warn!("Don't match with any childs");
                None
            }
        };

        match value {
            Some(e) => {
                (*Arc::make_mut(&mut self.childs)).remove(e);
            },
            None => {}
        }

    }

    // Setter
    pub fn set_name(&mut self, name: &str) -> &mut Self{
        self.name = String::from(name);
        self
    }

    pub fn set_token(&mut self, token: &str) {
        self.token = String::from(token);
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_pipe(&self) -> &[Arc<dyn PipeBox + Send + Sync>] {
        &self.pipe
    }

    pub fn cartBox<T: 'static + PipeBox + Send + Sync> (mut self, pipeBox: T) -> Self {
        self.pipe.push(Arc::new(pipeBox));
        self
    }

    pub fn find(&self,user: &BotUser) -> bool {
        match self.childs.iter().find(|x| {
            x.0.get_sender() == user.get_sender()
        }) {
            Some(_) => {
                return true
            }
            None => {
                return false
            }
        }
    }

    pub fn find_mut(&mut self,user: &BotUser) -> Option<&mut (BotUser,usize)> {
        (*Arc::make_mut(&mut self.childs)).iter_mut().find(|x| {
            x.0.get_sender() == user.get_sender()
        })
    }

    pub fn remove_child(&mut self,user: &BotUser) {
        let child = (*Arc::make_mut(&mut self.childs)).iter_mut().enumerate().find(|x| {
            x.1.0.get_sender() == user.get_sender()
        });

        let child = match child {
            Some(e) => Some(e.0),
            None => None,
        };

        match child {
            Some(e) => {(*Arc::make_mut(&mut self.childs)).remove(e);},
            None => {},
        }

    }
}

#[derive(Clone)]
pub struct CartBox {
    function_controle: Arc<dyn Fn(&BotUser) -> Option<&BotUser> + Send + Sync>,

    text: Option<String>,
    button: Option<Vec<Button>>,
    cards: Option<Vec<Card>>,
}

impl PipeBox for CartBox{
    fn consume(&self,message: &BotUser, token: &str) -> PipeStatus {
        info!("Consume in the block the pipebox");
        match (self.function_controle)(message) {
            Some(e) => {
                self.build().send(e,token);
                PipeStatus::NEXT
            }
            None => {
                PipeStatus::REPLAY
            }
        }
    }
}

impl CartBox {
    pub fn new() -> Self {
        let function_controle: Arc<dyn Fn(&BotUser) -> Option<&BotUser> + Send + Sync> = Arc::new(|u| {Some(u)});
        CartBox{
            function_controle: function_controle,

            text: None,
            button: None,
            cards: None,
        }
    }

    pub fn text(mut self,text: &str) -> Self {
        self.text = Some(String::from(text));
        self
    }

    pub fn button_postback(mut self,button_text: &str ,button_payload: &str) -> Self {
        match &mut self.button {
            Some(e) => {e.push(Button::new_button_pb(button_text, button_payload));},
            None => {
                let mut buttons = Vec::new();
                buttons.push(Button::new_button_pb(button_text, button_payload));

                self.button = Some(buttons);
            },
        }
        self
    }

    pub fn card(mut self, card: Card) -> Self {
        match &mut self.cards {
            Some(e) => {
                e.push(card);
            }
            None => {
                self.cards = Some(vec![card]);
            }
        }
        self
    }

    pub fn with_func_ctrl(&mut self,func: Arc<dyn Fn(&BotUser) -> Option<&BotUser> + Send + Sync>){
        self.function_controle = func;
    }

    fn build(&self) -> Box<dyn ApiMessage> {
        let text = &self.text;
        let button = &self.button;
        if text.is_some() && button.is_some() {
            Box::new(Message::new(Some(text.clone().unwrap()),Some(button.clone().unwrap()),None))
        }
        else if text.is_some() && button.is_none() {
            Box::new(Message::new(Some(text.clone().unwrap()),None,None))
        }
        else {
            Box::new(Message::new(Some(String::from("Basic Text")),None,None))
        }     
    }
}
