pub mod collection;
pub mod error;
pub mod packages;
pub mod published;
pub mod related;

use std::collections::HashMap;
use std::fmt::Display;

pub use crate::collection::Collection;
pub use crate::error::Error;
pub use crate::packages::Packages;
pub use crate::published::Published;
pub use crate::related::Related;

use chrono::{DateTime, Datelike, NaiveDate, Utc};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

const GOVINFO_BASE_URL: &str = "https://api.govinfo.gov";
const MAX_PAGE_SIZE: u16 = 1000;
const DEFAULT_OFFSET_MARK: &str = "*";
const FIRST_CONGRESS_YEAR: usize = 1789;

type Result<T> = std::result::Result<T, Error>;

pub struct GovInfo {
    pub endpoint: Endpoint,
    pub path_params: Vec<String>,
    pub api_key: Option<&'static str>,
    pub client: Client,
    pub params: HashMap<String, String>,
    pub next_page: Option<String>,
    pub elements: <Vec<Element> as IntoIterator>::IntoIter,
}

impl GovInfo {
    pub fn new(api_key: Option<&'static str>) -> Self {
        let mut client = Self::default();
        client.api_key = api_key;
        client
            .params
            .insert("pageSize".to_string(), MAX_PAGE_SIZE.to_string());
        client
            .params
            .insert("offsetMark".to_string(), DEFAULT_OFFSET_MARK.to_string());

        client
    }

    fn url(&self) -> String {
        let mut path = self.endpoint.to_string();
        if self.path_params.len() > 0 {
            path.push_str(&format!("/{}", self.path_params.join("/")));
        }
        format!("{GOVINFO_BASE_URL}/{}", path)
    }

    pub fn get(mut self) -> Result<Self> {
        let url = self.url();
        let request = self
            .client
            .get(&url)
            .header("X-Api-Key", self.api_key.unwrap_or("DEMO_KEY"))
            .query(&self.params);

        let response = request.send()?.json::<GovInfoResponse>()?;

        match response {
            GovInfoResponse::Paginated {
                next_page, payload, ..
            } => {
                self.elements = payload.into_iter();
                self.next_page = next_page;
            }
            GovInfoResponse::Collections(collections) => {
                self.elements = collections.into_iter();
            }
            GovInfoResponse::Related { payload, .. } => self.elements = payload.into_iter(),
        }

        Ok(self)
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
        self.endpoint = Endpoint::Published;
        self
    }

    pub fn related(mut self) -> GovInfo {
        self.endpoint = Endpoint::Related;
        self
    }

    fn validate_congress(congress: usize) -> Result<usize> {
        let max_congress = (Utc::now().year() as usize - FIRST_CONGRESS_YEAR).div_ceil(2);
        if congress <= max_congress && congress > 0 {
            Ok(congress)
        } else {
            Err(Error::InvalidCongressParam(congress))
        }
    }

    fn try_next(&mut self) -> Result<Option<Element>> {
        if let Some(elem) = self.elements.next() {
            return Ok(Some(elem));
        }

        if self.next_page.is_none() {
            return Ok(None);
        }

        let response = self
            .client
            .get(self.next_page.as_ref().unwrap())
            .header("X-Api-Key", self.api_key.unwrap_or("DEMO_KEY"))
            .send()?
            .json::<GovInfoResponse>()?;

        match response {
            GovInfoResponse::Paginated {
                next_page, payload, ..
            } => {
                self.elements = payload.into_iter();
                self.next_page = next_page;
            }
            GovInfoResponse::Collections(collections) => self.elements = collections.into_iter(),
            GovInfoResponse::Related { payload, .. } => self.elements = payload.into_iter(),
        }
        Ok(self.elements.next())
    }
}

