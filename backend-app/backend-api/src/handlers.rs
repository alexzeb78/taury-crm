use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::*;
use chrono::Utc;

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
        timestamp: Utc::now().to_rfc3339(),
    })
}

#[post("/sync/customer")]
pub async fn sync_customer(
    pool: web::Data<PgPool>,
    request: web::Json<SyncRequest>,
) -> impl Responder {
    match request.action.as_str() {
        "create" | "update" => {
            let customer: Customer = match serde_json::from_str(&request.payload) {
                Ok(c) => c,
                Err(e) => {
                    return HttpResponse::BadRequest().json(SyncResponse {
                        success: false,
                        message: format!("Invalid payload: {}", e),
                    });
                }
            };

            let result = sqlx::query(
                "INSERT INTO customers (id, name, email, phone, address, notes, created_at, updated_at)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                 ON CONFLICT (id) DO UPDATE SET
                    name = EXCLUDED.name,
                    email = EXCLUDED.email,
                    phone = EXCLUDED.phone,
                    address = EXCLUDED.address,
                    notes = EXCLUDED.notes,
                    updated_at = EXCLUDED.updated_at"
            )
            .bind(&customer.id)
            .bind(&customer.name)
            .bind(&customer.email)
            .bind(&customer.phone)
            .bind(&customer.address)
            .bind(&customer.notes)
            .bind(Utc::now())
            .bind(Utc::now())
            .execute(pool.get_ref())
            .await;

            match result {
                Ok(_) => HttpResponse::Ok().json(SyncResponse {
                    success: true,
                    message: "Customer synced successfully".to_string(),
                }),
                Err(e) => HttpResponse::InternalServerError().json(SyncResponse {
                    success: false,
                    message: format!("Failed to sync customer: {}", e),
                }),
            }
        }
        "delete" => {
            let result = sqlx::query("DELETE FROM customers WHERE id = $1")
                .bind(&request.entity_id)
                .execute(pool.get_ref())
                .await;

            match result {
                Ok(_) => HttpResponse::Ok().json(SyncResponse {
                    success: true,
                    message: "Customer deleted successfully".to_string(),
                }),
                Err(e) => HttpResponse::InternalServerError().json(SyncResponse {
                    success: false,
                    message: format!("Failed to delete customer: {}", e),
                }),
            }
        }
        _ => HttpResponse::BadRequest().json(SyncResponse {
            success: false,
            message: format!("Unknown action: {}", request.action),
        }),
    }
}

#[post("/sync/document")]
pub async fn sync_document(
    pool: web::Data<PgPool>,
    request: web::Json<SyncRequest>,
) -> impl Responder {
    match request.action.as_str() {
        "create" | "update" => {
            let document: Document = match serde_json::from_str(&request.payload) {
                Ok(d) => d,
                Err(e) => {
                    return HttpResponse::BadRequest().json(SyncResponse {
                        success: false,
                        message: format!("Invalid payload: {}", e),
                    });
                }
            };

            let result = sqlx::query(
                "INSERT INTO documents (id, customer_id, title, document_type, file_path, content, created_at, updated_at)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                 ON CONFLICT (id) DO UPDATE SET
                    customer_id = EXCLUDED.customer_id,
                    title = EXCLUDED.title,
                    document_type = EXCLUDED.document_type,
                    file_path = EXCLUDED.file_path,
                    content = EXCLUDED.content,
                    updated_at = EXCLUDED.updated_at"
            )
            .bind(&document.id)
            .bind(&document.customer_id)
            .bind(&document.title)
            .bind(&document.document_type)
            .bind(&document.file_path)
            .bind(&document.content)
            .bind(Utc::now())
            .bind(Utc::now())
            .execute(pool.get_ref())
            .await;

            match result {
                Ok(_) => HttpResponse::Ok().json(SyncResponse {
                    success: true,
                    message: "Document synced successfully".to_string(),
                }),
                Err(e) => HttpResponse::InternalServerError().json(SyncResponse {
                    success: false,
                    message: format!("Failed to sync document: {}", e),
                }),
            }
        }
        "delete" => {
            let result = sqlx::query("DELETE FROM documents WHERE id = $1")
                .bind(&request.entity_id)
                .execute(pool.get_ref())
                .await;

            match result {
                Ok(_) => HttpResponse::Ok().json(SyncResponse {
                    success: true,
                    message: "Document deleted successfully".to_string(),
                }),
                Err(e) => HttpResponse::InternalServerError().json(SyncResponse {
                    success: false,
                    message: format!("Failed to delete document: {}", e),
                }),
            }
        }
        _ => HttpResponse::BadRequest().json(SyncResponse {
            success: false,
            message: format!("Unknown action: {}", request.action),
        }),
    }
}

