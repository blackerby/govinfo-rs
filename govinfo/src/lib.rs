pub mod collections;
pub mod packages;
pub mod published;
pub mod related;

pub use crate::collections::{Collection, Collections};
pub use crate::published::Published;
pub use crate::related::Relationship;
use std::collections::HashMap;
use ureq::{Agent, Request};

use crate::packages::{Granule, Package};
use serde::{Deserialize, Serialize};

const GOVINFO_BASE_URL: &str = "https://api.govinfo.gov";
pub const MAX_PAGE_SIZE: &str = "1000";

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

pub trait Client {
    fn client(&self) -> &GovInfo;
    fn endpoint(&self) -> &str;
    fn create_request(&self, url: &str, params: HashMap<String, String>) -> Request {
        let param_pairs: Vec<(&str, &str)> = params
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        self.client()
            .agent
            .get(url)
            .set("X-Api-Key", &self.client().api_key)
            .query_pairs(param_pairs)
    }
}

pub struct GovInfo {
    pub api_key: String,
    pub agent: Agent,
}

impl GovInfo {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            agent: Agent::new(),
        }
    }
}

impl Default for GovInfo {
    fn default() -> Self {
        Self {
            api_key: String::from("DEMO_KEY"),
            agent: Agent::new(),
        }
    }
}
