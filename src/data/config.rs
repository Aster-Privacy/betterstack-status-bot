use std::collections::HashSet;

use poise::serenity_prelude as serenity;
use tracing::warn;

use crate::data::Error;

const DEFAULT_POLL_INTERVAL_SECS: u64 = 60;

#[derive(Debug, Clone)]
pub struct Config
{
    pub status_page_url: String,
    pub api_token: String,
    pub page_id: String,
    pub database_url: String,
    pub updates_channel: Option<serenity::ChannelId>,
    pub update_role: Option<serenity::RoleId>,
    pub poll_interval_secs: u64,
    pub command_prefix: String,
    pub owners: HashSet<serenity::UserId>,
}

impl Config
{
    pub fn from_env() -> Result<Self, Error>
    {
        let status_page_url = normalize_url(&required("STATUS_PAGE_URL")?);

        Ok(Self {
            status_page_url,
            api_token: required("API_TOKEN")?,
            page_id: required("STATUS_PAGE_ID")?,
            database_url: optional("DATABASE_URL").unwrap_or_else(|| "sqlite:status.db".to_string()),
            updates_channel: parse_id("UPDATES_CHANNEL_ID")?.map(serenity::ChannelId::new),
            update_role: parse_id("UPDATE_ROLE_ID")?.map(serenity::RoleId::new),
            poll_interval_secs: parse_id("POLL_INTERVAL_SECS")?.unwrap_or(DEFAULT_POLL_INTERVAL_SECS),
            command_prefix: optional("COMMAND_PREFIX").unwrap_or_else(|| "~".to_string()),
            owners: parse_owners()?,
        })
    }
}

fn required(key: &str) -> Result<String, Error>
{
    optional(key).ok_or_else(|| format!("`{key}` is not set. Copy `.env.example` to `.env` and fill it in.").into())
}

fn optional(key: &str) -> Option<String>
{
    std::env::var(key)
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

fn parse_id(key: &str) -> Result<Option<u64>, Error>
{
    match optional(key)
    {
        Some(value) => value
            .parse::<u64>()
            .map(Some)
            .map_err(|_| format!("`{key}` must be a number, got `{value}`.").into()),
        None => Ok(None),
    }
}

fn parse_owners() -> Result<HashSet<serenity::UserId>, Error>
{
    let Some(value) = optional("BOT_OWNERS")
    else
    {
        warn!("`BOT_OWNERS` is not set, so only the Discord application owner can run owner commands.");
        return Ok(HashSet::new());
    };

    let mut owners = HashSet::new();

    for id in value.split(',').map(str::trim).filter(|v| !v.is_empty())
    {
        let id = id
            .parse::<u64>()
            .map_err(|_| format!("`BOT_OWNERS` must be a comma separated list of user IDs, got `{id}`."))?;

        owners.insert(serenity::UserId::new(id));
    }

    Ok(owners)
}

fn normalize_url(url: &str) -> String
{
    match url.ends_with('/')
    {
        true => url.to_string(),
        false => format!("{url}/"),
    }
}
