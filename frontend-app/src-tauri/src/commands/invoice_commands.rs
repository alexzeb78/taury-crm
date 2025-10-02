use crate::db::{queries, DbPool};
use crate::db::models::*;
use tauri::State;
use chrono::Utc;

#[tauri::command]
pub async fn create_invoice_from_proposal(
    pool: State<'_, DbPool>,
    proposal_id: String,
) -> Result<Invoice, String> {
    // Vérifier si une invoice existe déjà pour cette proposal
    if queries::invoice_exists_for_proposal(&pool, &proposal_id).await.map_err(|e| e.to_string())? {
        return Err("Invoice already exists for this proposal".to_string());
    }

    // Récupérer la proposal pour obtenir les détails
    let proposal = queries::get_proposal_by_id(&pool, &proposal_id).await
        .map_err(|e| e.to_string())?
        .ok_or("Proposal not found")?;

    // Générer le numéro d'invoice
    let invoice_number = queries::generate_invoice_number(&pool).await.map_err(|e| e.to_string())?;

    // Date d'émission = aujourd'hui
    let issue_date = Utc::now().date_naive().format("%Y-%m-%d").to_string();
    
    // Date d'échéance = 30 jours après l'émission
    let due_date = Utc::now().date_naive()
        .checked_add_days(chrono::Days::new(30))
        .map(|d| d.format("%Y-%m-%d").to_string());

    // Créer l'invoice
    let invoice = queries::create_invoice(
        &pool,
        proposal_id,
        invoice_number,
        proposal.total_amount,
        proposal.currency,
        issue_date,
        due_date,
        None, // purchase_order
        None, // purchase_order_date
        None, // commercial_in_charge
        proposal.notes, // notes
    ).await.map_err(|e| e.to_string())?;

    Ok(invoice)
}

#[tauri::command]
pub async fn get_all_invoices(
    pool: State<'_, DbPool>,
) -> Result<Vec<InvoiceWithDetails>, String> {
    let invoices = queries::get_all_invoices(&pool).await.map_err(|e| e.to_string())?;
    Ok(invoices)
}

#[tauri::command]
pub async fn get_invoice_by_id(
    pool: State<'_, DbPool>,
    id: String,
) -> Result<Option<InvoiceWithDetails>, String> {
    let invoice = queries::get_invoice_by_id(&pool, &id).await.map_err(|e| e.to_string())?;
    Ok(invoice)
}

#[tauri::command]
pub async fn get_invoice_by_proposal_id(
    pool: State<'_, DbPool>,
    proposal_id: String,
) -> Result<Option<InvoiceWithDetails>, String> {
    let invoice = queries::get_invoice_by_proposal_id(&pool, &proposal_id).await.map_err(|e| e.to_string())?;
    Ok(invoice)
}

#[tauri::command]
pub async fn update_invoice(
    pool: State<'_, DbPool>,
    id: String,
    status: Option<String>,
    total_amount: Option<f64>,
    currency: Option<String>,
    issue_date: Option<String>,
    due_date: Option<String>,
    paid_date: Option<String>,
    purchase_order: Option<String>,
    purchase_order_date: Option<String>,
    commercial_in_charge: Option<String>,
    notes: Option<String>,
) -> Result<Invoice, String> {
    let invoice = queries::update_invoice(
        &pool,
        &id,
        status,
        total_amount,
        currency,
        issue_date,
        due_date,
        paid_date,
        purchase_order,
        purchase_order_date,
        commercial_in_charge,
        notes,
    ).await.map_err(|e| e.to_string())?;

    Ok(invoice)
}

#[tauri::command]
pub async fn update_invoice_status(
    pool: State<'_, DbPool>,
    id: String,
    status: String,
) -> Result<Invoice, String> {
    let paid_date = if status == "PAID" {
        Some(Utc::now().date_naive().format("%Y-%m-%d").to_string())
    } else {
        None
    };

    let invoice = queries::update_invoice(
        &pool,
        &id,
        Some(status),
        None, // total_amount
        None, // currency
        None, // issue_date
        None, // due_date
        paid_date,
        None, // purchase_order
        None, // purchase_order_date
        None, // commercial_in_charge
        None, // notes
    ).await.map_err(|e| e.to_string())?;

    Ok(invoice)
}

