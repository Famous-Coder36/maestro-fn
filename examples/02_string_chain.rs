use maestro_fn::*;

fn main() {
    let result = str(" a,b,c ")
        .trim()
        .explode(",")
        .implode("-");

    println!("string: {}", result);
}
