use ed25519_dalek::{Keypair, Signature, SecretKey, PublicKey};
use ed25519_dalek::Signer;
use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, Read};

fn load_private_key(username: &str) -> Result<Vec<u8>, String> {
    let private_key_path = format!("./private_keys/{}.private_key", username);

    let mut file = File::open(private_key_path).map_err(|e| e.to_string())?;
    let mut private_key = Vec::new();
    file.read_to_end(&mut private_key).map_err(|e| e.to_string())?;

    Ok(private_key)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Sign Message")
        .version("1.0")
        .about("Sign a message using the user's private key")
        .arg(
            Arg::new("username")
                .short('u')
                .long("username")
                .help("Username for which the private key will be loaded"),
        )
        .arg(
            Arg::new("target_key")
                .short('t')
                .long("target_key")
                .help("The file key to be signed"),
        )
        .get_matches();

    let username = matches
        .get_one::<String>("username")
        .expect("Username is required");
    let message = matches
        .get_one::<String>("target_key")
        .expect("Target_key is required");

    // load the private key
    let private_key_bytes = load_private_key(username)?;

    let private_key = SecretKey::from_bytes(&private_key_bytes).map_err(|e| e.to_string())?;
    let public_key: PublicKey = (&private_key).into();
    let keypair = Keypair {
        secret: private_key,
        public: public_key,
    };

    // sign the message with the private key
    let signature: Signature = keypair.sign(message.as_bytes());

    // print the signature so that it can be sent to the server
    println!("Signature: {}", hex::encode(signature.to_bytes()));

    Ok(())
}
