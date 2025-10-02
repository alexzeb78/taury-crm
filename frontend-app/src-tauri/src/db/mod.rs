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

    // Create connection options with create_if_missing enabled
    let options = SqliteConnectOptions::from_str(&db_url)?
        .create_if_missing(true);

    // Create the database if it doesn't exist
    let pool = SqlitePool::connect_with(options).await?;

    // Run migrations
    sqlx::query(schema::CREATE_ROLES_TABLE).execute(&pool).await?;
    sqlx::query(schema::INSERT_DEFAULT_ROLES).execute(&pool).await?;
    sqlx::query(schema::CREATE_USERS_TABLE).execute(&pool).await?;
    sqlx::query(schema::CREATE_USER_ROLES_TABLE).execute(&pool).await?;
    sqlx::query(schema::CREATE_COMPANIES_TABLE).execute(&pool).await?;
    sqlx::query(schema::CREATE_COMPANY_CONTACTS_TABLE).execute(&pool).await?;
    sqlx::query(schema::CREATE_LICENCE_PRICING_TABLE).execute(&pool).await?;
    sqlx::query(schema::INSERT_PRICING_DATA).execute(&pool).await?;
    sqlx::query(schema::CREATE_PROPOSALS_TABLE).execute(&pool).await?;
    sqlx::query(schema::CREATE_PROPOSAL_PRODUCTS_TABLE).execute(&pool).await?;
    sqlx::query(schema::MIGRATE_PROPOSAL_PRODUCTS_IDS).execute(&pool).await?;
    sqlx::query(schema::CREATE_INVOICES_TABLE).execute(&pool).await?;
    sqlx::query(schema::CREATE_CUSTOMERS_TABLE).execute(&pool).await?;
    sqlx::query(schema::CREATE_DOCUMENTS_TABLE).execute(&pool).await?;
    
    // Create sync metadata table
    sqlx::query(schema::CREATE_SYNC_METADATA_TABLE).execute(&pool).await?;
    
    // Add sync columns to existing tables (safe to run multiple times)
    let _ = sqlx::query(schema::ADD_SYNC_COLUMNS_COMPANIES).execute(&pool).await;
    let _ = sqlx::query(schema::ADD_SYNC_COLUMNS_COMPANY_CONTACTS).execute(&pool).await;
    let _ = sqlx::query(schema::ADD_SYNC_COLUMNS_CUSTOMERS).execute(&pool).await;
    let _ = sqlx::query(schema::ADD_SYNC_COLUMNS_PROPOSALS).execute(&pool).await;
    let _ = sqlx::query(schema::ADD_SYNC_COLUMNS_PROPOSAL_PRODUCTS).execute(&pool).await;
    let _ = sqlx::query(schema::ADD_SYNC_COLUMNS_INVOICES).execute(&pool).await;
    let _ = sqlx::query(schema::ADD_SYNC_COLUMNS_DOCUMENTS).execute(&pool).await;
    
    // Create sync indexes
    sqlx::query(schema::CREATE_SYNC_INDEXES).execute(&pool).await?;

    Ok(pool)
}