#[get("/customers")]
pub async fn get_customers(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, Customer>("SELECT * FROM customers ORDER BY created_at DESC")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(customers) => HttpResponse::Ok().json(customers),
        Err(e) => HttpResponse::InternalServerError().json(SyncResponse {
            success: false,
            message: format!("Failed to get customers: {}", e),
        }),
    }
}

#[get("/documents")]
pub async fn get_documents(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, Document>("SELECT * FROM documents ORDER BY created_at DESC")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(documents) => HttpResponse::Ok().json(documents),
        Err(e) => HttpResponse::InternalServerError().json(SyncResponse {
            success: false,
            message: format!("Failed to get documents: {}", e),
        }),
    }
}

#[post("/sync/user")]
pub async fn sync_user(
    pool: web::Data<PgPool>,
    request: web::Json<SyncRequest>,
) -> impl Responder {
    match request.action.as_str() {
        "create" | "update" => {
            let user: User = match serde_json::from_str(&request.payload) {
                Ok(u) => u,
                Err(e) => {
                    return HttpResponse::BadRequest().json(SyncResponse {
                        success: false,
                        message: format!("Invalid payload: {}", e),
                    });
                }
            };

            let result = sqlx::query(
                "INSERT INTO users (id, email, name, created_at, updated_at)
                 VALUES ($1, $2, $3, $4, $5)
                 ON CONFLICT (id) DO UPDATE SET
                    email = EXCLUDED.email,
                    name = EXCLUDED.name,
                    updated_at = EXCLUDED.updated_at"
            )
            .bind(&user.id)
            .bind(&user.email)
            .bind(&user.name)
            .bind(Utc::now())
            .bind(Utc::now())
            .execute(pool.get_ref())
            .await;

            match result {
                Ok(_) => HttpResponse::Ok().json(SyncResponse {
                    success: true,
                    message: "User synced successfully".to_string(),
                }),
                Err(e) => HttpResponse::InternalServerError().json(SyncResponse {
                    success: false,
                    message: format!("Failed to sync user: {}", e),
                }),
            }
        }
        "delete" => {
            let result = sqlx::query("DELETE FROM users WHERE id = $1")
                .bind(&request.entity_id)
                .execute(pool.get_ref())
                .await;

            match result {
                Ok(_) => HttpResponse::Ok().json(SyncResponse {
                    success: true,
                    message: "User deleted successfully".to_string(),
                }),
                Err(e) => HttpResponse::InternalServerError().json(SyncResponse {
                    success: false,
                    message: format!("Failed to delete user: {}", e),
                }),
            }
        }
        _ => HttpResponse::BadRequest().json(SyncResponse {
            success: false,
            message: format!("Unknown action: {}", request.action),
        }),
    }
}

#[get("/users")]
pub async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().json(SyncResponse {
            success: false,
            message: format!("Failed to get users: {}", e),
        }),
    }
}

#[get("/companies")]
pub async fn get_companies(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, Company>("SELECT * FROM companies ORDER BY name ASC")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(companies) => HttpResponse::Ok().json(companies),
        Err(e) => HttpResponse::InternalServerError().json(SyncResponse {
            success: false,
            message: format!("Failed to get companies: {}", e),
        }),
    }
}

#[post("/sync/company")]
pub async fn sync_company(
    pool: web::Data<PgPool>,
    request: web::Json<SyncRequest>,
) -> impl Responder {
    match request.action.as_str() {
        "create" | "update" => {
            let company: Company = match serde_json::from_str(&request.payload) {
                Ok(c) => c,
                Err(e) => {
                    return HttpResponse::BadRequest().json(SyncResponse {
                        success: false,
                        message: format!("Invalid payload: {}", e),
                    });
                }
            };

            let result = sqlx::query(
                "INSERT INTO companies (id, name, website, address, city, postal_code, country, description, created_at, updated_at)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                 ON CONFLICT (id) DO UPDATE SET
                    name = EXCLUDED.name,
                    website = EXCLUDED.website,
                    address = EXCLUDED.address,
                    city = EXCLUDED.city,
                    postal_code = EXCLUDED.postal_code,
                    country = EXCLUDED.country,
                    description = EXCLUDED.description,
                    updated_at = EXCLUDED.updated_at"
            )
            .bind(&company.id)
            .bind(&company.name)
            .bind(&company.website)
            .bind(&company.address)
            .bind(&company.city)
            .bind(&company.postal_code)
            .bind(&company.country)
            .bind(&company.description)
            .bind(Utc::now())
            .bind(Utc::now())
            .execute(pool.get_ref())
            .await;

            match result {
                Ok(_) => HttpResponse::Ok().json(SyncResponse {
                    success: true,
                    message: "Company synced successfully".to_string(),
                }),
                Err(e) => HttpResponse::InternalServerError().json(SyncResponse {
                    success: false,
                    message: format!("Failed to sync company: {}", e),
                }),
            }
        }
        "delete" => {
            let result = sqlx::query("DELETE FROM companies WHERE id = $1")
                .bind(&request.entity_id)
                .execute(pool.get_ref())
                .await;

            match result {
                Ok(_) => HttpResponse::Ok().json(SyncResponse {
                    success: true,
                    message: "Company deleted successfully".to_string(),
                }),
                Err(e) => HttpResponse::InternalServerError().json(SyncResponse {
                    success: false,
                    message: format!("Failed to delete company: {}", e),
                }),
            }
        }
        _ => HttpResponse::BadRequest().json(SyncResponse {
            success: false,
            message: format!("Unknown action: {}", request.action),
        }),
    }
}

