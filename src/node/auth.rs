use ed25519_dalek::{Keypair, PublicKey, Signature, Verifier};
use rand::rngs::OsRng;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{Write};

#[derive(Clone)]
pub struct UserManager {
    users: Arc<Mutex<HashMap<String, UserCredentials>>>,
    acl: Arc<Mutex<HashMap<String, Vec<String>>>>, // Key -> Authorized user public keys. Tell which users are allowed to acces which key
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct UserCredentials {
    username: String,
    public_key: Vec<u8>,
    is_admin: bool,
}

impl UserManager {
    pub fn new() -> Self {
        UserManager {
            users: Arc::new(Mutex::new(HashMap::new())),
            acl: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_user(&self, username: &str, is_admin: bool) -> Result<Vec<u8>, String> {
        let mut csprng = OsRng;
        let keypair = Keypair::generate(&mut csprng);
        
        let public_key = keypair.public.to_bytes().to_vec();
        let hex_public_key = hex::encode(&public_key);

        let mut users = self.users.lock().map_err(|_| "Lock poisoned")?;
        
        if users.contains_key(&hex_public_key) {
            return Err("User already exists".to_string());
        }

        // Generate a secure location to store the private key
        let private_key_path = format!("./private_keys/{}.private_key", username);
        
        // Store the private key securely (in this case, in a file)
        let private_key = keypair.secret.to_bytes().to_vec();
        self.store_private_key(&private_key_path, &private_key)?;

        users.insert(
            hex_public_key.clone(),
            UserCredentials {
                username: username.to_string(),
                public_key: public_key.clone(),
                is_admin,
            }
        );

        Ok(public_key)
    }

    // securely store private key (e.g., in a file)
    fn store_private_key(&self, path: &str, private_key: &[u8]) -> Result<(), String> {
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        file.write_all(&private_key).map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn authenticate(&self, public_key: &[u8], signature: &[u8], message: &[u8]) -> bool {
        let public_key = match PublicKey::from_bytes(public_key) {
            Ok(pk) => pk,
            Err(_) => return false,
        };

        let signature = match Signature::from_bytes(signature) {
            Ok(sig) => sig,
            Err(_) => return false,
        };

        public_key.verify(message, &signature).is_ok()
    }

    pub fn add_key_permission(&self, key: &str, authorized_public_key: &[u8]) -> Result<(), String> {
        let hex_key = hex::encode(key);
        //TODO: probably don't need to hex encode again here
        let hex_public_key = hex::encode(authorized_public_key);

        let mut acl = self.acl.lock().map_err(|_| "Lock poisoned")?;
        
        acl.entry(hex_key)
            .or_insert_with(Vec::new)
            .push(hex_public_key);

        Ok(())
    }

    pub fn check_key_permission(&self, key: &str, public_key: &[u8]) -> bool {
        let hex_key = hex::encode(key);
        let hex_public_key = hex::encode(public_key);

        let acl = self.acl.lock().unwrap();
        
        acl.get(&hex_key)
            .map(|authorized_keys| authorized_keys.contains(&hex_public_key))
            .unwrap_or(false)
    }

    // pub fn is_admin(&self, public_key: &[u8]) -> bool {
    //     let hex_public_key = hex::encode(public_key);
        
    //     let users = self.users.lock().unwrap();
        
    //     users.get(&hex_public_key)
    //         .map(|user| user.is_admin)
    //         .unwrap_or(false)
    // }
}