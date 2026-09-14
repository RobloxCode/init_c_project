use std::env;

fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        eprintln!("missing project name");
        eprintln!("Ussage:");
        eprintln!("     cargo run -- <destination (path)> <project name> [directories...]");
        eprintln!();
        eprintln!("Example:");
        eprintln!("     cargo run -- ../../example_file my_project utils helpers");
        eprintln!();

        return Err("missing required arguments".to_string());
    }

    println!("{:?}", args);

    Ok(())
}