#[tauri::command]
pub async fn delete_invoice(
    pool: State<'_, DbPool>,
    id: String,
) -> Result<(), String> {
    queries::delete_invoice(&pool, &id).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_invoices_by_status(
    pool: State<'_, DbPool>,
    status: String,
) -> Result<Vec<InvoiceWithDetails>, String> {
    let invoices = queries::get_invoices_by_status(&pool, &status).await.map_err(|e| e.to_string())?;
    Ok(invoices)
}

#[tauri::command]
pub async fn get_invoices_by_company(
    pool: State<'_, DbPool>,
    company_id: String,
) -> Result<Vec<InvoiceWithDetails>, String> {
    let invoices = queries::get_invoices_by_company(&pool, &company_id).await.map_err(|e| e.to_string())?;
    Ok(invoices)
}

#[tauri::command]
pub async fn generate_invoice_excel(
    pool: State<'_, DbPool>,
    invoice_id: String,
) -> Result<String, String> {
    // Récupérer l'invoice avec tous les détails
    let invoice = queries::get_invoice_by_id(&pool, &invoice_id).await
        .map_err(|e| e.to_string())?
        .ok_or("Invoice not found")?;

    // Récupérer la proposal et ses produits
    let proposal = queries::get_proposal_by_id(&pool, &invoice.proposal_id).await
        .map_err(|e| e.to_string())?
        .ok_or("Proposal not found")?;

    let products = queries::get_proposal_products(&pool, &invoice.proposal_id).await
        .map_err(|e| e.to_string())?;

    // Récupérer les détails de la company et du contact
    let company = queries::get_company_by_id(&pool, &proposal.company_id).await
        .map_err(|e| e.to_string())?
        .ok_or("Company not found")?;

    // Get the first contact for the company (or None if no contacts)
    let contacts = queries::get_company_contacts(&pool, &proposal.company_id).await
        .map_err(|e| e.to_string())?;
    let contact = contacts.first().cloned();

    // Préparer les données pour l'API Python
    let invoice_data = serde_json::json!({
        "invoice": {
            "id": invoice.id,
            "invoice_number": invoice.invoice_number,
            "status": invoice.status,
            "total_amount": invoice.total_amount,
            "currency": invoice.currency,
            "issue_date": invoice.issue_date,
            "due_date": invoice.due_date,
            "paid_date": invoice.paid_date,
            "purchase_order": invoice.purchase_order,
            "purchase_order_date": invoice.purchase_order_date,
            "commercial_in_charge": invoice.commercial_in_charge,
            "notes": invoice.notes,
        },
        "company": {
            "id": company.id,
            "name": company.name,
            "website": company.website,
            "address": company.address,
            "city": company.city,
            "postal_code": company.postal_code,
            "country": company.country,
            "description": company.description,
        },
        "contact": contact.map(|c| serde_json::json!({
            "id": c.id,
            "first_name": c.first_name,
            "last_name": c.last_name,
            "email": c.email,
            "phone_number": c.phone_number,
        })),
        "products": products.into_iter().map(|p| serde_json::json!({
            "id": p.id,
            "product_type": p.product_type,
            "user_count": p.user_count,
            "standalone_count": p.standalone_count,
            "server_key_count": p.server_key_count,
            "unit_price": p.unit_price,
            "total_price": p.total_price,
            "annual_reduction": p.annual_reduction,
            "training": p.training,
            "training_days": p.training_days,
            "training_cost_per_day": p.training_cost_per_day,
            "training_cost": p.training_cost,
            "licence": p.licence,
            "support": p.support,
            "support_years": p.support_years,
        })).collect::<Vec<_>>(),
    });

    // Appeler l'API Python pour générer l'Excel
    let client = reqwest::Client::new();
    let response = client
        .post("http://127.0.0.1:8001/generate-invoice-excel")
        .json(&invoice_data)
        .send()
        .await
        .map_err(|e| format!("Failed to call invoice API. Is the sidecar running? Error: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Invoice API error: {}", response.status()));
    }

    // Sauvegarder le fichier Excel
    let filename = format!("Invoice_{}.xlsx", invoice.invoice_number);
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let output_path = home_dir.join("Downloads").join(&filename);

    let bytes = response.bytes().await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    std::fs::write(&output_path, bytes)
        .map_err(|e| format!("Failed to save file: {}", e))?;

    Ok(output_path.to_string_lossy().to_string())
}
