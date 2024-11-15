use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};


pub struct CommunicationServer {
    address: String,
}

impl CommunicationServer {
    // Constructor for creating a new CommunicationServer instance
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
        }
    }

    // Method to start the server
    pub fn start_server(&self) {
        let listener = TcpListener::bind(&self.address).expect("Could not bind to address");

        println!("Server is listening on {}", self.address);

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    println!("New connection established!");

                    // Read the incoming message
                    let mut buffer = [0; 512];
                    stream.read(&mut buffer).expect("Failed to read from the client");

                    // Log the message received
                    let received_msg = String::from_utf8_lossy(&buffer);
                    println!("Received: {}", received_msg);

                    // Send response to the client
                    let response = "Hello, world!";
                    stream.write(response.as_bytes()).expect("Failed to write to the client");
                    println!("Response sent to client.");
                }
                Err(e) => {
                    println!("Connection failed: {}", e);
                }
            }
        }
    }

    // Method to send a message to a specified client
    pub fn send_message(&self, address: &str, message: &str) {
        match TcpStream::connect(address) {
            Ok(mut stream) => {
                stream.write(message.as_bytes()).expect("Failed to send message");
                println!("Message sent: {}", message);

                let mut buffer = [0; 512];
                let bytes_read = stream.read(&mut buffer).expect("Failed to read response");
                let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Received response: {}", response);
            }
            Err(e) => {
                println!("Failed to connect to {}: {}", address, e);
            }
        }
    }

    // Method to receive and print a message (server-side)
    pub fn receive_message(&self, mut stream: TcpStream) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).expect("Failed to read message");
        let message = String::from_utf8_lossy(&buffer);
        println!("Received message: {}", message);
    }
}
