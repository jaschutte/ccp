use copy_dir::copy_dir;
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
        }
    };

    for path in content.lines() {
        // We love tripple allocating strings!!
        let basename = path
            .chars()
            .rev()
            .skip_while(|c| *c == '/') // Skip any postfixed '/'
            .take_while(|c| *c != '/')
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();
        let target = format!("{}/{}", args[1], basename);

        match copy_dir(path, &target) {
            Ok(_) => (),
            Err(error) => {
                eprintln!("Failed to copy file {path}, error: {error}");
                exit(1);
            }
        }
    }
}
