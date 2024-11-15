// Client-side communication code
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn start_client() {
    // Connect to the server
    match TcpStream::connect("127.0.0.1:8080") {
        Ok(mut stream) => {
            println!("Connected to the server!");

            // Send a message to the server
            let message = "Hello from client!";
            stream.write(message.as_bytes()).expect("Failed to write to server");
            println!("Message sent: {}", message);

            // Receive the response from the server
            let mut buffer = [0; 512];
            let bytes_read = stream.read(&mut buffer).expect("Failed to read from server");

            // Display the server's response
            let response = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("Server response: {}", response);
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
