extern crate discord;

use discord::model::Message;

pub fn handle_message(message: &Message) -> String {
    let reply;
    if message.content == "!greet" {
        reply = format!("Hey {}",message.author.id.mention());
    } else {
        reply = "".to_string();
    }
    reply
}
