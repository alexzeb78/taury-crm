use super::{models::*, DbPool};
use chrono::{Utc, Datelike};
use sqlx;
use uuid::Uuid;

// Role queries
pub async fn get_role_by_name(pool: &DbPool, name: &str) -> Result<Option<Role>, sqlx::Error> {
    let role = sqlx::query_as::<_, Role>("SELECT * FROM roles WHERE name = ?")
        .bind(name)
        .fetch_optional(pool)
        .await?;
    Ok(role)
}

pub async fn get_all_roles(pool: &DbPool) -> Result<Vec<Role>, sqlx::Error> {
    let roles = sqlx::query_as::<_, Role>("SELECT * FROM roles ORDER BY name")
        .fetch_all(pool)
        .await?;
    Ok(roles)
}

// User queries
pub async fn create_user(
    pool: &DbPool,
    email: String,
    password_hash: String,
    name: String,
    role_name: Option<String>,
) -> Result<User, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().timestamp_millis().to_string();

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, email, password_hash, name, enabled, created_at, updated_at) 
         VALUES (?, ?, ?, ?, 1, ?, ?) RETURNING *"
    )
    .bind(&id)
    .bind(&email)
    .bind(&password_hash)
    .bind(&name)
    .bind(&now)
    .bind(&now)
    .fetch_one(pool)
    .await?;

    // Assign role (default to USER if not specified)
    let role_to_assign = role_name.unwrap_or_else(|| "USER".to_string());
    if let Some(role) = get_role_by_name(pool, &role_to_assign).await? {
        sqlx::query("INSERT INTO user_roles (user_id, role_id) VALUES (?, ?)")
            .bind(&user.id)
            .bind(role.id)
            .execute(pool)
            .await?;
    }

    Ok(user)
}

pub async fn get_user_roles(pool: &DbPool, user_id: &str) -> Result<Vec<String>, sqlx::Error> {
    let roles = sqlx::query_scalar::<_, String>(
        "SELECT r.name FROM roles r 
         INNER JOIN user_roles ur ON r.id = ur.role_id 
         WHERE ur.user_id = ?"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    Ok(roles)
}

pub async fn user_has_role(pool: &DbPool, user_id: &str, role_name: &str) -> Result<bool, sqlx::Error> {
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM user_roles ur 
         INNER JOIN roles r ON ur.role_id = r.id 
         WHERE ur.user_id = ? AND r.name = ?"
    )
    .bind(user_id)
    .bind(role_name)
    .fetch_one(pool)
    .await?;
    Ok(count > 0)
}

pub async fn get_all_users(pool: &DbPool) -> Result<Vec<UserWithRoles>, sqlx::Error> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(pool)
        .await?;
    
    let mut users_with_roles = Vec::new();
    for user in users {
        let roles = get_user_roles(pool, &user.id).await?;
        users_with_roles.push(UserWithRoles {
            id: user.id,
            email: user.email,
            name: user.name,
            enabled: user.enabled == 1,
            roles,
            created_at: user.created_at,
            updated_at: user.updated_at,
        });
    }
    Ok(users_with_roles)
}

