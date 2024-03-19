use std::error::Error;

use crate::{Client, Container, Endpoint, Params};

pub struct GovInfo {
    pub endpoint: Endpoint,
}

impl GovInfo {
    pub fn new(api_key: String, endpoint: String) -> Self {
        Self {
            endpoint: match Endpoint::from(endpoint) {
                Endpoint::Collections(collections) => {
                    Endpoint::Collections(collections.api_key(api_key))
                }
                _ => unimplemented!(),
            },
        }
    }

    pub fn collections(&self) -> Result<Container, Box<dyn Error>> {
        if let Endpoint::Collections(collections) = &self.endpoint {
            Ok(collections.get(None).call()?.into_json()?)
        } else {
            panic!("cannot...");
        }
    }
    pub fn packages() {}
    pub fn published() {}
    pub fn related() {}
}
