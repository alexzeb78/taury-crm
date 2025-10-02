use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncItem {
    pub table_name: String,
    pub id: String,
    pub data: serde_json::Value,
    pub version: i64,
    pub is_deleted: bool,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncRequest {
    pub last_sync_timestamp: i64,
    pub changes: Vec<SyncItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncResponse {
    pub success: bool,
    pub message: String,
    pub remote_changes: Vec<SyncItem>,
    pub new_timestamp: i64,
    pub items_synced: i64,
}

// Helper function to get current timestamp in milliseconds
fn current_timestamp() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

// Helper function to apply sync item to PostgreSQL
async fn apply_sync_item_to_server(pool: &PgPool, item: &SyncItem) -> Result<(), sqlx::Error> {
    if item.is_deleted {
        // Delete the item
        match item.table_name.as_str() {
            "companies" => {
                sqlx::query("DELETE FROM companies WHERE id = $1")
                    .bind(&item.id)
                    .execute(pool)
                    .await?;
            }
            "company_contacts" => {
                sqlx::query("DELETE FROM company_contacts WHERE id = $1")
                    .bind(&item.id)
                    .execute(pool)
                    .await?;
            }
            "customers" => {
                sqlx::query("DELETE FROM customers WHERE id = $1")
                    .bind(&item.id)
                    .execute(pool)
                    .await?;
            }
            "proposals" => {
                sqlx::query("DELETE FROM proposals WHERE id = $1")
                    .bind(&item.id)
                    .execute(pool)
                    .await?;
            }
            "proposal_products" => {
                sqlx::query("DELETE FROM proposal_products WHERE id = $1")
                    .bind(&item.id)
                    .execute(pool)
                    .await?;
            }
            "invoices" => {
                sqlx::query("DELETE FROM invoices WHERE id = $1")
                    .bind(&item.id)
                    .execute(pool)
                    .await?;
            }
            "documents" => {
                sqlx::query("DELETE FROM documents WHERE id = $1")
                    .bind(&item.id)
                    .execute(pool)
                    .await?;
            }
            _ => {
                return Err(sqlx::Error::Protocol("Unknown table name".to_string()));
            }
        }
    } else {
        // Insert or update the item
        match item.table_name.as_str() {
            "companies" => {
                if let Ok(company) = serde_json::from_value::<serde_json::Value>(item.data.clone()) {
                    sqlx::query(
                        "INSERT INTO companies (id, name, website, address, city, postal_code, country, description, created_at, updated_at, version, is_deleted) 
                         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                         ON CONFLICT (id) DO UPDATE SET
                         name = EXCLUDED.name,
                         website = EXCLUDED.website,
                         address = EXCLUDED.address,
                         city = EXCLUDED.city,
                         postal_code = EXCLUDED.postal_code,
                         country = EXCLUDED.country,
                         description = EXCLUDED.description,
                         updated_at = EXCLUDED.updated_at,
                         version = EXCLUDED.version,
                         is_deleted = EXCLUDED.is_deleted"
                    )
                    .bind(&item.id)
                    .bind(company.get("name").and_then(|v| v.as_str()).unwrap_or(""))
                    .bind(company.get("website").and_then(|v| v.as_str()))
                    .bind(company.get("address").and_then(|v| v.as_str()))
                    .bind(company.get("city").and_then(|v| v.as_str()))
                    .bind(company.get("postal_code").and_then(|v| v.as_str()))
                    .bind(company.get("country").and_then(|v| v.as_str()))
                    .bind(company.get("description").and_then(|v| v.as_str()))
                    .bind(company.get("created_at").and_then(|v| v.as_str()).and_then(|s| s.parse::<i64>().ok()).and_then(|ts| DateTime::<Utc>::from_timestamp_millis(ts)).unwrap_or_else(|| Utc::now()))
                    .bind(company.get("updated_at").and_then(|v| v.as_str()).and_then(|s| s.parse::<i64>().ok()).and_then(|ts| DateTime::<Utc>::from_timestamp_millis(ts)).unwrap_or_else(|| Utc::now()))
                    .bind(item.version)
                    .bind(item.is_deleted as i32)
                    .execute(pool)
                    .await?;
                }
            }
            "company_contacts" => {
                if let Ok(contact) = serde_json::from_value::<serde_json::Value>(item.data.clone()) {
                    sqlx::query(
                        "INSERT INTO company_contacts (id, company_id, first_name, last_name, email, phone_number, is_primary, created_at, updated_at, version, is_deleted) 
                         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                         ON CONFLICT (id) DO UPDATE SET
                         company_id = EXCLUDED.company_id,
                         first_name = EXCLUDED.first_name,
                         last_name = EXCLUDED.last_name,
                         email = EXCLUDED.email,
                         phone_number = EXCLUDED.phone_number,
                         is_primary = EXCLUDED.is_primary,
                         updated_at = EXCLUDED.updated_at,
                         version = EXCLUDED.version,
                         is_deleted = EXCLUDED.is_deleted"
                    )
                    .bind(&item.id)
                    .bind(contact.get("company_id").and_then(|v| v.as_str()).unwrap_or(""))
                    .bind(contact.get("first_name").and_then(|v| v.as_str()).unwrap_or(""))
                    .bind(contact.get("last_name").and_then(|v| v.as_str()).unwrap_or(""))
                    .bind(contact.get("email").and_then(|v| v.as_str()).unwrap_or(""))
                    .bind(contact.get("phone_number").and_then(|v| v.as_str()))
                    .bind(contact.get("is_primary").and_then(|v| v.as_i64()).unwrap_or(0) != 0)
                    .bind(contact.get("created_at").and_then(|v| v.as_str()).and_then(|s| s.parse::<i64>().ok()).and_then(|ts| DateTime::<Utc>::from_timestamp_millis(ts)).unwrap_or_else(|| Utc::now()))
                    .bind(contact.get("updated_at").and_then(|v| v.as_str()).and_then(|s| s.parse::<i64>().ok()).and_then(|ts| DateTime::<Utc>::from_timestamp_millis(ts)).unwrap_or_else(|| Utc::now()))
                    .bind(item.version)
                    .bind(item.is_deleted as i32)
                    .execute(pool)
                    .await?;
                }
            }
            "customers" => {
                if let Ok(customer) = serde_json::from_value::<serde_json::Value>(item.data.clone()) {
                    sqlx::query(
                        "INSERT INTO customers (id, name, email, phone, address, created_at, updated_at, version, is_deleted) 
                         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                         ON CONFLICT (id) DO UPDATE SET
                         name = EXCLUDED.name,
                         email = EXCLUDED.email,
                         phone = EXCLUDED.phone,
                         address = EXCLUDED.address,
                         updated_at = EXCLUDED.updated_at,
                         version = EXCLUDED.version,
                         is_deleted = EXCLUDED.is_deleted"
                    )
                    .bind(&item.id)
                    .bind(customer.get("name").and_then(|v| v.as_str()).unwrap_or(""))
                    .bind(customer.get("email").and_then(|v| v.as_str()))
                    .bind(customer.get("phone").and_then(|v| v.as_str()))
                    .bind(customer.get("address").and_then(|v| v.as_str()))
                    .bind(customer.get("created_at").and_then(|v| v.as_str()).and_then(|s| s.parse::<i64>().ok()).and_then(|ts| DateTime::<Utc>::from_timestamp_millis(ts)).unwrap_or_else(|| Utc::now()))
                    .bind(customer.get("updated_at").and_then(|v| v.as_str()).and_then(|s| s.parse::<i64>().ok()).and_then(|ts| DateTime::<Utc>::from_timestamp_millis(ts)).unwrap_or_else(|| Utc::now()))
                    .bind(item.version)
                    .bind(item.is_deleted as i32)
                    .execute(pool)
                    .await?;
                }
            }
            "proposals" => {
                if let Ok(proposal) = serde_json::from_value::<serde_json::Value>(item.data.clone()) {
                    sqlx::query(
                        "INSERT INTO proposals (id, company_id, proposal_number, status, total_amount, currency, valid_until, notes, created_at, updated_at, version, is_deleted) 
                         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                         ON CONFLICT (id) DO UPDATE SET
                         company_id = EXCLUDED.company_id,
                         proposal_number = EXCLUDED.proposal_number,
                         status = EXCLUDED.status,
                         total_amount = EXCLUDED.total_amount,
                         currency = EXCLUDED.currency,
                         valid_until = EXCLUDED.valid_until,
                         notes = EXCLUDED.notes,
                         updated_at = EXCLUDED.updated_at,
                         version = EXCLUDED.version,
                         is_deleted = EXCLUDED.is_deleted"
                    )
                    .bind(&item.id)
                    .bind(proposal.get("company_id").and_then(|v| v.as_str()).unwrap_or(""))
                    .bind(proposal.get("proposal_number").and_then(|v| v.as_str()))
                    .bind(proposal.get("status").and_then(|v| v.as_str()).unwrap_or("DRAFT"))
                    .bind(proposal.get("total_amount").and_then(|v| v.as_f64()).unwrap_or(0.0))
                    .bind(proposal.get("currency").and_then(|v| v.as_str()).unwrap_or("USD"))
                    .bind(proposal.get("valid_until").and_then(|v| v.as_str()).and_then(|s| s.parse::<i64>().ok()).and_then(|ts| DateTime::<Utc>::from_timestamp_millis(ts)))
                    .bind(proposal.get("notes").and_then(|v| v.as_str()))
                    .bind(proposal.get("created_at").and_then(|v| v.as_str()).and_then(|s| s.parse::<i64>().ok()).and_then(|ts| DateTime::<Utc>::from_timestamp_millis(ts)).unwrap_or_else(|| Utc::now()))
                    .bind(proposal.get("updated_at").and_then(|v| v.as_str()).and_then(|s| s.parse::<i64>().ok()).and_then(|ts| DateTime::<Utc>::from_timestamp_millis(ts)).unwrap_or_else(|| Utc::now()))
                    .bind(item.version)
                    .bind(item.is_deleted as i32)
                    .execute(pool)
                    .await?;
                }
            }
            "proposal_products" => {
                if let Ok(product) = serde_json::from_value::<serde_json::Value>(item.data.clone()) {
                    sqlx::query(
                        "INSERT INTO proposal_products (id, proposal_id, product_type, user_count, standalone_count, server_key_count, unit_price, total_price, annual_reduction, training, training_days, training_cost_per_day, training_cost, licence, support, support_years, updated_at, version, is_deleted) 
                         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
                         ON CONFLICT (id) DO UPDATE SET
                         proposal_id = EXCLUDED.proposal_id,
                         product_type = EXCLUDED.product_type,
                         user_count = EXCLUDED.user_count,
                         standalone_count = EXCLUDED.standalone_count,
                         server_key_count = EXCLUDED.server_key_count,
                         unit_price = EXCLUDED.unit_price,
                         total_price = EXCLUDED.total_price,
                         annual_reduction = EXCLUDED.annual_reduction,
                         training = EXCLUDED.training,
                         training_days = EXCLUDED.training_days,
                         training_cost_per_day = EXCLUDED.training_cost_per_day,
                         training_cost = EXCLUDED.training_cost,
                         licence = EXCLUDED.licence,
                         support = EXCLUDED.support,
                         support_years = EXCLUDED.support_years,
                         updated_at = EXCLUDED.updated_at,
                         version = EXCLUDED.version,
                         is_deleted = EXCLUDED.is_deleted"
                    )
                    .bind(&item.id)
                    .bind(product.get("proposal_id").and_then(|v| v.as_str()).unwrap_or(""))
                    .bind(product.get("product_type").and_then(|v| v.as_str()).unwrap_or(""))
                    .bind(product.get("user_count").and_then(|v| v.as_i64()).unwrap_or(0))
                    .bind(product.get("standalone_count").and_then(|v| v.as_i64()).unwrap_or(0))
                    .bind(product.get("server_key_count").and_then(|v| v.as_i64()).unwrap_or(0))
                    .bind(product.get("unit_price").and_then(|v| v.as_f64()).unwrap_or(0.0))
                    .bind(product.get("total_price").and_then(|v| v.as_f64()).unwrap_or(0.0))
                    .bind(product.get("annual_reduction").and_then(|v| v.as_f64()).unwrap_or(0.0))
                    .bind(product.get("training").and_then(|v| v.as_i64()).unwrap_or(0))
                    .bind(product.get("training_days").and_then(|v| v.as_i64()).unwrap_or(0))
                    .bind(product.get("training_cost_per_day").and_then(|v| v.as_f64()).unwrap_or(0.0))
                    .bind(product.get("training_cost").and_then(|v| v.as_f64()).unwrap_or(0.0))
                    .bind(product.get("licence").and_then(|v| v.as_i64()).unwrap_or(0))
                    .bind(product.get("support").and_then(|v| v.as_i64()).unwrap_or(0))
                    .bind(product.get("support_years").and_then(|v| v.as_i64()).unwrap_or(0))
                    .bind(product.get("updated_at").and_then(|v| v.as_str()).and_then(|s| s.parse::<i64>().ok()).and_then(|ts| DateTime::<Utc>::from_timestamp_millis(ts)).unwrap_or_else(|| Utc::now()))
                    .bind(item.version)
                    .bind(item.is_deleted as i32)
                    .execute(pool)
                    .await?;
                }
            }
            "invoices" => {
                if let Ok(invoice) = serde_json::from_value::<serde_json::Value>(item.data.clone()) {
                    sqlx::query(
                        "INSERT INTO invoices (id, proposal_id, invoice_number, status, total_amount, currency, issue_date, due_date, paid_date, purchase_order, purchase_order_date, commercial_in_charge, notes, created_at, updated_at, version, is_deleted) 
                         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
                         ON CONFLICT (id) DO UPDATE SET
                         proposal_id = EXCLUDED.proposal_id,
                         invoice_number = EXCLUDED.invoice_number,
                         status = EXCLUDED.status,
                         total_amount = EXCLUDED.total_amount,
                         currency = EXCLUDED.currency,
                         issue_date = EXCLUDED.issue_date,
                         due_date = EXCLUDED.due_date,
                         paid_date = EXCLUDED.paid_date,
                         purchase_order = EXCLUDED.purchase_order,
                         purchase_order_date = EXCLUDED.purchase_order_date,
                         commercial_in_charge = EXCLUDED.commercial_in_charge,
                         notes = EXCLUDED.notes,
                         updated_at = EXCLUDED.updated_at,
                         version = EXCLUDED.version,
                         is_deleted = EXCLUDED.is_deleted"
                    )
                    .bind(&item.id)
                    .bind(invoice.get("proposal_id").and_then(|v| v.as_str()).unwrap_or(""))
                    .bind(invoice.get("invoice_number").and_then(|v| v.as_str()).unwrap_or(""))
                    .bind(invoice.get("status").and_then(|v| v.as_str()).unwrap_or("DRAFT"))
                    .bind(invoice.get("total_amount").and_then(|v| v.as_f64()).unwrap_or(0.0))
                    .bind(invoice.get("currency").and_then(|v| v.as_str()).unwrap_or("USD"))
                    .bind(invoice.get("issue_date").and_then(|v| v.as_str()).unwrap_or("").parse::<i64>().ok().and_then(|ts| DateTime::<Utc>::from_timestamp_millis(ts)))
                    .bind(invoice.get("due_date").and_then(|v| v.as_str()).and_then(|s| s.parse::<i64>().ok()).and_then(|ts| DateTime::<Utc>::from_timestamp_millis(ts)))
                    .bind(invoice.get("paid_date").and_then(|v| v.as_str()).and_then(|s| s.parse::<i64>().ok()).and_then(|ts| DateTime::<Utc>::from_timestamp_millis(ts)))
                    .bind(invoice.get("purchase_order").and_then(|v| v.as_str()))
                    .bind(invoice.get("purchase_order_date").and_then(|v| v.as_str()).and_then(|s| s.parse::<i64>().ok()).and_then(|ts| DateTime::<Utc>::from_timestamp_millis(ts)))
                    .bind(invoice.get("commercial_in_charge").and_then(|v| v.as_str()))
                    .bind(invoice.get("notes").and_then(|v| v.as_str()))
                    .bind(invoice.get("created_at").and_then(|v| v.as_str()).and_then(|s| s.parse::<i64>().ok()).and_then(|ts| DateTime::<Utc>::from_timestamp_millis(ts)).unwrap_or_else(|| Utc::now()))
                    .bind(invoice.get("updated_at").and_then(|v| v.as_str()).and_then(|s| s.parse::<i64>().ok()).and_then(|ts| DateTime::<Utc>::from_timestamp_millis(ts)).unwrap_or_else(|| Utc::now()))
                    .bind(item.version)
                    .bind(item.is_deleted as i32)
                    .execute(pool)
                    .await?;
                }
            }
            "documents" => {
                if let Ok(document) = serde_json::from_value::<serde_json::Value>(item.data.clone()) {
                    sqlx::query(
                        "INSERT INTO documents (id, customer_id, title, document_type, file_path, content, created_at, updated_at, version, is_deleted) 
                         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                         ON CONFLICT (id) DO UPDATE SET
                         customer_id = EXCLUDED.customer_id,
                         title = EXCLUDED.title,
                         document_type = EXCLUDED.document_type,
                         file_path = EXCLUDED.file_path,
                         content = EXCLUDED.content,
                         updated_at = EXCLUDED.updated_at,
                         version = EXCLUDED.version,
                         is_deleted = EXCLUDED.is_deleted"
                    )
                    .bind(&item.id)
                    .bind(document.get("customer_id").and_then(|v| v.as_str()).unwrap_or(""))
                    .bind(document.get("title").and_then(|v| v.as_str()).unwrap_or(""))
                    .bind(document.get("document_type").and_then(|v| v.as_str()).unwrap_or(""))
                    .bind(document.get("file_path").and_then(|v| v.as_str()))
                    .bind(document.get("content").and_then(|v| v.as_str()))
                    .bind(document.get("created_at").and_then(|v| v.as_str()).and_then(|s| s.parse::<i64>().ok()).and_then(|ts| DateTime::<Utc>::from_timestamp_millis(ts)).unwrap_or_else(|| Utc::now()))
                    .bind(document.get("updated_at").and_then(|v| v.as_str()).and_then(|s| s.parse::<i64>().ok()).and_then(|ts| DateTime::<Utc>::from_timestamp_millis(ts)).unwrap_or_else(|| Utc::now()))
                    .bind(item.version)
                    .bind(item.is_deleted as i32)
                    .execute(pool)
                    .await?;
                }
            }
            _ => {
                return Err(sqlx::Error::Protocol("Unknown table name".to_string()));
            }
        }
    }
    Ok(())
}

// Helper function to get changed items from server since timestamp
async fn get_server_changes_since(pool: &PgPool, since_timestamp: i64) -> Result<Vec<SyncItem>, sqlx::Error> {
    let mut items = Vec::new();
    
    // Get companies
    let companies = sqlx::query(
        "SELECT * FROM companies WHERE EXTRACT(EPOCH FROM updated_at) * 1000 > $1 ORDER BY EXTRACT(EPOCH FROM updated_at) * 1000 ASC"
    )
    .bind(since_timestamp)
    .fetch_all(pool)
    .await?;

    for row in companies {
        let updated_at: chrono::DateTime<chrono::Utc> = row.get("updated_at");
        let updated_at_ts = updated_at.timestamp_millis();
        let version: i32 = row.get("version");
        let is_deleted: i32 = row.get("is_deleted");
        
        items.push(SyncItem {
            table_name: "companies".to_string(),
            id: row.get("id"),
            data: serde_json::json!({
                "id": row.get::<String, _>("id"),
                "name": row.get::<String, _>("name"),
                "website": row.get::<Option<String>, _>("website"),
                "address": row.get::<Option<String>, _>("address"),
                "city": row.get::<Option<String>, _>("city"),
                "postal_code": row.get::<Option<String>, _>("postal_code"),
                "country": row.get::<Option<String>, _>("country"),
                "description": row.get::<Option<String>, _>("description"),
                "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
                "updated_at": row.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").to_rfc3339(),
                "version": row.get::<i32, _>("version"),
                "is_deleted": row.get::<i32, _>("is_deleted")
            }),
            version: version as i64,
            is_deleted: is_deleted != 0,
            updated_at: updated_at_ts,
        });
    }

    // Get company_contacts
    let contacts = sqlx::query(
        "SELECT * FROM company_contacts WHERE EXTRACT(EPOCH FROM updated_at) * 1000 > $1 ORDER BY EXTRACT(EPOCH FROM updated_at) * 1000 ASC"
    )
    .bind(since_timestamp)
    .fetch_all(pool)
    .await?;

    for row in contacts {
        let updated_at: chrono::DateTime<chrono::Utc> = row.get("updated_at");
        let updated_at_ts = updated_at.timestamp_millis();
        let version: i32 = row.get("version");
        let is_deleted: i32 = row.get("is_deleted");
        
        items.push(SyncItem {
            table_name: "company_contacts".to_string(),
            id: row.get("id"),
            data: serde_json::json!({
                "id": row.get::<String, _>("id"),
                "company_id": row.get::<String, _>("company_id"),
                "first_name": row.get::<String, _>("first_name"),
                "last_name": row.get::<String, _>("last_name"),
                "email": row.get::<String, _>("email"),
                "phone_number": row.get::<Option<String>, _>("phone_number"),
                "is_primary": row.get::<bool, _>("is_primary") as i64,
                "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
                "updated_at": row.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").to_rfc3339(),
                "version": row.get::<i32, _>("version"),
                "is_deleted": row.get::<i32, _>("is_deleted")
            }),
            version: version as i64,
            is_deleted: is_deleted != 0,
            updated_at: updated_at_ts,
        });
    }

    // Get customers
    let customers = sqlx::query(
        "SELECT * FROM customers WHERE EXTRACT(EPOCH FROM updated_at) * 1000 > $1 ORDER BY EXTRACT(EPOCH FROM updated_at) * 1000 ASC"
    )
    .bind(since_timestamp)
    .fetch_all(pool)
    .await?;

    for row in customers {
        let updated_at: chrono::DateTime<chrono::Utc> = row.get("updated_at");
        let updated_at_ts = updated_at.timestamp_millis();
        let version: i32 = row.get("version");
        let is_deleted: i32 = row.get("is_deleted");
        
        items.push(SyncItem {
            table_name: "customers".to_string(),
            id: row.get("id"),
            data: serde_json::json!({
                "id": row.get::<String, _>("id"),
                "name": row.get::<String, _>("name"),
                "email": row.get::<Option<String>, _>("email"),
                "phone": row.get::<Option<String>, _>("phone"),
                "address": row.get::<Option<String>, _>("address"),
                "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
                "updated_at": row.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").to_rfc3339(),
                "version": row.get::<i32, _>("version"),
                "is_deleted": row.get::<i32, _>("is_deleted")
            }),
            version: version as i64,
            is_deleted: is_deleted != 0,
            updated_at: updated_at_ts,
        });
    }

    // Get proposals
    let proposals = sqlx::query(
        "SELECT * FROM proposals WHERE EXTRACT(EPOCH FROM updated_at) * 1000 > $1 ORDER BY EXTRACT(EPOCH FROM updated_at) * 1000 ASC"
    )
    .bind(since_timestamp)
    .fetch_all(pool)
    .await?;

    for row in proposals {
        let updated_at: chrono::DateTime<chrono::Utc> = row.get("updated_at");
        let updated_at_ts = updated_at.timestamp_millis();
        let version: i32 = row.get("version");
        let is_deleted: i32 = row.get("is_deleted");
        
        items.push(SyncItem {
            table_name: "proposals".to_string(),
            id: row.get("id"),
            data: serde_json::json!({
                "id": row.get::<String, _>("id"),
                "company_id": row.get::<String, _>("company_id"),
                "proposal_number": row.get::<Option<String>, _>("proposal_number"),
                "status": row.get::<String, _>("status"),
                "total_amount": row.get::<f64, _>("total_amount"),
                "currency": row.get::<String, _>("currency"),
                "valid_until": row.get::<Option<String>, _>("valid_until"),
                "notes": row.get::<Option<String>, _>("notes"),
                "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
                "updated_at": row.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").to_rfc3339(),
                "version": row.get::<i32, _>("version"),
                "is_deleted": row.get::<i32, _>("is_deleted")
            }),
            version: version as i64,
            is_deleted: is_deleted != 0,
            updated_at: updated_at_ts,
        });
    }

    // Get proposal_products
    let products = sqlx::query(
        "SELECT * FROM proposal_products WHERE EXTRACT(EPOCH FROM updated_at) * 1000 > $1 ORDER BY EXTRACT(EPOCH FROM updated_at) * 1000 ASC"
    )
    .bind(since_timestamp)
    .fetch_all(pool)
    .await?;

    for row in products {
        let updated_at: chrono::DateTime<chrono::Utc> = row.get("updated_at");
        let updated_at_ts = updated_at.timestamp_millis();
        let version: i32 = row.get("version");
        let is_deleted: i32 = row.get("is_deleted");
        
        items.push(SyncItem {
            table_name: "proposal_products".to_string(),
            id: row.get("id"),
            data: serde_json::json!({
                "id": row.get::<String, _>("id"),
                "proposal_id": row.get::<String, _>("proposal_id"),
                "product_type": row.get::<String, _>("product_type"),
                "user_count": row.get::<i32, _>("user_count") as i64,
                "standalone_count": row.get::<i32, _>("standalone_count") as i64,
                "server_key_count": row.get::<i32, _>("server_key_count") as i64,
                "unit_price": row.get::<f64, _>("unit_price"),
                "total_price": row.get::<f64, _>("total_price"),
                "annual_reduction": row.get::<f64, _>("annual_reduction"),
                "training": row.get::<i32, _>("training") as i64,
                "training_days": row.get::<i32, _>("training_days") as i64,
                "training_cost_per_day": row.get::<f64, _>("training_cost_per_day"),
                "training_cost": row.get::<f64, _>("training_cost"),
                "licence": row.get::<i32, _>("licence") as i64,
                "support": row.get::<i32, _>("support") as i64,
                "support_years": row.get::<i32, _>("support_years") as i64,
                "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").to_rfc3339(),
                "updated_at": row.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").to_rfc3339(),
                "version": row.get::<i32, _>("version"),
                "is_deleted": row.get::<i32, _>("is_deleted")
            }),
            version: version as i64,
            is_deleted: is_deleted != 0,
            updated_at: updated_at_ts,
        });
    }

    // Get invoices
    let invoices = sqlx::query(
        "SELECT * FROM invoices WHERE EXTRACT(EPOCH FROM updated_at) * 1000 > $1 ORDER BY EXTRACT(EPOCH FROM updated_at) * 1000 ASC"
    )
    .bind(since_timestamp)
    .fetch_all(pool)
    .await?;

    for row in invoices {
        let updated_at: String = row.get("updated_at");
        let updated_at_ts = updated_at.parse::<i64>().unwrap_or(0);
        let version: i32 = row.get("version");
        let is_deleted: i32 = row.get("is_deleted");
        
        items.push(SyncItem {
            table_name: "invoices".to_string(),
            id: row.get("id"),
            data: serde_json::json!({
                "id": row.get::<String, _>("id"),
                "proposal_id": row.get::<String, _>("proposal_id"),
                "invoice_number": row.get::<String, _>("invoice_number"),
                "status": row.get::<String, _>("status"),
                "total_amount": row.get::<f64, _>("total_amount"),
                "currency": row.get::<String, _>("currency"),
                "issue_date": row.get::<String, _>("issue_date"),
                "due_date": row.get::<Option<String>, _>("due_date"),
                "paid_date": row.get::<Option<String>, _>("paid_date"),
                "purchase_order": row.get::<Option<String>, _>("purchase_order"),
                "purchase_order_date": row.get::<Option<String>, _>("purchase_order_date"),
                "commercial_in_charge": row.get::<Option<String>, _>("commercial_in_charge"),
                "notes": row.get::<Option<String>, _>("notes"),
                "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
                "updated_at": row.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").to_rfc3339(),
                "version": row.get::<i32, _>("version"),
                "is_deleted": row.get::<i32, _>("is_deleted")
            }),
            version: version as i64,
            is_deleted: is_deleted != 0,
            updated_at: updated_at_ts,
        });
    }

    // Get documents
    let documents = sqlx::query(
        "SELECT * FROM documents WHERE EXTRACT(EPOCH FROM updated_at) * 1000 > $1 ORDER BY EXTRACT(EPOCH FROM updated_at) * 1000 ASC"
    )
    .bind(since_timestamp)
    .fetch_all(pool)
    .await?;

    for row in documents {
        let updated_at: String = row.get("updated_at");
        let updated_at_ts = updated_at.parse::<i64>().unwrap_or(0);
        let version: i32 = row.get("version");
        let is_deleted: i32 = row.get("is_deleted");
        
        items.push(SyncItem {
            table_name: "documents".to_string(),
            id: row.get("id"),
            data: serde_json::json!({
                "id": row.get::<String, _>("id"),
                "customer_id": row.get::<String, _>("customer_id"),
                "title": row.get::<String, _>("title"),
                "document_type": row.get::<String, _>("document_type"),
                "file_path": row.get::<Option<String>, _>("file_path"),
                "content": row.get::<Option<String>, _>("content"),
                "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
                "updated_at": row.get::<chrono::DateTime<chrono::Utc>, _>("updated_at").to_rfc3339(),
                "version": row.get::<i32, _>("version"),
                "is_deleted": row.get::<i32, _>("is_deleted")
            }),
            version: version as i64,
            is_deleted: is_deleted != 0,
            updated_at: updated_at_ts,
        });
    }

    Ok(items)
}

pub async fn sync_all(pool: web::Data<PgPool>, request: web::Json<SyncRequest>) -> Result<HttpResponse> {
    println!("🔄 [ServerSync] Received sync request with {} changes", request.changes.len());
    
    let mut items_synced = 0;
    let mut errors = Vec::new();
    
    // Apply client changes to server
    for item in &request.changes {
        match apply_sync_item_to_server(&pool, item).await {
            Ok(_) => {
                items_synced += 1;
                println!("✅ [ServerSync] Applied client change: {} {}", item.table_name, item.id);
            }
            Err(e) => {
                let error_msg = format!("Failed to apply {} {}: {}", item.table_name, item.id, e);
                errors.push(error_msg.clone());
                eprintln!("❌ [ServerSync] {}", error_msg);
            }
        }
    }
    
    // Get server changes since last sync
    let remote_changes = match get_server_changes_since(&pool, request.last_sync_timestamp).await {
        Ok(changes) => {
            println!("🔄 [ServerSync] Found {} server changes", changes.len());
            changes
        }
        Err(e) => {
            eprintln!("❌ [ServerSync] Failed to get server changes: {}", e);
            return Ok(HttpResponse::InternalServerError().json(SyncResponse {
                success: false,
                message: format!("Failed to get server changes: {}", e),
                remote_changes: vec![],
                new_timestamp: request.last_sync_timestamp,
                items_synced: 0,
            }));
        }
    };
    
    let new_timestamp = current_timestamp();
    let success = errors.is_empty();
    let message = if success {
        format!("Synchronization completed successfully. {} items synced.", items_synced)
    } else {
        format!("Synchronization completed with {} errors. {} items synced.", errors.len(), items_synced)
    };
    
    println!("✅ [ServerSync] Sync completed. Success: {}, Items synced: {}, Remote changes: {}", 
             success, items_synced, remote_changes.len());
    
    Ok(HttpResponse::Ok().json(SyncResponse {
        success,
        message,
        remote_changes,
        new_timestamp,
        items_synced,
    }))
}
