use std::sync::Arc;

use poise::serenity_prelude::{
    self as serenity,
    Mentionable,
};
use tracing::{
    debug,
    error,
    info,
    warn,
};

use crate::data::{
    Data,
    Error,
};

pub async fn ready(http: &Arc<serenity::Http>, bot_data: &serenity::Ready, custom_data: &Arc<Data>)
-> Result<(), Error>
{
    info!("Name: {}", bot_data.user.name);
    info!("ID: {}", bot_data.user.id.get());

    let Some(updates_channel) = custom_data.guild.updates_channel
    else
    {
        warn!("`UPDATES_CHANNEL_ID` is not set, so incident announcements are off.");
        return Ok(());
    };

    let data = custom_data.clone();
    let http = http.clone();

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(data.poll_interval_secs));

        let action_row = serenity::CreateComponent::ActionRow(serenity::CreateActionRow::Buttons(
            vec![serenity::CreateButton::new_link(data.status_page.link.clone()).label("Status Page")].into(),
        ));

        loop
        {
            interval.tick().await;

            match data.status_page.get_rss_feed(&data.client, &data.database).await
            {
                Ok(new_entries) =>
                {
                    debug!("Checked the status feed, {} new entries", new_entries.len());

                    if new_entries.is_empty()
                    {
                        continue;
                    }

                    for entry_number in (0..new_entries.len()).rev()
                    {
                        let entry = &new_entries[entry_number];

                        let mention = data
                            .guild
                            .update_role
                            .map(|role| format!("{}\n", role.mention()))
                            .unwrap_or_default();

                        let message = format!(
                            "{}<t:{}:F>\n{}\n\n{}",
                            mention,
                            entry
                                .pub_date
                                .map(|v| v.timestamp())
                                .unwrap_or(chrono::Utc::now().timestamp()),
                            entry.description,
                            entry.link,
                        );

                        let allowed_mentions = serenity::CreateAllowedMentions::new()
                            .everyone(false)
                            .all_users(false)
                            .roles(data.guild.update_role.into_iter().collect::<Vec<_>>());

                        let _ = serenity::CreateMessage::new()
                            .content(message)
                            .allowed_mentions(allowed_mentions)
                            .components(vec![action_row.clone()])
                            .execute(&http, updates_channel.widen())
                            .await;
                    }
                },
                Err(e) =>
                {
                    error!("Error checking RSS feed: {:?}", e);
                },
            }
        }
    });

    Ok(())
}
