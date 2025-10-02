use sqlx::PgPool;

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Create users table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            email TEXT UNIQUE NOT NULL,
            name TEXT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )"
    )
    .execute(pool)
    .await?;

    // Create index for users
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_email ON users(email)")
        .execute(pool)
        .await?;

    // Create companies table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS companies (
            id TEXT PRIMARY KEY,
            name TEXT UNIQUE NOT NULL,
            website TEXT,
            address TEXT,
            city TEXT,
            postal_code TEXT,
            country TEXT,
            description TEXT,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            version INTEGER DEFAULT 1,
            is_deleted INTEGER DEFAULT 0
        )"
    )
    .execute(pool)
    .await?;

    // Create company_contacts table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS company_contacts (
            id TEXT PRIMARY KEY,
            company_id TEXT NOT NULL,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            email TEXT NOT NULL,
            phone_number TEXT,
            is_primary BOOLEAN DEFAULT FALSE,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            version INTEGER DEFAULT 1,
            is_deleted INTEGER DEFAULT 0,
            FOREIGN KEY (company_id) REFERENCES companies(id) ON DELETE CASCADE
        )"
    )
    .execute(pool)
    .await?;

    // Create indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_companies_name ON companies(name)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_company_contacts_company_id ON company_contacts(company_id)")
        .execute(pool)
        .await?;

    // Create customers table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS customers (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT,
            phone TEXT,
            address TEXT,
            notes TEXT,
            created_at TIMESTAMPTZ NOT NULL,
            updated_at TIMESTAMPTZ NOT NULL,
            version INTEGER DEFAULT 1,
            is_deleted INTEGER DEFAULT 0
        )"
    )
    .execute(pool)
    .await?;

    // Create documents table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS documents (
            id TEXT PRIMARY KEY,
            customer_id TEXT NOT NULL,
            title TEXT NOT NULL,
            document_type TEXT NOT NULL,
            file_path TEXT,
            content TEXT,
            created_at TIMESTAMPTZ NOT NULL,
            updated_at TIMESTAMPTZ NOT NULL,
            version INTEGER DEFAULT 1,
            is_deleted INTEGER DEFAULT 0,
            FOREIGN KEY (customer_id) REFERENCES customers(id) ON DELETE CASCADE
        )"
    )
    .execute(pool)
    .await?;

    // Create proposals table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS proposals (
            id TEXT PRIMARY KEY,
            company_id TEXT NOT NULL,
            proposal_number TEXT,
            status TEXT NOT NULL DEFAULT 'DRAFT',
            total_amount DOUBLE PRECISION,
            currency TEXT NOT NULL DEFAULT 'USD',
            valid_until TIMESTAMPTZ,
            notes TEXT,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            version INTEGER DEFAULT 1,
            is_deleted INTEGER DEFAULT 0,
            FOREIGN KEY (company_id) REFERENCES companies(id) ON DELETE CASCADE
        )"
    )
    .execute(pool)
    .await?;

    // Create proposal_products table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS proposal_products (
            id TEXT PRIMARY KEY,
            proposal_id TEXT NOT NULL,
            product_type TEXT NOT NULL,
            user_count INTEGER NOT NULL DEFAULT 0,
            standalone_count INTEGER NOT NULL DEFAULT 0,
            server_key_count INTEGER NOT NULL DEFAULT 0,
            unit_price DOUBLE PRECISION,
            total_price DOUBLE PRECISION,
            annual_reduction DOUBLE PRECISION NOT NULL DEFAULT 0,
            training INTEGER NOT NULL DEFAULT 0,
            training_days INTEGER NOT NULL DEFAULT 0,
            training_cost_per_day DOUBLE PRECISION NOT NULL DEFAULT 0,
            training_cost DOUBLE PRECISION NOT NULL DEFAULT 0,
            licence INTEGER NOT NULL DEFAULT 1,
            support INTEGER NOT NULL DEFAULT 0,
            support_years INTEGER NOT NULL DEFAULT 0,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            version INTEGER DEFAULT 1,
            is_deleted INTEGER DEFAULT 0,
            FOREIGN KEY (proposal_id) REFERENCES proposals(id) ON DELETE CASCADE
        )"
    )
    .execute(pool)
    .await?;

    // Create indexes for proposals
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_proposals_company_id ON proposals(company_id)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_proposals_created_at ON proposals(created_at DESC)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_proposal_products_proposal_id ON proposal_products(proposal_id)")
        .execute(pool)
        .await?;

    // Create invoices table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS invoices (
            id TEXT PRIMARY KEY,
            proposal_id TEXT NOT NULL,
            invoice_number TEXT UNIQUE NOT NULL,
            status TEXT NOT NULL DEFAULT 'DRAFT',
            total_amount DOUBLE PRECISION NOT NULL DEFAULT 0.0,
            currency TEXT NOT NULL DEFAULT 'USD',
            issue_date TIMESTAMPTZ NOT NULL,
            due_date TIMESTAMPTZ,
            paid_date TIMESTAMPTZ,
            purchase_order TEXT,
            purchase_order_date TIMESTAMPTZ,
            commercial_in_charge TEXT,
            notes TEXT,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            version INTEGER DEFAULT 1,
            is_deleted INTEGER DEFAULT 0,
            FOREIGN KEY (proposal_id) REFERENCES proposals(id) ON DELETE CASCADE
        )"
    )
    .execute(pool)
    .await?;

    // Add sync columns to existing tables (safe to run multiple times)
    let _ = sqlx::query("ALTER TABLE companies ADD COLUMN IF NOT EXISTS version INTEGER DEFAULT 1").execute(pool).await;
    let _ = sqlx::query("ALTER TABLE companies ADD COLUMN IF NOT EXISTS is_deleted INTEGER DEFAULT 0").execute(pool).await;
    let _ = sqlx::query("ALTER TABLE company_contacts ADD COLUMN IF NOT EXISTS version INTEGER DEFAULT 1").execute(pool).await;
    let _ = sqlx::query("ALTER TABLE company_contacts ADD COLUMN IF NOT EXISTS is_deleted INTEGER DEFAULT 0").execute(pool).await;
    let _ = sqlx::query("ALTER TABLE customers ADD COLUMN IF NOT EXISTS version INTEGER DEFAULT 1").execute(pool).await;
    let _ = sqlx::query("ALTER TABLE customers ADD COLUMN IF NOT EXISTS is_deleted INTEGER DEFAULT 0").execute(pool).await;
    let _ = sqlx::query("ALTER TABLE customers ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT NOW()").execute(pool).await;
    let _ = sqlx::query("ALTER TABLE proposals ADD COLUMN IF NOT EXISTS version INTEGER DEFAULT 1").execute(pool).await;
    let _ = sqlx::query("ALTER TABLE proposals ADD COLUMN IF NOT EXISTS is_deleted INTEGER DEFAULT 0").execute(pool).await;
    let _ = sqlx::query("ALTER TABLE proposal_products ADD COLUMN IF NOT EXISTS version INTEGER DEFAULT 1").execute(pool).await;
    let _ = sqlx::query("ALTER TABLE proposal_products ADD COLUMN IF NOT EXISTS is_deleted INTEGER DEFAULT 0").execute(pool).await;
    let _ = sqlx::query("ALTER TABLE proposal_products ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT NOW()").execute(pool).await;
    let _ = sqlx::query("ALTER TABLE invoices ADD COLUMN IF NOT EXISTS version INTEGER DEFAULT 1").execute(pool).await;
    let _ = sqlx::query("ALTER TABLE invoices ADD COLUMN IF NOT EXISTS is_deleted INTEGER DEFAULT 0").execute(pool).await;
    let _ = sqlx::query("ALTER TABLE documents ADD COLUMN IF NOT EXISTS version INTEGER DEFAULT 1").execute(pool).await;
    let _ = sqlx::query("ALTER TABLE documents ADD COLUMN IF NOT EXISTS is_deleted INTEGER DEFAULT 0").execute(pool).await;
    let _ = sqlx::query("ALTER TABLE documents ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT NOW()").execute(pool).await;

    // Create sync indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_companies_updated_at ON companies(updated_at)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_companies_is_deleted ON companies(is_deleted)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_company_contacts_updated_at ON company_contacts(updated_at)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_company_contacts_is_deleted ON company_contacts(is_deleted)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_customers_updated_at ON customers(updated_at)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_customers_is_deleted ON customers(is_deleted)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_proposals_updated_at ON proposals(updated_at)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_proposals_is_deleted ON proposals(is_deleted)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_proposal_products_updated_at ON proposal_products(updated_at)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_proposal_products_is_deleted ON proposal_products(is_deleted)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_invoices_updated_at ON invoices(updated_at)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_invoices_is_deleted ON invoices(is_deleted)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_documents_updated_at ON documents(updated_at)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_documents_is_deleted ON documents(is_deleted)")
        .execute(pool)
        .await?;

    Ok(())
}

