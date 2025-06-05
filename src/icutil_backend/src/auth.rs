use candid::{CandidType, Deserialize};
use ic_cdk::{api, caller};
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone)]
pub struct User {
    pub principal: String,
    pub roles: Vec<String>,
    pub created_at: u64,
}

#[derive(CandidType, Deserialize)]
pub struct AuthPayload {
    pub principal: String,
    pub exp: u64,
}

static mut USERS: HashMap<String, User> = HashMap::new();

pub fn init_admin() {
    let admin_principal = api::id().to_string();
    unsafe {
        USERS.insert(
            admin_principal.clone(),
            User {
                principal: admin_principal,
                roles: vec!["admin".to_string()],
                created_at: api::time(),
            },
        );
    }
}

pub fn authenticate(roles: &[&str]) -> Result<User, String> {
    let caller_principal = caller().to_string();
    
    unsafe {
        USERS
            .get(&caller_principal)
            .filter(|user| roles.iter().any(|r| user.roles.contains(&r.to_string())))
            .cloned()
            .ok_or_else(|| "Unauthorized access".to_string())
    }
}

#[update]
fn create_user(new_user: User) -> Result<(), String> {
    let auth_user = authenticate(&["admin"])?;
    
    unsafe {
        USERS.insert(new_user.principal.clone(), new_user);
    }
    Ok(())
}

#[query]
fn get_roles() -> Result<Vec<String>, String> {
    let user = authenticate(&[])?;
    Ok(user.roles)
}