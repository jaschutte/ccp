use std::{fs::exists, process::exit};

fn main() {
    if std::env::args().len() <= 1 {
        eprintln!("Please provide files to copy");
        exit(1);
    }

    let paths = std::env::args()
        .skip(1) // Skip the first argument, being our path lol
        .inspect(|path| match exists(path) {
            Ok(true) => (),
            Ok(false) => {
                eprintln!("Failed to copy file(s), file at {path} does not exist");
                exit(1);
            }
            Err(error) => {
                eprintln!("Failed to copy file(s), file at {path} has an IO error: {error}");
                exit(1);
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    if let Err(error) = std::fs::write("/tmp/___CCP_COPIED_FILES", paths) {
        eprintln!("Failed to copy file(s), IO error: {error}");
        exit(1);
    }
}
