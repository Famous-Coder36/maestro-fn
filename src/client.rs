use reqwest::Client;
use std::sync::OnceLock;
use std::time::Duration;

static CLIENT: OnceLock<Client> = OnceLock::new();

pub fn client() -> &'static Client {
    CLIENT.get_or_init(|| {
        Client::builder()
            .user_agent("maestro-fn/1.0")
            .cookie_store(true)
            .gzip(true)
            .brotli(true)
            .deflate(true)
            .timeout(Duration::from_secs(30))
            .build()
            .expect("client build failed")
    })
}