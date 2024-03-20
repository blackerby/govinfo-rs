use crate::{Collections, Params};

pub struct GovInfo {
    pub api_key: Option<String>,
}

impl GovInfo {
    pub fn new(api_key: Option<String>) -> Self {
        Self { api_key }
    }

    pub fn collections(&self) -> Collections {
        Collections::new()
    }

    pub fn collection(&self, collection: String) -> Collections {
        Collections::new().collection(collection)
    }

    pub fn packages() {}
    pub fn published() {}
    pub fn related() {}
}
