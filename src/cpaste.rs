use std::process::exit;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("You must provide precisely one argument, the path to copy the files to");
        exit(1);
    }

    let content = match std::fs::read_to_string("/tmp/___CCP_COPIED_FILES") {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Failed to retrieve copied files, error: {error}");
            exit(1);
        },
    };

    for path in content.lines() {
        match std::fs::copy(path, &args[1]) {
            Ok(_) => (),
            Err(error) => {
                eprintln!("Failed to copy file {path}, error: {error}");
                exit(1);
            },
        }
    }
}

