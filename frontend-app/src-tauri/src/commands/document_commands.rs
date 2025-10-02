use crate::db::{models::Document, queries, DbPool};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDocumentRequest {
    pub customer_id: String,
    pub title: String,
    pub document_type: String,
    pub file_path: Option<String>,
    pub content: Option<String>,
}

#[tauri::command]
pub async fn get_documents(pool: State<'_, DbPool>) -> Result<Vec<Document>, String> {
    queries::get_all_documents(&pool)
        .await
        .map_err(|e| format!("Failed to get documents: {}", e))
}

#[tauri::command]
pub async fn get_customer_documents(
    pool: State<'_, DbPool>,
    customer_id: String,
) -> Result<Vec<Document>, String> {
    queries::get_documents_by_customer(&pool, &customer_id)
        .await
        .map_err(|e| format!("Failed to get customer documents: {}", e))
}

#[tauri::command]
pub async fn create_document(
    pool: State<'_, DbPool>,
    request: CreateDocumentRequest,
) -> Result<Document, String> {
    let document = queries::create_document(
        &pool,
        request.customer_id,
        request.title,
        request.document_type,
        request.file_path,
        request.content,
    )
    .await
    .map_err(|e| format!("Failed to create document: {}", e))?;


    Ok(document)
}

#[tauri::command]
pub async fn delete_document(pool: State<'_, DbPool>, id: String) -> Result<(), String> {

    queries::delete_document(&pool, &id)
        .await
        .map_err(|e| format!("Failed to delete document: {}", e))
}

