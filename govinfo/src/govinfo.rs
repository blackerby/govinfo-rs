use std::{collections::HashMap, error::Error};
use ureq::{Agent, Request};

use crate::{Collection, Collections, Params, GOVINFO_BASE_URL};

pub struct GovInfo {
    pub api_key: Option<&'static str>,
    pub agent: Agent,
}

impl GovInfo {
    pub fn new(api_key: Option<&'static str>) -> Self {
        Self {
            api_key,
            agent: Agent::new(),
        }
    }

    fn get(&self, endpoint: &str, params: Option<HashMap<String, String>>) -> Request {
        let url = format!("{GOVINFO_BASE_URL}/{}", endpoint);
        let param_pairs = if let Some(ref params) = params {
            params
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect()
        } else {
            Vec::new()
        };
        self.agent
            .get(&url)
            .set("X-Api-Key", &self.api_key.unwrap_or("DEMO_KEY"))
            .query_pairs(param_pairs)
    }

    pub fn collections(&self) -> Result<Collection, Box<dyn Error>> {
        Ok(self.get("collections", None).call()?.into_json()?)
    }

    pub fn collection(&self, collection: &str) -> Collections {
        Collections::new().collection(String::from(collection))
    }

    pub fn packages() {}
    pub fn published() {}
    pub fn related() {}
}
