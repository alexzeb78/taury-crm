use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Company {
    pub id: String,
    pub name: String,
    pub website: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CompanyContact {
    pub id: String,
    pub company_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub is_primary: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Customer {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Document {
    pub id: String,
    pub customer_id: String,
    pub title: String,
    pub document_type: String,
    pub file_path: Option<String>,
    pub content: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncRequest {
    pub action: String,
    pub entity_id: String,
    pub payload: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Proposal {
    pub id: String,
    pub company_id: String,
    pub proposal_number: Option<String>,
    pub status: String,
    pub total_amount: Option<f64>,
    pub currency: String,
    pub valid_until: Option<chrono::DateTime<chrono::Utc>>,
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProposalProduct {
    pub id: String,
    pub proposal_id: String,
    pub product_type: String,
    pub user_count: i32,
    pub standalone_count: i32,
    pub server_key_count: i32,
    pub unit_price: Option<f64>,
    pub total_price: Option<f64>,
    pub annual_reduction: f64,
    pub training: i32,
    pub training_days: i32,
    pub training_cost_per_day: f64,
    pub training_cost: f64,
    pub licence: i32,
    pub support: i32,
    pub support_years: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Invoice {
    pub id: String,
    pub proposal_id: String,
    pub invoice_number: String,
    pub status: String,
    pub total_amount: f64,
    pub currency: String,
    pub issue_date: chrono::DateTime<chrono::Utc>,
    pub due_date: Option<chrono::DateTime<chrono::Utc>>,
    pub paid_date: Option<chrono::DateTime<chrono::Utc>>,
    pub purchase_order: Option<String>,
    pub purchase_order_date: Option<chrono::DateTime<chrono::Utc>>,
    pub commercial_in_charge: Option<String>,
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

