use ic_cdk_macros::{query, update};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub enum UserRole {
    Admin,
    DeviceManager,
    Viewer,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub principal: Principal,
    pub roles: Vec<UserRole>,
    pub session_token: String,
    pub expires_at: u64,
}

#[derive(Serialize, Deserialize)]
struct AuthState {
    users: HashMap<Principal, User>,
    jwt_secret: String,
}

// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: Principal,
    exp: u64,
    roles: Vec<UserRole>,
}