#[get("/proposals")]
pub async fn get_proposals(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, Proposal>("SELECT * FROM proposals ORDER BY created_at DESC")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(proposals) => HttpResponse::Ok().json(proposals),
        Err(e) => HttpResponse::InternalServerError().json(SyncResponse {
            success: false,
            message: format!("Failed to get proposals: {}", e),
        }),
    }
}

#[post("/sync/proposal")]
pub async fn sync_proposal(
    pool: web::Data<PgPool>,
    request: web::Json<SyncRequest>,
) -> impl Responder {
    match request.action.as_str() {
        "create" | "update" => {
            // Parse the full payload with products
            let payload_value: serde_json::Value = match serde_json::from_str(&request.payload) {
                Ok(v) => v,
                Err(e) => {
                    return HttpResponse::BadRequest().json(SyncResponse {
                        success: false,
                        message: format!("Invalid payload: {}", e),
                    });
                }
            };
            
            // Extract proposal data
            let proposal: Proposal = match serde_json::from_value(payload_value.clone()) {
                Ok(p) => p,
                Err(e) => {
                    return HttpResponse::BadRequest().json(SyncResponse {
                        success: false,
                        message: format!("Invalid proposal: {}", e),
                    });
                }
            };
            
            // Extract products array
            let products: Vec<ProposalProduct> = match payload_value.get("products") {
                Some(products_value) => match serde_json::from_value(products_value.clone()) {
                    Ok(p) => p,
                    Err(_) => Vec::new(),
                },
                None => Vec::new(),
            };

            // Save proposal
            let result = sqlx::query(
                "INSERT INTO proposals (id, company_id, proposal_number, status, total_amount, currency, valid_until, notes, created_at, updated_at)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                 ON CONFLICT (id) DO UPDATE SET
                    company_id = EXCLUDED.company_id,
                    proposal_number = EXCLUDED.proposal_number,
                    status = EXCLUDED.status,
                    total_amount = EXCLUDED.total_amount,
                    currency = EXCLUDED.currency,
                    valid_until = EXCLUDED.valid_until,
                    notes = EXCLUDED.notes,
                    updated_at = EXCLUDED.updated_at"
            )
            .bind(&proposal.id)
            .bind(&proposal.company_id)
            .bind(&proposal.proposal_number)
            .bind(&proposal.status)
            .bind(&proposal.total_amount)
            .bind(&proposal.currency)
            .bind(&proposal.valid_until)
            .bind(&proposal.notes)
            .bind(Utc::now())
            .bind(Utc::now())
            .execute(pool.get_ref())
            .await;

            if result.is_err() {
                return HttpResponse::InternalServerError().json(SyncResponse {
                    success: false,
                    message: format!("Failed to sync proposal: {}", result.unwrap_err()),
                });
            }
            
            // Delete old products for this proposal
            let _ = sqlx::query("DELETE FROM proposal_products WHERE proposal_id = $1")
                .bind(&proposal.id)
                .execute(pool.get_ref())
                .await;
            
            // Save all products
            for product in products {
                let _ = sqlx::query(
                    "INSERT INTO proposal_products (id, proposal_id, product_type, user_count, standalone_count, server_key_count, 
                     unit_price, total_price, annual_reduction, training, training_days, training_cost_per_day, training_cost, 
                     licence, support, support_years)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)"
                )
                .bind(&product.id)
                .bind(&product.proposal_id)
                .bind(&product.product_type)
                .bind(&product.user_count)
                .bind(&product.standalone_count)
                .bind(&product.server_key_count)
                .bind(&product.unit_price)
                .bind(&product.total_price)
                .bind(&product.annual_reduction)
                .bind(&product.training)
                .bind(&product.training_days)
                .bind(&product.training_cost_per_day)
                .bind(&product.training_cost)
                .bind(&product.licence)
                .bind(&product.support)
                .bind(&product.support_years)
                .execute(pool.get_ref())
                .await;
            }

            HttpResponse::Ok().json(SyncResponse {
                success: true,
                message: "Proposal and products synced successfully".to_string(),
            })
        }
        "delete" => {
            let _ = sqlx::query("DELETE FROM proposal_products WHERE proposal_id = $1")
                .bind(&request.entity_id)
                .execute(pool.get_ref())
                .await;

            let result = sqlx::query("DELETE FROM proposals WHERE id = $1")
                .bind(&request.entity_id)
                .execute(pool.get_ref())
                .await;

            match result {
                Ok(_) => HttpResponse::Ok().json(SyncResponse {
                    success: true,
                    message: "Proposal deleted successfully".to_string(),
                }),
                Err(e) => HttpResponse::InternalServerError().json(SyncResponse {
                    success: false,
                    message: format!("Failed to delete proposal: {}", e),
                }),
            }
        }
        _ => HttpResponse::BadRequest().json(SyncResponse {
            success: false,
            message: format!("Unknown action: {}", request.action),
        }),
    }
}

