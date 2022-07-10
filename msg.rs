#[derive(Debug, Clone,PartialEq)]
pub enum Type {
    Hbeat,
    ChatMsg,
    Ignore
}

#[derive(Debug,Clone)]
pub struct Msg { 
    pub msg_type: Type,
    pub username: String,
    pub channel: String,
    pub msg: Option<Vec<String>>
}
impl Msg {
    fn create_ignore() -> Self {
        return Msg{ msg_type: Type::Ignore, username: "".to_string(), channel: "".to_string(), msg: None};
    }
    fn create_hbeat() -> Self {
        return Msg{ msg_type: Type::Hbeat, username: "twitch".to_string(), channel: "".to_string(), msg: None};
    }
}

impl From<String> for Msg {
    fn from(text: String) -> Self {
        let msg_arr: Vec<String> = text.split_terminator(":").collect::<String>().split_ascii_whitespace().map(|s| s.to_owned()).collect();
        if msg_arr[1] == "JOIN" || msg_arr[1] == "353" {
            return Msg::create_ignore();
        }
        let offset = msg_arr[0].find('!');
        if offset.is_some() {
            let name = msg_arr[0][..offset.unwrap()].to_string();
            let chan = msg_arr[2][1..].to_string();
            let msg: Vec<String> = msg_arr.into_iter().skip(3).collect();
            return Msg{ msg_type: Type::ChatMsg, username: name, channel: chan, msg: Some(msg)}
        } else {
            return Msg::create_hbeat();
        }
    }
}