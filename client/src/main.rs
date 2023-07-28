use serde::{Serialize, Deserialize};
use bincode::{serialize, deserialize};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::io;
use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::tcp::TcpPacket;
use pnet::packet::ip::IpNextHeaderProtocol;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use tokio::runtime::Runtime;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct PacketMetadata {
    src_ip: Option<String>,
    dst_ip: Option<String>,
    src_port: u32,
    dst_port: u32,
    seq_number: u32,
    ack_number: u32,
    flags: u32,
    window_size: u32,
}

async fn packet_capture(mut write_stream: TcpStream) {
    // Retrieve the list of available network interfaces
    let interfaces = datalink::interfaces();

    // Find the desired network interface by name
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == "wlp0s20f3")
        .expect("Wi-Fi interface not found");

    // Create a channel to receive packets on the selected Wi-Fi interface
    let mut channel = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(_, receiver)) => receiver,
        Ok(_) => panic!("Unsupported channel type"),
        Err(e) => panic!("Error creating channel: {}", e),
    };

    // Start capturing packets
    loop {
        match channel.next() {
            Ok(packet) => {
                // Access Ethernet packet metadata
                if let Some(transport_packet) = TcpPacket::new(packet) {
                    let src_port = transport_packet.get_source();
                    let dst_port = transport_packet.get_destination();
                    let seq_number = transport_packet.get_sequence();
                    let ack_number = transport_packet.get_acknowledgement();
                    let flags = transport_packet.get_flags();
                    let window_size = transport_packet.get_window();

                    // Access IP packet metadata
                    if let Some(ip_packet) = Ipv4Packet::new(packet) {
                        let src_ip = ip_packet.get_source().to_string();
                        let dst_ip = ip_packet.get_destination().to_string();

                        println!("Source Ip: {:?}", src_ip);
                        println!("Destination Ip: {:?}", dst_ip);
                        println!("Source Port: {}", src_port);
                        println!("Destination Port: {}", dst_port);
                        println!("Sequence Number: {}", seq_number);
                        println!("Acknowledgment Number: {}", ack_number);
                        println!("TCP Flags: {:?}", flags);
                        println!("TCP Window Size: {}", window_size);

                        // Create a struct with the relevant packet metadata
                        let packet_metadata = PacketMetadata {
                            src_ip: Some(src_ip),
                            dst_ip: Some(dst_ip),
                            src_port: src_port.into(),
                            dst_port: dst_port.into(),
                            seq_number,
                            ack_number,
                            flags: flags.into(),
                            window_size: window_size.into(),
                        };

                        // Serialize the metadata to bytes using bincode
                        let metadata_bytes = serialize(&packet_metadata).expect("Failed to serialize metadata");

                        // Send the metadata as a message to the server
                        write_stream.write_all(&metadata_bytes).expect("Failed to send metadata");
                        write_stream.flush().expect("Failed to flush");
                    
                    }
                }
            }
            Err(_) => todo!()
        }
    }
}

fn io_operations(rt: &Runtime, stream_clone: &mut TcpStream, input: &str) -> bool {
    // Send the metadata to the server
    stream_clone.write_all(input.as_bytes()).expect("Failed to send metadata");
    stream_clone.flush().expect("Failed to flush");

    // Receive and print the response from the server
    let mut buffer = [0; 1024];
    let size = rt.block_on(async {
        stream_clone.read(&mut buffer).expect("Failed to read from server")
    });
    if size == 0 {
        // Connection closed by the server
        return true; // Signal the main loop to break
    }

    let response = String::from_utf8_lossy(&buffer[..size]);
    println!("Received response: {}", response);
    false // Signal the main loop to continue
}

fn main() {
    // Establish the tokio runtime
    let rt = Runtime::new().unwrap();

    // Connect to the server
    let stream = TcpStream::connect("localhost:8000").expect("Failed to connect to server");
    println!("Connected to server");

    // Clone the stream for packet capture
    let write_stream = stream.try_clone().expect("Failed to clone stream");

    // Start packet capture in a separate tokio task
    rt.spawn(async move {
        packet_capture(write_stream).await;
    });

    // Main loop for reading input and interacting with the server
    let mut stream_clone = stream.try_clone().expect("Failed to clone stream");
    loop {
        let mut input = String::new();
        println!("Enter metadata (KEY:VALUE):");
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if io_operations(&rt, &mut stream_clone, &input) {
            break; // Exit the main loop if the I/O operations signal to break
        }
    }
}