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

    let token = match serenity::Token::from_env("DISCORD_TOKEN")
    {
        Ok(token) => token,
        Err(e) =>
        {
            error!("`DISCORD_TOKEN` is not a valid bot token: {e}");
            std::process::exit(1);
        },
    };
    let intents = serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let data = match Data::new(&config).await
    {
        Ok(data) => data,
        Err(e) =>
        {
            error!("Failed to initialize: {e}");
            std::process::exit(1);
        },
    };

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(Box::new(framework))
        .event_handler(Arc::new(Handler {}))
        .data(Arc::new(data))
        .await;

    match client
    {
        Ok(mut client) =>
        {
            if let Err(e) = client.start().await
            {
                error!("Client stopped: {e}");
                std::process::exit(1);
            }
        },
        Err(e) =>
        {
            error!("Failed to build the Discord client: {e}");
            std::process::exit(1);
        },
    }
}
