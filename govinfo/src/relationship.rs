use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
struct Relationship {
    relationship_link: String,
    collection: String,
    relationship: String,
}
