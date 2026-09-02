use std::io::Cursor;

use rss::Channel;
use sqlx::SqlitePool;
use tracing::error;

use crate::data::{
    Error,
    status_page::StatusPageSettings,
};

#[derive(Debug, Clone)]
pub struct Entry
{
    pub _title: String,
    pub link: String,
    pub pub_date: Option<chrono::DateTime<chrono::Utc>>,
    pub _guid: String,
    pub description: String,
}

impl StatusPageSettings
{
    pub async fn get_rss_feed(&self, client: &reqwest::Client, pool: &SqlitePool) -> Result<Vec<Entry>, Error>
    {
        let response = client.get(format!("{}feed.rss", self.link)).send().await?;
        let bytes = response.bytes().await?;
        let channel = Channel::read_from(Cursor::new(bytes))?;

        let mut new_entries = Vec::new();

        for item in channel.items()
        {
            let current_date = chrono::Utc::now().to_rfc2822();

            let guid = match item.guid()
            {
                Some(g) => format!("{}-{}", g.value(), item.pub_date().unwrap_or(&current_date)),
                None =>
                {
                    // jus incase if it ever happens doubt it will
                    format!(
                        "{}-{}",
                        item.link().unwrap_or(&self.link),
                        item.pub_date().unwrap_or(&current_date)
                    )
                },
            };

            let date = match item.pub_date()
            {
                Some(d) => d,
                None => &current_date,
            };

            let result = sqlx::query("INSERT OR IGNORE INTO guids (id, date) VALUES (?, ?)")
                .bind(&guid)
                .bind(date)
                .execute(pool)
                .await;

            match result
            {
                Ok(query_result) =>
                {
                    if query_result.rows_affected() > 0
                    {
                        let date = item.pub_date().unwrap_or("");
                        let pub_date = chrono::DateTime::parse_from_rfc2822(date)
                            .ok()
                            .map(|dt| dt.with_timezone(&chrono::Utc));

                        new_entries.push(Entry {
                            _title: item.title().unwrap_or("No Title").to_string(),
                            link: item.link().unwrap_or("").to_string(),
                            pub_date,
                            _guid: guid,
                            description: item.description().unwrap_or("").to_string(),
                        });
                    }
                },
                Err(e) =>
                {
                    error!("Database error while checking RSS feed: {:?}", e);
                },
            }
        }

        Ok(new_entries)
    }
}
