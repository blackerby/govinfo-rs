pub mod collections;
pub mod govinfo;
pub mod packages;
pub mod published;
pub mod related;

use std::fmt::Display;

pub use crate::collections::Collection;
pub use crate::govinfo::GovInfo;
pub use crate::published::Published;
pub use crate::related::{Related, Relationship};

pub use crate::packages::{Granule, Package, Packages};
use serde::{Deserialize, Serialize};

const GOVINFO_BASE_URL: &str = "https://api.govinfo.gov";
const MAX_PAGE_SIZE: &str = "1000";
const DEFAULT_OFFSET_MARK: &str = "*";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum GovInfoResponse {
    #[serde(untagged)]
    Payload(Payload),
    #[serde(untagged)]
    Container(Container),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    count: usize,
    message: Option<String>,
    next_page: Option<String>,
    previous_page: Option<String>,
    #[serde(alias = "packages", alias = "collections", alias = "granules")]
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

pub trait Params {
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

pub enum Endpoint {
    Collections,
    Packages,
    Published,
    Related,
}

impl From<&str> for Endpoint {
    fn from(value: &str) -> Self {
        match value {
            "collections" => Endpoint::Collections,
            "packages" => Endpoint::Packages,
            "published" => Endpoint::Published,
            "related" => Endpoint::Related,
            _ => panic!("unsupported endpoint"),
        }
    }
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Endpoint::Collections => write!(f, "collections"),
            Endpoint::Packages => write!(f, "packages"),
            Endpoint::Published => write!(f, "published"),
            Endpoint::Related => write!(f, "related"),
        }
    }
}
