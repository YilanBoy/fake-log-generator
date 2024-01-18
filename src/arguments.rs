use std::collections::HashMap;
use std::net::Ipv4Addr;

pub fn handler(args: Vec<String>) -> HashMap<String, String> {
    // if there are no arguments, print the help message
    if args.len() == 1 {
        println!("This is a simple TCP client that sends fake data to a TCP server.\n");
        println!("Usage:");
        println!("--host, -h <ipv4 address>. Default is 127.0.0.1");
        println!("--ports, -p <port numbers>. Default is 80");
        println!("--number, -n <number of requests>. Default is 10,000");

        std::process::exit(1);
    }

    let mut args_map: HashMap<String, String> = HashMap::from([
        (String::from("host"), String::from("127.0.0.1")),
        (String::from("ports"), String::from("80")),
        (String::from("number"), String::from("10000")),
    ]);

    // if args contains --host or -h, then the next argument should be the host
    if args.contains(&String::from("--host")) || args.contains(&String::from("-h")) {
        // get the index of the --host or -h argument
        let index = args.iter().position(|x| x == "--host" || x == "-h").unwrap();

        // get the next argument
        let host = args.get(index + 1);

        match host {
            Some(host) => {
                // check if the host is a valid IPv4 address
                match host.parse::<Ipv4Addr>() {
                    Ok(_) => {}
                    Err(_) => {
                        panic!("Host is not a valid IPv4 address");
                    }
                }

                args_map.insert(String::from("host"), host.to_string());
            }
            None => {
                panic!("Host is not a valid IPv4 address")
            }
        }
    }

    if args.contains(&String::from("--ports")) || args.contains(&String::from("-p")) {
        let index = args.iter().position(|x| x == "--ports" || x == "-p").unwrap();

        let port = args.get(index + 1);

        match port {
            Some(port) => {
                args_map.insert(String::from("ports"), port.to_string());
            }
            None => {
                panic!("Ports is not the valid port number")
            }
        }
    }

    if args.contains(&String::from("--number")) || args.contains(&String::from("-n")) {
        let index = args.iter().position(|x| x == "--number" || x == "-n").unwrap();

        let number = args.get(index + 1);

        match number {
            Some(number) => {
                args_map.insert(String::from("number"), number.to_string());
            }
            None => {}
        }
    }

    return args_map;
}

// A test for arguments_handler
#[cfg(test)]
mod tests {
    use crate::arguments::handler;

    #[test]
    #[should_panic]
    fn it_should_have_a_args_after_host_flag() {
        let args: Vec<String> = vec![
            String::from("program_itself"),
            String::from("--host"),
        ];

        handler(args);
    }

    #[test]
    #[should_panic]
    fn it_should_have_a_args_after_host_flag_2() {
        let args: Vec<String> = vec![
            String::from("program_itself"),
            String::from("-h"),
        ];

        handler(args);
    }

    #[test]
    fn it_should_return_the_host() {
        let args: Vec<String> = vec![
            String::from("program_itself"),
            String::from("--host"),
            String::from("127.0.0.1"),
        ];

        let args_map = handler(args);

        assert_eq!(args_map.get("host").unwrap(), "127.0.0.1");
    }

    #[test]
    #[should_panic]
    fn it_should_be_a_valid_ip() {
        let args: Vec<String> = vec![
            String::from("program_itself"),
            String::from("--host"),
            String::from("hello!"),
        ];

        handler(args);
    }

    #[test]
    #[should_panic]
    fn it_should_have_a_args_after_ports_flag() {
        let args: Vec<String> = vec![
            String::from("program_itself"),
            String::from("--ports"),
        ];

        handler(args);
    }

    #[test]
    #[should_panic]
    fn it_should_have_a_args_after_ports_flag_2() {
        let args: Vec<String> = vec![
            String::from("program_itself"),
            String::from("-p"),
        ];

        handler(args);
    }

    #[test]
    fn it_should_return_the_ports() {
        let args: Vec<String> = vec![
            String::from("program_itself"),
            String::from("--host"),
            String::from("127.0.0.1"),
            String::from("--ports"),
            String::from("8080,8081,8082"),
        ];

        let args_map = handler(args);

        assert_eq!(args_map.get("ports").unwrap(), "8080,8081,8082");
    }
}