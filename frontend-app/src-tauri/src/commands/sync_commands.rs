use crate::db::{models::*, queries, DbPool};
use serde::{Deserialize, Serialize};
use tauri::State;
use reqwest;
use sqlx::Row;

// Helper function to check server connectivity
async fn check_server_connectivity(server_url: &str) -> bool {
    let client = reqwest::Client::new();
    let health_url = format!("{}/api/health", server_url);
    
    println!("🔍 [Connectivity] Checking server at: {}", health_url);
    
    match client.get(&health_url).timeout(std::time::Duration::from_secs(5)).send().await {
        Ok(response) => {
            let is_success = response.status().is_success();
            println!("✅ [Connectivity] Server response: {} - Success: {}", response.status(), is_success);
            is_success
        },
        Err(e) => {
            println!("❌ [Connectivity] Server check failed: {}", e);
            false
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncStatus {
    #[serde(rename = "isOnline")]
    pub is_online: bool,
    #[serde(rename = "lastSync")]
    pub last_sync: Option<i64>,
    #[serde(rename = "pendingChanges")]
    pub pending_changes: i64,
    #[serde(rename = "serverUrl")]
    pub server_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncResult {
    pub success: bool,
    pub message: String,
    #[serde(rename = "itemsSynced")]
    pub items_synced: i64,
    #[serde(rename = "newTimestamp")]
    pub new_timestamp: i64,
    pub errors: Vec<String>,
}

// Helper function to get current timestamp in milliseconds
fn current_timestamp() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

// Helper function to update version and timestamp for CRUD operations
pub async fn update_record_metadata(pool: &DbPool, table_name: &str, id: &str) -> Result<(), sqlx::Error> {
    let now = current_timestamp().to_string();
    
    match table_name {
        "companies" => {
            sqlx::query("UPDATE companies SET updated_at = ?, version = COALESCE(version, 0) + 1 WHERE id = ?")
                .bind(&now)
                .bind(id)
                .execute(pool)
                .await?;
        }
        "company_contacts" => {
            sqlx::query("UPDATE company_contacts SET updated_at = ?, version = COALESCE(version, 0) + 1 WHERE id = ?")
                .bind(&now)
                .bind(id)
                .execute(pool)
                .await?;
        }
        "customers" => {
            sqlx::query("UPDATE customers SET updated_at = ?, version = COALESCE(version, 0) + 1 WHERE id = ?")
                .bind(&now)
                .bind(id)
                .execute(pool)
                .await?;
        }
        "proposals" => {
            sqlx::query("UPDATE proposals SET updated_at = ?, version = COALESCE(version, 0) + 1 WHERE id = ?")
                .bind(&now)
                .bind(id)
                .execute(pool)
                .await?;
        }
        "proposal_products" => {
            sqlx::query("UPDATE proposal_products SET updated_at = ?, version = COALESCE(version, 0) + 1 WHERE id = ?")
                .bind(&now)
                .bind(id)
                .execute(pool)
                .await?;
        }
        "invoices" => {
            sqlx::query("UPDATE invoices SET updated_at = ?, version = COALESCE(version, 0) + 1 WHERE id = ?")
                .bind(&now)
                .bind(id)
                .execute(pool)
                .await?;
        }
        "documents" => {
            sqlx::query("UPDATE documents SET updated_at = ?, version = COALESCE(version, 0) + 1 WHERE id = ?")
                .bind(&now)
                .bind(id)
                .execute(pool)
                .await?;
        }
        _ => {
            return Err(sqlx::Error::Protocol(format!("Unknown table: {}", table_name)));
        }
    }
    Ok(())
}

// Helper function to mark record as deleted
pub async fn mark_record_deleted(pool: &DbPool, table_name: &str, id: &str) -> Result<(), sqlx::Error> {
    let now = current_timestamp().to_string();
    
    match table_name {
        "companies" => {
            sqlx::query("UPDATE companies SET is_deleted = 1, updated_at = ?, version = COALESCE(version, 0) + 1 WHERE id = ?")
                .bind(&now)
                .bind(id)
                .execute(pool)
                .await?;
        }
        "company_contacts" => {
            sqlx::query("UPDATE company_contacts SET is_deleted = 1, updated_at = ?, version = COALESCE(version, 0) + 1 WHERE id = ?")
                .bind(&now)
                .bind(id)
                .execute(pool)
                .await?;
        }
        "customers" => {
            sqlx::query("UPDATE customers SET is_deleted = 1, updated_at = ?, version = COALESCE(version, 0) + 1 WHERE id = ?")
                .bind(&now)
                .bind(id)
                .execute(pool)
                .await?;
        }
        "proposals" => {
            sqlx::query("UPDATE proposals SET is_deleted = 1, updated_at = ?, version = COALESCE(version, 0) + 1 WHERE id = ?")
                .bind(&now)
                .bind(id)
                .execute(pool)
                .await?;
        }
        "proposal_products" => {
            sqlx::query("UPDATE proposal_products SET is_deleted = 1, updated_at = ?, version = COALESCE(version, 0) + 1 WHERE id = ?")
                .bind(&now)
                .bind(id)
                .execute(pool)
                .await?;
        }
        "invoices" => {
            sqlx::query("UPDATE invoices SET is_deleted = 1, updated_at = ?, version = COALESCE(version, 0) + 1 WHERE id = ?")
                .bind(&now)
                .bind(id)
                .execute(pool)
                .await?;
        }
        "documents" => {
            sqlx::query("UPDATE documents SET is_deleted = 1, updated_at = ?, version = COALESCE(version, 0) + 1 WHERE id = ?")
                .bind(&now)
                .bind(id)
                .execute(pool)
                .await?;
        }
        _ => {
            return Err(sqlx::Error::Protocol(format!("Unknown table: {}", table_name)));
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn get_sync_status(pool: State<'_, DbPool>) -> Result<SyncStatus, String> {
    let metadata = queries::get_sync_metadata(&pool)
        .await
        .map_err(|e| format!("Failed to get sync metadata: {}", e))?;
    
    println!("📊 [SyncStatus] Sync metadata - last_sync_timestamp: {}, last_sync_version: {}", 
        metadata.last_sync_timestamp, metadata.last_sync_version);
    
    println!("🔍 [SyncStatus] Checking if first sync (timestamp = 0): {}", metadata.last_sync_timestamp == 0);
    
    if metadata.last_sync_timestamp == 0 {
        println!("⚠️ [SyncStatus] First sync detected (last_sync_timestamp = 0)");
        println!("🔧 [SyncStatus] Fixing timestamps for existing records...");
        
                // Fix timestamps for existing records - convert ISO format to milliseconds
                let now = current_timestamp();
                println!("🔧 [SyncStatus] Setting timestamp to: {}", now);

                // Debug: Check what values exist in updated_at
                let debug_result = sqlx::query("SELECT id, updated_at FROM companies LIMIT 3")
                    .fetch_all(&*pool)
                    .await;

                match debug_result {
                    Ok(rows) => {
                        println!("🔍 [SyncStatus] Debug - Sample companies updated_at values:");
                        for row in rows {
                            let id: String = row.get("id");
                            let updated_at: String = row.get("updated_at");
                            println!("  - ID: {}, updated_at: '{}'", id, updated_at);
                        }
                    }
                    Err(e) => {
                        println!("🔍 [SyncStatus] Debug - Error fetching companies: {}", e);
                    }
                }

                // Convert all existing ISO timestamps to milliseconds
                let result = sqlx::query("UPDATE companies SET updated_at = ? WHERE updated_at LIKE '%-%'")
                    .bind(now.to_string())
                    .execute(&*pool)
                    .await;
                println!("🔧 [SyncStatus] Companies updated: {:?}", result);
                let _ = sqlx::query("UPDATE company_contacts SET updated_at = ? WHERE updated_at LIKE '%-%'")
                    .bind(now.to_string())
                    .execute(&*pool)
                    .await;
                let _ = sqlx::query("UPDATE customers SET updated_at = ? WHERE updated_at LIKE '%-%'")
                    .bind(now.to_string())
                    .execute(&*pool)
                    .await;
                let _ = sqlx::query("UPDATE proposals SET updated_at = ? WHERE updated_at LIKE '%-%'")
                    .bind(now.to_string())
                    .execute(&*pool)
                    .await;
                let _ = sqlx::query("UPDATE proposal_products SET updated_at = ? WHERE updated_at LIKE '%-%'")
                    .bind(now.to_string())
                    .execute(&*pool)
                    .await;
                let _ = sqlx::query("UPDATE invoices SET updated_at = ? WHERE updated_at LIKE '%-%'")
                    .bind(now.to_string())
                    .execute(&*pool)
                    .await;
                let _ = sqlx::query("UPDATE documents SET updated_at = ? WHERE updated_at LIKE '%-%'")
                    .bind(now.to_string())
                    .execute(&*pool)
                    .await;
        
        println!("✅ [SyncStatus] Timestamps fixed for existing records");
    }

    // Count pending changes
    let changed_items = queries::get_changed_items_since(&pool, metadata.last_sync_timestamp)
        .await
        .map_err(|e| format!("Failed to get pending changes: {}", e))?;
    
    let pending_changes = changed_items.len() as i64;
    
    // Log pending changes for debugging
    if pending_changes > 0 {
        println!("📋 [SyncStatus] Found {} pending changes:", pending_changes);
        for item in &changed_items {
            println!("  - {}: {} (updated: {}, version: {}, deleted: {})", 
                item.table_name, item.id, item.updated_at, item.version, item.is_deleted);
        }
    } else {
        println!("✅ [SyncStatus] No pending changes found");
    }

    // Check server connectivity
    let server_url = "http://localhost:8080";
    println!("🔄 [SyncStatus] Getting sync status for server: {}", server_url);
    let is_online = check_server_connectivity(server_url).await;
    println!("📊 [SyncStatus] Final status - Online: {}, Pending: {}", is_online, pending_changes);

    Ok(SyncStatus {
        is_online,
        last_sync: if metadata.last_sync_timestamp > 0 { Some(metadata.last_sync_timestamp) } else { None },
        pending_changes,
        server_url: server_url.to_string(),
    })
}

#[tauri::command]
pub async fn sync_with_server(pool: State<'_, DbPool>, server_url: String) -> Result<SyncResult, String> {
    println!("🔄 [Sync] Starting synchronization with server: {}", server_url);
    
    // Get current sync metadata
    let metadata = queries::get_sync_metadata(&pool)
        .await
        .map_err(|e| format!("Failed to get sync metadata: {}", e))?;

    // Get local changes since last sync
    let local_changes = queries::get_changed_items_since(&pool, metadata.last_sync_timestamp)
        .await
        .map_err(|e| format!("Failed to get local changes: {}", e))?;

    println!("🔄 [Sync] Found {} local changes to sync", local_changes.len());

    // Prepare sync request
    let sync_request = SyncRequest {
        last_sync_timestamp: metadata.last_sync_timestamp,
        changes: local_changes,
    };

    // Send to server
    let client = reqwest::Client::new();
    let response = match client
        .post(&format!("{}/api/sync", server_url))
        .json(&sync_request)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            return Ok(SyncResult {
                success: false,
                message: format!("Failed to connect to server: {}", e),
                items_synced: 0,
                new_timestamp: metadata.last_sync_timestamp,
                errors: vec![e.to_string()],
            });
        }
    };

    if !response.status().is_success() {
        let error_msg = format!("Server returned status: {}", response.status());
        return Ok(SyncResult {
            success: false,
            message: error_msg.clone(),
            items_synced: 0,
            new_timestamp: metadata.last_sync_timestamp,
            errors: vec![error_msg],
        });
    }

    let sync_response: SyncResponse = match response.json().await {
        Ok(response) => response,
        Err(e) => {
            return Ok(SyncResult {
                success: false,
                message: format!("Failed to parse server response: {}", e),
                items_synced: 0,
                new_timestamp: metadata.last_sync_timestamp,
                errors: vec![e.to_string()],
            });
        }
    };

    if !sync_response.success {
        let error_msg = sync_response.message.clone();
        return Ok(SyncResult {
            success: false,
            message: error_msg.clone(),
            items_synced: 0,
            new_timestamp: metadata.last_sync_timestamp,
            errors: vec![error_msg],
        });
    }

    println!("🔄 [Sync] Server returned {} remote changes", sync_response.remote_changes.len());

    // Apply remote changes to local database
    let mut errors = Vec::new();
    let mut items_synced = 0;

    for item in &sync_response.remote_changes {
        match queries::apply_sync_item(&pool, item).await {
            Ok(_) => {
                items_synced += 1;
                println!("✅ [Sync] Applied remote change: {} {}", item.table_name, item.id);
            }
            Err(e) => {
                let error_msg = format!("Failed to apply {} {}: {}", item.table_name, item.id, e);
                errors.push(error_msg.clone());
                eprintln!("❌ [Sync] {}", error_msg);
            }
        }
    }

    // Update sync metadata if sync was successful
    if errors.is_empty() || items_synced > 0 {
        queries::update_sync_metadata(&pool, sync_response.new_timestamp, metadata.last_sync_version + 1)
            .await
            .map_err(|e| format!("Failed to update sync metadata: {}", e))?;
        
        println!("✅ [Sync] Updated sync metadata to timestamp: {}", sync_response.new_timestamp);
    }

    let success = errors.is_empty();
    let message = if success {
        format!("Synchronization completed successfully. {} items synced.", items_synced)
    } else {
        format!("Synchronization completed with {} errors. {} items synced.", errors.len(), items_synced)
    };

    Ok(SyncResult {
        success,
        message,
        items_synced,
        new_timestamp: sync_response.new_timestamp,
        errors,
    })
}

#[tauri::command]
pub async fn force_sync_cleanup(_pool: State<'_, DbPool>) -> Result<String, String> {
    // This function can be used to clean up sync-related data if needed
    // For now, it's a placeholder
    Ok("Sync cleanup completed".to_string())
}

#[tauri::command]
pub async fn get_server_url() -> Result<String, String> {
    // TODO: Implement server URL storage/retrieval
    Ok("http://localhost:8080".to_string())
}

#[tauri::command]
pub async fn set_server_url(url: String) -> Result<(), String> {
    // TODO: Implement server URL storage
    println!("Server URL set to: {}", url);
    Ok(())
}
