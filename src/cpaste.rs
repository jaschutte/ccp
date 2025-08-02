use copy_dir::copy_dir;
use std::process::exit;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let arg_target = match args.len() {
        1 => String::from("."),
        2 => args[1].clone(),
        _ => {
            eprintln!(
                "You must provide one path to copy the files to (if none given, the current directory will be used)"
            );
            exit(1);
        }
    };

    let content = match std::fs::read_to_string("/tmp/___CCP_COPIED_FILES") {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Failed to retrieve copied files, error: {error}");
            exit(1);
        }
    };

    for path in content.lines() {
        let is_directory = match std::fs::metadata(&arg_target) {
            Ok(meta) => meta.is_dir(),
            Err(_) => false,
        };

        let target = if is_directory {
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
            &format!("{}/{}", arg_target, basename)
        } else {
            &arg_target
        };

        match copy_dir(path, target) {
            Ok(_) => (),
            Err(error) => {
                eprintln!("Failed to copy file {path}, error: {error}");
                exit(1);
            }
        }
    }
}
