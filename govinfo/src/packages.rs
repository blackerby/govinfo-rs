use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    package_id: String,
    last_modified: String,
    package_link: String,
    doc_class: String,
    title: String,
    congress: String,
    date_issued: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Granule {
    title: String,
    granule_id: String,
    granule_class: String,
    md5: String,
}
