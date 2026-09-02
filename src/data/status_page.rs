use serde::Deserialize;

use crate::data::Error;

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
        let url = format!(
            "https://uptime.betterstack.com/api/v2/status-pages/{}/resources",
            self.page_id
        );

        let res = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;

        let response = res.json::<Response>().await?;

        let resources = response
            .data
            .into_iter()
            .map(|resource| StatusPageResource {
                _id: resource.id,
                name: resource.attributes.public_name,
                availability: resource.attributes.availability,
                status: resource.attributes.status,
            })
            .collect();

        Ok(resources)
    }
}
