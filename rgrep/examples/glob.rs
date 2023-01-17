use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("glob: ");

    let files = glob::glob("/bin/*.py")?;
    for entry in files {
        match entry {
            Ok(path) => {
                println!("\t{:?}", path.display())
            }
            Err(e) => println!("{:?}", e),
        }
    }

    println!("glob_with: ");

    let files = glob::glob_with(
        "/usr/local/*a*",
        glob::MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        },
    )?;

    for entry in files {
        if let Ok(path) = entry {
            println!("\t{:?}", path.display())
        }
    }

    Ok(())
}
