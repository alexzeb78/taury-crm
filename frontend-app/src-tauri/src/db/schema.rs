pub const CREATE_ROLES_TABLE: &str = "
CREATE TABLE IF NOT EXISTS roles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    description TEXT
);
";

pub const CREATE_USERS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    name TEXT NOT NULL,
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
";

pub const CREATE_USER_ROLES_TABLE: &str = "
CREATE TABLE IF NOT EXISTS user_roles (
    user_id TEXT NOT NULL,
    role_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, role_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE
);
";

pub const INSERT_DEFAULT_ROLES: &str = "
INSERT OR IGNORE INTO roles (name, description) VALUES 
('ADMIN', 'Administrator with full privileges'),
('MODERATOR', 'Moderator with limited privileges'),
('USER', 'Standard user');
";

pub const CREATE_COMPANIES_TABLE: &str = "
CREATE TABLE IF NOT EXISTS companies (
    id TEXT PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    website TEXT,
    address TEXT,
    city TEXT,
    postal_code TEXT,
    country TEXT,
    description TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    server_id TEXT
);
";

pub const CREATE_COMPANY_CONTACTS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS company_contacts (
    id TEXT PRIMARY KEY,
    company_id TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT NOT NULL,
    phone_number TEXT,
    is_primary INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (company_id) REFERENCES companies(id) ON DELETE CASCADE
);
";

pub const CREATE_CUSTOMERS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS customers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    address TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    server_id TEXT
);
";

pub const CREATE_LICENCE_PRICING_TABLE: &str = "
CREATE TABLE IF NOT EXISTS licence_pricing (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    product_type TEXT NOT NULL,
    user_count INTEGER NOT NULL,
    price_usd REAL NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(product_type, user_count)
);
";

// DELETION_QUEUE_TABLE - REMOVED as part of synchronization cleanup

// Migration to add IDs to existing proposal_products that don't have them
pub const MIGRATE_PROPOSAL_PRODUCTS_IDS: &str = "
UPDATE proposal_products 
SET id = 'prod_' || substr(hex(randomblob(4)), 1, 8) || '_' || substr(hex(randomblob(4)), 1, 8)
WHERE id IS NULL OR id = '';
";

// ACTIONS_QUEUE_TABLE - REMOVED as part of synchronization cleanup

