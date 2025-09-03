use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PubsubTopic(String);

impl From<String> for PubsubTopic {
    fn from(value: String) -> Self {
        PubsubTopic(value)
    }
}

impl From<&str> for PubsubTopic {
    fn from(value: &str) -> Self {
        PubsubTopic(value.to_string())
    }
}

impl Into<Vec<u8>> for PubsubTopic {
    fn into(self) -> Vec<u8> {
        self.0.into()
    }
}
