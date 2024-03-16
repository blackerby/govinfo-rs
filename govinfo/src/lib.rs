#[allow(unused)]
use serde::Deserialize;
use serde::Serialize;

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
struct Collection {
    collection_code: String,
    collection_name: String,
    package_count: usize,
    granule_count: Option<usize>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
struct Package {
    package_id: String,
    last_modified: String,
    package_link: String,
    doc_class: String,
    title: String,
    congress: String,
    date_issued: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
struct Granule {
    title: String,
    granule_id: String,
    granule_class: String,
    md5: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
struct Relationship {
    relationship_link: String,
    collection: String,
    relationship: String,
}
