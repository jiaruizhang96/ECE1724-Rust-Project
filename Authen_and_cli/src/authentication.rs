// auth.rs
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

pub struct AuthService {
    pub pool: Pool<Postgres>,
    pub jwt_secret: String,
}

impl AuthService {
    pub async fn new() -> Self {
        dotenv().ok(); // Load environment variables from .env

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let pool = PgPoolOptions::new()
            .connect(&database_url)
            .await
            .expect("Failed to connect to the database");
        AuthService {
            pool,
            jwt_secret,
        }
    }
    
    // register a new user by storing its username, hashed password, and role in the db
    pub async fn register(&self, username: &str, password: &str, role: &str) -> Result<(), sqlx::Error> {
        let hashed_password = hash(password, DEFAULT_COST).expect("Failed to hash password");
        sqlx::query(
            "INSERT INTO users (username, password_hash, role) VALUES ($1, $2, $3)",
        )
        .bind(username)
        .bind(hashed_password)
        .bind(role)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // verify the credential and return a JWT token if successful
    pub async fn login(&self, username: &str, password: &str) -> Result<String, String> {
        let user = sqlx::query(
            "SELECT password_hash, role FROM users WHERE username = $1",
        )
        .bind(username)
        .fetch_one(&self.pool)
        .await
        .map_err(|_| "User not found".to_string())?;

        let password_hash: String = user.get("password_hash");
        let role: String = user.get("role");

        if verify(password, &password_hash).unwrap() {
            let expiration = Utc::now()
                .checked_add_signed(Duration::hours(24))
                .expect("valid timestamp")
                .timestamp() as usize;

            let claims = Claims {
                sub: username.to_owned(),
                role: role.clone(),
                exp: expiration,
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(self.jwt_secret.as_ref()),
            )
            .map_err(|_| "Token creation error".to_string())?;

            Ok(token)
        } else {
            Err("Invalid credentials".to_string())
        }
    }

    // verify the JWT token
    pub fn verify_token(&self, token: &str) -> Result<Claims, String> {
        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| "Token verification failed".to_string())?;

        Ok(decoded.claims)
    }

    // check if the user has the required_role
    pub fn authorize(&self, claims: &Claims, required_role: &str) -> bool {
        claims.role == required_role
    }
}
