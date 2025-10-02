use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDate};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Invoice {
    pub id: String,
    pub proposal_id: String,
    pub invoice_number: String,
    pub status: InvoiceStatus,
    pub total_amount: f64,
    pub currency: String,
    pub issue_date: String, // Format: YYYY-MM-DD
    pub due_date: Option<String>,
    pub paid_date: Option<String>,
    pub purchase_order: Option<String>,
    pub purchase_order_date: Option<String>,
    pub commercial_in_charge: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub sync_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum InvoiceStatus {
    #[serde(rename = "DRAFT")]
    Draft,
    #[serde(rename = "SENT")]
    Sent,
    #[serde(rename = "PAID")]
    Paid,
    #[serde(rename = "OVERDUE")]
    Overdue,
    #[serde(rename = "CANCELLED")]
    Cancelled,
}

impl std::fmt::Display for InvoiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvoiceStatus::Draft => write!(f, "DRAFT"),
            InvoiceStatus::Sent => write!(f, "SENT"),
            InvoiceStatus::Paid => write!(f, "PAID"),
            InvoiceStatus::Overdue => write!(f, "OVERDUE"),
            InvoiceStatus::Cancelled => write!(f, "CANCELLED"),
        }
    }
}

impl std::str::FromStr for InvoiceStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "DRAFT" => Ok(InvoiceStatus::Draft),
            "SENT" => Ok(InvoiceStatus::Sent),
            "PAID" => Ok(InvoiceStatus::Paid),
            "OVERDUE" => Ok(InvoiceStatus::Overdue),
            "CANCELLED" => Ok(InvoiceStatus::Cancelled),
            _ => Err(format!("Invalid invoice status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvoiceRequest {
    pub proposal_id: String,
    pub status: Option<String>,
    pub total_amount: Option<f64>,
    pub currency: Option<String>,
    pub issue_date: Option<String>,
    pub due_date: Option<String>,
    pub purchase_order: Option<String>,
    pub purchase_order_date: Option<String>,
    pub commercial_in_charge: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInvoiceRequest {
    pub status: Option<String>,
    pub total_amount: Option<f64>,
    pub currency: Option<String>,
    pub issue_date: Option<String>,
    pub due_date: Option<String>,
    pub paid_date: Option<String>,
    pub purchase_order: Option<String>,
    pub purchase_order_date: Option<String>,
    pub commercial_in_charge: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceWithDetails {
    pub id: String,
    pub proposal_id: String,
    pub proposal_number: String,
    pub company_name: String,
    pub invoice_number: String,
    pub status: InvoiceStatus,
    pub total_amount: f64,
    pub currency: String,
    pub issue_date: String,
    pub due_date: Option<String>,
    pub paid_date: Option<String>,
    pub purchase_order: Option<String>,
    pub purchase_order_date: Option<String>,
    pub commercial_in_charge: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub sync_status: String,
}

impl Invoice {
    pub fn new(proposal_id: String, invoice_number: String, total_amount: f64) -> Self {
        let now = chrono::Utc::now().timestamp_millis().to_string();
        let today = Utc::now().date_naive().format("%Y-%m-%d").to_string();
        
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            proposal_id,
            invoice_number,
            status: InvoiceStatus::Draft,
            total_amount,
            currency: "USD".to_string(),
            issue_date: today,
            due_date: None,
            paid_date: None,
            purchase_order: None,
            purchase_order_date: None,
            commercial_in_charge: None,
            notes: None,
            created_at: now.clone(),
            updated_at: now,
            sync_status: "pending".to_string(),
        }
    }

    pub fn generate_invoice_number() -> String {
        let now = Utc::now();
        let year = now.year() % 100;
        let month = now.month();
        
        // Format: YYMMXXXX (year, month, sequential number)
        // Pour l'instant, on utilise un timestamp pour l'unicité
        let timestamp = now.timestamp() % 10000;
        format!("{:02}{:02}{:04}", year, month, timestamp)
    }
}
