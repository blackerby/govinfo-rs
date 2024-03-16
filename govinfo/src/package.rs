use serde::{Deserialize, Serialize};

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
