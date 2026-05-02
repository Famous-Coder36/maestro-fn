use maestro_fn::*;

fn main() {
    let result = pipeline(10)
        .through(|x| x + 1)
        .through(|x| x * 2)
        .run();

    println!("middleware: {}", result);
}	