pub async fn update_user_enabled(pool: &DbPool, user_id: &str, enabled: bool) -> Result<(), sqlx::Error> {
    let enabled_val = if enabled { 1 } else { 0 };
    sqlx::query("UPDATE users SET enabled = ? WHERE id = ?")
        .bind(enabled_val)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_user(pool: &DbPool, user_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_user_by_email(pool: &DbPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(email)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

// Company queries
pub async fn create_company(
    pool: &DbPool,
    name: String,
    website: Option<String>,
    address: Option<String>,
    city: Option<String>,
    postal_code: Option<String>,
    country: Option<String>,
    description: Option<String>,
) -> Result<Company, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().timestamp_millis().to_string();

    let company = sqlx::query_as::<_, Company>(
        "INSERT INTO companies (id, name, website, address, city, postal_code, country, description, created_at, updated_at, sync_status) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'pending') RETURNING *"
    )
    .bind(&id)
    .bind(&name)
    .bind(&website)
    .bind(&address)
    .bind(&city)
    .bind(&postal_code)
    .bind(&country)
    .bind(&description)
    .bind(&now)
    .bind(&now)
    .fetch_one(pool)
    .await?;

    Ok(company)
}

pub async fn get_all_companies(pool: &DbPool) -> Result<Vec<Company>, sqlx::Error> {
    let companies = sqlx::query_as::<_, Company>("SELECT * FROM companies ORDER BY name ASC")
        .fetch_all(pool)
        .await?;
    Ok(companies)
}

pub async fn get_company_by_id(pool: &DbPool, id: &str) -> Result<Option<Company>, sqlx::Error> {
    let company = sqlx::query_as::<_, Company>("SELECT * FROM companies WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(company)
}

pub async fn update_company(
    pool: &DbPool,
    id: &str,
    name: String,
    website: Option<String>,
    address: Option<String>,
    city: Option<String>,
    postal_code: Option<String>,
    country: Option<String>,
    description: Option<String>,
) -> Result<Company, sqlx::Error> {
    let now = chrono::Utc::now().timestamp_millis().to_string();

    let company = sqlx::query_as::<_, Company>(
        "UPDATE companies SET name = ?, website = ?, address = ?, city = ?, postal_code = ?, country = ?, description = ?, 
         updated_at = ?, sync_status = 'pending' WHERE id = ? RETURNING *"
    )
    .bind(&name)
    .bind(&website)
    .bind(&address)
    .bind(&city)
    .bind(&postal_code)
    .bind(&country)
    .bind(&description)
    .bind(&now)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(company)
}

pub async fn delete_company(pool: &DbPool, id: &str) -> Result<(), sqlx::Error> {
    // Supprimer en local immédiatement
    sqlx::query("DELETE FROM company_contacts WHERE company_id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    
    sqlx::query("DELETE FROM companies WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    
    // Enregistrer l'action pour la synchronisation
    
    Ok(())
}

// Company Contact queries
pub async fn create_company_contact(
    pool: &DbPool,
    company_id: String,
    first_name: String,
    last_name: String,
    email: String,
    phone_number: Option<String>,
    is_primary: bool,
) -> Result<CompanyContact, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().timestamp_millis().to_string();
    let is_primary_val = if is_primary { 1 } else { 0 };

    let contact = sqlx::query_as::<_, CompanyContact>(
        "INSERT INTO company_contacts (id, company_id, first_name, last_name, email, phone_number, is_primary, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING *"
    )
    .bind(&id)
    .bind(&company_id)
    .bind(&first_name)
    .bind(&last_name)
    .bind(&email)
    .bind(&phone_number)
    .bind(is_primary_val)
    .bind(&now)
    .bind(&now)
    .fetch_one(pool)
    .await?;

    Ok(contact)
}

pub async fn get_company_contact_by_id(pool: &DbPool, contact_id: &str) -> Result<Option<CompanyContact>, sqlx::Error> {
    let contact = sqlx::query_as::<_, CompanyContact>(
        "SELECT * FROM company_contacts WHERE id = ?"
    )
    .bind(contact_id)
    .fetch_optional(pool)
    .await?;
    
    Ok(contact)
}

pub async fn get_company_contacts(pool: &DbPool, company_id: &str) -> Result<Vec<CompanyContact>, sqlx::Error> {
    let contacts = sqlx::query_as::<_, CompanyContact>(
        "SELECT * FROM company_contacts WHERE company_id = ? ORDER BY is_primary DESC, first_name ASC"
    )
    .bind(company_id)
    .fetch_all(pool)
    .await?;
    Ok(contacts)
}

pub async fn delete_company_contacts(pool: &DbPool, company_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM company_contacts WHERE company_id = ?")
        .bind(company_id)
        .execute(pool)
        .await?;
    Ok(())
}

// Pricing queries
pub async fn get_price_for_users(
    pool: &DbPool,
    product_type: &str,
    user_count: i64,
) -> Result<Option<LicencePricing>, sqlx::Error> {
    let pricing = sqlx::query_as::<_, LicencePricing>(
        "SELECT * FROM licence_pricing WHERE product_type = ? AND user_count = ?"
    )
    .bind(product_type)
    .bind(user_count)
    .fetch_optional(pool)
    .await?;
    Ok(pricing)
}

pub async fn get_pricing_up_to_users(
    pool: &DbPool,
    product_type: &str,
    user_count: i64,
) -> Result<Vec<LicencePricing>, sqlx::Error> {
    let pricing = sqlx::query_as::<_, LicencePricing>(
        "SELECT * FROM licence_pricing WHERE product_type = ? AND user_count <= ? ORDER BY user_count ASC"
    )
    .bind(product_type)
    .bind(user_count)
    .fetch_all(pool)
    .await?;
    Ok(pricing)
}

pub async fn calculate_price(
    pool: &DbPool,
    product_type: &str,
    user_count: i64,
) -> Result<f64, sqlx::Error> {
    if product_type == "ICS Manager" {
        // Special logic for ICS Manager: 1 server + (N-1) additional clients
        let base = get_price_for_users(pool, "ICS Manager", 1).await?;
        let additional = get_price_for_users(pool, "ICS Manager Additional", 1).await?;
        
        if let (Some(base_price), Some(add_price)) = (base, additional) {
            let total = base_price.price_usd + (add_price.price_usd * (user_count - 1) as f64);
            Ok(total)
        } else {
            Ok(0.0)
        }
    } else {
        // For HTZ Communications and HTZ Warfare: PROGRESSIVE SUM
        // Total = Prix(1) + Prix(2) + Prix(3) + ... + Prix(user_count)
        
        let mut total = 0.0;
        
        for i in 1..=user_count {
            // Try to get exact price for this user count
            let pricing = get_price_for_users(pool, product_type, i).await?;
            
            if let Some(p) = pricing {
                // Exact price found
                total += p.price_usd;
            } else {
                // No exact price - find the closest lower tier and use that price
                let closest = sqlx::query_as::<_, LicencePricing>(
                    "SELECT * FROM licence_pricing 
                     WHERE product_type = ? AND user_count <= ? 
                     ORDER BY user_count DESC 
                     LIMIT 1"
                )
                .bind(product_type)
                .bind(i)
                .fetch_optional(pool)
                .await?;
                
                if let Some(p) = closest {
                    total += p.price_usd;
                }
            }
        }
        
        Ok(total)
    }
}

// Proposal queries
pub async fn create_proposal(
    pool: &DbPool,
    company_id: String,
    status: String,
    currency: String,
    valid_until: Option<String>,
    notes: Option<String>,
) -> Result<Proposal, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().timestamp_millis().to_string();

    let proposal = sqlx::query_as::<_, Proposal>(
        "INSERT INTO proposals (id, company_id, status, currency, valid_until, notes, created_at, updated_at, sync_status, total_amount) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'pending', 0) RETURNING *"
    )
    .bind(&id)
    .bind(&company_id)
    .bind(&status)
    .bind(&currency)
    .bind(&valid_until)
    .bind(&notes)
    .bind(&now)
    .bind(&now)
    .fetch_one(pool)
    .await?;

    Ok(proposal)
}

pub async fn get_all_proposals(pool: &DbPool) -> Result<Vec<Proposal>, sqlx::Error> {
    let proposals = sqlx::query_as::<_, Proposal>(
        "SELECT * FROM proposals ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;
    Ok(proposals)
}

pub async fn get_proposal_by_id(pool: &DbPool, id: &str) -> Result<Option<Proposal>, sqlx::Error> {
    let proposal = sqlx::query_as::<_, Proposal>("SELECT * FROM proposals WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(proposal)
}

pub async fn delete_proposal(pool: &DbPool, id: &str) -> Result<(), sqlx::Error> {
    // Supprimer en local immédiatement
    sqlx::query("DELETE FROM proposal_products WHERE proposal_id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    
    sqlx::query("DELETE FROM proposals WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    
    // Enregistrer l'action pour la synchronisation
    
    Ok(())
}

pub async fn create_proposal_product(
    pool: &DbPool,
    proposal_id: String,
    product_type: String,
    user_count: i64,
    standalone_count: i64,
    server_key_count: i64,
    unit_price: f64,
    total_price: f64,
    annual_reduction: f64,
    training: bool,
    training_days: i64,
    training_cost_per_day: f64,
    training_cost: f64,
    licence: bool,
    support: bool,
    support_years: i64,
) -> Result<ProposalProduct, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().timestamp_millis().to_string();

    let product = sqlx::query_as::<_, ProposalProduct>(
        "INSERT INTO proposal_products (id, proposal_id, product_type, user_count, standalone_count, server_key_count, 
         unit_price, total_price, annual_reduction, training, training_days, training_cost_per_day, training_cost, 
         licence, support, support_years, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) RETURNING *"
    )
    .bind(&id)
    .bind(&proposal_id)
    .bind(&product_type)
    .bind(user_count)
    .bind(standalone_count)
    .bind(server_key_count)
    .bind(unit_price)
    .bind(total_price)
    .bind(annual_reduction)
    .bind(if training { 1 } else { 0 })
    .bind(training_days)
    .bind(training_cost_per_day)
    .bind(training_cost)
    .bind(if licence { 1 } else { 0 })
    .bind(if support { 1 } else { 0 })
    .bind(support_years)
    .bind(&now)
    .bind(&now)
    .fetch_one(pool)
    .await?;

    Ok(product)
}

pub async fn update_proposal_product(
    pool: &DbPool,
    product_id: &str,
    product_type: String,
    user_count: i64,
    standalone_count: i64,
    server_key_count: i64,
    unit_price: f64,
    total_price: f64,
    annual_reduction: f64,
    training: bool,
    training_days: i64,
    training_cost_per_day: f64,
    training_cost: f64,
    licence: bool,
    support: bool,
    support_years: i64,
) -> Result<ProposalProduct, sqlx::Error> {
    let now = chrono::Utc::now().timestamp_millis().to_string();

    sqlx::query(
        "UPDATE proposal_products SET 
            product_type = ?, user_count = ?, standalone_count = ?, server_key_count = ?,
            unit_price = ?, total_price = ?, annual_reduction = ?, training = ?, training_days = ?,
            training_cost_per_day = ?, training_cost = ?, licence = ?, support = ?, support_years = ?,
            updated_at = ?
        WHERE id = ?"
    )
    .bind(&product_type)
    .bind(user_count)
    .bind(standalone_count)
    .bind(server_key_count)
    .bind(unit_price)
    .bind(total_price)
    .bind(annual_reduction)
    .bind(if training { 1 } else { 0 })
    .bind(training_days)
    .bind(training_cost_per_day)
    .bind(training_cost)
    .bind(if licence { 1 } else { 0 })
    .bind(if support { 1 } else { 0 })
    .bind(support_years)
    .bind(&now)
    .bind(product_id)
    .execute(pool)
    .await?;

    // Get the updated product
    let product = sqlx::query_as::<_, ProposalProduct>(
        "SELECT * FROM proposal_products WHERE id = ?"
    )
    .bind(product_id)
    .fetch_one(pool)
    .await?;

    Ok(product)
}

pub async fn get_proposal_products(pool: &DbPool, proposal_id: &str) -> Result<Vec<ProposalProduct>, sqlx::Error> {
    let products = sqlx::query_as::<_, ProposalProduct>(
        "SELECT * FROM proposal_products WHERE proposal_id = ?"
    )
    .bind(proposal_id)
    .fetch_all(pool)
    .await?;
    Ok(products)
}

pub async fn delete_proposal_products(pool: &DbPool, proposal_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM proposal_products WHERE proposal_id = ?")
        .bind(proposal_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_proposal_product(pool: &DbPool, product_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM proposal_products WHERE id = ?")
        .bind(product_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_proposal_total(pool: &DbPool, proposal_id: &str, total: f64) -> Result<(), sqlx::Error> {
    let now = chrono::Utc::now().timestamp_millis().to_string();
    sqlx::query("UPDATE proposals SET total_amount = ?, updated_at = ? WHERE id = ?")
        .bind(total)
        .bind(&now)
        .bind(proposal_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn generate_proposal_number(
    pool: &DbPool,
    company_id: &str,
    product_type: &str,
    has_training: bool,
    has_support: bool,
    has_licence: bool,
) -> Result<String, sqlx::Error> {
    // Get company name
    let company = get_company_by_id(pool, company_id).await?;
    let company_name = company.map(|c| c.name).unwrap_or_else(|| "Unknown".to_string());
    
    // Format: D_CompanyName_Product_YYMMDD
    let first_letter = "D";
    let now = Utc::now();
    let date_suffix = format!("{:02}{:02}{:02}", now.year() % 100, now.month(), now.day());
    
    // Product code
    let product_code = if product_type.contains("HTZ Communications") {
        "HTZc"
    } else if product_type.contains("HTZ Warfare") {
        "HTZw"
    } else if product_type.contains("ICS Manager") {
        "ICSm"
    } else {
        "PROD"
    };
    
    // Build tags
    let mut tags = vec![product_code];
    if has_training {
        tags.push("training");
    }
    if has_support {
        tags.push("MC");
    }
    
    let separator = if has_licence { "+" } else { "_" };
    let product_tag = tags.join(separator);
    
    let clean_company = company_name.replace(" ", "_").replace("/", "_");
    let proposal_number = format!("{}_{}_{}_{}", first_letter, clean_company, product_tag, date_suffix);
    
    Ok(proposal_number)
}

// Customer queries
pub async fn create_customer(
    pool: &DbPool,
    name: String,
    email: Option<String>,
    phone: Option<String>,
    address: Option<String>,
    notes: Option<String>,
) -> Result<Customer, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().timestamp_millis().to_string();

    let customer = sqlx::query_as::<_, Customer>(
        "INSERT INTO customers (id, name, email, phone, address, notes, created_at, updated_at, sync_status) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'pending') RETURNING *"
    )
    .bind(&id)
    .bind(&name)
    .bind(&email)
    .bind(&phone)
    .bind(&address)
    .bind(&notes)
    .bind(&now)
    .bind(&now)
    .fetch_one(pool)
    .await?;

    Ok(customer)
}

pub async fn get_all_customers(pool: &DbPool) -> Result<Vec<Customer>, sqlx::Error> {
    let customers = sqlx::query_as::<_, Customer>("SELECT * FROM customers ORDER BY created_at DESC")
        .fetch_all(pool)
        .await?;

    Ok(customers)
}

pub async fn get_customer_by_id(pool: &DbPool, id: &str) -> Result<Option<Customer>, sqlx::Error> {
    let customer = sqlx::query_as::<_, Customer>("SELECT * FROM customers WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    Ok(customer)
}

pub async fn update_customer(
    pool: &DbPool,
    id: &str,
    name: String,
    email: Option<String>,
    phone: Option<String>,
    address: Option<String>,
    notes: Option<String>,
) -> Result<Customer, sqlx::Error> {
    let now = chrono::Utc::now().timestamp_millis().to_string();

    let customer = sqlx::query_as::<_, Customer>(
        "UPDATE customers SET name = ?, email = ?, phone = ?, address = ?, notes = ?, 
         updated_at = ?, sync_status = 'pending' WHERE id = ? RETURNING *"
    )
    .bind(&name)
    .bind(&email)
    .bind(&phone)
    .bind(&address)
    .bind(&notes)
    .bind(&now)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(customer)
}

pub async fn delete_customer(pool: &DbPool, id: &str) -> Result<(), sqlx::Error> {
    // Supprimer en local immédiatement
    sqlx::query("DELETE FROM documents WHERE customer_id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    
    sqlx::query("DELETE FROM customers WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    
    // Enregistrer l'action pour la synchronisation
    
    Ok(())
}

// Document queries
pub async fn create_document(
    pool: &DbPool,
    customer_id: String,
    title: String,
    document_type: String,
    file_path: Option<String>,
    content: Option<String>,
) -> Result<Document, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().timestamp_millis().to_string();

    let document = sqlx::query_as::<_, Document>(
        "INSERT INTO documents (id, customer_id, title, document_type, file_path, content, created_at, updated_at, sync_status) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'pending') RETURNING *"
    )
    .bind(&id)
    .bind(&customer_id)
    .bind(&title)
    .bind(&document_type)
    .bind(&file_path)
    .bind(&content)
    .bind(&now)
    .bind(&now)
    .fetch_one(pool)
    .await?;

    Ok(document)
}

pub async fn get_all_documents(pool: &DbPool) -> Result<Vec<Document>, sqlx::Error> {
    let documents = sqlx::query_as::<_, Document>("SELECT * FROM documents ORDER BY created_at DESC")
        .fetch_all(pool)
        .await?;

    Ok(documents)
}



pub async fn get_documents_by_customer(pool: &DbPool, customer_id: &str) -> Result<Vec<Document>, sqlx::Error> {
    let documents = sqlx::query_as::<_, Document>(
        "SELECT * FROM documents WHERE customer_id = ? ORDER BY created_at DESC"
    )
    .bind(customer_id)
    .fetch_all(pool)
    .await?;

    Ok(documents)
}

pub async fn delete_document(pool: &DbPool, id: &str) -> Result<(), sqlx::Error> {
    // Supprimer en local immédiatement
    sqlx::query("DELETE FROM documents WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    
    // Enregistrer l'action pour la synchronisation
    
    Ok(())
}

// add_to_sync_queue - REMOVED as part of synchronization cleanup

// get_pending_sync_items and remove_from_sync_queue - REMOVED as part of synchronization cleanup

// increment_sync_attempts - REMOVED as part of synchronization cleanup

// Settings queries - REMOVED as part of synchronization cleanup

// === SYNC QUERIES ===
pub async fn get_sync_metadata(pool: &DbPool) -> Result<SyncMetadata, sqlx::Error> {
    let metadata = sqlx::query_as::<_, SyncMetadata>(
        "SELECT * FROM sync_metadata WHERE id = 1"
    )
    .fetch_optional(pool)
    .await?;

    match metadata {
        Some(meta) => Ok(meta),
        None => {
            // Create default sync metadata
            let now = chrono::Utc::now().timestamp_millis();
            sqlx::query(
                "INSERT INTO sync_metadata (id, last_sync_timestamp, last_sync_version, updated_at) 
                 VALUES (1, 0, 0, ?)"
            )
            .bind(now.to_string())
            .execute(pool)
            .await?;

            Ok(SyncMetadata {
                id: 1,
                last_sync_timestamp: 0,
                last_sync_version: 0,
                updated_at: now.to_string(),
            })
        }
    }
}

pub async fn update_sync_metadata(pool: &DbPool, timestamp: i64, version: i64) -> Result<(), sqlx::Error> {
    let now = chrono::Utc::now().timestamp_millis();
    sqlx::query(
        "UPDATE sync_metadata SET last_sync_timestamp = ?, last_sync_version = ?, updated_at = ? WHERE id = 1"
    )
    .bind(timestamp)
    .bind(version)
    .bind(now.to_string())
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_changed_items_since(pool: &DbPool, since_timestamp: i64) -> Result<Vec<SyncItem>, sqlx::Error> {
    let mut items = Vec::new();
    
    // Get companies
    let companies = sqlx::query_as::<_, Company>(
        "SELECT * FROM companies WHERE updated_at > ? ORDER BY updated_at ASC"
    )
    .bind(since_timestamp.to_string())
    .fetch_all(pool)
    .await?;

    for company in companies {
        let updated_at = company.updated_at.parse::<i64>().unwrap_or(0);
        let version = company.version.unwrap_or(1);
        let is_deleted = company.is_deleted.unwrap_or(0) != 0;
        
        items.push(SyncItem {
            table_name: "companies".to_string(),
            id: company.id.clone(),
            data: serde_json::to_value(company).unwrap_or_default(),
            version,
            is_deleted,
            updated_at,
        });
    }

    // Get company_contacts
    let contacts = sqlx::query_as::<_, CompanyContact>(
        "SELECT * FROM company_contacts WHERE updated_at > ? ORDER BY updated_at ASC"
    )
    .bind(since_timestamp.to_string())
    .fetch_all(pool)
    .await?;

    for contact in contacts {
        let updated_at = contact.updated_at.parse::<i64>().unwrap_or(0);
        let version = contact.version.unwrap_or(1);
        let is_deleted = contact.is_deleted.unwrap_or(0) != 0;
        
        items.push(SyncItem {
            table_name: "company_contacts".to_string(),
            id: contact.id.clone(),
            data: serde_json::to_value(contact).unwrap_or_default(),
            version,
            is_deleted,
            updated_at,
        });
    }

    // Get customers
    let customers = sqlx::query_as::<_, Customer>(
        "SELECT * FROM customers WHERE updated_at > ? ORDER BY updated_at ASC"
    )
    .bind(since_timestamp.to_string())
    .fetch_all(pool)
    .await?;

    for customer in customers {
        let updated_at = customer.updated_at.parse::<i64>().unwrap_or(0);
        let version = customer.version.unwrap_or(1);
        let is_deleted = customer.is_deleted.unwrap_or(0) != 0;
        
        items.push(SyncItem {
            table_name: "customers".to_string(),
            id: customer.id.clone(),
            data: serde_json::to_value(customer).unwrap_or_default(),
            version,
            is_deleted,
            updated_at,
        });
    }

    // Get proposals
    let proposals = sqlx::query_as::<_, Proposal>(
        "SELECT * FROM proposals WHERE updated_at > ? ORDER BY updated_at ASC"
    )
    .bind(since_timestamp.to_string())
    .fetch_all(pool)
    .await?;

    for proposal in proposals {
        let updated_at = proposal.updated_at.parse::<i64>().unwrap_or(0);
        let version = proposal.version.unwrap_or(1);
        let is_deleted = proposal.is_deleted.unwrap_or(0) != 0;
        
        items.push(SyncItem {
            table_name: "proposals".to_string(),
            id: proposal.id.clone(),
            data: serde_json::to_value(proposal).unwrap_or_default(),
            version,
            is_deleted,
            updated_at,
        });
    }

    // Get proposal_products
    let products = sqlx::query_as::<_, ProposalProduct>(
        "SELECT * FROM proposal_products WHERE updated_at > ? ORDER BY updated_at ASC"
    )
    .bind(since_timestamp.to_string())
    .fetch_all(pool)
    .await?;

    for product in products {
        let updated_at = product.updated_at.parse::<i64>().unwrap_or(0);
        let version = product.version.unwrap_or(1);
        let is_deleted = product.is_deleted.unwrap_or(0) != 0;
        
        items.push(SyncItem {
            table_name: "proposal_products".to_string(),
            id: product.id.clone(),
            data: serde_json::to_value(product).unwrap_or_default(),
            version,
            is_deleted,
            updated_at,
        });
    }

    // Get invoices
    let invoices = sqlx::query_as::<_, Invoice>(
        "SELECT * FROM invoices WHERE updated_at > ? ORDER BY updated_at ASC"
    )
    .bind(since_timestamp.to_string())
    .fetch_all(pool)
    .await?;

    for invoice in invoices {
        let updated_at = invoice.updated_at.parse::<i64>().unwrap_or(0);
        let version = invoice.version.unwrap_or(1);
        let is_deleted = invoice.is_deleted.unwrap_or(0) != 0;
        
        items.push(SyncItem {
            table_name: "invoices".to_string(),
            id: invoice.id.clone(),
            data: serde_json::to_value(invoice).unwrap_or_default(),
            version,
            is_deleted,
            updated_at,
        });
    }

    // Get documents
    let documents = sqlx::query_as::<_, Document>(
        "SELECT * FROM documents WHERE updated_at > ? ORDER BY updated_at ASC"
    )
    .bind(since_timestamp.to_string())
    .fetch_all(pool)
    .await?;

    for document in documents {
        let updated_at = document.updated_at.parse::<i64>().unwrap_or(0);
        let version = document.version.unwrap_or(1);
        let is_deleted = document.is_deleted.unwrap_or(0) != 0;
        
        items.push(SyncItem {
            table_name: "documents".to_string(),
            id: document.id.clone(),
            data: serde_json::to_value(document).unwrap_or_default(),
            version,
            is_deleted,
            updated_at,
        });
    }

    Ok(items)
}

pub async fn apply_sync_item(pool: &DbPool, item: &SyncItem) -> Result<(), sqlx::Error> {
    if item.is_deleted {
        // Delete the item
        match item.table_name.as_str() {
            "companies" => {
                sqlx::query("DELETE FROM companies WHERE id = ?")
                    .bind(&item.id)
                    .execute(pool)
                    .await?;
            }
            "company_contacts" => {
                sqlx::query("DELETE FROM company_contacts WHERE id = ?")
                    .bind(&item.id)
                    .execute(pool)
                    .await?;
            }
            "customers" => {
                sqlx::query("DELETE FROM customers WHERE id = ?")
                    .bind(&item.id)
                    .execute(pool)
                    .await?;
            }
            "proposals" => {
                sqlx::query("DELETE FROM proposals WHERE id = ?")
                    .bind(&item.id)
                    .execute(pool)
                    .await?;
            }
            "proposal_products" => {
                sqlx::query("DELETE FROM proposal_products WHERE id = ?")
                    .bind(&item.id)
                    .execute(pool)
                    .await?;
            }
            "invoices" => {
                sqlx::query("DELETE FROM invoices WHERE id = ?")
                    .bind(&item.id)
                    .execute(pool)
                    .await?;
            }
            "documents" => {
                sqlx::query("DELETE FROM documents WHERE id = ?")
                    .bind(&item.id)
                    .execute(pool)
                    .await?;
            }
            _ => {
                return Err(sqlx::Error::Protocol("Unknown table name".to_string()));
            }
        }
    } else {
        // Insert or replace the item
        match item.table_name.as_str() {
            "companies" => {
                if let Ok(company) = serde_json::from_value::<Company>(item.data.clone()) {
                    sqlx::query(
                        "INSERT OR REPLACE INTO companies (id, name, website, address, city, postal_code, country, description, created_at, updated_at, version, is_deleted) 
                         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
                    )
                    .bind(&company.id)
                    .bind(&company.name)
                    .bind(&company.website)
                    .bind(&company.address)
                    .bind(&company.city)
                    .bind(&company.postal_code)
                    .bind(&company.country)
                    .bind(&company.description)
                    .bind(&company.created_at)
                    .bind(item.updated_at.to_string())
                    .bind(company.version.unwrap_or(1))
                    .bind(company.is_deleted.unwrap_or(0))
                    .execute(pool)
                    .await?;
                }
            }
            "company_contacts" => {
                if let Ok(contact) = serde_json::from_value::<CompanyContact>(item.data.clone()) {
                    sqlx::query(
                        "INSERT OR REPLACE INTO company_contacts (id, company_id, first_name, last_name, email, phone_number, is_primary, created_at, updated_at, version, is_deleted) 
                         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
                    )
                    .bind(&contact.id)
                    .bind(&contact.company_id)
                    .bind(&contact.first_name)
                    .bind(&contact.last_name)
                    .bind(&contact.email)
                    .bind(&contact.phone_number)
                    .bind(contact.is_primary)
                    .bind(&contact.created_at)
                    .bind(item.updated_at.to_string())
                    .bind(contact.version.unwrap_or(1))
                    .bind(contact.is_deleted.unwrap_or(0))
                    .execute(pool)
                    .await?;
                }
            }
            "customers" => {
                if let Ok(customer) = serde_json::from_value::<Customer>(item.data.clone()) {
                    sqlx::query(
                        "INSERT OR REPLACE INTO customers (id, name, email, phone, address, created_at, updated_at, version, is_deleted) 
                         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
                    )
                    .bind(&customer.id)
                    .bind(&customer.name)
                    .bind(&customer.email)
                    .bind(&customer.phone)
                    .bind(&customer.address)
                    .bind(&customer.created_at)
                    .bind(item.updated_at.to_string())
                    .bind(customer.version.unwrap_or(1))
                    .bind(customer.is_deleted.unwrap_or(0))
                    .execute(pool)
                    .await?;
                }
            }
            "proposals" => {
                if let Ok(proposal) = serde_json::from_value::<Proposal>(item.data.clone()) {
                    sqlx::query(
                        "INSERT OR REPLACE INTO proposals (id, company_id, proposal_number, status, total_amount, currency, valid_until, notes, created_at, updated_at, version, is_deleted) 
                         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
                    )
                    .bind(&proposal.id)
                    .bind(&proposal.company_id)
                    .bind(&proposal.proposal_number)
                    .bind(&proposal.status)
                    .bind(proposal.total_amount)
                    .bind(&proposal.currency)
                    .bind(&proposal.valid_until)
                    .bind(&proposal.notes)
                    .bind(&proposal.created_at)
                    .bind(item.updated_at.to_string())
                    .bind(proposal.version.unwrap_or(1))
                    .bind(proposal.is_deleted.unwrap_or(0))
                    .execute(pool)
                    .await?;
                }
            }
            "proposal_products" => {
                if let Ok(product) = serde_json::from_value::<ProposalProduct>(item.data.clone()) {
                    sqlx::query(
                        "INSERT OR REPLACE INTO proposal_products (id, proposal_id, product_type, user_count, standalone_count, server_key_count, unit_price, total_price, annual_reduction, training, training_days, training_cost_per_day, training_cost, licence, support, support_years, created_at, updated_at, version, is_deleted) 
                         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
                    )
                    .bind(&product.id)
                    .bind(&product.proposal_id)
                    .bind(&product.product_type)
                    .bind(product.user_count)
                    .bind(product.standalone_count)
                    .bind(product.server_key_count)
                    .bind(product.unit_price)
                    .bind(product.total_price)
                    .bind(product.annual_reduction)
                    .bind(product.training)
                    .bind(product.training_days)
                    .bind(product.training_cost_per_day)
                    .bind(product.training_cost)
                    .bind(product.licence)
                    .bind(product.support)
                    .bind(product.support_years)
                    .bind(&product.created_at)
                    .bind(item.updated_at.to_string())
                    .bind(product.version.unwrap_or(1))
                    .bind(product.is_deleted.unwrap_or(0))
                    .execute(pool)
                    .await?;
                }
            }
            "invoices" => {
                if let Ok(invoice) = serde_json::from_value::<Invoice>(item.data.clone()) {
                    sqlx::query(
                        "INSERT OR REPLACE INTO invoices (id, proposal_id, invoice_number, status, total_amount, currency, issue_date, due_date, paid_date, purchase_order, purchase_order_date, commercial_in_charge, notes, created_at, updated_at, version, is_deleted) 
                         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
                    )
                    .bind(&invoice.id)
                    .bind(&invoice.proposal_id)
                    .bind(&invoice.invoice_number)
                    .bind(&invoice.status)
                    .bind(invoice.total_amount)
                    .bind(&invoice.currency)
                    .bind(&invoice.issue_date)
                    .bind(&invoice.due_date)
                    .bind(&invoice.paid_date)
                    .bind(&invoice.purchase_order)
                    .bind(&invoice.purchase_order_date)
                    .bind(&invoice.commercial_in_charge)
                    .bind(&invoice.notes)
                    .bind(&invoice.created_at)
                    .bind(item.updated_at.to_string())
                    .bind(invoice.version.unwrap_or(1))
                    .bind(invoice.is_deleted.unwrap_or(0))
                    .execute(pool)
                    .await?;
                }
            }
            "documents" => {
                if let Ok(document) = serde_json::from_value::<Document>(item.data.clone()) {
                    sqlx::query(
                        "INSERT OR REPLACE INTO documents (id, customer_id, title, document_type, file_path, content, created_at, updated_at, version, is_deleted) 
                         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
                    )
                    .bind(&document.id)
                    .bind(&document.customer_id)
                    .bind(&document.title)
                    .bind(&document.document_type)
                    .bind(&document.file_path)
                    .bind(&document.content)
                    .bind(&document.created_at)
                    .bind(item.updated_at.to_string())
                    .bind(document.version.unwrap_or(1))
                    .bind(document.is_deleted.unwrap_or(0))
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

// Invoice queries
pub async fn create_invoice(
    pool: &DbPool,
    proposal_id: String,
    invoice_number: String,
    total_amount: f64,
    currency: String,
    issue_date: String,
    due_date: Option<String>,
    purchase_order: Option<String>,
    purchase_order_date: Option<String>,
    commercial_in_charge: Option<String>,
    notes: Option<String>,
) -> Result<Invoice, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().timestamp_millis().to_string();

    let invoice = sqlx::query_as::<_, Invoice>(
        "INSERT INTO invoices (
            id, proposal_id, invoice_number, status, total_amount, currency, 
            issue_date, due_date, purchase_order, purchase_order_date, 
            commercial_in_charge, notes, created_at, updated_at, sync_status
        ) VALUES (?, ?, ?, 'DRAFT', ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'pending') 
        RETURNING *"
    )
    .bind(&id)
    .bind(&proposal_id)
    .bind(&invoice_number)
    .bind(total_amount)
    .bind(&currency)
    .bind(&issue_date)
    .bind(&due_date)
    .bind(&purchase_order)
    .bind(&purchase_order_date)
    .bind(&commercial_in_charge)
    .bind(&notes)
    .bind(&now)
    .bind(&now)
    .fetch_one(pool)
    .await?;

    Ok(invoice)
}

pub async fn get_all_invoices(pool: &DbPool) -> Result<Vec<InvoiceWithDetails>, sqlx::Error> {
    let invoices = sqlx::query_as::<_, InvoiceWithDetails>(
        "SELECT 
            i.*,
            p.proposal_number,
            c.name as company_name
        FROM invoices i
        JOIN proposals p ON i.proposal_id = p.id
        JOIN companies c ON p.company_id = c.id
        ORDER BY i.created_at DESC"
    )
    .fetch_all(pool)
    .await?;

    Ok(invoices)
}

pub async fn get_invoice_by_id(pool: &DbPool, id: &str) -> Result<Option<InvoiceWithDetails>, sqlx::Error> {
    let invoice = sqlx::query_as::<_, InvoiceWithDetails>(
        "SELECT 
            i.*,
            p.proposal_number,
            c.name as company_name
        FROM invoices i
        JOIN proposals p ON i.proposal_id = p.id
        JOIN companies c ON p.company_id = c.id
        WHERE i.id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(invoice)
}

pub async fn get_invoice_by_proposal_id(pool: &DbPool, proposal_id: &str) -> Result<Option<InvoiceWithDetails>, sqlx::Error> {
    let invoice = sqlx::query_as::<_, InvoiceWithDetails>(
        "SELECT 
            i.*,
            p.proposal_number,
            c.name as company_name
        FROM invoices i
        JOIN proposals p ON i.proposal_id = p.id
        JOIN companies c ON p.company_id = c.id
        WHERE i.proposal_id = ?"
    )
    .bind(proposal_id)
    .fetch_optional(pool)
    .await?;

    Ok(invoice)
}

pub async fn update_invoice(
    pool: &DbPool,
    id: &str,
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
) -> Result<Invoice, sqlx::Error> {
    let now = chrono::Utc::now().timestamp_millis().to_string();

    // Build dynamic update query
    let _query_parts = vec!["updated_at = ?".to_string()];
    
    let mut query_builder = sqlx::query("UPDATE invoices SET updated_at = ?, sync_status = 'pending' WHERE id = ?")
        .bind(&now)
        .bind(id);

    if let Some(status) = status {
        query_builder = sqlx::query("UPDATE invoices SET updated_at = ?, status = ?, sync_status = 'pending' WHERE id = ?")
            .bind(&now)
            .bind(status)
            .bind(id);
    }
    if let Some(total_amount) = total_amount {
        query_builder = sqlx::query("UPDATE invoices SET updated_at = ?, total_amount = ?, sync_status = 'pending' WHERE id = ?")
            .bind(&now)
            .bind(total_amount)
            .bind(id);
    }
    if let Some(currency) = currency {
        query_builder = sqlx::query("UPDATE invoices SET updated_at = ?, currency = ?, sync_status = 'pending' WHERE id = ?")
            .bind(&now)
            .bind(currency)
            .bind(id);
    }
    if let Some(issue_date) = issue_date {
        query_builder = sqlx::query("UPDATE invoices SET updated_at = ?, issue_date = ?, sync_status = 'pending' WHERE id = ?")
            .bind(&now)
            .bind(issue_date)
            .bind(id);
    }
    if let Some(due_date) = due_date {
        query_builder = sqlx::query("UPDATE invoices SET updated_at = ?, due_date = ?, sync_status = 'pending' WHERE id = ?")
            .bind(&now)
            .bind(due_date)
            .bind(id);
    }
    if let Some(paid_date) = paid_date {
        query_builder = sqlx::query("UPDATE invoices SET updated_at = ?, paid_date = ?, sync_status = 'pending' WHERE id = ?")
            .bind(&now)
            .bind(paid_date)
            .bind(id);
    }
    if let Some(purchase_order) = purchase_order {
        query_builder = sqlx::query("UPDATE invoices SET updated_at = ?, purchase_order = ?, sync_status = 'pending' WHERE id = ?")
            .bind(&now)
            .bind(purchase_order)
            .bind(id);
    }
    if let Some(purchase_order_date) = purchase_order_date {
        query_builder = sqlx::query("UPDATE invoices SET updated_at = ?, purchase_order_date = ?, sync_status = 'pending' WHERE id = ?")
            .bind(&now)
            .bind(purchase_order_date)
            .bind(id);
    }
    if let Some(commercial_in_charge) = commercial_in_charge {
        query_builder = sqlx::query("UPDATE invoices SET updated_at = ?, commercial_in_charge = ?, sync_status = 'pending' WHERE id = ?")
            .bind(&now)
            .bind(commercial_in_charge)
            .bind(id);
    }
    if let Some(notes) = notes {
        query_builder = sqlx::query("UPDATE invoices SET updated_at = ?, notes = ?, sync_status = 'pending' WHERE id = ?")
            .bind(&now)
            .bind(notes)
            .bind(id);
    }

    query_builder.execute(pool).await?;

    // Return updated invoice
    let invoice = sqlx::query_as::<_, Invoice>("SELECT * FROM invoices WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(invoice)
}

pub async fn delete_invoice(pool: &DbPool, id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM invoices WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_invoices_by_status(pool: &DbPool, status: &str) -> Result<Vec<InvoiceWithDetails>, sqlx::Error> {
    let invoices = sqlx::query_as::<_, InvoiceWithDetails>(
        "SELECT 
            i.*,
            p.proposal_number,
            c.name as company_name
        FROM invoices i
        JOIN proposals p ON i.proposal_id = p.id
        JOIN companies c ON p.company_id = c.id
        WHERE i.status = ?
        ORDER BY i.created_at DESC"
    )
    .bind(status)
    .fetch_all(pool)
    .await?;

    Ok(invoices)
}

pub async fn get_invoices_by_company(pool: &DbPool, company_id: &str) -> Result<Vec<InvoiceWithDetails>, sqlx::Error> {
    let invoices = sqlx::query_as::<_, InvoiceWithDetails>(
        "SELECT 
            i.*,
            p.proposal_number,
            c.name as company_name
        FROM invoices i
        JOIN proposals p ON i.proposal_id = p.id
        JOIN companies c ON p.company_id = c.id
        WHERE p.company_id = ?
        ORDER BY i.created_at DESC"
    )
    .bind(company_id)
    .fetch_all(pool)
    .await?;

    Ok(invoices)
}

pub async fn invoice_exists_for_proposal(pool: &DbPool, proposal_id: &str) -> Result<bool, sqlx::Error> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM invoices WHERE proposal_id = ?")
        .bind(proposal_id)
        .fetch_one(pool)
        .await?;

    Ok(count.0 > 0)
}

pub async fn generate_invoice_number(pool: &DbPool) -> Result<String, sqlx::Error> {
    let now = Utc::now();
    let year = now.year() % 100;
    let month = now.month();

    // Count existing invoices for current month
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM invoices 
         WHERE strftime('%Y', issue_date) = ? AND strftime('%m', issue_date) = ?"
    )
    .bind(now.year().to_string())
    .bind(format!("{:02}", month))
    .fetch_one(pool)
    .await?;

    let next_number = count.0 + 1;
    Ok(format!("{:02}{:02}{:04}", year, month, next_number))
}

// add_to_deletion_queue - REMOVED as part of synchronization cleanup

// get_pending_deletions - REMOVED as part of synchronization cleanup

// mark_deletion_synced and remove_synced_deletions - REMOVED as part of synchronization cleanup

// === ACTION QUEUE FUNCTIONS - REMOVED ===
// These functions were removed as part of synchronization cleanup

// === UPSERT FUNCTIONS (for sync) - REMOVED ===
// These functions were removed as part of synchronization cleanup

// === DIRECT DELETION FUNCTIONS (for sync) - REMOVED ===
// These functions were removed as part of synchronization cleanup


