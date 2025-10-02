use crate::db::{models::Customer, queries, DbPool};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCustomerRequest {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCustomerRequest {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub notes: Option<String>,
}

#[tauri::command]
pub async fn get_customers(pool: State<'_, DbPool>) -> Result<Vec<Customer>, String> {
    queries::get_all_customers(&pool)
        .await
        .map_err(|e| format!("Failed to get customers: {}", e))
}

#[tauri::command]
pub async fn get_customer(pool: State<'_, DbPool>, id: String) -> Result<Option<Customer>, String> {
    queries::get_customer_by_id(&pool, &id)
        .await
        .map_err(|e| format!("Failed to get customer: {}", e))
}

#[tauri::command]
pub async fn create_customer(
    pool: State<'_, DbPool>,
    request: CreateCustomerRequest,
) -> Result<Customer, String> {
    let customer = queries::create_customer(
        &pool,
        request.name,
        request.email,
        request.phone,
        request.address,
        request.notes,
    )
    .await
    .map_err(|e| format!("Failed to create customer: {}", e))?;

    // Add to sync queue

    Ok(customer)
}

#[tauri::command]
pub async fn update_customer(
    pool: State<'_, DbPool>,
    request: UpdateCustomerRequest,
) -> Result<Customer, String> {
    let customer = queries::update_customer(
        &pool,
        &request.id,
        request.name,
        request.email,
        request.phone,
        request.address,
        request.notes,
    )
    .await
    .map_err(|e| format!("Failed to update customer: {}", e))?;


    Ok(customer)
}

#[tauri::command]
pub async fn delete_customer(pool: State<'_, DbPool>, id: String) -> Result<(), String> {

    queries::delete_customer(&pool, &id)
        .await
        .map_err(|e| format!("Failed to delete customer: {}", e))
}

