use tokio;
use msg::{Msg, Type};
mod user;
mod tconnection;
mod utils;
mod msg;
mod contest;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
   let mut stream = utils::connection_channels_init().await.unwrap();
    loop {
        let buffer = stream.receive().await;
   
        let msg: Msg  = buffer.into();
        if msg.msg_type == Type::Hbeat {
            stream.twitch_ping().await;
        }
       
        if msg.msg_type == Type::ChatMsg && msg.username == "streamelements" {
            contest::add_points(&mut stream, contest::get_winner_prize(&msg)).await;
        }
    }   
}
