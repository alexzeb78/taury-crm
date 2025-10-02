use sqlx::{sqlite::{SqlitePool, SqliteConnectOptions}, Pool, Sqlite};
use std::path::PathBuf;
use std::str::FromStr;

pub mod schema;
pub mod models;
pub mod queries;

pub type DbPool = Pool<Sqlite>;

pub async fn init_database(app_dir: PathBuf) -> Result<DbPool, sqlx::Error> {
    let db_path = app_dir.join("crm.db");
    let db_url = format!("sqlite:{}", db_path.display());

    println!("🔧 Initializing database at: {}", db_path.display());
    
    // Ensure the directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| sqlx::Error::Configuration(format!("Failed to create app directory: {}", e).into()))?;
    }

    // Create connection options with create_if_missing enabled
    let options = SqliteConnectOptions::from_str(&db_url)?
        .create_if_missing(true);

    println!("🔧 Connecting to database...");
    // Create the database if it doesn't exist
    let pool = SqlitePool::connect_with(options).await?;
    println!("✅ Database connection established");

    // Run migrations
    println!("🔧 Running database migrations...");
    
    let migrations = [
        ("roles", schema::CREATE_ROLES_TABLE),
        ("default_roles", schema::INSERT_DEFAULT_ROLES),
        ("users", schema::CREATE_USERS_TABLE),
        ("user_roles", schema::CREATE_USER_ROLES_TABLE),
        ("companies", schema::CREATE_COMPANIES_TABLE),
        ("company_contacts", schema::CREATE_COMPANY_CONTACTS_TABLE),
        ("licence_pricing", schema::CREATE_LICENCE_PRICING_TABLE),
        ("pricing_data", schema::INSERT_PRICING_DATA),
        ("proposals", schema::CREATE_PROPOSALS_TABLE),
        ("proposal_products", schema::CREATE_PROPOSAL_PRODUCTS_TABLE),
        ("proposal_products_ids", schema::MIGRATE_PROPOSAL_PRODUCTS_IDS),
        ("invoices", schema::CREATE_INVOICES_TABLE),
        ("customers", schema::CREATE_CUSTOMERS_TABLE),
        ("documents", schema::CREATE_DOCUMENTS_TABLE),
        ("sync_metadata", schema::CREATE_SYNC_METADATA_TABLE),
    ];

    for (name, sql) in migrations {
        println!("  📋 Running migration: {}", name);
        sqlx::query(sql).execute(&pool).await
            .map_err(|e| {
                eprintln!("❌ Migration failed for {}: {}", name, e);
                e
            })?;
    }
    
    // Add sync columns to existing tables (safe to run multiple times)
    println!("🔧 Adding sync columns...");
    let sync_migrations = [
        ("companies_sync", schema::ADD_SYNC_COLUMNS_COMPANIES),
        ("company_contacts_sync", schema::ADD_SYNC_COLUMNS_COMPANY_CONTACTS),
        ("customers_sync", schema::ADD_SYNC_COLUMNS_CUSTOMERS),
        ("proposals_sync", schema::ADD_SYNC_COLUMNS_PROPOSALS),
        ("proposal_products_sync", schema::ADD_SYNC_COLUMNS_PROPOSAL_PRODUCTS),
        ("invoices_sync", schema::ADD_SYNC_COLUMNS_INVOICES),
        ("documents_sync", schema::ADD_SYNC_COLUMNS_DOCUMENTS),
    ];

    for (name, sql) in sync_migrations {
        println!("  📋 Adding sync columns: {}", name);
        let _ = sqlx::query(sql).execute(&pool).await;
    }
    
    // Create sync indexes
    println!("🔧 Creating sync indexes...");
    sqlx::query(schema::CREATE_SYNC_INDEXES).execute(&pool).await?;

    println!("✅ Database initialization completed successfully");
    Ok(pool)
}

