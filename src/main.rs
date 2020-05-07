use serenity::prelude::{EventHandler, Context};
use serenity::model::gateway::{Ready, Activity};
use serenity::model::channel::Message;
use serenity::model::user::OnlineStatus;
use std::env;
use serenity::Client;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, mut new_message: Message) {
        if new_message.content.starts_with("Ich bin ") {
            let new_string = new_message.content.split_off(8);
            let _ = new_message.channel_id.say(&ctx.http, format!("Hallo {}, ich bin Vater!", new_string));
        }

        if new_message.content.starts_with("!einladung") {
            let _ = new_message.channel_id.say(&ctx.http, "Hol dir jetzt den besten Bot auf Discord\nhttps://discordapp.com/api/oauth2/authorize?client_id=698244880970940488&permissions=3072&scope=bot");
        }
    }

    fn ready(&self, ctx: Context, data_about_bot: Ready) {
        ctx.set_presence(Option::from(Activity::listening("!einladung")), OnlineStatus::Idle);
        println!("Verbunden als {}", data_about_bot.user.name)
    }
}

fn main() {
    let token = env::var("TOKEN").expect("token supplied");

    let mut client = Client::new(&token, Handler).expect("Error creating Client!");

    if let Err(why) = client.start() {
        panic!("Error while starting {:#?}", why);
    }
}