impl Iterator for GovInfo {
    type Item = Result<Element>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(elem)) => Some(Ok(elem)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

impl Default for GovInfo {
    fn default() -> Self {
        Self {
            endpoint: Endpoint::Collections,
            path_params: Vec::new(),
            api_key: None,
            client: Client::new(),
            params: HashMap::new(),
            next_page: None,
            elements: Vec::new().into_iter(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum GovInfoResponse {
    #[serde(rename = "collections")]
    Collections(Vec<Element>),
    #[serde(rename_all = "camelCase", untagged)]
    Related {
        #[serde(alias = "relationships", alias = "results")]
        payload: Vec<Element>,
        related_id: String,
    },
    #[serde(rename_all = "camelCase", untagged)]
    Paginated {
        count: usize,
        message: Option<String>,
        next_page: Option<String>,
        previous_page: Option<String>,
        #[serde(alias = "granules", alias = "packages")]
        payload: Vec<Element>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Element {
    #[serde(rename_all = "camelCase")]
    Collection {
        collection_code: String,
        collection_name: String,
        package_count: usize,
        granule_count: Option<usize>,
    },
    #[serde(rename_all = "camelCase")]
    Package {
        package_id: String,
        last_modified: String,
        package_link: String,
        doc_class: String,
        title: String,
        congress: String,
        date_issued: String,
    },
    #[serde(rename_all = "camelCase")]
    Granule {
        title: String,
        granule_id: String,
        granule_class: String,
        md5: String,
    },
    #[serde(rename_all = "camelCase")]
    Relationship {
        relationship_link: String,
        collection: String,
        relationship: String,
    },
    #[serde(rename_all = "camelCase")]
    Result {
        date_issued: String,
        bill_version: String,
        package_id: String,
        package_link: String,
        bill_version_label: String,
        last_modified: String,
    },
}

pub trait Params {
    fn collection(self, collection: String) -> Result<Self>
    where
        Self: Sized;
    fn start_date(self, start_date: String) -> Result<Self>
    where
        Self: Sized;
    fn end_date(self, end_date: String) -> Result<Self>
    where
        Self: Sized;
    fn page_size(self, page_size: u16) -> Self;
    fn doc_class(self, doc_class: String) -> Self;
    fn congress(self, congress: usize) -> Result<Self>
    where
        Self: Sized;
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

impl TryFrom<&str> for Endpoint {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "collections" => Ok(Endpoint::Collections),
            "packages" => Ok(Endpoint::Packages),
            "published" => Ok(Endpoint::Published),
            "related" => Ok(Endpoint::Related),
            other => Err(Error::UnsupportedEndpoint(other.to_string())),
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
    fn collection(mut self, collection: String) -> Result<Self> {
        let collection = Collection::try_from(collection.as_str())?;
        match self.endpoint {
            Endpoint::Collections | Endpoint::Related => {
                self.path_params.push(collection.to_string());
            }
            Endpoint::Published => {
                self.params
                    .insert("collection".to_string(), collection.to_string());
            }
            endpoint => return Err(Error::InvalidPathParam(endpoint.to_string())),
        }
        Ok(self)
    }

    fn start_date(mut self, start_date: String) -> Result<Self> {
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
            endpoint => return Err(Error::InvalidPathParam(endpoint.to_string())),
        }
        Ok(self)
    }

    fn end_date(mut self, end_date: String) -> Result<Self> {
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
            endpoint => return Err(Error::InvalidPathParam(endpoint.to_string())),
        }
        Ok(self)
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

    fn congress(mut self, congress: usize) -> Result<Self> {
        self.params.insert(
            "congress".to_string(),
            Self::validate_congress(congress)?.to_string(),
        );
        Ok(self)
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
            .unwrap()
            .url();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_valid_congress() {
        let congress = 118;
        let result = GovInfo::validate_congress(congress);
        assert!(result.is_ok());
        assert!(result.unwrap() == 118);
    }

    #[test]
    fn test_congress_too_low() {
        let congress = 0;
        let result = GovInfo::validate_congress(congress);
        assert!(result.is_err());
    }

    #[test]
    fn test_congress_too_high() {
        let congress = 119;
        let result = GovInfo::validate_congress(congress);
        assert!(result.is_err());
    }
}
