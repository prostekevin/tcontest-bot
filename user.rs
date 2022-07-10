use serde::{Deserialize,Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    username: String,
    oauth: String
}

impl User {
    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn get_oauth(&self) -> String {
        self.oauth.clone()
    }
    pub fn _new() -> Self {
        User {
            username:"".to_string(),
            oauth:"".to_string(),
        }
    }

    pub fn _login(&mut self, name: String, auth: String) {
        self.username = name;
        self.oauth = auth;
    }
}