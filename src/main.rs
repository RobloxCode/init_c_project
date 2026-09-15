use std::{env, fs};

#[warn(unused)]
enum CInitializerError {
    InvalidArguments,
    Io(std::io::Error),
}

fn main() -> std::io::Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 3 {
        eprintln!("missing project name");
        eprintln!("Ussage:");
        eprintln!("     cargo run -- <destination (path)> <project name> [directories...]");
        eprintln!();
        eprintln!("Example:");
        eprintln!("     cargo run -- ../../example_file my_project utils helpers");
        eprintln!();

        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "missing required arguments",
        ));
    }

    let project_path = args[1].to_string();
    let project_name = args[2].to_string();
    let directories = &args[3..];

    let mut final_path = String::new();
    final_path.push_str(&project_path);
    final_path.push_str(&project_name);

    // TODO: create the directories and the src, build and include directories

    fs::create_dir(&final_path)?;

    let mut src_path = String::new();
    src_path.push_str(&final_path);
    src_path.push_str("/src");
    fs::create_dir(src_path)?;

    // for dir in directories {
    //     fs::create_dir(final_path);
    // }

    Ok(())
}
