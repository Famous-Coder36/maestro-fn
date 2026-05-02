use maestro_fn::*;

fn main() {
    let result = pipe(5)
        .then(|x| x + 1)
        .then(|x| x * 2)
        .get();

    println!("pipe result: {}", result);
}
