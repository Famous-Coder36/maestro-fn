use serde::Serialize;
use serde::de::DeserializeOwned;

pub struct Json<T> {
    value: T,
}

impl<T> Json<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn from_map(value: T) -> Self {
        Self { value }
    }
}

impl<T: Serialize> Json<T> {

    pub fn encode(&self) -> String {
        serde_json::to_string(&self.value)
            .unwrap_or_else(|_| "null".to_string())
    }

    pub fn pretty(&self) -> String {
        serde_json::to_string_pretty(&self.value)
            .unwrap_or_else(|_| "null".to_string())
    }

    pub fn minify(&self) -> String {
        self.encode() 
    }
}

pub fn json_decode<T: DeserializeOwned>(s: &str) -> Option<T> {
    serde_json::from_str(s).ok()
}

