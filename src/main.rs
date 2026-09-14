use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    for (i, a) in args.iter().enumerate() {
        println!("i: {}, arg: {}", i, a);
    }
}
