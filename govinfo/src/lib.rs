// TODO: lots of string allocation happening. investigate using how to use string literals instead.

pub mod packages;
pub mod published;
pub mod related;

use core::panic;
use std::fmt::Display;
use std::{collections::HashMap, error::Error};

pub use crate::packages::{Granule, Package, Packages};
pub use crate::published::Published;
pub use crate::related::{Related, Relationship};

use chrono::{DateTime, NaiveDate, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const GOVINFO_BASE_URL: &str = "https://api.govinfo.gov";
const MAX_PAGE_SIZE: u16 = 1000;
const DEFAULT_OFFSET_MARK: &str = "*";

pub struct GovInfo {
    pub endpoint: Endpoint,
    pub path_params: Vec<String>,
    pub api_key: Option<&'static str>,
    pub client: Client,
    pub params: HashMap<String, String>,
}

impl GovInfo {
    pub fn new(api_key: Option<&'static str>) -> Self {
        Self {
            endpoint: Endpoint::Collections,
            path_params: Vec::new(),
            api_key,
            client: Client::new(),
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

    pub async fn get(&self) -> Result<GovInfoResponse, Box<dyn Error>> {
        let url = self.url();
        Ok(self
            .client
            .get(&url)
            .header("X-Api-Key", self.api_key.unwrap_or("DEMO_KEY"))
            .query(&self.params)
            .send()
            .await?
            .json::<GovInfoResponse>()
            .await?)
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum GovInfoResponse {
    #[serde(untagged)]
    Payload(Payload),
    #[serde(untagged)]
    Container(Container),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    count: usize,
    message: Option<String>,
    next_page: Option<String>,
    previous_page: Option<String>,
    #[serde(alias = "packages", alias = "collections", alias = "granules")]
    #[serde(flatten)]
    container: Container,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Container {
    Collections(Vec<Collection>),
    Packages(Vec<Package>),
    Relationships(Vec<Relationship>),
    Granules(Vec<Granule>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    collection_code: String,
    collection_name: String,
    package_count: usize,
    granule_count: Option<usize>,
}

// TODO: research which params can be encoded as enums
pub trait Params {
    fn collection(self, collection: String) -> Self;
    fn start_date(self, start_date: String) -> Self;
    fn end_date(self, end_date: String) -> Self;
    fn page_size(self, page_size: u16) -> Self;
    fn doc_class(self, doc_class: String) -> Self;
    fn congress(self, congress: String) -> Self;
    fn court_type(self, court_type: String) -> Self;
    fn state(self, state: String) -> Self;
    fn topic(self, topic: String) -> Self;
    fn is_glp(self, is_glp: bool) -> Self;
    fn nature_suit_code(self, nature_suit_code: String) -> Self;
    fn nature_suit(self, nature_suirt: String) -> Self;
    fn offset_mark(self, offset_mark: String) -> Self;
}

pub enum Endpoint {
    Collections,
    Packages,
    Published,
    Related,
}

impl From<&str> for Endpoint {
    fn from(value: &str) -> Self {
        match value {
            "collections" => Endpoint::Collections,
            "packages" => Endpoint::Packages,
            "published" => Endpoint::Published,
            "related" => Endpoint::Related,
            _ => panic!("unsupported endpoint"),
        }
    }
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Endpoint::Collections => write!(f, "collections"),
            Endpoint::Packages => write!(f, "packages"),
            Endpoint::Published => write!(f, "published"),
            Endpoint::Related => write!(f, "related"),
        }
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
        match self.endpoint {
            Endpoint::Collections => {
                if start_date.parse::<DateTime<Utc>>().is_ok() {
                    self.path_params.push(start_date);
                }
            }
            Endpoint::Published => {
                if start_date.parse::<NaiveDate>().is_ok() {
                    self.path_params.push(start_date);
                }
            }
            _ => panic!("not a valid thingy"),
        }
        self
    }

    fn end_date(mut self, end_date: String) -> Self {
        match self.endpoint {
            Endpoint::Collections => {
                if end_date.parse::<DateTime<Utc>>().is_ok() {
                    self.path_params.push(end_date);
                }
            }
            Endpoint::Published => {
                if end_date.parse::<NaiveDate>().is_ok() {
                    self.path_params.push(end_date);
                }
            }
            _ => panic!("not a valid thingy"),
        }
        self
    }

    fn page_size(mut self, page_size: u16) -> Self {
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
