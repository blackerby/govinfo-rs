use ureq::Agent;

pub struct GovInfo {
    pub api_key: String,
    pub agent: Agent,
}

impl GovInfo {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            agent: Agent::new(),
        }
    }
}

impl Default for GovInfo {
    fn default() -> Self {
        Self {
            api_key: String::from("DEMO_KEY"),
            agent: Agent::new(),
        }
    }
}
