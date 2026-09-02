use std::sync::Arc;

use error_handler::on_error;
use poise::serenity_prelude as serenity;
use tracing::{
    debug,
    error,
};

use crate::{
    commands::get_commands,
    data::{
        Data,
        config::Config,
    },
    event_handler::Handler,
};

mod commands;
mod data;
mod error_handler;
mod event_handler;

#[tokio::main]
async fn main()
{
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let config = match Config::from_env()
    {
        Ok(config) => config,
        Err(e) =>
        {
            error!("{e}");
            std::process::exit(1);
        },
    };

    let options = poise::FrameworkOptions {
        commands: get_commands(),

        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(config.command_prefix.clone().into()),
            ..Default::default()
        },

        on_error: |error| Box::pin(on_error(error)),

        pre_command: |ctx| {
            Box::pin(async move {
                debug!("Executing command {}...", ctx.command().qualified_name);
            })
        },

        post_command: |ctx| {
            Box::pin(async move {
                debug!("Executed command {}!", ctx.command().qualified_name);
            })
        },

        skip_checks_for_owners: false,
        owners: config.owners.clone(),

        ..Default::default()
    };

    let framework = poise::Framework::new(options);

    let token = serenity::Token::from_env("DISCORD_TOKEN").expect("`DISCORD_TOKEN` not in env.");
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let data = Data::new(&config).await.expect("Failed to initialize data");

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(Box::new(framework))
        .event_handler(Arc::new(Handler {}))
        .data(Arc::new(data))
        .await;

    client.unwrap().start().await.unwrap()
}
