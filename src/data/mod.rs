use poise::serenity_prelude as serenity;
use sqlx::{
    Pool,
    Sqlite,
    SqlitePool,
    sqlite::SqlitePoolOptions,
};

use crate::data::{
    config::Config,
    status_page::StatusPageSettings,
};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Command = poise::Command<Data, Error>;

pub mod config;
pub mod rss;
pub mod status_page;

#[derive(Debug, Clone)]
pub struct Data
{
    pub database: SqlitePool,
    pub client: reqwest::Client,
    pub guild: GuildSettings,
    pub status_page: StatusPageSettings,
    pub poll_interval_secs: u64,
}

#[derive(Debug, Clone)]
pub struct GuildSettings
{
    pub updates_channel: Option<serenity::ChannelId>,
    pub update_role: Option<serenity::RoleId>,
}

impl Data
{
    pub async fn new(config: &Config) -> Result<Self, Error>
    {
        let database = Data::setup_db(&config.database_url).await?;
        let client = reqwest::Client::new();

        let status_page = StatusPageSettings {
            link: config.status_page_url.clone(),
            token: config.api_token.clone(),
            page_id: config.page_id.clone(),
        };

        if !Data::check_db(&database).await?
        {
            status_page.get_rss_feed(&client, &database).await?;
        }

        Ok(Self {
            database,
            client,
            guild: GuildSettings {
                updates_channel: config.updates_channel,
                update_role: config.update_role,
            },
            status_page,
            poll_interval_secs: config.poll_interval_secs,
        })
    }

    async fn setup_db(database_url: &str) -> Result<Pool<Sqlite>, Error>
    {
        let database = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(
                database_url
                    .parse::<sqlx::sqlite::SqliteConnectOptions>()?
                    .create_if_missing(true),
            )
            .await?;

        Ok(database)
    }

    // Creates the table if needed and reports whether it already holds entries
    async fn check_db(database: &Pool<Sqlite>) -> Result<bool, Error>
    {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS guids (
                id TEXT PRIMARY KEY,
                date TEXT
            )",
        )
        .execute(database)
        .await?;

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM guids").fetch_one(database).await?;

        match count.0
        {
            0 => Ok(false),
            _ => Ok(true),
        }
    }
}
