use maestro_fn::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let html = get("https://example.com")
        .send()
        .await?
        .bytes();

    File::open("site.html")
        .put(html)
        .await?;

    Ok(())
}
