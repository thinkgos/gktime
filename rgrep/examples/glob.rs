fn main() {
    println!("glob: ");
    let files = glob::glob("/home/thinkgo/Pictures/*.jpg").unwrap();
    for entry in files {
        match entry {
            Ok(path) => {
                println!("\t{:?}", path.display())
            }
            Err(e) => println!("{:?}", e),
        }
    }

    println!("glob_with: ");
    let options = glob::MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };
    for entry in glob::glob_with("/usr/local/*a*", options).unwrap() {
        if let Ok(path) = entry {
            println!("\t{:?}", path.display())
        }
    }
}
