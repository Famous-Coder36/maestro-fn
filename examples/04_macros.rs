use maestro_fn::*;

fn main() {
    define!(APP_NAME, "Maestro");

    let m = map! {
        "a" => 1,
        "b" => 2,
    };

    println!("app: {APP_NAME}");
    println!("map: {:?}", m);

    let safe = try_or!(Err("error"), 42);
    println!("try_or: {}", safe);
}
