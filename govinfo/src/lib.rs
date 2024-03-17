pub mod client;
pub mod collections;
pub mod packages;
pub mod published;
pub mod related;

pub use crate::client::GovInfo;
pub use crate::collections::{Collection, Collections};
pub use crate::related::Relationship;

use crate::packages::{Granule, Package};
use serde::{Deserialize, Serialize};
use std::error::Error;

const GOVINFO_BASE_URL: &str = "https://api.govinfo.gov";
const MAX_PAGE_SIZE: &str = "1000";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    count: usize,
    message: Option<String>,
    next_page: Option<String>,
    previous_page: Option<String>,
    #[serde(
        alias = "packages",
        alias = "collections",
        alias = "packages",
        alias = "granules"
    )]
    #[serde(flatten)]
    container: Container,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Container {
    Collections(Vec<Collection>),
    Packages(Vec<Package>),
    Relationships(Vec<Relationship>),
    Granules(Vec<Granule>),
}

pub trait Interval {
    fn client(&self) -> &GovInfo;
    fn endpoint(&self) -> &str;
    fn since(&self, collection: &str, start_date: &str) -> Result<Payload, Box<dyn Error>> {
        Ok(self
            .client()
            .agent
            .get(
                format!(
                    "{GOVINFO_BASE_URL}/{}/{}/{start_date}",
                    self.endpoint(),
                    collection.to_uppercase()
                )
                .as_str(),
            )
            .set("X-Api-Key", &self.client().api_key)
            .query_pairs(vec![("offsetMark", "*"), ("pageSize", MAX_PAGE_SIZE)])
            .call()?
            .into_json()?)
    }

    fn between(
        &self,
        collection: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<Payload, Box<dyn Error>> {
        Ok(self
            .client()
            .agent
            .get(
                format!(
                    "{GOVINFO_BASE_URL}/{}/{}/{start_date}/{end_date}",
                    self.endpoint(),
                    collection.to_uppercase()
                )
                .as_str(),
            )
            .set("X-Api-Key", &self.client().api_key)
            .query_pairs(vec![("offsetMark", "*"), ("pageSize", MAX_PAGE_SIZE)])
            .call()?
            .into_json()?)
    }
}
