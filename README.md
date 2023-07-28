# Network Sniffer Project

![Sniffer](sniffer_image.png)

## Description

The Network Sniffer project is an implementation of a packet capturing and analysis tool written in Rust. It allows you to monitor network traffic, capture packets, and analyze their metadata. The project is designed to be a lightweight and efficient tool for gaining insights into the network traffic passing through a specific interface.

## Features

- **Packet Capturing**: The network sniffer captures packets from the specified network interface in real-time.

- **Packet Metadata Analysis**: It extracts important metadata from the captured packets, including source and destination IP addresses, source and destination ports, sequence numbers, acknowledgment numbers, flags, and window size.

- **Database Storage**: The captured packet metadata is stored in a PostgreSQL database, providing a persistent record of network activity.

- **Concurrency**: The sniffer uses Tokio, a powerful asynchronous runtime for Rust, to handle multiple client connections concurrently.

- **Error Handling**: The project implements robust error handling to ensure smooth operation and graceful degradation in case of unexpected errors.

## How to Use

1. Clone the repository to your local machine:

```
git clone https://github.com/yourusername/network-sniffer.git
cd network-sniffer
```

2. Install the required dependencies:

```
cargo build
```

3. Set up the PostgreSQL database with the following credentials:

   - Database URL: `postgres://postgres:your_password@localhost:5432/demo`

4. Start the network sniffer:

```
cargo run
```

5. The sniffer will start listening on `localhost:8000` for incoming client connections.

6. Connect to the sniffer using a TCP client, such as `telnet` or a custom client application.

7. Once connected, the sniffer will capture incoming packets, display their content, and store the extracted metadata in the database.

## Example Usage

```
// Connect to the sniffer using a TCP client
telnet localhost 8000

// Sample output on the client side
Received message: This is a sample message from the client.

// Sample output on the server side
data: Some(PacketMetadata { src_ip: None, dst_ip: None, src_port: 1234, dst_port: 5678, seq_number: 1, ack_number: 2, flags: 0, window_size: 4096 })
```

## Contributing

Contributions are welcome! If you find a bug or have a feature request, please open an issue or submit a pull request.

## Acknowledgments

Special thanks to the Rust community and Tokio contributors for their amazing work in developing the language and asynchronous runtime.

Happy sniffing! 🕵️‍♂️
