use crate::Params;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Collections {
    collection: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    offset: Option<String>,
    page_size: Option<String>,
    doc_class: Option<String>,
    congress: Option<String>,
    court_type: Option<String>,
    state: Option<String>,
    topic: Option<String>,
    is_glp: Option<bool>,
    nature_suit_code: Option<String>,
    nature_suit: Option<String>,
    offset_mark: Option<String>,
}

impl Default for Collections {
    fn default() -> Self {
        Self {
            collection: None,
            start_date: None,
            end_date: None,
            offset: None,
            page_size: None,
            doc_class: None,
            congress: None,
            court_type: None,
            state: None,
            topic: None,
            is_glp: None,
            nature_suit_code: None,
            nature_suit: None,
            offset_mark: None,
        }
    }
}

impl Collections {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn since(&self, _start_date: String) {} // -> Result<Container, Box<dyn Error>> {}
}

impl Params for Collections {
    fn collection(mut self, collection: String) -> Self {
        self.collection = Some(collection);
        self
    }

    fn start_date(mut self, start_date: String) -> Self {
        self.start_date = Some(start_date);
        self
    }

    fn end_date(mut self, end_date: String) -> Self {
        self.end_date = Some(end_date);
        self
    }

    fn offset(mut self, offset: String) -> Self {
        self.offset = Some(offset);
        self
    }

    fn page_size(mut self, page_size: String) -> Self {
        self.page_size = Some(page_size);
        self
    }

    fn doc_class(mut self, doc_class: String) -> Self {
        self.doc_class = Some(doc_class);
        self
    }

    fn congress(mut self, congress: String) -> Self {
        self.congress = Some(congress);
        self
    }

    fn court_type(mut self, court_type: String) -> Self {
        self.court_type = Some(court_type);
        self
    }

    fn state(mut self, state: String) -> Self {
        self.state = Some(state);
        self
    }

    fn topic(mut self, topic: String) -> Self {
        self.topic = Some(topic);
        self
    }

    fn is_glp(mut self, is_glp: bool) -> Self {
        self.is_glp = Some(is_glp);
        self
    }

    fn nature_suit_code(mut self, nature_suit_code: String) -> Self {
        self.nature_suit_code = Some(nature_suit_code);
        self
    }

    fn nature_suit(mut self, nature_suit: String) -> Self {
        self.nature_suit = Some(nature_suit);
        self
    }

    fn offset_mark(mut self, offset_mark: String) -> Self {
        self.offset_mark = Some(offset_mark);
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    collection_code: String,
    collection_name: String,
    package_count: usize,
    granule_count: Option<usize>,
}
