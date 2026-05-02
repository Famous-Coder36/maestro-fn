use maestro_fn::*;

fn main() {
    let v = vec_from![1, 2, 3, 4];

    let first = first!(v);

    println!("vector: {:?}", v);
    println!("first: {:?}", first);
}
