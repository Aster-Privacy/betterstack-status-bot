use serde::Deserialize;

use crate::data::Error;

const MAX_PAGES: usize = 20;

#[derive(Debug, Clone)]
pub struct StatusPageSettings
{
    pub link: String,
    pub token: String,
    pub page_id: String,
}

#[derive(Debug, Clone)]
pub struct StatusPageResource
{
    pub _id: String,
    pub name: String,
    pub availability: f64,
    pub status: String,
}

#[derive(Deserialize)]
struct Response
{
    data: Vec<Resource>,
    #[serde(default)]
    pagination: Pagination,
}

#[derive(Default, Deserialize)]
struct Pagination
{
    #[serde(default)]
    next: Option<String>,
}

#[derive(Deserialize)]
struct Attributes
{
    public_name: String,
    availability: f64,
    status: String,
}

#[derive(Deserialize)]
struct Resource
{
    id: String,
    attributes: Attributes,
}

impl StatusPageSettings
{
    pub async fn get_status_page_resource(&self, client: &reqwest::Client) -> Result<Vec<StatusPageResource>, Error>
    {
        let mut url = Some(format!(
            "https://uptime.betterstack.com/api/v2/status-pages/{}/resources?per_page=50",
            self.page_id
        ));

        let mut resources = Vec::new();

        for _ in 0..MAX_PAGES
        {
            let Some(next) = url.take()
            else
            {
                break;
            };

            let res = client
                .get(&next)
                .header("Authorization", format!("Bearer {}", self.token))
                .send()
                .await?
                .error_for_status()?;

            let response = res.json::<Response>().await?;

            resources.extend(response.data.into_iter().map(|resource| StatusPageResource {
                _id: resource.id,
                name: resource.attributes.public_name,
                availability: resource.attributes.availability,
                status: resource.attributes.status,
            }));

            url = response.pagination.next;
        }

        Ok(resources)
    }
}
