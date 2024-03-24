use crate::GovInfo;
use chrono::{DateTime, Utc};

pub trait Published {
    fn bill_version(self, bill_version: String) -> Self;
    fn modified_since(self, modified_since: String) -> Self;
}

impl Published for GovInfo {
    fn bill_version(mut self, bill_version: String) -> Self {
        self.params
            .insert("billVersion".to_string(), bill_version.to_string());
        self
    }

    fn modified_since(mut self, modified_since: String) -> Self {
        if modified_since.parse::<DateTime<Utc>>().is_ok() {
            self.params
                .insert("modifiedSince".to_string(), modified_since);
        }
        self
    }
}
