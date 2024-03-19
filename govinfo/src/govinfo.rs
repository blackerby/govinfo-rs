use std::error::Error;

use crate::{Client, Collections, Container};

pub struct GovInfo {
    pub api_key: Option<String>,
}

impl GovInfo {
    pub fn new(api_key: Option<String>) -> Self {
        Self { api_key }
    }

    pub fn collections(&self) -> Result<Container, Box<dyn Error>> {
        let collections = Collections::new();
        Ok(collections.get(None).call()?.into_json()?)
    }
    pub fn packages() {}
    pub fn published() {}
    pub fn related() {}
}
