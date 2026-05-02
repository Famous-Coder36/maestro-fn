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
     
     get("https://avatars.githubusercontent.com/u/208019943?v=4")
        .timeout(20)
        .retry(3)
        .progress(|p| println!("{}%", p))
        .send()
        .await?
        .save("image.png")
        .await?;
        
    Ok(())
}
