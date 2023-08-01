use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use std::io::{Read, Write};
use std::sync::Arc;
use tokio_postgres::NoTls;
use bincode;

const DATABASE_URL: &str = "postgres://postgres:1234567@localhost:5432/demo"; // Replace with your database URL

pub async fn handle_client(mut stream: TcpStream, db_client: Arc<Mutex<tokio_postgres::Client>>) {
    // Define the PacketMetadata struct here
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    pub struct PacketMetadata {
        src_ip: Option<String>,
        dst_ip: Option<String>,
        src_port: i32,
        dst_port: i32,
        seq_number: i32,
        ack_number: i32,
        flags: i32,
        window_size: i32,
    }

    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer).await {
            Ok(size) => {
                if size == 0 {
                    // Connection closed by client
                    break;
                } else {
                    let message = String::from_utf8_lossy(&buffer[..size]);
                    println!("Received message: {}", message);

                    // Deserialize the received data into a PacketMetadata struct
                    let packet_metadata: Result<PacketMetadata, bincode::Error> = bincode::deserialize(&buffer[..size]);
                    println!("data:{:?}", packet_metadata);
                    match packet_metadata {
                        Ok(metadata) => {
                            // Store the metadata in the database
                            let db_client_lock = db_client.lock().await;
                            if let Err(err) = db_client_lock.query(
                                "INSERT INTO sniffer (src_ip, dst_ip, src_port, dst_port, seq_number, ack_number, flags, window_size) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                            &[
                                        &metadata.src_ip,
                                        &metadata.dst_ip,
                                        &metadata.src_port,
                                        &metadata.dst_port,
                                        &metadata.seq_number,
                                        &metadata.ack_number,
                                        &metadata.flags,
                                        &metadata.window_size,],
                            ).await {
                                eprintln!("Failed to insert into database: {}", err);
                            }
                        }
                        Err(err) => {
                            eprintln!("Failed to deserialize packet metadata: {}", err);
                        }
                    }

                    // Send the message back to the client
                    if let Err(err) = stream.write_all(&buffer[..size]).await {
                        eprintln!("Failed to write response: {}", err);
                    }
                }
            }
            Err(_) => {
                eprintln!("An error occurred while reading from stream");
                break;
            }
        }
    }
   
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Establish a connection to the PostgreSQL database
    let (db_client, connection) = tokio_postgres::connect(DATABASE_URL, NoTls).await?;

    // Spawn a new Tokio task to process the database connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    // Wrap the database client in Arc<Mutex> to share safely between threads
    let db_client = Arc::new(Mutex::new(db_client));

    // Bind the TCP listener
    let listener = TcpListener::bind("localhost:8000").await?;
    println!("Server listening on localhost:8000");

    while let Ok((stream, _)) = listener.accept().await {
        // Clone the Arc<Mutex<Client>> for each connection
        let db_client_clone = Arc::clone(&db_client);
        // Spawn a new Tokio task for each client connection
        tokio::spawn(handle_client(stream, db_client_clone));
    }

    Ok(())
}






