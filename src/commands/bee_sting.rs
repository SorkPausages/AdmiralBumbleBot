use {
    rand::seq::SliceRandom, serenity::{model::channel::Message, prelude::Context}
};

mod create_dumb_channel;
mod kick;
mod mute;

enum Sting {
    CreateDumbChannel,
    Kick,
    Mute,
}

const STINGS: &[Sting] = &[Sting::CreateDumbChannel, Sting::Kick, Sting::Mute];

pub async fn bee_sting(ctx: &Context, msg: &Message) {
    msg.channel_id
        .say(&ctx.http, "*Stings you*")
        .await
        .expect("Error sending message");

    let selection = { STINGS.choose(&mut rand::thread_rng()) };

    match selection {
        Some(Sting::CreateDumbChannel) => create_dumb_channel::create_dumb_channel(ctx, msg).await,
        Some(Sting::Kick) => kick::kick(ctx, msg).await,
        Some(Sting::Mute) => mute::mute(ctx, msg).await,
        None => {}
    }
}
