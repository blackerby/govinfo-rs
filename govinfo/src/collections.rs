use crate::GOVINFO_BASE_URL;
use crate::{Client, GovInfo};
use crate::{Container, Payload};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

pub struct Collections {
    client: GovInfo,
    endpoint: &'static str,
}

impl Collections {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: GovInfo::new(String::from(api_key)),
            endpoint: "collections",
        }
    }

    pub fn all(&self) -> Result<Container, Box<dyn Error>> {
        Ok(self
            .client
            .agent
            .get(format!("{GOVINFO_BASE_URL}/{}", self.endpoint).as_str())
            .set("X-Api-Key", &self.client.api_key)
            .call()?
            .into_json()?)
    }

    pub fn since(
        &self,
        collection: &str,
        start_date: &str,
        params: HashMap<&str, &str>,
    ) -> Result<Payload, Box<dyn Error>> {
        let url = self.build_url(collection, start_date, None);
        let request = self.create_request(&url, params);
        Ok(request.call()?.into_json()?)
    }

    pub fn between(
        &self,
        collection: &str,
        start_date: &str,
        end_date: &str,
        params: HashMap<&str, &str>,
    ) -> Result<Payload, Box<dyn Error>> {
        let url = self.build_url(collection, start_date, Some(end_date));
        let request = self.create_request(&url, params);
        Ok(request.call()?.into_json()?)
    }

    fn build_url(&self, collection: &str, start_date: &str, end_date: Option<&str>) -> String {
        format!(
            "{GOVINFO_BASE_URL}/{}/{}/{}/{}",
            self.endpoint(),
            collection.to_uppercase(),
            start_date,
            end_date.unwrap_or_default()
        );
        String::new()
    }
}

impl Client for Collections {
    fn client(&self) -> &GovInfo {
        &self.client
    }

    fn endpoint(&self) -> &str {
        self.endpoint
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
