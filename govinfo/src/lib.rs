pub mod collections;
pub mod govinfo;
pub mod packages;
pub mod related;

pub use crate::collections::{Collection, Collections};
pub use crate::govinfo::GovInfo;
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

pub trait Client: Params {
    fn get(&self, params: Option<HashMap<String, String>>) -> Request {
        let url = format!("{GOVINFO_BASE_URL}/{}", self.endpoint());
        let param_pairs = if let Some(ref params) = params {
            params
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect()
        } else {
            Vec::new()
        };
        self.agent()
            .get(&url)
            .set("X-Api-Key", &self.get_api_key())
            .query_pairs(param_pairs)
    }
}

pub enum Endpoint {
    Collections(Collections),
    Packages,
    Related,
    Published,
}

impl From<String> for Endpoint {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "collections" => Endpoint::Collections(Collections::new()),
            "packages" => Endpoint::Packages,
            "related" => Endpoint::Related,
            "published" => Endpoint::Published,
            _ => panic!("No matching endpoint"),
        }
    }
}

pub trait Params {
    fn api_key(self, api_key: String) -> Self;
    fn get_api_key(&self) -> &str;
    fn agent(&self) -> &Agent;
    fn endpoint(&self) -> &str;
    fn collection(self, collection: String) -> Self;
    fn start_date(self, start_date: String) -> Self;
    fn end_date(self, end_date: String) -> Self;
    fn offset(self, offset: String) -> Self;
    fn page_size(self, page_size: String) -> Self;
    fn doc_class(self, doc_class: String) -> Self;
    fn congress(self, congress: String) -> Self;
    fn court_type(self, court_type: String) -> Self;
    fn state(self, state: String) -> Self;
    fn topic(self, topic: String) -> Self;
    fn is_glp(self, is_glp: bool) -> Self;
    fn nature_suit_code(self, nature_suit_code: String) -> Self;
    fn nature_suit(self, nature_suirt: String) -> Self;
    fn offset_mark(self, offset_mark: String) -> Self;
}
