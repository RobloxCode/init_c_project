use std::fs::File;
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

    if final_path.is_dir() {
        eprintln!("directory already exists");

        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "directory already exists",
        ));
    }

    fs::create_dir(&final_path)?;

    fs::create_dir(final_path.join("src"))?;
    fs::create_dir(final_path.join("build"))?;
    fs::create_dir(final_path.join("include"))?;

    create_include_dir(directories, &final_path, project_name)?;

    Ok(())
}

fn create_include_dir(
    directories: &[String],
    final_path: &PathBuf,
    project_name: &str,
) -> std::io::Result<()> {
    for directory in directories {
        let directory_path = final_path
            .join("include")
            .join(project_name)
            .join(directory);

        fs::create_dir_all(directory_path)?;
    }

    for directory in directories {
        let mut name = String::from(directory);
        name.push_str(".h");

        let directory_path = final_path
            .join("include")
            .join(project_name)
            .join(directory)
            .join(name);

        File::create(directory_path)?;
    }

    Ok(())
}
