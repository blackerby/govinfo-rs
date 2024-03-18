use crate::Payload;
use crate::GOVINFO_BASE_URL;
use crate::{Client, GovInfo};
use std::collections::HashMap;
use std::error::Error;

pub struct Published {
    client: GovInfo,
    endpoint: &'static str,
}

impl Published {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: GovInfo::new(String::from(api_key)),
            endpoint: "published",
        }
    }

    pub fn since(
        &self,
        collections: Vec<&str>,
        start_date: &str,
        mut params: HashMap<String, String>,
    ) -> Result<Payload, Box<dyn Error>> {
        let url = self.build_url(start_date, None);
        let collections_string = collections.join("/").to_uppercase();
        params.insert("collection".to_string(), collections_string);
        let request = self.create_request(&url, params);
        Ok(request.call()?.into_json()?)
    }

    pub fn between(
        &self,
        collections: Vec<&str>,
        start_date: &str,
        end_date: &str,
        mut params: HashMap<String, String>,
    ) -> Result<Payload, Box<dyn Error>> {
        let url = self.build_url(start_date, Some(end_date));
        let collections_string = collections.join("/").to_uppercase();
        params.insert("collection".to_string(), collections_string);
        let request = self.create_request(&url, params);
        Ok(request.call()?.into_json()?)
    }

    fn build_url(&self, start_date: &str, end_date: Option<&str>) -> String {
        if end_date.is_some() {
            format!(
                "{GOVINFO_BASE_URL}/{}/{}/{}",
                self.endpoint(),
                start_date,
                end_date.unwrap()
            )
        } else {
            format!("{GOVINFO_BASE_URL}/{}/{}", self.endpoint(), start_date,)
        }
    }
}

impl Client for Published {
    fn client(&self) -> &GovInfo {
        &self.client
    }

    fn endpoint(&self) -> &str {
        self.endpoint
    }
}
