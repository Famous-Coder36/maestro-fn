use maestro_fn::*;
use serde_json::json;

fn main() {
    let data = json!({
        "name": "Otabek",
        "age": 20
    });

    let j = Json::new(data);

    println!("pretty:\n{}", j.pretty());
    println!("minify: {}", j.minify());
}
