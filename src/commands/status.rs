use poise::serenity_prelude as serenity;
use tracing::error;

use crate::data::{
    Context,
    Error,
};

const FIELDS_PER_EMBED: usize = 25;
const MAX_EMBEDS: usize = 10;

/// Check the status of our services
#[poise::command(slash_command, guild_only)]
pub async fn status(ctx: Context<'_>) -> Result<(), Error>
{
    ctx.defer_ephemeral().await?;

    let data = &ctx.data();
    let guild_icon = ctx.guild().and_then(|v| v.icon_url()).unwrap_or_default();

    let services = match data.status_page.get_status_page_resource(&data.client).await
    {
        Ok(services) => services,
        Err(e) =>
        {
            error!("Failed to read the status page: {e:?}");

            ctx.send(
                poise::CreateReply::new()
                    .content("I could not reach the status page. Try again in a moment.")
                    .ephemeral(true),
            )
            .await?;

            return Ok(());
        },
    };

    if services.is_empty()
    {
        ctx.send(
            poise::CreateReply::new()
                .content("The status page has no services on it yet.")
                .ephemeral(true),
        )
        .await?;

        return Ok(());
    }

    let mut reply = poise::CreateReply::new().ephemeral(true);

    for (page, chunk) in services.chunks(FIELDS_PER_EMBED).take(MAX_EMBEDS).enumerate()
    {
        let fields = chunk.iter().map(|service| {
            (
                service.name.clone(),
                format!(
                    "{} ({:.2}% uptime)",
                    capitalize(&service.status),
                    service.availability * 100.0
                ),
                false,
            )
        });

        let mut embed = serenity::CreateEmbed::default()
            .color(serenity::Colour::DARK_BLUE)
            .fields(fields);

        if page == 0
        {
            embed = embed.title("Service Status").thumbnail(guild_icon.clone());
        }

        reply = reply.embed(embed);
    }

    ctx.send(reply).await?;

    Ok(())
}

// Taken right from the `capitalize` crate xD
fn capitalize(word: &str) -> String
{
    let mut chars = word.chars();
    let Some(first) = chars.next()
    else
    {
        return String::with_capacity(0);
    };
    first.to_uppercase().chain(chars.flat_map(char::to_lowercase)).collect()
}
