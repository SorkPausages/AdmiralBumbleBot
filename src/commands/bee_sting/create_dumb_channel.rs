use {
    crate::util::get_id_from_env, rand::{Rng, thread_rng}, serenity::{all::CreateChannel, json::from_reader, model::channel::Message, prelude::Context}, std::{collections::HashMap, fs::File}
};

pub async fn create_dumb_channel(ctx: &Context, msg: &Message) {
    let (chan_name, chan_description) = match get_random_channel() {
        Some(res) => res,
        None => return,
    };

    msg.channel_id
        .say(&ctx.http, "Creating a fun new channel!")
        .await
        .expect("Error sending message");

    msg.guild_id
        .expect("Error getting guild ID")
        .create_channel(
            &ctx.http,
            CreateChannel::new(chan_name)
                .topic(chan_description)
                .category(get_id_from_env("ABB_MAIN_CHANNEL_CATEGORY")),
        )
        .await
        .expect("Error creating channel");
}

fn get_random_channel() -> Option<(String, String)> {
    let f = File::open("dumb_channels.ron").expect("Error opening dumb channel list");

    let channel_names: HashMap<String, String> = match from_reader(f) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Error reading dumb_channels.ron: {}", e);
            return None;
        }
    };

    let names: Vec<String> = channel_names.keys().cloned().collect();
    let roll = thread_rng().gen_range(0..names.len());

    Some((
        names[roll as usize - 1].clone(),
        channel_names[names[roll as usize - 1].as_str()].clone(),
    ))
}
