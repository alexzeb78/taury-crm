use crate::db::{queries, DbPool};
use tauri::State;
use serde_json::json;

const DOC_API_URL: &str = "http://127.0.0.1:8001";

#[tauri::command]
pub async fn generate_proposal_word(
    pool: State<'_, DbPool>,
    proposal_id: String,
) -> Result<String, String> {
    // Get proposal data
    let proposal = queries::get_proposal_by_id(&pool, &proposal_id)
        .await
        .map_err(|e| format!("Failed to get proposal: {}", e))?
        .ok_or("Proposal not found")?;
    
    let products = queries::get_proposal_products(&pool, &proposal_id)
        .await
        .map_err(|e| format!("Failed to get products: {}", e))?;
    
    let company = queries::get_company_by_id(&pool, &proposal.company_id)
        .await
        .map_err(|e| format!("Failed to get company: {}", e))?
        .ok_or("Company not found")?;
    
    let contacts = queries::get_company_contacts(&pool, &company.id)
        .await
        .map_err(|e| format!("Failed to get contacts: {}", e))?;
    
    let primary_contact = contacts.first();
    
    // Préparer les données pour l'API
    let products_data: Vec<_> = products.iter().map(|p| {
        json!({
            "product_type": p.product_type,
            "user_count": p.user_count,
            "standalone_count": p.standalone_count,
            "server_key_count": p.server_key_count,
            "unit_price": p.unit_price,
            "total_price": p.total_price,
            "annual_reduction": p.annual_reduction,
            "licence": p.licence == 1,
            "training": p.training == 1,
            "training_days": p.training_days,
            "training_cost_per_day": p.training_cost_per_day,
            "support": p.support == 1,
            "support_years": p.support_years,
        })
    }).collect();
    
    let proposal_num = proposal.proposal_number.clone().unwrap_or("DRAFT".to_string());
    
    let payload = json!({
        "proposal_number": &proposal_num,
        "company": {
            "name": company.name,
            "address": company.address,
            "city": company.city,
            "postal_code": company.postal_code,
            "country": company.country,
        },
        "contact": primary_contact.map(|c| json!({
            "first_name": c.first_name,
            "last_name": c.last_name,
            "email": c.email,
            "phone_number": c.phone_number,
        })),
        "products": products_data,
        "currency": proposal.currency,
        "valid_until": proposal.valid_until,
        "notes": proposal.notes,
    });
    
    // Appeler l'API Python locale
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/generate-word", DOC_API_URL))
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to call document API. Is the sidecar running? Error: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Document API error: {}", response.status()));
    }
    
    // Sauvegarder le fichier
    let filename = format!("Proposal_{}.docx", proposal_num);
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let output_path = home_dir.join("Downloads").join(&filename);
    
    let bytes = response.bytes().await
        .map_err(|e| format!("Failed to read response: {}", e))?;
    
    std::fs::write(&output_path, bytes)
        .map_err(|e| format!("Failed to save file: {}", e))?;
    
    Ok(output_path.to_string_lossy().to_string())
}
