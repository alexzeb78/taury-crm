use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub name: String,
    pub enabled: i64,
    pub created_at: String,
    pub updated_at: String,
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
    pub created_at: String,
    pub updated_at: String,
    pub version: Option<i64>,
    pub is_deleted: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CompanyContact {
    pub id: String,
    pub company_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub is_primary: i64,
    pub created_at: String,
    pub updated_at: String,
    pub version: Option<i64>,
    pub is_deleted: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LicencePricing {
    pub id: i64,
    pub product_type: String,
    pub user_count: i64,
    pub price_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Proposal {
    pub id: String,
    pub company_id: String,
    pub proposal_number: Option<String>,
    pub status: String,
    pub total_amount: f64,
    pub currency: String,
    pub valid_until: String,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub version: Option<i64>,
    pub is_deleted: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProposalProduct {
    pub id: String,
    pub proposal_id: String,
    pub product_type: String,
    pub user_count: i64,
    pub standalone_count: i64,
    pub server_key_count: i64,
    pub unit_price: f64,
    pub total_price: f64,
    pub annual_reduction: f64,
    pub training: i64,
    pub training_days: i64,
    pub training_cost_per_day: f64,
    pub training_cost: f64,
    pub licence: i64,
    pub support: i64,
    pub support_years: i64,
    pub created_at: String,
    pub updated_at: String,
    pub version: Option<i64>,
    pub is_deleted: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Invoice {
    pub id: String,
    pub proposal_id: String,
    pub invoice_number: String,
    pub status: String,
    pub total_amount: f64,
    pub currency: String,
    pub issue_date: String,
    pub due_date: Option<String>,
    pub paid_date: Option<String>,
    pub purchase_order: Option<String>,
    pub purchase_order_date: Option<String>,
    pub commercial_in_charge: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub version: Option<i64>,
    pub is_deleted: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InvoiceWithDetails {
    pub id: String,
    pub proposal_id: String,
    pub proposal_number: String,
    pub company_name: String,
    pub invoice_number: String,
    pub status: String,
    pub total_amount: f64,
    pub currency: String,
    pub issue_date: String,
    pub due_date: Option<String>,
    pub paid_date: Option<String>,
    pub purchase_order: Option<String>,
    pub purchase_order_date: Option<String>,
    pub commercial_in_charge: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Customer {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub version: Option<i64>,
    pub is_deleted: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Document {
    pub id: String,
    pub customer_id: String,
    pub title: String,
    pub document_type: String,
    pub file_path: Option<String>,
    pub content: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub version: Option<i64>,
    pub is_deleted: Option<i64>,
}

// SyncQueueItem - REMOVED as part of synchronization cleanup

// Setting - REMOVED as part of synchronization cleanup

// === SYNC MODELS ===
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SyncMetadata {
    pub id: i64,
    pub last_sync_timestamp: i64,
    pub last_sync_version: i64,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncItem {
    pub table_name: String,
    pub id: String,
    pub data: serde_json::Value,
    pub version: i64,
    pub is_deleted: bool,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRequest {
    pub last_sync_timestamp: i64,
    pub changes: Vec<SyncItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResponse {
    pub success: bool,
    pub message: String,
    pub remote_changes: Vec<SyncItem>,
    pub new_timestamp: i64,
    pub items_synced: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWithRoles {
    pub id: String,
    pub email: String,
    pub name: String,
    pub enabled: bool,
    pub roles: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

// DeletionQueue - REMOVED as part of synchronization cleanup

// ActionQueue - REMOVED as part of synchronization cleanup

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyWithContacts {
    #[serde(flatten)]
    pub company: Company,
    pub contacts: Vec<CompanyContact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalWithProducts {
    #[serde(flatten)]
    pub proposal: Proposal,
    pub products: Vec<ProposalProduct>,
    pub company_name: String,
}