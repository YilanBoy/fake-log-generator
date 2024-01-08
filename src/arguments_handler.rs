use std::path::Path;

pub fn arguments_handler(args: Vec<String>) -> String {
    // if there are no arguments, print the help message
    if args.len() == 1 {
        println!("This is a simple TCP client that sends fake data to a TCP server.\n");
        println!("Usage: --file, -f <config file>");

        std::process::exit(1);
    }

    // if args contains --file or -f, then the next argument should be the config file
    if args.contains(&String::from("--file")) || args.contains(&String::from("-f")) {
        // get the index of the --file or -f argument
        let index = args.iter().position(|x| x == "--file" || x == "-f").unwrap();

        // get the next argument
        let filename = args.get(index + 1).unwrap();

        // check if the file exists
        if Path::new(filename).exists() {
            return filename.to_string();
        } else {
            println!("File {} does not exist", filename);
            std::process::exit(1);
        }
    } else {
        println!("This is a simple TCP client that sends fake data to a TCP server.\n");
        println!("Usage: --file, -f <config file>");

        std::process::exit(0);
    }
}

// A test for arguments_handler
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn it_should_have_a_args_after_file_flag() {
        let args: Vec<String> = vec![
            String::from("program_name"),
            String::from("--file"),
        ];

        arguments_handler(args);
    }

    #[test]
    fn it_can_return_filename_1() {
        let args: Vec<String> = vec![
            String::from("program_name"),
            String::from("-f"),
            String::from("connection_settings.toml"),
        ];

        assert_eq!(arguments_handler(args), String::from("connection_settings.toml"));
    }

    #[test]
    fn it_can_return_filename_2() {
        let args: Vec<String> = vec![
            String::from("program_name"),
            String::from("--file"),
            String::from("connection_settings.toml"),
        ];

        assert_eq!(arguments_handler(args), String::from("connection_settings.toml"));
    }
}