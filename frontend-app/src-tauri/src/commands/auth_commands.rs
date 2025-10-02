use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
    pub user_id: Option<String>,
    pub user_email: Option<String>,
    pub user_name: Option<String>,
}

#[tauri::command]
pub async fn login(
    request: LoginRequest,
) -> Result<AuthResponse, String> {
    // Pour l'instant, on accepte n'importe quel email/password
    // Dans une vraie application, on vérifierait le hash du mot de passe
    
    let user_id = Uuid::new_v4().to_string();
    
    Ok(AuthResponse {
        success: true,
        message: "Login successful".to_string(),
        user_id: Some(user_id),
        user_email: Some(request.email),
        user_name: Some("User".to_string()),
    })
}

#[tauri::command]
pub async fn register(
    request: RegisterRequest,
) -> Result<AuthResponse, String> {
    // Pour l'instant, on accepte n'importe quel email/password
    // Dans une vraie application, on hasherait le mot de passe et on l'enregistrerait
    
    let user_id = Uuid::new_v4().to_string();
    
    Ok(AuthResponse {
        success: true,
        message: "Registration successful".to_string(),
        user_id: Some(user_id),
        user_email: Some(request.email),
        user_name: Some(request.name),
    })
}

#[tauri::command]
pub async fn logout() -> Result<(), String> {
    // Pour l'instant, on ne fait rien
    // Dans une vraie application, on pourrait invalider un token JWT
    Ok(())
}