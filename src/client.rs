use ed25519_dalek::{Keypair, Signature, SecretKey, PublicKey};
use ed25519_dalek::Signer;
use std::fs::File;
use std::io::{Read};
use std::io::ErrorKind;

pub fn sign_message(username: &str, message: &str) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), String> {
    // load the private key
    let private_key_path = format!("./private_keys/{}.private_key", username);

    let mut private_key_bytes = Vec::new();
    match File::open(&private_key_path) {
        Ok(mut file) => {
            file.read_to_end(&mut private_key_bytes).map_err(|e| e.to_string())?;
        },
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            return Err("This client is not registered".to_string());
        },
        Err(e) => {
            return Err(e.to_string());
        }
    }

    let private_key = SecretKey::from_bytes(&private_key_bytes).map_err(|e| e.to_string())?;
    let public_key: PublicKey = (&private_key).into();
    let keypair = Keypair {
        secret: private_key,
        public: public_key,
    };

    // sign the message with the private key
    let signature: Signature = keypair.sign(message.as_bytes());

    // print all the thing the user need to make file operations
    Ok((
        keypair.public.to_bytes().to_vec(),
        signature.to_bytes().to_vec(),
        message.as_bytes().to_vec()
    ))
}