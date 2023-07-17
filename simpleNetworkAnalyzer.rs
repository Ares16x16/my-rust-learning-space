use std::net::TcpListener;
use std::io::{Read, Write, ErrorKind};
use std::time::{Instant, Duration};

fn main() {
    // localhost
    let listener = match TcpListener::bind("127.0.0.1:8000") {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to port: {}", e);
            std::process::exit(1);
        }
    };

    println!("Server listening on localhost");

    let mut packets_received = 0;
    let mut total_packet_size = 0;
    let mut max_packet_size = 0;
    let mut previous_packet_time = Instant::now();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Client connected");

                let start_time = Instant::now();
                let mut buffer = [0; 512];

                match stream.read(&mut buffer) {
                    Ok(bytes_read) => {
                        let end_time = Instant::now();
                        let latency = end_time.duration_since(start_time);
                        let packet_size = bytes_read;

                        println!("Received packet: {:?}", &buffer[..bytes_read]);
                        println!("Latency: {:?}", latency);

                        packets_received += 1;
                        total_packet_size += packet_size;

                        if packet_size > max_packet_size {
                            max_packet_size = packet_size;
                        }

                        let time_since_last_packet = start_time.duration_since(previous_packet_time);
                        
                        // Calculate packet loss (naive), for sending continuous packets
                        if time_since_last_packet > Duration::from_secs(1) {
                            let expected_packets = time_since_last_packet.as_secs() as u32;
                            let actual_packets = packets_received;
                            let packet_loss = expected_packets - actual_packets;

                            println!("Packet loss: {}", packet_loss);

                            previous_packet_time = start_time;
                            packets_received = 0;
                            total_packet_size = 0;
                            max_packet_size = 0;
                        }
                    },
                    Err(e) => {
                        if e.kind() == ErrorKind::ConnectionReset {
                            println!("Client disconnected");
                        } else {
                            eprintln!("Error: {}", e);
                        }
                    }
                }

                stream.write(b"Hi").unwrap();
                println!("Closing connection");
            },
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}