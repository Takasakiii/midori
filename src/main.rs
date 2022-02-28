use commands::Data;
use config::Config;
use poise::{
    serenity_prelude::GatewayIntents, Framework, FrameworkOptions, PrefixFrameworkOptions,
};

use crate::commands::owner;

mod commands;
mod config;

#[tokio::main]
async fn main() {
    let config = Config::get();
    let prefix = config.prefix.clone();

    Framework::build()
        .token(&config.token)
        .user_data_setup(move |_ctx, _ready, _framework| {
            Box::pin(async move { Ok(Data { config }) })
        })
        .options(FrameworkOptions {
            prefix_options: PrefixFrameworkOptions {
                prefix: Some(prefix),
                ..Default::default()
            },
            commands: vec![owner::register_commands()],
            ..Default::default()
        })
        .client_settings(|s| s.intents(GatewayIntents::all()))
        .run()
        .await
        .unwrap();

    todo!()
}