pub const INSERT_PRICING_DATA: &str = "
INSERT OR IGNORE INTO licence_pricing (product_type, user_count, price_usd, created_at, updated_at) VALUES 
-- HTZ Communications (prix unitaire par licence)
('HTZ Communications', 1, 25000.00, datetime('now'), datetime('now')),
('HTZ Communications', 2, 22500.00, datetime('now'), datetime('now')),
('HTZ Communications', 3, 20000.00, datetime('now'), datetime('now')),
('HTZ Communications', 4, 17500.00, datetime('now'), datetime('now')),
('HTZ Communications', 5, 15750.00, datetime('now'), datetime('now')),
('HTZ Communications', 6, 15750.00, datetime('now'), datetime('now')),
('HTZ Communications', 7, 15750.00, datetime('now'), datetime('now')),
('HTZ Communications', 8, 15750.00, datetime('now'), datetime('now')),
('HTZ Communications', 9, 15750.00, datetime('now'), datetime('now')),
('HTZ Communications', 10, 15750.00, datetime('now'), datetime('now')),
('HTZ Communications', 11, 15750.00, datetime('now'), datetime('now')),
('HTZ Communications', 12, 15750.00, datetime('now'), datetime('now')),
('HTZ Communications', 13, 15750.00, datetime('now'), datetime('now')),
('HTZ Communications', 14, 15750.00, datetime('now'), datetime('now')),
('HTZ Communications', 15, 15750.00, datetime('now'), datetime('now')),
('HTZ Communications', 16, 15750.00, datetime('now'), datetime('now')),
('HTZ Communications', 17, 15750.00, datetime('now'), datetime('now')),
('HTZ Communications', 18, 15750.00, datetime('now'), datetime('now')),
('HTZ Communications', 19, 15750.00, datetime('now'), datetime('now')),
('HTZ Communications', 20, 15750.00, datetime('now'), datetime('now')),
-- HTZ Warfare (prix unitaire par licence)
('HTZ Warfare', 1, 38000.00, datetime('now'), datetime('now')),
('HTZ Warfare', 2, 34200.00, datetime('now'), datetime('now')),
('HTZ Warfare', 3, 30400.00, datetime('now'), datetime('now')),
('HTZ Warfare', 4, 26600.00, datetime('now'), datetime('now')),
('HTZ Warfare', 5, 26000.00, datetime('now'), datetime('now')),
('HTZ Warfare', 6, 26000.00, datetime('now'), datetime('now')),
('HTZ Warfare', 7, 26000.00, datetime('now'), datetime('now')),
('HTZ Warfare', 8, 26000.00, datetime('now'), datetime('now')),
('HTZ Warfare', 9, 26000.00, datetime('now'), datetime('now')),
('HTZ Warfare', 10, 26000.00, datetime('now'), datetime('now')),
('HTZ Warfare', 11, 26000.00, datetime('now'), datetime('now')),
('HTZ Warfare', 12, 26000.00, datetime('now'), datetime('now')),
('HTZ Warfare', 13, 26000.00, datetime('now'), datetime('now')),
('HTZ Warfare', 14, 26000.00, datetime('now'), datetime('now')),
('HTZ Warfare', 15, 26000.00, datetime('now'), datetime('now')),
('HTZ Warfare', 16, 26000.00, datetime('now'), datetime('now')),
('HTZ Warfare', 17, 26000.00, datetime('now'), datetime('now')),
('HTZ Warfare', 18, 26000.00, datetime('now'), datetime('now')),
('HTZ Warfare', 19, 26000.00, datetime('now'), datetime('now')),
('HTZ Warfare', 20, 26000.00, datetime('now'), datetime('now')),
-- ICS Manager (prix pour le serveur)
('ICS Manager', 1, 39600.00, datetime('now'), datetime('now')),
('ICS Manager', 2, 35640.00, datetime('now'), datetime('now')),
('ICS Manager', 3, 31680.00, datetime('now'), datetime('now')),
('ICS Manager', 4, 27720.00, datetime('now'), datetime('now')),
('ICS Manager', 5, 24940.00, datetime('now'), datetime('now')),
('ICS Manager', 6, 24940.00, datetime('now'), datetime('now')),
('ICS Manager', 7, 24940.00, datetime('now'), datetime('now')),
('ICS Manager', 8, 24940.00, datetime('now'), datetime('now')),
('ICS Manager', 9, 24940.00, datetime('now'), datetime('now')),
('ICS Manager', 10, 22450.00, datetime('now'), datetime('now')),
('ICS Manager', 11, 22450.00, datetime('now'), datetime('now')),
('ICS Manager', 12, 22450.00, datetime('now'), datetime('now')),
('ICS Manager', 13, 22450.00, datetime('now'), datetime('now')),
('ICS Manager', 14, 22450.00, datetime('now'), datetime('now')),
('ICS Manager', 15, 20190.00, datetime('now'), datetime('now')),
('ICS Manager', 16, 20190.00, datetime('now'), datetime('now')),
('ICS Manager', 17, 20190.00, datetime('now'), datetime('now')),
('ICS Manager', 18, 20190.00, datetime('now'), datetime('now')),
('ICS Manager', 19, 20190.00, datetime('now'), datetime('now')),
('ICS Manager', 20, 20190.00, datetime('now'), datetime('now')),
-- ICS Manager Additional (prix par client additionnel)
('ICS Manager Additional', 1, 10700.00, datetime('now'), datetime('now'));
";

pub const CREATE_PROPOSALS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS proposals (
    id TEXT PRIMARY KEY,
    company_id TEXT NOT NULL,
    proposal_number TEXT UNIQUE,
    status TEXT NOT NULL DEFAULT 'DRAFT',
    total_amount REAL,
    currency TEXT DEFAULT 'USD',
    valid_until TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (company_id) REFERENCES companies(id) ON DELETE CASCADE
);
";

pub const CREATE_PROPOSAL_PRODUCTS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS proposal_products (
    id TEXT PRIMARY KEY,
    proposal_id TEXT NOT NULL,
    product_type TEXT NOT NULL,
    user_count INTEGER NOT NULL,
    standalone_count INTEGER DEFAULT 0,
    server_key_count INTEGER DEFAULT 0,
    unit_price REAL,
    total_price REAL,
    annual_reduction REAL DEFAULT 0,
    training INTEGER DEFAULT 0,
    training_days INTEGER DEFAULT 0,
    training_cost_per_day REAL DEFAULT 0,
    training_cost REAL DEFAULT 0,
    licence INTEGER DEFAULT 0,
    support INTEGER DEFAULT 0,
    support_years INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (proposal_id) REFERENCES proposals(id) ON DELETE CASCADE
);
";

pub const CREATE_INVOICES_TABLE: &str = "
CREATE TABLE IF NOT EXISTS invoices (
    id TEXT PRIMARY KEY,
    proposal_id TEXT NOT NULL,
    invoice_number TEXT UNIQUE NOT NULL,
    status TEXT NOT NULL DEFAULT 'DRAFT',
    total_amount REAL NOT NULL DEFAULT 0.0,
    currency TEXT NOT NULL DEFAULT 'USD',
    issue_date TEXT NOT NULL,
    due_date TEXT,
    paid_date TEXT,
    purchase_order TEXT,
    purchase_order_date TEXT,
    commercial_in_charge TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (proposal_id) REFERENCES proposals(id) ON DELETE CASCADE
);
";

