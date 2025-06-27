use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AccountStore {
    pub hash: Option<String>,
    pub account_address: Option<String>,
}

impl Default for AccountStore {
    fn default() -> Self {
        Self {
            hash: None,
            account_address: None,
        }
    }
}
