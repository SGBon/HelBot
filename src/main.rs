extern crate discord;

use discord::Discord;
use discord::model::Event;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

mod handlers;

fn main() {
    let f = File::open("token").unwrap();
    let mut reader = BufReader::new(f);
    let mut token = String::new();

    reader.read_line(&mut token).unwrap();
    token.pop(); /* remove the newline from the token */

    /* Log in to Discord using a bot token from file */
    let discord = Discord::from_bot_token(&token).expect("login failed");

    /* establish and use websocket connection */
    let (mut connection, _) = discord.connect().expect("connect failed");
    println!("Hel is ready");
    loop{
        match connection.recv_event(){
            Ok(Event::MessageCreate(message)) => {
                let reply = handlers::handle_message(&message);
                if reply.len() > 0 {
                    let _ = discord.send_message(&message.channel_id,&reply,"",false);
                }
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code,body)) => {
                println!("Gateway closed o nus with code {:?}: {}",code,body);
                break
            }
            Err(err) => println!("received error: {:?}",err)
        }
    }
}
