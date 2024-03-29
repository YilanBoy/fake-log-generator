use std::env;
use std::io::Write;
use std::net::Ipv4Addr;
use std::time::Instant;

use rand::Rng;

pub mod tcp_connection;
pub mod fake_data_generator;
mod console;

fn main() {
    // get the current time, used to calculate elapsed time
    let instant = Instant::now();

    let args: Vec<String> = env::args().collect();

    let (host, ports, total_requests) = console::handler(args);

    let total_bytes = send_logs(host, ports, total_requests);

    execute_information(instant, total_requests, total_bytes);
}

fn send_logs(host: Ipv4Addr, ports: Vec<u16>, total_request: usize) -> usize {
    let mut streams = tcp_connection::create_connections(
        host,
        ports,
    );

    let mut rng = rand::thread_rng();

    let mut total_bytes = 0;

    for i in 0..total_request {
        let message = fake_data_generator::generate_fake_data();

        // randomize which stream to write to
        let random_index = rng.gen_range(0..streams.len());
        streams[random_index].write(message.as_bytes()).expect("Failed to write to stream");

        total_bytes += message.len();

        // print every 1000 requests
        if i % 1000 == 0 && i != 0 {
            println!("Requests sent: {}", i);
        }
    }

    total_bytes
}

fn execute_information(instance: Instant, total_requests: usize, total_bytes: usize) {
    // print total requests
    println!("Total requests: {}", total_requests);
    // print elapsed time
    println!("Elapsed time: {:.2?}", instance.elapsed());
    // print total mb sent
    println!("Total sent: {} MB", total_bytes / 1_000_000);

    if instance.elapsed().as_secs() > 0 {
        // print request per second
        println!("Requests per second: {}", total_requests / (instance.elapsed().as_secs() as usize));
    }
}

