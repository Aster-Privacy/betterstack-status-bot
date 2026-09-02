use std::sync::Arc;

use poise::serenity_prelude::{
    self as serenity,
    EventHandler,
};
use tracing::{
    debug,
    error,
};

use crate::data::{
    Data,
    Error,
};

pub mod events;

pub struct Handler {}

#[serenity::async_trait]
impl EventHandler for Handler
{
    async fn dispatch(&self, ctx: &serenity::Context, event: &serenity::FullEvent)
    {
        let data: Arc<Data> = ctx.data();

        if let Err(e) = event_handler(ctx, event, &data).await
        {
            error!("Error handling event: {:?}", e);
        }
    }
}

#[allow(clippy::single_match)]
pub async fn event_handler(ctx: &serenity::Context, event: &serenity::FullEvent, data: &Arc<Data>)
-> Result<(), Error>
{
    let event_name: &str = event.into();
    let http = &ctx.http;

    debug!("Event received: {}", event_name);

    match event
    {
        serenity::FullEvent::Ready { data_about_bot, .. } =>
        {
            events::ready::ready(http, data_about_bot, data).await?;
        },

        _ =>
        {},
    }

    Ok(())
}
