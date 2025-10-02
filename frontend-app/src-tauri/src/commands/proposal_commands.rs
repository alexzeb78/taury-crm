use crate::db::{models::*, queries, DbPool};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalProductData {
    pub id: Option<String>,
    pub product_type: String,
    pub user_count: i64,
    pub standalone_count: i64,
    pub server_key_count: i64,
    pub annual_reduction: f64,
    pub training: bool,
    pub training_days: i64,
    pub training_cost_per_day: f64,
    pub licence: bool,
    pub support: bool,
    pub support_years: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProposalRequest {
    pub company_id: String,
    pub status: String,
    pub currency: String,
    pub valid_until: Option<String>,
    pub notes: Option<String>,
    pub products: Vec<ProposalProductData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProposalRequest {
    pub id: String,
    pub company_id: String,
    pub status: String,
    pub currency: String,
    pub valid_until: Option<String>,
    pub notes: Option<String>,
    pub products: Vec<ProposalProductData>,
}

#[tauri::command]
pub async fn get_proposals(pool: State<'_, DbPool>) -> Result<Vec<ProposalWithProducts>, String> {
    let proposals = queries::get_all_proposals(&pool)
        .await
        .map_err(|e| format!("Failed to get proposals: {}", e))?;

    println!("🔄 [get_proposals] Found {} proposals in database", proposals.len());

    let mut proposals_with_products = Vec::new();
    for proposal in proposals {
        println!("🔄 [get_proposals] Processing proposal: {} (ID: {})", 
                 proposal.proposal_number.as_ref().unwrap_or(&"Unknown".to_string()), 
                 proposal.id);
        
        let products = queries::get_proposal_products(&pool, &proposal.id)
            .await
            .map_err(|e| format!("Failed to get products: {}", e))?;
        
        let company = queries::get_company_by_id(&pool, &proposal.company_id)
            .await
            .map_err(|e| format!("Failed to get company: {}", e))?;
        
        let company_name = company.map(|c| c.name).unwrap_or_else(|| "Unknown".to_string());
        
        println!("🔄 [get_proposals] Proposal {} has {} products, company: {}", 
                 proposal.proposal_number.as_ref().unwrap_or(&"Unknown".to_string()), 
                 products.len(), 
                 company_name);
        
        proposals_with_products.push(ProposalWithProducts {
            proposal,
            products,
            company_name,
        });
    }

    println!("✅ [get_proposals] Returning {} proposals with products", proposals_with_products.len());
    for (i, pwp) in proposals_with_products.iter().enumerate() {
        println!("  [{}] Proposal: {} (Company: {})", 
                 i, 
                 pwp.proposal.proposal_number.as_ref().unwrap_or(&"Unknown".to_string()),
                 pwp.company_name);
    }
    Ok(proposals_with_products)
}

#[tauri::command]
pub async fn create_proposal(
    pool: State<'_, DbPool>,
    request: CreateProposalRequest,
) -> Result<Proposal, String> {
    // Create proposal
    let proposal = queries::create_proposal(
        &pool,
        request.company_id.clone(),
        request.status,
        request.currency,
        request.valid_until,
        request.notes,
    )
    .await
    .map_err(|e| format!("Failed to create proposal: {}", e))?;

    // Calculate and create products
    let mut total_amount = 0.0;
    let mut has_training = false;
    let mut has_support = false;
    let mut has_licence = false;
    let mut first_product_type = String::new();

    for product_data in request.products {
        // IMPORTANT: Recalculate user_count from standalone + server_key (don't trust frontend)
        let user_count = product_data.standalone_count + product_data.server_key_count;
        
        // Calculate base price using the correct user_count
        let base_price = queries::calculate_price(&pool, &product_data.product_type, user_count as i64)
            .await
            .map_err(|e| format!("Failed to calculate price: {}", e))?;
        
        // Apply annual reduction
        let reduction_factor = 1.0 - (product_data.annual_reduction / 100.0);
        let reduced_price = base_price * reduction_factor;
        
        // Calculate total for this product
        let mut product_total = 0.0;
        
        if product_data.licence {
            product_total += reduced_price;
            has_licence = true;
        }
        
        // Add training cost
        let training_cost = if product_data.training {
            has_training = true;
            product_data.training_days as f64 * product_data.training_cost_per_day
        } else {
            0.0
        };
        product_total += training_cost;
        
        // Add support cost (20% × years)
        if product_data.support && product_data.support_years > 0 {
            has_support = true;
            let support_cost = reduced_price * 0.20 * product_data.support_years as f64;
            product_total += support_cost;
        }
        
        if first_product_type.is_empty() {
            first_product_type = product_data.product_type.clone();
        }
        
        // Create product with recalculated user_count
        queries::create_proposal_product(
            &pool,
            proposal.id.clone(),
            product_data.product_type,
            user_count as i64,
            product_data.standalone_count,
            product_data.server_key_count,
            base_price,
            product_total,
            product_data.annual_reduction,
            product_data.training,
            product_data.training_days,
            product_data.training_cost_per_day,
            training_cost,
            product_data.licence,
            product_data.support,
            product_data.support_years,
        )
        .await
        .map_err(|e| format!("Failed to create product: {}", e))?;
        
        total_amount += product_total;
    }

    // Update proposal total and proposal number
    queries::update_proposal_total(&pool, &proposal.id, total_amount)
        .await
        .map_err(|e| format!("Failed to update total: {}", e))?;

    let proposal_number = queries::generate_proposal_number(
        &pool,
        &request.company_id,
        &first_product_type,
        has_training,
        has_support,
        has_licence,
    )
    .await
    .map_err(|e| format!("Failed to generate number: {}", e))?;

    sqlx::query("UPDATE proposals SET proposal_number = ? WHERE id = ?")
        .bind(&proposal_number)
        .bind(&proposal.id)
        .execute(pool.inner())
        .await
        .map_err(|e| format!("Failed to update proposal number: {}", e))?;

    // Get the updated proposal with correct total and number
    let updated_proposal = queries::get_proposal_by_id(&pool, &proposal.id)
        .await
        .map_err(|e| format!("Failed to get updated proposal: {}", e))?
        .ok_or("Proposal not found after creation")?;
    
    // Get all products for this proposal to sync with it
    let _products = queries::get_proposal_products(&pool, &proposal.id)
        .await
        .map_err(|e| format!("Failed to get products: {}", e))?;
    
    // Add to sync queue - convert valid_until to full DateTime format
    let mut proposal_json = serde_json::to_value(&updated_proposal)
        .map_err(|e| format!("Failed to serialize proposal: {}", e))?;
    
    // Ensure valid_until is in full DateTime format if present
    if let Some(valid_until) = proposal_json.get_mut("valid_until") {
        if let Some(date_str) = valid_until.as_str() {
            if !date_str.contains('T') {
                // Convert date to DateTime format
                *valid_until = serde_json::Value::String(format!("{}T00:00:00+00:00", date_str));
            }
        }
    }
    

    Ok(updated_proposal)
}

#[tauri::command]
pub async fn update_proposal(
    pool: State<'_, DbPool>,
    request: UpdateProposalRequest,
) -> Result<Proposal, String> {
    println!("🔄 [update_proposal] Updating proposal: {}", request.id);
    
    // 1. Supprimer les anciens produits en local
    queries::delete_proposal_products(&pool, &request.id)
        .await
        .map_err(|e| format!("Failed to delete old products: {}", e))?;
    
    // 2. Créer les nouveaux produits en local
    let mut total_amount = 0.0;
    let mut has_training = false;
    let mut has_support = false;
    let mut has_licence = false;
    let mut first_product_type = String::new();

    for product_data in request.products {
        // IMPORTANT: Recalculate user_count from standalone + server_key (don't trust frontend)
        let user_count = product_data.standalone_count + product_data.server_key_count;
        
        let base_price = queries::calculate_price(&pool, &product_data.product_type, user_count as i64)
            .await
            .map_err(|e| format!("Failed to calculate price: {}", e))?;
        
        let reduction_factor = 1.0 - (product_data.annual_reduction / 100.0);
        let reduced_price = base_price * reduction_factor;
        
        let mut product_total = 0.0;
        
        if product_data.licence {
            product_total += reduced_price;
            has_licence = true;
        }
        
        let training_cost = if product_data.training {
            has_training = true;
            product_data.training_days as f64 * product_data.training_cost_per_day
        } else {
            0.0
        };
        product_total += training_cost;
        
        if product_data.support && product_data.support_years > 0 {
            has_support = true;
            let support_cost = reduced_price * 0.20 * product_data.support_years as f64;
            product_total += support_cost;
        }
        
        if first_product_type.is_empty() {
            first_product_type = product_data.product_type.clone();
        }
        
        // Créer le nouveau produit
        queries::create_proposal_product(
            &pool,
            request.id.clone(),
            product_data.product_type,
            user_count as i64,
            product_data.standalone_count,
            product_data.server_key_count,
            base_price,
            product_total,
            product_data.annual_reduction,
            product_data.training,
            product_data.training_days,
            product_data.training_cost_per_day,
            training_cost,
            product_data.licence,
            product_data.support,
            product_data.support_years,
        )
        .await
        .map_err(|e| format!("Failed to create product: {}", e))?;
        
        total_amount += product_total;
    }

    // 3. Mettre à jour la proposition
    let now = chrono::Utc::now().timestamp_millis().to_string();
    sqlx::query(
        "UPDATE proposals SET company_id = ?, status = ?, currency = ?, valid_until = ?, notes = ?, 
         total_amount = ?, updated_at = ? WHERE id = ?"
    )
    .bind(&request.company_id)
    .bind(&request.status)
    .bind(&request.currency)
    .bind(&request.valid_until)
    .bind(&request.notes)
    .bind(total_amount)
    .bind(&now)
    .bind(&request.id)
    .execute(pool.inner())
    .await
    .map_err(|e| format!("Failed to update proposal: {}", e))?;

    // 4. Générer le numéro de proposition si nécessaire
    let proposal_number = queries::generate_proposal_number(
        &pool,
        &request.company_id,
        &first_product_type,
        has_training,
        has_support,
        has_licence,
    )
    .await
    .map_err(|e| format!("Failed to generate number: {}", e))?;

    sqlx::query("UPDATE proposals SET proposal_number = ? WHERE id = ?")
        .bind(&proposal_number)
        .bind(&request.id)
        .execute(pool.inner())
        .await
        .map_err(|e| format!("Failed to update proposal number: {}", e))?;

    // 5. Récupérer la proposition mise à jour
    let proposal = queries::get_proposal_by_id(&pool, &request.id)
        .await
        .map_err(|e| format!("Failed to get updated proposal: {}", e))?
        .ok_or("Proposal not found")?;


    println!("✅ [update_proposal] Proposal {} updated successfully", request.id);
    Ok(proposal)
}

#[tauri::command]
pub async fn get_proposal(pool: State<'_, DbPool>, id: String) -> Result<Option<ProposalWithProducts>, String> {
    let proposal = queries::get_proposal_by_id(&pool, &id)
        .await
        .map_err(|e| format!("Failed to get proposal: {}", e))?;

    match proposal {
        Some(prop) => {
            let products = queries::get_proposal_products(&pool, &prop.id)
                .await
                .map_err(|e| format!("Failed to get products: {}", e))?;
            
            let company = queries::get_company_by_id(&pool, &prop.company_id)
                .await
                .map_err(|e| format!("Failed to get company: {}", e))?;
            
            let company_name = company.map(|c| c.name).unwrap_or_else(|| "Unknown".to_string());
            
            Ok(Some(ProposalWithProducts {
                proposal: prop,
                products,
                company_name,
            }))
        }
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn delete_proposal(pool: State<'_, DbPool>, id: String) -> Result<(), String> {
    println!("🔄 [delete_proposal] Deleting proposal: {}", id);
    
    queries::delete_proposal(&pool, &id)
        .await
        .map_err(|e| format!("Failed to delete proposal: {}", e))?;
    
    println!("✅ [delete_proposal] Proposal {} added to deletion queue", id);
    Ok(())
}

#[tauri::command]
pub async fn calculate_product_price(
    pool: State<'_, DbPool>,
    product_type: String,
    user_count: i64,
) -> Result<f64, String> {
    queries::calculate_price(&pool, &product_type, user_count)
        .await
        .map_err(|e| format!("Failed to calculate price: {}", e))
}

#[tauri::command]
pub async fn test_get_proposals(pool: State<'_, DbPool>) -> Result<String, String> {
    println!("🔄 [test_get_proposals] Starting test...");
    
    let proposals = queries::get_all_proposals(&pool)
        .await
        .map_err(|e| format!("Failed to get proposals: {}", e))?;
    
    println!("🔄 [test_get_proposals] Found {} proposals", proposals.len());
    
    for proposal in &proposals {
        println!("🔄 [test_get_proposals] Proposal: {} (ID: {})", 
                 proposal.proposal_number.as_ref().unwrap_or(&"Unknown".to_string()),
                 proposal.id);
    }
    
    Ok(format!("Found {} proposals", proposals.len()))
}

#[tauri::command]
pub async fn delete_proposal_product(pool: State<'_, DbPool>, product_id: String) -> Result<(), String> {
    println!("🔄 [delete_proposal_product] Deleting product: {}", product_id);
    
    // Delete directly (no sync queue)
    queries::delete_proposal_product(&pool, &product_id)
        .await
        .map_err(|e| format!("Failed to delete product: {}", e))?;
    
    println!("✅ [delete_proposal_product] Product {} deleted", product_id);
    Ok(())
}

// get_deletion_queue - REMOVED as part of synchronization cleanup

