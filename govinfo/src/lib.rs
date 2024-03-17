pub mod client;
pub mod collections;
pub mod packages;
pub mod relationship;

pub use crate::client::GovInfo;
pub use crate::collections::{Collection, Collections};

use crate::packages::{Granule, Package};
use crate::relationship::Relationship;
use serde::{Deserialize, Serialize};

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
