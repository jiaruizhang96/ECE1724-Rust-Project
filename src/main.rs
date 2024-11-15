//mod hashing;
//mod storage;
//mod utils;

mod network;
mod client;
use async_std::task;
use std::env;
use std::thread;

fn main() {
    // Capture command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- [server|client]");
        return;
    }

    let mode = &args[1];

    match mode.as_str() {
        "server" => {
            // Run the server in its own thread
            thread::spawn(|| {
                task::block_on(async {
                    network::communication::CommunicationServer::new("127.0.0.1:8080").start_server();
                });
            });

            // Keep the main thread running to keep the server alive
            loop {
                std::thread::park();
            }
        }
        "client" => {
            // Run the client in a separate thread
            thread::spawn(|| {
                task::block_on(async {
                    client::client::start_client();
                });
            });

            // Give time for the client thread to finish before exiting main
            loop {
                std::thread::park();
            }
        }
        _ => {
            eprintln!("Invalid argument. Use 'server' or 'client'.");
        }
    }
}
