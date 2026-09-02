use poise::serenity_prelude as serenity;

use crate::data::{
    Context,
    Error,
};

/// Check the status of our services
#[poise::command(slash_command, guild_only)]
pub async fn status(ctx: Context<'_>) -> Result<(), Error>
{
    ctx.defer_ephemeral().await?;

    let data = &ctx.data();
    let guild_icon = ctx.guild().and_then(|v| v.icon_url()).unwrap_or_default();

    let services = data.status_page.get_status_page_resource(&data.client).await?;

    let mut fields = vec![];

    for service in services.iter()
    {
        fields.push((
            service.name.clone(),
            format!(
                "{} ({:.2}% uptime)",
                capitalize(&service.status),
                service.availability * 100.0
            ),
            false,
        ));
    }

    let embed = serenity::CreateEmbed::default()
        .title("Service Status")
        .color(serenity::Colour::DARK_BLUE)
        .thumbnail(guild_icon)
        .fields(fields);

    ctx.send(poise::CreateReply::new().embed(embed).ephemeral(true)).await?;

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
