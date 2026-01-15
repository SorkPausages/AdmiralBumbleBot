use serenity::{model::channel::Message, prelude::Context};

pub async fn health(ctx: &Context, msg: &Message) {
    if let Err(e) = msg.channel_id.say(&ctx.http, "bad").await {
        eprintln!("Error displaying health: {}", e);
    }
}
