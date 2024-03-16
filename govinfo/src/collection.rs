use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct Collection {
    collection_code: String,
    collection_name: String,
    package_count: usize,
    granule_count: Option<usize>,
}
