use maestro_fn::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let x = pipe(10)
        .then(|v| v * 2)
        .get();

    println!("pipe: {}", x);

    let s = str("1,2,3")
        .explode(",")
        .map(|x| format!("num:{x}"))
        .implode(" | ");

    println!("string: {}", s);

    let m = map! {
        "a" => 1,
        "b" => 2,
    };

    foreach(m.clone(), |(k, v)| {
        println!("{k} => {v}");
    });

    let user = User {
        name: "Otabek".to_string(),
        age: 20,
    };

    let res = response!(200, user);
    println!("{}", res.pretty());

    Time::sleep_sec(1).await;

    println!("done");

    Ok(())
}	
