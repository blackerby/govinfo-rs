use crate::{Container, GOVINFO_BASE_URL};
use std::error::Error;

pub struct GovInfo {
    api_key: String,
}

impl GovInfo {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub fn collections(&self) -> Result<Container, Box<dyn Error>> {
        Ok(
            ureq::get(format!("{}/collections", GOVINFO_BASE_URL).as_str())
                .set("X-Api-Key", &self.api_key)
                .call()?
                .into_json()?,
        )
    }
}

impl Default for GovInfo {
    fn default() -> Self {
        Self {
            api_key: String::from("DEMO_KEY"),
        }
    }
}
