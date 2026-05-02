use serde::de::DeserializeOwned;

pub struct Response {
    pub data: Vec<u8>,
}

impl Response {

    pub fn text(self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(String::from_utf8(self.data)?)
    }

    pub fn bytes(self) -> Vec<u8> {
        self.data
    }

    pub fn json<T: DeserializeOwned>(self) -> Result<T, Box<dyn std::error::Error>> {
        Ok(serde_json::from_slice(&self.data)?)
    }

    pub async fn save(self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        tokio::fs::write(path, &self.data).await?;
        Ok(())
    }
}