#[get("/proposal_products/{proposal_id}")]
pub async fn get_proposal_products(
    pool: web::Data<PgPool>,
    proposal_id: web::Path<String>,
) -> impl Responder {
    match sqlx::query_as::<_, ProposalProduct>(
        "SELECT * FROM proposal_products WHERE proposal_id = $1 ORDER BY id ASC"
    )
    .bind(proposal_id.as_str())
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(e) => HttpResponse::InternalServerError().json(SyncResponse {
            success: false,
            message: format!("Failed to get proposal products: {}", e),
        }),
    }
}

#[post("/sync/proposal_product")]
pub async fn sync_proposal_product(
    pool: web::Data<PgPool>,
    request: web::Json<SyncRequest>,
) -> impl Responder {
    match request.action.as_str() {
        "create" | "update" => {
            let product: ProposalProduct = match serde_json::from_str(&request.payload) {
                Ok(p) => p,
                Err(e) => {
                    return HttpResponse::BadRequest().json(SyncResponse {
                        success: false,
                        message: format!("Invalid payload: {}", e),
                    });
                }
            };

            let result = sqlx::query(
                "INSERT INTO proposal_products (id, proposal_id, product_type, user_count, standalone_count, server_key_count, 
                 unit_price, total_price, annual_reduction, training, training_days, training_cost_per_day, training_cost, 
                 licence, support, support_years)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
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
                    support_years = EXCLUDED.support_years"
            )
            .bind(&product.id)
            .bind(&product.proposal_id)
            .bind(&product.product_type)
            .bind(&product.user_count)
            .bind(&product.standalone_count)
            .bind(&product.server_key_count)
            .bind(&product.unit_price)
            .bind(&product.total_price)
            .bind(&product.annual_reduction)
            .bind(&product.training)
            .bind(&product.training_days)
            .bind(&product.training_cost_per_day)
            .bind(&product.training_cost)
            .bind(&product.licence)
            .bind(&product.support)
            .bind(&product.support_years)
            .execute(pool.get_ref())
            .await;

            match result {
                Ok(_) => HttpResponse::Ok().json(SyncResponse {
                    success: true,
                    message: "Proposal product synced successfully".to_string(),
                }),
                Err(e) => HttpResponse::InternalServerError().json(SyncResponse {
                    success: false,
                    message: format!("Failed to sync proposal product: {}", e),
                }),
            }
        }
        "delete" => {
            let result = sqlx::query("DELETE FROM proposal_products WHERE id = $1")
                .bind(&request.entity_id)
                .execute(pool.get_ref())
                .await;

            match result {
                Ok(_) => HttpResponse::Ok().json(SyncResponse {
                    success: true,
                    message: "Proposal product deleted successfully".to_string(),
                }),
                Err(e) => HttpResponse::InternalServerError().json(SyncResponse {
                    success: false,
                    message: format!("Failed to delete proposal product: {}", e),
                }),
            }
        }
        _ => HttpResponse::BadRequest().json(SyncResponse {
            success: false,
            message: format!("Unknown action: {}", request.action),
        }),
    }
}

