use std::collections::BTreeMap;
use std::sync::Mutex;

pub struct AgentMarketplace {
    listings: Mutex<BTreeMap<String, String>>, // name -> code
}

impl AgentMarketplace {
    pub fn new() -> Self {
        Self {
            listings: Mutex::new(BTreeMap::new()),
        }
    }
    pub fn publish(&self, name: String, code: String) {
        self.listings.lock().unwrap().insert(name, code);
    }
    pub fn list(&self) -> Vec<String> {
        self.listings.lock().unwrap().keys().cloned().collect()
    }
}
