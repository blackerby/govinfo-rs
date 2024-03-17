use crate::client::GovInfo;
use crate::{Container, Payload};
use crate::{GOVINFO_BASE_URL, MAX_PAGE_SIZE};
use serde::{Deserialize, Serialize};
use std::error::Error;

pub struct Collections {
    client: GovInfo,
}

impl Collections {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: GovInfo::new(String::from(api_key)),
        }
    }
    pub fn list(&self) -> Result<Container, Box<dyn Error>> {
        Ok(self
            .client
            .agent
            .get(format!("{}/collections", GOVINFO_BASE_URL).as_str())
            .set("X-Api-Key", &self.client.api_key)
            .call()?
            .into_json()?)
    }

    pub fn collection_since(
        &self,
        collection: &str,
        start_date: &str,
    ) -> Result<Payload, Box<dyn Error>> {
        Ok(self
            .client
            .agent
            .get(format!("{}/published/{}", GOVINFO_BASE_URL, start_date).as_str())
            .set("X-Api-Key", &self.client.api_key)
            .query_pairs(vec![
                ("offsetMark", "*"),
                ("pageSize", MAX_PAGE_SIZE),
                ("collection", collection.to_uppercase().as_str()),
            ])
            .call()?
            .into_json()?)
    }

    pub fn collection_between(
        &self,
        collection: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Payload, Box<dyn Error>> {
        Ok(self
            .client
            .agent
            .get(format!("{}/published/{}/{}", GOVINFO_BASE_URL, start_date, end_date).as_str())
            .set("X-Api-Key", &self.client.api_key)
            .query_pairs(vec![
                ("offsetMark", "*"),
                ("pageSize", MAX_PAGE_SIZE),
                ("collection", collection.to_uppercase().as_str()),
            ])
            .call()?
            .into_json()?)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    collection_code: String,
    collection_name: String,
    package_count: usize,
    granule_count: Option<usize>,
}
