use crate::db::{models::{Company, CompanyWithContacts}, queries, DbPool};
use crate::commands::sync_commands::{update_record_metadata, mark_record_deleted};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactData {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub is_primary: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCompanyRequest {
    pub name: String,
    pub website: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub description: Option<String>,
    pub contacts: Vec<ContactData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCompanyRequest {
    pub id: String,
    pub name: String,
    pub website: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub description: Option<String>,
    pub contacts: Vec<ContactData>,
}

#[tauri::command]
pub async fn get_companies(pool: State<'_, DbPool>) -> Result<Vec<CompanyWithContacts>, String> {
    let companies = queries::get_all_companies(&pool)
        .await
        .map_err(|e| format!("Failed to get companies: {}", e))?;

    let mut companies_with_contacts = Vec::new();
    for company in companies {
        let contacts = queries::get_company_contacts(&pool, &company.id)
            .await
            .map_err(|e| format!("Failed to get contacts: {}", e))?;
        
        companies_with_contacts.push(CompanyWithContacts {
            company,
            contacts,
        });
    }

    Ok(companies_with_contacts)
}

#[tauri::command]
pub async fn get_company(pool: State<'_, DbPool>, id: String) -> Result<Option<CompanyWithContacts>, String> {
    let company = queries::get_company_by_id(&pool, &id)
        .await
        .map_err(|e| format!("Failed to get company: {}", e))?;

    match company {
        Some(comp) => {
            let contacts = queries::get_company_contacts(&pool, &comp.id)
                .await
                .map_err(|e| format!("Failed to get contacts: {}", e))?;
            
            Ok(Some(CompanyWithContacts {
                company: comp,
                contacts,
            }))
        }
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn create_company(
    pool: State<'_, DbPool>,
    request: CreateCompanyRequest,
) -> Result<Company, String> {
    let company = queries::create_company(
        &pool,
        request.name,
        request.website,
        request.address,
        request.city,
        request.postal_code,
        request.country,
        request.description,
    )
    .await
    .map_err(|e| format!("Failed to create company: {}", e))?;

    // Create contacts
    for contact_data in request.contacts {
        let contact = queries::create_company_contact(
            &pool,
            company.id.clone(),
            contact_data.first_name,
            contact_data.last_name,
            contact_data.email,
            contact_data.phone_number,
            contact_data.is_primary,
        )
        .await
        .map_err(|e| format!("Failed to create contact: {}", e))?;

        // Update contact metadata for sync
        update_record_metadata(&pool, "company_contacts", &contact.id).await
            .map_err(|e| format!("Failed to update contact metadata: {}", e))?;
    }

    // Update company metadata for sync
    update_record_metadata(&pool, "companies", &company.id).await
        .map_err(|e| format!("Failed to update company metadata: {}", e))?;

    Ok(company)
}

#[tauri::command]
pub async fn update_company(
    pool: State<'_, DbPool>,
    request: UpdateCompanyRequest,
) -> Result<Company, String> {
    let company = queries::update_company(
        &pool,
        &request.id,
        request.name,
        request.website,
        request.address,
        request.city,
        request.postal_code,
        request.country,
        request.description,
    )
    .await
    .map_err(|e| format!("Failed to update company: {}", e))?;

    // Delete old contacts and create new ones
    queries::delete_company_contacts(&pool, &request.id)
        .await
        .map_err(|e| format!("Failed to delete old contacts: {}", e))?;

    for contact_data in request.contacts {
        let contact = queries::create_company_contact(
            &pool,
            company.id.clone(),
            contact_data.first_name,
            contact_data.last_name,
            contact_data.email,
            contact_data.phone_number,
            contact_data.is_primary,
        )
        .await
        .map_err(|e| format!("Failed to create contact: {}", e))?;

        // Update contact metadata for sync
        update_record_metadata(&pool, "company_contacts", &contact.id).await
            .map_err(|e| format!("Failed to update contact metadata: {}", e))?;
    }

    // Update company metadata for sync
    update_record_metadata(&pool, "companies", &company.id).await
        .map_err(|e| format!("Failed to update company metadata: {}", e))?;

    Ok(company)
}

#[tauri::command]
pub async fn delete_company(pool: State<'_, DbPool>, id: String) -> Result<(), String> {
    // Mark as deleted instead of hard delete for sync
    mark_record_deleted(&pool, "companies", &id).await
        .map_err(|e| format!("Failed to mark company as deleted: {}", e))?;

    // Also mark all contacts as deleted
    let contacts = queries::get_company_contacts(&pool, &id).await
        .map_err(|e| format!("Failed to get company contacts: {}", e))?;
    
    for contact in contacts {
        mark_record_deleted(&pool, "company_contacts", &contact.id).await
            .map_err(|e| format!("Failed to mark contact as deleted: {}", e))?;
    }

    Ok(())
}

