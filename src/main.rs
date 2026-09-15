use std::path::PathBuf;
use std::{env, fs};

#[warn(unused)]
enum CInitializerError {
    InvalidArguments,
    Io(std::io::Error),
    FileAlreadyExists,
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

    let project_path = PathBuf::from(&args[1]);
    let project_name = &args[2];
    let directories = &args[3..];

    let final_path = project_path.join(project_name);

    fs::create_dir(&final_path)?;

    fs::create_dir(final_path.join("src"))?;
    fs::create_dir(final_path.join("build"))?;
    fs::create_dir(final_path.join("include"))?;

    for directory in directories {
        let directory_path = final_path
            .join("include")
            .join(project_name)
            .join(directory);

        fs::create_dir_all(directory_path)?;
    }

    Ok(())
}
