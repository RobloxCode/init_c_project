use std::env;

fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        eprintln!("missing project name");
        eprintln!("Ussage:");
        eprintln!(
            "     ./{} <destination (path)> <project name> [directories...]",
            args[0]
        );
        eprintln!();
        eprintln!("Example:");
        eprintln!("     ./you_program ../../example_file my_project utils helpers");
        eprintln!();

        return Err("missing required arguments".to_string());
    }

    Ok(())
}
