use std::io::Result;
use crate::tconnection::Tconnection;
use crate::contest::Info;

pub fn init() -> Result<Info> {
    let file = include_str!("settings.json");
    
    let json: Info = serde_json::from_str(file).unwrap();

    Ok(json)
}
pub async fn connection_channels_init() -> Result<Tconnection> {
    let data = init().unwrap();
    let mut con = Tconnection::initialize(data.user);
    con.connect().await?;
    con.authenticate().await?;
    for channel in data.channels {
        con.join_channel(&channel).await;
        println!("joined {}", channel);
    }
    Ok(con)
}
pub fn escape_msg(msg : &str) -> String {
    let ret = format!{"{}\r\n",msg};
    ret
}