pub const CREATE_DOCUMENTS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS documents (
    id TEXT PRIMARY KEY,
    customer_id TEXT NOT NULL,
    title TEXT NOT NULL,
    document_type TEXT NOT NULL,
    file_path TEXT,
    content TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (customer_id) REFERENCES customers(id) ON DELETE CASCADE
);
";

// SYNC_QUEUE_TABLE - REMOVED as part of synchronization cleanup

// SETTINGS_TABLE - REMOVED as part of synchronization cleanup

// === SYNC METADATA TABLE ===
pub const CREATE_SYNC_METADATA_TABLE: &str = "
CREATE TABLE IF NOT EXISTS sync_metadata (
    id INTEGER PRIMARY KEY DEFAULT 1,
    last_sync_timestamp INTEGER NOT NULL DEFAULT 0,
    last_sync_version INTEGER NOT NULL DEFAULT 0,
    updated_at TEXT NOT NULL DEFAULT (strftime('%s', 'now') * 1000)
);
";

// === SYNC COLUMNS MIGRATIONS ===
pub const ADD_SYNC_COLUMNS_COMPANIES: &str = "
ALTER TABLE companies ADD COLUMN version INTEGER DEFAULT 1;
ALTER TABLE companies ADD COLUMN is_deleted INTEGER DEFAULT 0;
";

pub const ADD_SYNC_COLUMNS_COMPANY_CONTACTS: &str = "
ALTER TABLE company_contacts ADD COLUMN version INTEGER DEFAULT 1;
ALTER TABLE company_contacts ADD COLUMN is_deleted INTEGER DEFAULT 0;
";

pub const ADD_SYNC_COLUMNS_CUSTOMERS: &str = "
ALTER TABLE customers ADD COLUMN version INTEGER DEFAULT 1;
ALTER TABLE customers ADD COLUMN is_deleted INTEGER DEFAULT 0;
";

pub const ADD_SYNC_COLUMNS_PROPOSALS: &str = "
ALTER TABLE proposals ADD COLUMN version INTEGER DEFAULT 1;
ALTER TABLE proposals ADD COLUMN is_deleted INTEGER DEFAULT 0;
";

pub const ADD_SYNC_COLUMNS_PROPOSAL_PRODUCTS: &str = "
ALTER TABLE proposal_products ADD COLUMN version INTEGER DEFAULT 1;
ALTER TABLE proposal_products ADD COLUMN is_deleted INTEGER DEFAULT 0;
";

pub const ADD_SYNC_COLUMNS_INVOICES: &str = "
ALTER TABLE invoices ADD COLUMN version INTEGER DEFAULT 1;
ALTER TABLE invoices ADD COLUMN is_deleted INTEGER DEFAULT 0;
";

pub const ADD_SYNC_COLUMNS_DOCUMENTS: &str = "
ALTER TABLE documents ADD COLUMN version INTEGER DEFAULT 1;
ALTER TABLE documents ADD COLUMN is_deleted INTEGER DEFAULT 0;
";

// === INDEXES FOR SYNC ===
pub const CREATE_SYNC_INDEXES: &str = "
CREATE INDEX IF NOT EXISTS idx_companies_updated_at ON companies(updated_at);
CREATE INDEX IF NOT EXISTS idx_companies_is_deleted ON companies(is_deleted);
CREATE INDEX IF NOT EXISTS idx_company_contacts_updated_at ON company_contacts(updated_at);
CREATE INDEX IF NOT EXISTS idx_company_contacts_is_deleted ON company_contacts(is_deleted);
CREATE INDEX IF NOT EXISTS idx_customers_updated_at ON customers(updated_at);
CREATE INDEX IF NOT EXISTS idx_customers_is_deleted ON customers(is_deleted);
CREATE INDEX IF NOT EXISTS idx_proposals_updated_at ON proposals(updated_at);
CREATE INDEX IF NOT EXISTS idx_proposals_is_deleted ON proposals(is_deleted);
CREATE INDEX IF NOT EXISTS idx_proposal_products_updated_at ON proposal_products(updated_at);
CREATE INDEX IF NOT EXISTS idx_proposal_products_is_deleted ON proposal_products(is_deleted);
CREATE INDEX IF NOT EXISTS idx_invoices_updated_at ON invoices(updated_at);
CREATE INDEX IF NOT EXISTS idx_invoices_is_deleted ON invoices(is_deleted);
CREATE INDEX IF NOT EXISTS idx_documents_updated_at ON documents(updated_at);
CREATE INDEX IF NOT EXISTS idx_documents_is_deleted ON documents(is_deleted);
";

