use core::panic;
use std::{collections::HashMap, error::Error};
use ureq::Agent;

use crate::{Endpoint, Params};
use crate::{GovInfoResponse, DEFAULT_OFFSET_MARK, GOVINFO_BASE_URL, MAX_PAGE_SIZE};

pub struct GovInfo {
    pub endpoint: Endpoint,
    pub path_params: Vec<String>,
    pub api_key: Option<&'static str>,
    pub agent: Agent,
    pub params: HashMap<String, String>,
}

impl GovInfo {
    pub fn new(api_key: Option<&'static str>) -> Self {
        Self {
            endpoint: Endpoint::Collections,
            path_params: Vec::new(),
            api_key,
            agent: Agent::new(),
            params: HashMap::new(),
        }
    }

    // TODO: give this a better name
    fn url(&self) -> String {
        let mut path = self.endpoint.to_string();
        if self.path_params.len() > 0 {
            path.push_str(&format!("/{}", self.path_params.join("/")));
        }
        format!("{GOVINFO_BASE_URL}/{}", path)
    }

    pub fn get(&self) -> Result<GovInfoResponse, Box<dyn Error>> {
        let url = self.url();
        let param_pairs: Vec<(&str, &str)> = self
            .params
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        Ok(self
            .agent
            .get(&url)
            .set("X-Api-Key", &self.api_key.unwrap_or("DEMO_KEY"))
            .query_pairs(param_pairs)
            .call()?
            .into_json()?)
    }

    pub fn collections(mut self) -> GovInfo {
        self.endpoint = Endpoint::Collections;
        self
    }

    pub fn packages(mut self) -> GovInfo {
        self.endpoint = Endpoint::Packages;
        self
    }

    pub fn published(mut self) -> GovInfo {
        self.params
            .insert("pageSize".to_string(), MAX_PAGE_SIZE.to_string());
        self.params
            .insert("offsetMark".to_string(), DEFAULT_OFFSET_MARK.to_string());
        self.endpoint = Endpoint::Published;
        self
    }

    pub fn related(mut self) -> GovInfo {
        self.endpoint = Endpoint::Related;
        self
    }
}

impl Params for GovInfo {
    fn collection(mut self, collection: String) -> Self {
        match self.endpoint {
            Endpoint::Collections | Endpoint::Related => {
                self.path_params.push(collection.to_uppercase());
            }
            Endpoint::Published => {
                self.params
                    .insert("collection".to_string(), collection.to_string());
            }
            _ => panic!("You shouldn't be able to get here"),
        }
        self
    }

    fn start_date(mut self, start_date: String) -> Self {
        self.path_params.push(start_date);
        self
    }

    fn end_date(mut self, end_date: String) -> Self {
        self.path_params.push(end_date);
        self
    }

    fn offset(mut self, offset: String) -> Self {
        self.params.insert("offset".to_string(), offset.to_string());
        self
    }

    fn page_size(mut self, page_size: String) -> Self {
        self.params
            .insert("pageSize".to_string(), page_size.to_string());
        self
    }

    fn doc_class(mut self, doc_class: String) -> Self {
        self.params
            .insert("docClass".to_string(), doc_class.to_string());
        self
    }

    fn congress(mut self, congress: String) -> Self {
        self.params
            .insert("congress".to_string(), congress.to_string());
        self
    }

    fn court_type(mut self, court_type: String) -> Self {
        self.params
            .insert("courtType".to_string(), court_type.to_string());
        self
    }

    fn state(mut self, state: String) -> Self {
        self.params.insert("state".to_string(), state.to_string());
        self
    }

    fn topic(mut self, topic: String) -> Self {
        self.params.insert("topic".to_string(), topic.to_string());
        self
    }

    fn is_glp(mut self, is_glp: bool) -> Self {
        self.params.insert("isGlp".to_string(), is_glp.to_string());
        self
    }

    fn nature_suit_code(mut self, nature_suit_code: String) -> Self {
        self.params
            .insert("natureSuitCode".to_string(), nature_suit_code.to_string());
        self
    }

    fn nature_suit(mut self, nature_suit: String) -> Self {
        self.params
            .insert("natureSuit".to_string(), nature_suit.to_string());
        self
    }

    fn offset_mark(mut self, offset_mark: String) -> Self {
        self.params
            .insert("offsetMark".to_string(), offset_mark.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::Params;

    use super::GovInfo;

    #[test]
    fn test_url_new_client() {
        let expected = "https://api.govinfo.gov/collections";
        let result = &GovInfo::new(None).url();
        assert_eq!(expected, result)
    }

    #[test]
    fn test_non_default_endpoint() {
        let expected = "https://api.govinfo.gov/published";
        let result = &GovInfo::new(None).published().url();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_one_path_param() {
        let expected = "https://api.govinfo.gov/published/2024-03-23";
        let result = &GovInfo::new(None)
            .published()
            .start_date("2024-03-23".to_string())
            .url();
        assert_eq!(expected, result);
    }
}
