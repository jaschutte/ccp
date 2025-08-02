use std::{path::PathBuf, process::exit, str::FromStr};

fn main() {
    if std::env::args().len() <= 1 {
        eprintln!("Please provide files to copy");
        exit(1);
    }

    let paths = std::env::args()
        .skip(1) // Skip the first argument, being our path lol
        .map(|path| {
            match PathBuf::from_str(&path)
                .map_err(|_| std::io::Error::from(std::io::ErrorKind::InvalidFilename))
                .and_then(|path| path.canonicalize())
                .and_then(|path| path.try_exists().map(|b| (b, path)))
            {
                Ok((true, path)) => path.display().to_string(),
                Ok((false, _)) => {
                    eprintln!("Failed to copy file(s), file at {path} does not exist");
                    exit(1);
                }
                Err(error) => {
                    eprintln!("Failed to copy file(s), file at {path} has an IO error: {error}");
                    exit(1);
                }
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    if let Err(error) = std::fs::write("/tmp/___CCP_COPIED_FILES", paths) {
        eprintln!("Failed to copy file(s), IO error: {error}");
        exit(1);
    }
}
