use crate::GovInfo;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    relationship_link: String,
    collection: String,
    relationship: String,
}

pub trait Related {
    fn access_id(self, access_id: String) -> Self;
    fn sub_granule_class(self, sub_granule_class: String) -> Self;
}

impl Related for GovInfo {
    fn access_id(mut self, access_id: String) -> Self {
        self.path_params.push(access_id);
        self
    }

    fn sub_granule_class(mut self, sub_granule_class: String) -> Self {
        self.params
            .insert("subGranuleClass".to_string(), sub_granule_class);
        self
    }
}
