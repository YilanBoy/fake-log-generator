use std::net::Ipv4Addr;

pub fn print_help_message() {
    println!("This is a simple TCP client that sends fake data to a TCP server.\n");
    println!("Usage:");
    println!("--host, -h <ipv4 address>. Required.");
    println!("--ports, -p <port numbers>. Default is 80.");
    println!("--number, -n <number of requests>. Default is 10000.");
}

pub fn handler(args: Vec<String>) -> (Ipv4Addr, Vec<u16>, usize) {
    let mut ip4v_address = Ipv4Addr::new(0, 0, 0, 0);
    let mut target_ports = vec![80];
    let mut number_of_requests: usize = 10000;

    // if there are no arguments, print the help message
    if args.len() == 1 {
        print_help_message();

        std::process::exit(1);
    }

    // remove the first argument, which is the program itself
    let args: Vec<String> = args[1..].to_vec();

    // for loop all the arguments
    for (i, arg) in args.iter().enumerate() {
        if arg == "--host" || arg == "-h" {
            let host = args.get(i + 1);

            let host = match host {
                Some(host) => host,
                None => {
                    print_help_message();

                    panic!("There is no host after the --host flag");
                }
            };

            match host.parse::<Ipv4Addr>() {
                Ok(_) => {
                    ip4v_address = host.parse().unwrap();
                }
                Err(_) => {
                    print_help_message();

                    panic!("Host is not a valid IPv4 address");
                }
            }
        }

        if arg == "--ports" || arg == "-p" {
            let ports = args.get(i + 1);

            let ports = match ports {
                Some(ports) => ports,
                None => {
                    print_help_message();

                    panic!("There is no ports number after the --port flag");
                }
            };

            target_ports = ports.split(",")
                .map(|port| {
                    let port = port.parse::<u16>();

                    let port = match port {
                        Ok(port) => port,
                        Err(_) => {
                            print_help_message();

                            panic!("Port is not a valid number");
                        }
                    };

                    // port must be between 1 and 65535
                    // u16 max value is 65535, we don't have to check for that
                    if port < 1 {
                        println!("Port {} is not valid", port);
                        std::process::exit(1);
                    }

                    port
                })
                .collect();
        }

        if arg == "--number" || arg == "-n" {
            let number = args.get(i + 1);

            let number = match number {
                Some(number) => number,
                None => {
                    print_help_message();

                    panic!("There is no number after the --number flag");
                }
            };

            match number.parse::<usize>() {
                Ok(_) => {
                    number_of_requests = number.parse().unwrap();
                }
                Err(_) => {
                    print_help_message();

                    panic!("Number is not a valid number");
                }
            }
        }
    }

    if ip4v_address == Ipv4Addr::new(0, 0, 0, 0) {
        print_help_message();

        panic!("There is no host after the --host flag");
    }


    return (ip4v_address, target_ports, number_of_requests);
}

// A test for arguments_handler
#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use crate::console::handler;

    #[test]
    #[should_panic(expected = "There is no host after the --host flag")]
    fn it_should_have_a_args_after_host_flag() {
        let args: Vec<String> = vec![
            String::from("program_itself"),
            String::from("--host"),
        ];

        handler(args);
    }

    #[test]
    #[should_panic(expected = "There is no host after the --host flag")]
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

        let (hosts, ports, total_requests) = handler(args);

        assert_eq!(hosts, Ipv4Addr::new(127, 0, 0, 1));
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
    #[should_panic(expected = "There is no ports number after the --port flag")]
    fn it_should_have_a_args_after_ports_flag() {
        let args: Vec<String> = vec![
            String::from("program_itself"),
            String::from("--ports"),
        ];

        handler(args);
    }

    #[test]
    #[should_panic(expected = "There is no ports number after the --port flag")]
    fn it_should_have_a_args_after_ports_flag_2() {
        let args: Vec<String> = vec![
            String::from("program_itself"),
            String::from("-p"),
        ];

        handler(args);
    }

    #[test]
    fn it_should_return_the_port() {
        let args: Vec<String> = vec![
            String::from("program_itself"),
            String::from("--host"),
            String::from("127.0.0.1"),
            String::from("--ports"),
            String::from("80,443"),
        ];

        let (hosts, ports, total_requests) = handler(args);

        assert_eq!(ports, vec![80, 443]);
    }
}