use crate::{Container, Payload, GOVINFO_BASE_URL, MAX_PAGE_SIZE};
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

    pub fn published_since(
        &self,
        start_date: &str,
        collection: &str,
    ) -> Result<Payload, Box<dyn Error>> {
        Ok(
            ureq::get(format!("{}/published/{}", GOVINFO_BASE_URL, start_date).as_str())
                .set("X-Api-Key", &self.api_key)
                .query_pairs(vec![
                    ("offsetMark", "*"),
                    ("pageSize", MAX_PAGE_SIZE),
                    ("collection", collection.to_uppercase().as_str()),
                ])
                .call()?
                .into_json()?,
        )
    }

    pub fn published_between(
        &self,
        start_date: &str,
        end_date: &str,
        collection: &str,
    ) -> Result<Payload, Box<dyn Error>> {
        Ok(ureq::get(
            format!("{}/published/{}/{}", GOVINFO_BASE_URL, start_date, end_date).as_str(),
        )
        .set("X-Api-Key", &self.api_key)
        .query_pairs(vec![
            ("offsetMark", "*"),
            ("pageSize", MAX_PAGE_SIZE),
            ("collection", collection.to_uppercase().as_str()),
        ])
        .call()?
        .into_json()?)
    }
}

impl Default for GovInfo {
    fn default() -> Self {
        Self {
            api_key: String::from("DEMO_KEY"),
        }
    }
}
