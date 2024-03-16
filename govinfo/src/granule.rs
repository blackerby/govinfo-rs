use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Granule {
    title: String,
    granule_id: String,
    granule_class: String,
    md5: String,
}
