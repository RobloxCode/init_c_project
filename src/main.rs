use std::env;

fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 3 {
        eprintln!("missing project name");
        eprintln!("Ussage:");
        eprintln!("     cargo run -- <destination (path)> <project name> [directories...]");
        eprintln!();
        eprintln!("Example:");
        eprintln!("     cargo run -- ../../example_file my_project utils helpers");
        eprintln!();

        return Err("missing required arguments".to_string());
    }

    let project_path = args[1].to_string();
    let project_name = args[2].to_string();
    let directories = &args[3..];

    if directories.is_empty() {
        println!("no specified directories");
    } else {
        println!("{:?}", directories);
    }

    Ok(())
}
