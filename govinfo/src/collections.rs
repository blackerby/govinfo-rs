use crate::client::GovInfo;
use crate::GOVINFO_BASE_URL;
use crate::{Container, Interval};
use serde::{Deserialize, Serialize};
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
}

impl Interval for Collections {
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
