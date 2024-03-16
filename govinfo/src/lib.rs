pub mod client;
pub mod collection;
pub mod granule;
pub mod package;
pub mod relationship;

pub use crate::client::GovInfo;

use crate::collection::Collection;
use crate::granule::Granule;
use crate::package::Package;
use crate::relationship::Relationship;
use serde::{Deserialize, Serialize};

const GOVINFO_BASE_URL: &str = "https://api.govinfo.gov";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Container {
    Collections(Vec<Collection>),
    Packages(Vec<Package>),
    Relationships(Vec<Relationship>),
    Granules(Vec<Granule>),
}
