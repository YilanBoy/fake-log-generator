use std::env;
use std::io::Write;
use std::time::Instant;

use rand::Rng;

use crate::arguments_handler::arguments_handler;
use crate::connection_settings::parse_connection_settings;
use crate::fake_data_generator::generate_fake_data;
use crate::tcp_connection::create_connections;

pub mod tcp_connection;
pub mod fake_data_generator;
mod connection_settings;
mod arguments_handler;

fn main() {
    // get the command line arguments
    let args: Vec<String> = env::args().collect();

    let filename: String = arguments_handler(args);

    println!("Filename is {}", filename);

    let content = connection_settings::read_connection_settings_file(filename);

    let config = parse_connection_settings(content);

    let total_requests = config.host.total_requests as usize;

    let before = Instant::now();

    let mut streams = create_connections(
        config.host.ip,
        config.host.ports,
    );

    let mut rng = rand::thread_rng();

    let mut total_bytes = 0;

    for i in 0..total_requests {
        let message = generate_fake_data();

        // randomize which stream to write to
        let random_index = rng.gen_range(0..streams.len());
        streams[random_index].write(message.as_bytes()).expect("Failed to write to stream");

        total_bytes += message.len();

        // print every 1000 requests
        if i % 1000 == 0 && i != 0 {
            println!("Requests sent: {}", i);
        }
    }

    // print total requests
    println!("Total requests: {}", total_requests);
    // print elapsed time
    println!("Elapsed time: {:.2?}", before.elapsed());
    // print total mb sent
    println!("Total sent: {} MB", total_bytes / 1_000_000);

    if before.elapsed().as_secs() > 0 {
        // print request per second
        println!("Requests per second: {}", total_requests / (before.elapsed().as_secs() as usize));
    }
}

