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

// Inside AuthState
#[init]
fn init() {
    let mut state = AuthState {
        users: HashMap::new(),
        jwt_secret: generate_secure_secret(),
    };
    // Initialize admin user
}

fn generate_secure_secret() -> String {
    let mut rng = rand::thread_rng();
    let bytes: [u8; 32] = rng.gen();
    hex::encode(bytes)
}

#[update]
fn generate_token(principal: Principal) -> String {
    let expiration = ic_cdk::api::time() + 86_400_000_000_000; // 24 hours
    let claims = Claims {
        sub: principal,
        exp: expiration,
        roles: vec![UserRole::DeviceManager],
    };
    
    encode(&Header::default(), &claims, &EncodingKey::from_secret(state.jwt_secret.as_bytes()))
        .expect("Failed to generate token")
}

#[query]
fn validate_token(token: &str) -> Result<Claims, String> {
    decode::<Claims>(token, &DecodingKey::from_secret(state.jwt_secret.as_bytes()), &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| format!("Invalid token: {:?}", e))
}

// Role storage
thread_local! {
    static ROLE_STORE: RefCell<HashMap<Principal, Vec<String>>> = RefCell::new(HashMap::new());
}

// Role management endpoints
#[ic_cdk::update]
fn assign_roles(user: Principal, roles: Vec<String>) {
    ROLE_STORE.with(|store| {
        store.borrow_mut().insert(user, roles);
    });
}

#[ic_cdk::query]
fn get_roles(user: Principal) -> Vec<String> {
    ROLE_STORE.with(|store| {
        store.borrow().get(&user).cloned().unwrap_or_default()
    })
}

// Updated JWT generation with roles
async fn generate_token(user: &User) -> String {
    let roles = get_roles(user.principal);
    let claims = Claims {
        sub: user.principal.to_string(),
        exp: (ic_cdk::api::time() + 3600_000_000_000) as usize,
        roles,
    };
    
    encode(&Header::default(), &claims, &EncodingKey::from_secret(state.jwt_secret.as_bytes()))
        .expect("Failed to generate token")
}