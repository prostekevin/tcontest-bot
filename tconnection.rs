use tokio::net::TcpStream;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, AsyncReadExt};
use std::error::Error;
use chrono;
use super::user::User;
use super::utils;

const URL: &str = "irc.chat.twitch.tv:6667";


pub struct Tconnection {
    user: User,
    connection: Option<TcpStream>,
    address: String
}

impl Tconnection {
    pub fn initialize(new_user: User) -> Tconnection {
       Tconnection {
        user: new_user,
        connection: None,
        address: URL.to_string(),
       } 
    }

    pub async fn connect(&mut self) -> io::Result<()> {
        let stream = TcpStream::connect(&self.address).await?;
        println!("connection established");
        self.connection = Some(stream);
        Ok(())
    }

    pub async fn authenticate(&mut self)  -> std::io::Result<()> {

        let first_msg = format!("{}{}\n", "PASS oauth:".to_string(), &self.user.get_oauth());
        let second_msg = format!("{}{}\n","NICK ".to_string(),self.user.get_username());
        
        let mut buffer = vec![0; 333];
        let mut stream = self.connection.take().expect("not connected");
        stream.write_all(first_msg.as_bytes()).await?;
        stream.write_all(second_msg.as_bytes()).await?;

        
        stream.read_exact(&mut buffer).await.unwrap();
        let uname = self.user.get_username();

        let buff_check = format!(":tmi.twitch.tv 001 {} :Welcome, GLHF!\r
:tmi.twitch.tv 002 {} :Your host is tmi.twitch.tv\r
:tmi.twitch.tv 003 {} :This server is rather new\r
:tmi.twitch.tv 004 {} :-\r
:tmi.twitch.tv 375 {} :-\r
:tmi.twitch.tv 372 {} :You are in a maze of twisty passages, all alike.\r
:tmi.twitch.tv 376 {} :>\r\n", uname, uname, uname, uname, uname, uname, uname,);

        if buff_check.as_bytes() == buffer {
            println!("authenticated")
        }
        self.connection = Some(stream);
        Ok(())
    }


    pub async fn join_channel(&mut self, ch_name: &str) {
        
        let msg = format!("JOIN #{}", ch_name);  
        self.send_msg(&msg).await.unwrap(); 
    }


    async fn send_msg(&mut self, msg: &str) -> Result<(),Box<dyn Error>> {
        let stream = self.connection.take().expect("not connected");
        loop {
            stream.writable().await?;

            match stream.try_write(utils::escape_msg(msg).as_bytes()) {
                Ok(_n) => {
                    break;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }       
            }
        }
        self.connection = Some(stream);
        Ok(())
    }
    pub async fn send_channel_msg(&mut self, ch: String, msg: &str) -> Result<(), Box<dyn Error>> {
        self.send_msg(&format!("PRIVMSG #{} : {}", &ch, &msg)).await.unwrap();
        Ok(())
    }

    pub async fn receive(&mut self) -> String {
        let mut stream = self.connection.take().expect("not connected");
        let mut buff = String::new();
        let mut line = io::BufReader::new(&mut stream);
        line.read_line(&mut buff).await.unwrap();
        self.connection = Some(stream);
        //String::from_utf8(buff).unwrap()
        buff
    }

    pub async fn twitch_ping(&mut self) {
        self.send_msg("PONG :tmi.twitch.tv").await.unwrap();
        println!("active.. {}", chrono::Utc::now());
    }
}
   

    /*pub fn set_non_block(&mut self){
        let stream = self.connection.take().expect("not connected");

        stream.set_nonblocking(true).unwrap();

        self.connection = Some(stream);
    } */

//---------------------------------------------------------------------------------------------------------------





